//! Native built-in Radicle issue and patch execution for validated forge plans.
//!
//! Planning and receipt decisions remain in `valence-core`; this module owns
//! one selected local Radicle Git repository and signed COB effects.

// r[impl valence.radicle_forge_ops.native_cobs]
// r[impl valence.radicle_forge_ops.shell]
// r[impl valence.radicle_forge_ops.boundaries]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::str::FromStr as _;

use radicle::cob::Label;
use radicle::cob::Title;
use radicle::cob::issue;
use radicle::cob::patch;
use radicle::git;
use radicle::identity::Did;
use radicle::identity::RepoId;
use radicle::storage::git::Repository;
use radicle_crypto::PublicKey;
use radicle_crypto::Signature;
use radicle_crypto::signature::Keypair;
use radicle_crypto::signature::Signer;
use radicle_crypto::signature::Verifier;
use radicle_crypto::ssh::ExtendedSignature;
use serde::Serialize;
use thiserror::Error;

use crate::model::ATTRIBUTION_HEADER;
use crate::model::ISSUE_COB_TYPE;
use crate::model::IssueSourceState;
use crate::model::MigrationDecision;
use crate::model::MigrationPlan;
use crate::model::ObservedRecordBinding;
use crate::model::PATCH_COB_TYPE;
use crate::model::PatchLifecycle;
use crate::model::PlannedOperation;
use crate::model::PlannedRecord;

#[derive(Debug, Error)]
pub enum NativeImportError {
    #[error("invalid native import input: {0}")]
    Input(String),
    #[error("native Radicle issue operation failed: {0}")]
    Issue(String),
    #[error("native Radicle patch operation failed: {0}")]
    Patch(String),
    #[error("selected Git repository operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("native import changed an unexpected ref: {0}")]
    RefConfinement(String),
}

pub trait NativeSigner:
    Keypair<VerifyingKey = PublicKey>
    + Signer<Signature>
    + Signer<ExtendedSignature>
    + Verifier<Signature>
    + radicle_crypto::Signer
{
}

impl<T> NativeSigner for T where T: Keypair<VerifyingKey = PublicKey>
        + Signer<Signature>
        + Signer<ExtendedSignature>
        + Verifier<Signature>
        + radicle_crypto::Signer
{
}

/// Open one explicitly selected Radicle storage repository.
///
/// # Errors
/// Returns an error for malformed RIDs or repositories that cannot be opened.
pub fn open_radicle_repository(path: &Path, rid: &str) -> Result<Repository, NativeImportError> {
    let rid = RepoId::from_str(rid).map_err(|error| NativeImportError::Input(error.to_string()))?;
    Repository::open(path, rid).map_err(|error| NativeImportError::Input(error.to_string()))
}

/// Execute a validated migration plan against one local repository.
///
/// The caller must keep preview as the default and call this only after explicit
/// execution authorization. Each built-in COB is reloaded and checked before an
/// observation is returned.
///
/// # Errors
/// Returns an error on signer mismatch, malformed plans, missing Git objects,
/// native evaluator failure, post-write mismatch, or ref escape.
pub fn execute_native_plan<G>(
    repository: &Repository,
    signer: &G,
    plan: &MigrationPlan,
) -> Result<Vec<ObservedRecordBinding>, NativeImportError>
where
    G: NativeSigner,
{
    crate::model::validate_plan(plan).map_err(NativeImportError::Input)?;
    let signer_did = Did::from(*signer.public_key()).to_string();
    if signer_did != plan.importer_did {
        return Err(NativeImportError::Input("native signer does not match the planned importer DID".to_owned()));
    }
    if repository.id.to_string() != plan.repository_rid {
        return Err(NativeImportError::Input("selected repository RID does not match the migration plan".to_owned()));
    }
    let before = reference_targets(repository)?;
    let mut observations = Vec::with_capacity(plan.records.len());
    let mut intended = BTreeSet::from([format!("refs/namespaces/{}/refs/rad/sigrefs", signer.public_key())]);
    for record in &plan.records {
        let observation = execute_record(repository, signer, record)?;
        intended.insert(format!(
            "refs/namespaces/{}/refs/cobs/{}/{}",
            signer.public_key(),
            observation.target_type,
            observation.target_object_id
        ));
        observations.push(observation);
    }
    let after = reference_targets(repository)?;
    for name in changed_refs(&before, &after) {
        if !intended.contains(&name) {
            return Err(NativeImportError::RefConfinement(name));
        }
    }
    observations.sort_by(|left, right| left.source_key.cmp(&right.source_key));
    Ok(observations)
}

fn execute_record<G>(
    repository: &Repository,
    signer: &G,
    record: &PlannedRecord,
) -> Result<ObservedRecordBinding, NativeImportError>
where
    G: NativeSigner,
{
    match &record.decision {
        MigrationDecision::VerifyExisting { target_object_id } => verify_existing(repository, record, target_object_id),
        MigrationDecision::Create if record.target_type == ISSUE_COB_TYPE => create_issue(repository, signer, record),
        MigrationDecision::Create if record.target_type == PATCH_COB_TYPE => create_patch(repository, signer, record),
        MigrationDecision::Create => {
            Err(NativeImportError::Input(format!("unsupported built-in target type {}", record.target_type)))
        }
    }
}

fn create_issue<G>(
    repository: &Repository,
    signer: &G,
    record: &PlannedRecord,
) -> Result<ObservedRecordBinding, NativeImportError>
where
    G: NativeSigner,
{
    let Some(PlannedOperation::CreateIssue {
        title,
        description,
        labels,
        state,
    }) = record.operations.first()
    else {
        return Err(NativeImportError::Input("issue plan does not start with create_issue".to_owned()));
    };
    let title = Title::new(title).map_err(|error| NativeImportError::Input(error.to_string()))?;
    let expected_title = title.to_string();
    let labels = labels
        .iter()
        .map(Label::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| NativeImportError::Input(error.to_string()))?;
    let mut issues =
        issue::Cache::no_cache(repository, signer).map_err(|error| NativeImportError::Issue(error.to_string()))?;
    let mut native = issues
        .create(title, description, &labels, &[], [])
        .map_err(|error| NativeImportError::Issue(error.to_string()))?;
    let target = *native.id();
    let mut entries = vec![target.to_string()];
    for operation in record.operations.iter().skip(1) {
        let PlannedOperation::AppendComment { body, .. } = operation else {
            return Err(NativeImportError::Input("issue plan contains a non-comment trailing operation".to_owned()));
        };
        let entry = native.comment(body, *target, []).map_err(|error| NativeImportError::Issue(error.to_string()))?;
        entries.push(entry.to_string());
    }
    let expected_state = match state {
        IssueSourceState::Open => issue::State::Open,
        IssueSourceState::Closed => issue::State::Closed {
            reason: issue::CloseReason::Other,
        },
        IssueSourceState::Solved => issue::State::Closed {
            reason: issue::CloseReason::Solved,
        },
    };
    if expected_state != issue::State::Open {
        let entry = native.lifecycle(expected_state).map_err(|error| NativeImportError::Issue(error.to_string()))?;
        entries.push(entry.to_string());
    }
    native.reload().map_err(|error| NativeImportError::Issue(error.to_string()))?;
    if native.title() != expected_title
        || native.description() != description
        || native.state() != &expected_state
        || native.labels().map(ToString::to_string).collect::<Vec<_>>()
            != labels.iter().map(ToString::to_string).collect::<Vec<_>>()
        || !native.description().contains(ATTRIBUTION_HEADER)
        || !native.description().contains(&record.source_key)
        || native.comments().count() != record.operations.len()
    {
        return Err(NativeImportError::Issue("post-write issue state does not match the plan".to_owned()));
    }
    let observed_state_blake3 = serialized_blake3(&*native)?;
    Ok(ObservedRecordBinding {
        source_key: record.source_key.clone(),
        source_record_blake3: record.source_record_blake3.clone(),
        target_type: ISSUE_COB_TYPE.to_owned(),
        target_object_id: target.to_string(),
        entry_oids: entries,
        observed_state_blake3,
    })
}

fn create_patch<G>(
    repository: &Repository,
    signer: &G,
    record: &PlannedRecord,
) -> Result<ObservedRecordBinding, NativeImportError>
where
    G: NativeSigner,
{
    let Some(PlannedOperation::CreatePatch {
        title,
        description,
        labels,
        lifecycle,
        base_oid,
        head_oid,
        target_ref,
    }) = record.operations.first()
    else {
        return Err(NativeImportError::Input("patch plan does not start with create_patch".to_owned()));
    };
    if target_ref != "refs/heads/main" {
        return Err(NativeImportError::Input("native patch target must be refs/heads/main".to_owned()));
    }
    let base = git::Oid::from_str(base_oid).map_err(|error| NativeImportError::Input(error.to_string()))?;
    let head = git::Oid::from_str(head_oid).map_err(|error| NativeImportError::Input(error.to_string()))?;
    require_commit(repository, &base)?;
    require_commit(repository, &head)?;
    if !repository.backend.graph_descendant_of(head.into(), base.into())? {
        return Err(NativeImportError::Input("planned patch head is not a descendant of its base".to_owned()));
    }
    let title = Title::new(title).map_err(|error| NativeImportError::Input(error.to_string()))?;
    let expected_title = title.to_string();
    let labels = labels
        .iter()
        .map(Label::new)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| NativeImportError::Input(error.to_string()))?;
    let mut patches =
        patch::Cache::no_cache(repository, signer).map_err(|error| NativeImportError::Patch(error.to_string()))?;
    let mut native = match lifecycle {
        PatchLifecycle::Draft => {
            patches.draft(title.clone(), description, patch::MergeTarget::Delegates, base, head, &labels)
        }
        PatchLifecycle::Open | PatchLifecycle::Archived => {
            patches.create(title.clone(), description, patch::MergeTarget::Delegates, base, head, &labels)
        }
    }
    .map_err(|error| NativeImportError::Patch(error.to_string()))?;
    let target = native.id;
    let revision = native.latest().0;
    let mut entries = vec![target.to_string()];
    for operation in record.operations.iter().skip(1) {
        let PlannedOperation::AppendComment { body, .. } = operation else {
            return Err(NativeImportError::Input("patch plan contains a non-comment trailing operation".to_owned()));
        };
        let entry = native
            .comment(revision, body, None, None, [])
            .map_err(|error| NativeImportError::Patch(error.to_string()))?;
        entries.push(entry.to_string());
    }
    if *lifecycle == PatchLifecycle::Archived {
        let entry = native
            .lifecycle(patch::Lifecycle::Archived)
            .map_err(|error| NativeImportError::Patch(error.to_string()))?;
        entries.push(entry.to_string());
    }
    native.reload().map_err(|error| NativeImportError::Patch(error.to_string()))?;
    let (observed_revision, latest) = native.latest();
    let state_matches = match lifecycle {
        PatchLifecycle::Open => native.is_open(),
        PatchLifecycle::Draft => native.is_draft(),
        PatchLifecycle::Archived => native.is_archived(),
    };
    if native.title() != expected_title
        || native.description() != description
        || native.target() != patch::MergeTarget::Delegates
        || !state_matches
        || observed_revision != revision
        || *latest.base() != base
        || latest.head() != head
        || latest.discussion().comments().count() != record.operations.len().saturating_sub(1)
        || !native.description().contains(ATTRIBUTION_HEADER)
        || !native.description().contains(&record.source_key)
    {
        return Err(NativeImportError::Patch("post-write patch state does not match the plan".to_owned()));
    }
    let observed_state_blake3 = serialized_blake3(&*native)?;
    Ok(ObservedRecordBinding {
        source_key: record.source_key.clone(),
        source_record_blake3: record.source_record_blake3.clone(),
        target_type: PATCH_COB_TYPE.to_owned(),
        target_object_id: target.to_string(),
        entry_oids: entries,
        observed_state_blake3,
    })
}

fn verify_existing(
    repository: &Repository,
    record: &PlannedRecord,
    target_object_id: &str,
) -> Result<ObservedRecordBinding, NativeImportError> {
    let object_id = radicle_cob::ObjectId::from_str(target_object_id)
        .map_err(|error| NativeImportError::Input(error.to_string()))?;
    let observed_state_blake3 = match record.target_type.as_str() {
        ISSUE_COB_TYPE => {
            let issues = issue::Issues::open(repository, radicle::cob::store::access::ReadOnly)
                .map_err(|error| NativeImportError::Issue(error.to_string()))?;
            let native = issues
                .get(&object_id)
                .map_err(|error| NativeImportError::Issue(error.to_string()))?
                .ok_or_else(|| NativeImportError::Issue("mapped issue does not exist".to_owned()))?;
            if !native.description().contains(ATTRIBUTION_HEADER) || !native.description().contains(&record.source_key)
            {
                return Err(NativeImportError::Issue(
                    "mapped issue does not contain the expected source marker".to_owned(),
                ));
            }
            serialized_blake3(&native)?
        }
        PATCH_COB_TYPE => {
            let patches = patch::Patches::open(repository, radicle::cob::store::access::ReadOnly)
                .map_err(|error| NativeImportError::Patch(error.to_string()))?;
            let native = patches
                .get(&object_id)
                .map_err(|error| NativeImportError::Patch(error.to_string()))?
                .ok_or_else(|| NativeImportError::Patch("mapped patch does not exist".to_owned()))?;
            if !native.description().contains(ATTRIBUTION_HEADER) || !native.description().contains(&record.source_key)
            {
                return Err(NativeImportError::Patch(
                    "mapped patch does not contain the expected source marker".to_owned(),
                ));
            }
            serialized_blake3(&native)?
        }
        other => {
            return Err(NativeImportError::Input(format!("unsupported mapped target type {other}")));
        }
    };
    Ok(ObservedRecordBinding {
        source_key: record.source_key.clone(),
        source_record_blake3: record.source_record_blake3.clone(),
        target_type: record.target_type.clone(),
        target_object_id: target_object_id.to_owned(),
        entry_oids: vec![target_object_id.to_owned()],
        observed_state_blake3,
    })
}

fn require_commit(repository: &Repository, oid: &git::Oid) -> Result<(), NativeImportError> {
    repository
        .backend
        .find_commit((*oid).into())
        .map(|_| ())
        .map_err(|_| NativeImportError::Input(format!("planned Git commit is absent: {oid}")))
}

fn serialized_blake3(value: &impl Serialize) -> Result<String, NativeImportError> {
    let bytes = serde_json::to_vec(value).map_err(|error| NativeImportError::Input(error.to_string()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn reference_targets(repository: &Repository) -> Result<BTreeMap<String, String>, NativeImportError> {
    let mut targets = BTreeMap::new();
    for reference in repository.backend.references()? {
        let reference = reference?;
        let (Some(name), Some(target)) = (reference.name(), reference.target()) else {
            continue;
        };
        targets.insert(name.to_owned(), target.to_string());
    }
    Ok(targets)
}

fn changed_refs(before: &BTreeMap<String, String>, after: &BTreeMap<String, String>) -> Vec<String> {
    let names = before.keys().chain(after.keys()).collect::<BTreeSet<_>>();
    names.into_iter().filter(|name| before.get(*name) != after.get(*name)).cloned().collect()
}

#[cfg(test)]
mod tests {
    // r[verify valence.radicle_forge_ops.native_cobs]
    // r[verify valence.radicle_forge_ops.validation]

    use super::*;
    use crate::model::ADMISSION_NON_CLAIM;
    use crate::model::MIGRATION_NON_CLAIM;
    use crate::model::PLAN_SCHEMA;
    use crate::model::REVIEW_NON_CLAIM;
    use crate::model::expected_plan_blake3;

    const DIGEST_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const DIGEST_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn plan_for(repository: &Repository, importer_did: String, records: Vec<PlannedRecord>) -> MigrationPlan {
        let mut plan = MigrationPlan {
            schema: PLAN_SCHEMA.to_owned(),
            profile_blake3: DIGEST_A.to_owned(),
            source_batch_blake3: DIGEST_B.to_owned(),
            plan_blake3: DIGEST_A.to_owned(),
            source_system: "github".to_owned(),
            repository_url: "https://github.com/onix/repo".to_owned(),
            repository_rid: repository.id.to_string(),
            importer_did,
            records,
            required_non_claims: vec![
                MIGRATION_NON_CLAIM.to_owned(),
                REVIEW_NON_CLAIM.to_owned(),
                ADMISSION_NON_CLAIM.to_owned(),
            ],
        };
        plan.plan_blake3 = expected_plan_blake3(&plan).expect("plan identity");
        plan
    }

    fn issue_record() -> PlannedRecord {
        PlannedRecord {
            source_key: "github:onix/repo:issue:1".to_owned(),
            source_record_blake3: DIGEST_A.to_owned(),
            target_type: ISSUE_COB_TYPE.to_owned(),
            decision: MigrationDecision::Create,
            operations: vec![
                PlannedOperation::CreateIssue {
                    title: "Imported issue".to_owned(),
                    description: format!("{ATTRIBUTION_HEADER}\nsource-key: github:onix/repo:issue:1\n\n---\n\nBody"),
                    labels: vec!["source-bug".to_owned()],
                    state: IssueSourceState::Solved,
                },
                PlannedOperation::AppendComment {
                    event_id: "comment-1".to_owned(),
                    body: format!(
                        "{ATTRIBUTION_HEADER}\nsource-key: github:onix/repo:issue:1\nsource-event-id: comment-1\n\n---\n\nComment"
                    ),
                },
            ],
        }
    }

    #[test]
    fn creates_and_reloads_native_issue_without_ref_escape() {
        let fixture = radicle::test::setup::NodeWithRepo::default();
        let importer = Did::from(*fixture.signer.public_key()).to_string();
        let plan = plan_for(&fixture.repo, importer, vec![issue_record()]);
        let observed = execute_native_plan(&fixture.repo, &fixture.signer, &plan).expect("native issue");
        assert_eq!(observed.len(), 1);
        assert_eq!(observed[0].target_type, ISSUE_COB_TYPE);
        assert!(observed[0].entry_oids.len() > 1);

        let object = radicle_cob::ObjectId::from_str(&observed[0].target_object_id).expect("object id");
        let issues = issue::Issues::open(&*fixture.repo, radicle::cob::store::access::ReadOnly).expect("issue store");
        let native = issues.get(&object).expect("get issue").expect("issue exists");
        assert_eq!(native.title(), "Imported issue");
        assert_eq!(native.comments().count(), 2);
        assert_eq!(native.state(), &issue::State::Closed {
            reason: issue::CloseReason::Solved
        });
    }

    #[test]
    fn creates_archived_native_patch_and_keeps_imported_review_as_comment() {
        let fixture = radicle::test::setup::NodeWithRepo::default();
        let checkout = fixture.repo.checkout();
        let branch = checkout.branch_with([("README", b"imported patch")]);
        let importer = Did::from(*fixture.signer.public_key()).to_string();
        let record = PlannedRecord {
            source_key: "github:onix/repo:pull_request:2".to_owned(),
            source_record_blake3: DIGEST_B.to_owned(),
            target_type: PATCH_COB_TYPE.to_owned(),
            decision: MigrationDecision::Create,
            operations: vec![
                PlannedOperation::CreatePatch {
                    title: "Imported patch".to_owned(),
                    description: format!(
                        "{ATTRIBUTION_HEADER}\nsource-key: github:onix/repo:pull_request:2\n\n---\n\nBody"
                    ),
                    labels: vec!["source-enhancement".to_owned()],
                    lifecycle: PatchLifecycle::Archived,
                    base_oid: branch.base.to_string(),
                    head_oid: branch.oid.to_string(),
                    target_ref: "refs/heads/main".to_owned(),
                },
                PlannedOperation::AppendComment {
                    event_id: "review-1".to_owned(),
                    body: format!(
                        "{ATTRIBUTION_HEADER}\nsource-key: github:onix/repo:pull_request:2\nsource-event-kind: review\nsource-review-state: approved\n\n---\n\nLooks good"
                    ),
                },
            ],
        };
        let plan = plan_for(&fixture.repo, importer, vec![record]);
        let observed = execute_native_plan(&fixture.repo, &fixture.signer, &plan).expect("native patch");
        assert_eq!(observed[0].target_type, PATCH_COB_TYPE);

        let object = radicle_cob::ObjectId::from_str(&observed[0].target_object_id).expect("object id");
        let patches = patch::Patches::open(&*fixture.repo, radicle::cob::store::access::ReadOnly).expect("patch store");
        let native = patches.get(&object).expect("get patch").expect("patch exists");
        assert!(native.is_archived());
        assert_eq!(native.latest().1.discussion().comments().count(), 1);
        assert_eq!(native.latest().1.reviews().count(), 0);
    }

    #[test]
    fn rejects_signer_mismatch_missing_commit_and_wrong_operation_shape() {
        let fixture = radicle::test::setup::NodeWithRepo::default();
        let other = radicle::test::setup::NodeWithRepo::default();
        let plan = plan_for(&fixture.repo, Did::from(*other.signer.public_key()).to_string(), vec![issue_record()]);
        assert!(matches!(
            execute_native_plan(&fixture.repo, &fixture.signer, &plan),
            Err(NativeImportError::Input(_))
        ));

        let importer = Did::from(*fixture.signer.public_key()).to_string();
        let mut wrong = issue_record();
        wrong.target_type = PATCH_COB_TYPE.to_owned();
        let plan = plan_for(&fixture.repo, importer.clone(), vec![wrong]);
        assert!(matches!(
            execute_native_plan(&fixture.repo, &fixture.signer, &plan),
            Err(NativeImportError::Input(_))
        ));

        let missing_patch = PlannedRecord {
            source_key: "github:onix/repo:pull_request:missing".to_owned(),
            source_record_blake3: DIGEST_A.to_owned(),
            target_type: PATCH_COB_TYPE.to_owned(),
            decision: MigrationDecision::Create,
            operations: vec![PlannedOperation::CreatePatch {
                title: "Missing objects".to_owned(),
                description: format!("{ATTRIBUTION_HEADER}\nsource-key: github:onix/repo:pull_request:missing"),
                labels: Vec::new(),
                lifecycle: PatchLifecycle::Open,
                base_oid: "1111111111111111111111111111111111111111".to_owned(),
                head_oid: "2222222222222222222222222222222222222222".to_owned(),
                target_ref: "refs/heads/main".to_owned(),
            }],
        };
        let plan = plan_for(&fixture.repo, importer.clone(), vec![missing_patch]);
        assert!(matches!(
            execute_native_plan(&fixture.repo, &fixture.signer, &plan),
            Err(NativeImportError::Input(_))
        ));

        let missing_existing = PlannedRecord {
            source_key: "github:onix/repo:issue:missing".to_owned(),
            source_record_blake3: DIGEST_A.to_owned(),
            target_type: ISSUE_COB_TYPE.to_owned(),
            decision: MigrationDecision::VerifyExisting {
                target_object_id: "3333333333333333333333333333333333333333".to_owned(),
            },
            operations: vec![PlannedOperation::VerifyExisting {
                target_object_id: "3333333333333333333333333333333333333333".to_owned(),
            }],
        };
        let plan = plan_for(&fixture.repo, importer, vec![missing_existing]);
        assert!(matches!(
            execute_native_plan(&fixture.repo, &fixture.signer, &plan),
            Err(NativeImportError::Issue(_))
        ));
    }

    #[test]
    fn changed_ref_detection_exposes_unintended_mutation() {
        let before = BTreeMap::from([
            ("refs/heads/main".to_owned(), "1111111111111111111111111111111111111111".to_owned()),
            (
                "refs/namespaces/actor/refs/rad/sigrefs".to_owned(),
                "2222222222222222222222222222222222222222".to_owned(),
            ),
        ]);
        let after = BTreeMap::from([
            ("refs/heads/main".to_owned(), "3333333333333333333333333333333333333333".to_owned()),
            (
                "refs/namespaces/actor/refs/rad/sigrefs".to_owned(),
                "2222222222222222222222222222222222222222".to_owned(),
            ),
        ]);
        assert_eq!(changed_refs(&before, &after), vec!["refs/heads/main"]);
    }
}
