//! Local imperative adapter between Valence Evidence IR and signed Radicle COB history.
//!
//! `valence-core::cob_evidence` remains the deterministic authority for identity,
//! authorization input, reduction, role, and receipt decisions. This crate owns
//! selected-repository access, `radicle-cob`, Git object measurement, signing,
//! special-ref mutation, and operator-facing plans.
//!
//! r[impl valence.radicle_cob_evidence.adapter_boundary]
//! r[impl valence.radicle_cob_evidence.repository_safety]

mod artifact_auth;
mod store;
mod transport;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::str::FromStr as _;

pub use artifact_auth::*;
use nonempty::NonEmpty;
use radicle_cob::Create;
use radicle_cob::Embed;
use radicle_cob::ObjectId;
use radicle_cob::TypeName;
use radicle_cob::Update;
use radicle_cob::Version;
use radicle_cob::change::Storage as _;
use radicle_cob::object::Storage as _;
use radicle_cob::signatures::ExtendedSignature;
use radicle_crypto::PublicKey;
use radicle_crypto::Signer;
use radicle_oid::Oid;
use serde::Deserialize;
use serde::Serialize;
pub use store::GitCobStore;
pub use store::StoreOpenError;
use thiserror::Error;
pub use transport::PhysicalSidecar;
pub use transport::TransportObservedEvent;
use valence_core::cob_evidence::ActorBinding;
use valence_core::cob_evidence::AttachmentPayload;
use valence_core::cob_evidence::AttachmentRelationship;
use valence_core::cob_evidence::BindingReceipt;
use valence_core::cob_evidence::CobAuthorizationProfile;
use valence_core::cob_evidence::CobEvent;
use valence_core::cob_evidence::CobEventAction;
use valence_core::cob_evidence::CobEvidenceAttachment;
use valence_core::cob_evidence::CobEvidenceProfile;
use valence_core::cob_evidence::CobOperation;
use valence_core::cob_evidence::CobReductionInput;
use valence_core::cob_evidence::CobReductionReport;
use valence_core::cob_evidence::CobSnapshot;
use valence_core::cob_evidence::CobTargetRef;
use valence_core::cob_evidence::ConfidentialityClass;
use valence_core::cob_evidence::EVENT_SCHEMA;
use valence_core::cob_evidence::EvidenceReference;
use valence_core::cob_evidence::ObservedTargetState;
use valence_core::cob_evidence::ROOT_SCHEMA;
use valence_core::cob_evidence::RevokeAction;
use valence_core::cob_evidence::SidecarObservation;
use valence_core::cob_evidence::SidecarRoot;
use valence_core::cob_evidence::SupersedeAction;
use valence_core::cob_evidence::actor_is_authorized;
use valence_core::cob_evidence::build_attachment;
use valence_core::cob_evidence::build_binding_receipt;
use valence_core::cob_evidence::build_observed_target_state;
use valence_core::cob_evidence::profile_identity;
use valence_core::cob_evidence::reduce_logical_sidecar;
use valence_core::cob_evidence::snapshot_evidence_artifact;
use valence_core::cob_evidence::target_key;
use valence_core::cob_evidence::validate_authorization_profile;
use valence_core::cob_evidence::validate_event;
use valence_core::cob_evidence::validate_profile;
use valence_core::cob_evidence::verify_binding_receipt;
use valence_core::cob_evidence::verify_snapshot;
use valence_core::evidence_ir::EvidenceArtifact;
use valence_core::evidence_ir::VerificationRole;
use valence_core::evidence_ir::check_evidence_artifact;

pub const MUTATION_REQUEST_SCHEMA: &str = "valence.radicle-cob-evidence.mutation-request.v1";
pub const MUTATION_PLAN_SCHEMA: &str = "valence.radicle-cob-evidence.mutation-plan.v1";
pub const MUTATION_RESULT_SCHEMA: &str = "valence.radicle-cob-evidence.mutation-result.v1";
const CREATE_REF_PLACEHOLDER: &str = "<new-object-id>";
const DEFAULT_EMBED_NAME: &str = "evidence.json";
const JSON_MEDIA_TYPE: &str = "application/json";
const MUTATION_MESSAGE: &str = "Record Valence evidence linkage";

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("artifact access failed: {0}")]
    ArtifactIo(#[from] std::io::Error),
    #[error("invalid adapter input: {0}")]
    Input(String),
    #[error("Git operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("Radicle COB operation failed: {0}")]
    Cob(String),
    #[error("JSON operation failed: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedPayloadMode {
    ReferenceOnly,
    Embedded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttachRequest {
    pub relationship: AttachmentRelationship,
    pub artifact_locator: String,
    pub payload_mode: RequestedPayloadMode,
    pub confidentiality: ConfidentialityClass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupersedeRequest {
    pub attachment_id: String,
    pub replacement_attachment_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevokeRequest {
    pub attachment_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationRequestAction {
    Attach(AttachRequest),
    Supersede(SupersedeRequest),
    Revoke(RevokeRequest),
}

impl MutationRequestAction {
    #[must_use]
    pub const fn operation(&self) -> CobOperation {
        match self {
            Self::Attach(_) => CobOperation::Attach,
            Self::Supersede(_) => CobOperation::Supersede,
            Self::Revoke(_) => CobOperation::Revoke,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationRequest {
    pub schema: String,
    pub target: CobTargetRef,
    pub actor: String,
    pub sidecar_object_id: Option<String>,
    pub action: MutationRequestAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationPlan {
    pub schema: String,
    pub valid: bool,
    pub issues: Vec<String>,
    pub mutated: bool,
    pub actor: String,
    pub operation: CobOperation,
    pub target: CobTargetRef,
    pub target_key: String,
    pub profile_hash: String,
    pub sidecar_candidates: Vec<String>,
    pub selected_sidecar_object_id: Option<String>,
    pub intended_ref: String,
    pub attachment_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationResult {
    pub schema: String,
    pub plan: MutationPlan,
    pub mutated: bool,
    pub sidecar_object_id: String,
    pub entry_oid: String,
    pub updated_ref: String,
    pub binding_receipt: Option<BindingReceipt>,
}

#[derive(Debug, Serialize)]
pub struct SnapshotMaterialization {
    pub report: CobReductionReport,
    pub snapshot_artifact: Option<EvidenceArtifact>,
    pub adapter_issues: Vec<String>,
}

#[derive(Debug)]
pub struct PreparedMutation {
    pub plan: MutationPlan,
    request: MutationRequest,
    event: Option<CobEvent>,
    attachment: Option<CobEvidenceAttachment>,
    embed: Option<PreparedEmbed>,
}

#[derive(Debug)]
struct PreparedEmbed {
    name: String,
    bytes: Vec<u8>,
    git_oid: String,
}

#[derive(Debug)]
struct LoadedSidecar {
    object_id: String,
    tips: Vec<String>,
    events: Vec<TransportObservedEvent>,
}

#[derive(Debug)]
struct PreparedAction {
    event: CobEvent,
    attachment: Option<CobEvidenceAttachment>,
    embed: Option<PreparedEmbed>,
}

pub fn parse_profile(bytes: &[u8]) -> Result<CobEvidenceProfile, AdapterError> {
    serde_json::from_slice(bytes).map_err(AdapterError::from)
}

pub fn parse_authorization(bytes: &[u8]) -> Result<CobAuthorizationProfile, AdapterError> {
    serde_json::from_slice(bytes).map_err(AdapterError::from)
}

pub fn parse_mutation_request(bytes: &[u8]) -> Result<MutationRequest, AdapterError> {
    serde_json::from_slice(bytes).map_err(AdapterError::from)
}

#[must_use]
pub fn build_authorization_for_actor(
    profile: &CobEvidenceProfile,
    actor: String,
    role: String,
) -> CobAuthorizationProfile {
    valence_core::cob_evidence::build_authorization_profile(&profile_identity(profile), vec![ActorBinding {
        signer: actor,
        role,
    }])
}

pub fn prepare_mutation(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    authorization: &CobAuthorizationProfile,
    artifact_root: &cap_std::fs::Dir,
    request: MutationRequest,
) -> Result<PreparedMutation, AdapterError> {
    let mut issues = validate_profile(profile)
        .into_iter()
        .chain(validate_authorization_profile(profile, authorization))
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    if request.schema != MUTATION_REQUEST_SCHEMA {
        issues.push("unsupported_mutation_request_schema".to_owned());
    }
    let actor = PublicKey::from_str(&request.actor).map_err(|error| AdapterError::Input(error.to_string()));
    if actor.is_err() {
        issues.push("invalid_actor_public_key".to_owned());
    }
    if !actor_is_authorized(profile, authorization, &request.actor, request.action.operation()) {
        issues.push("operation_unauthorized".to_owned());
    }
    let target_key = target_key(&request.target).unwrap_or_default();
    if target_key.is_empty() {
        issues.push("invalid_target".to_owned());
    }
    let observed_target_state = match load_target_state(store, &request.target) {
        Ok(state) => Some(state),
        Err(_) => {
            issues.push("target_not_found_or_invalid".to_owned());
            None
        }
    };
    let sidecars = load_target_sidecars(store, profile, &request.target)?;
    let sidecar_candidates = sidecars.iter().map(|sidecar| sidecar.object_id.clone()).collect::<Vec<_>>();
    let selected = select_sidecar(&sidecar_candidates, request.sidecar_object_id.as_deref(), &mut issues);
    validate_mutation_predecessors(
        store,
        profile,
        authorization,
        artifact_root,
        &request,
        selected.as_deref(),
        &sidecars,
        &mut issues,
    )?;
    let prepared_action =
        prepare_action(profile, artifact_root, &request, observed_target_state.as_ref(), &mut issues)?;
    if let Some(event) = prepared_action.as_ref().map(|action| &action.event) {
        issues.extend(validate_event(profile, event).into_iter().map(|issue| issue.code));
    }
    issues.sort();
    issues.dedup();
    let intended_ref = match (&actor, &selected) {
        (Ok(actor), Some(object_id)) => intended_ref(profile, actor, object_id)?,
        (Ok(actor), None) if sidecar_candidates.is_empty() => {
            format!("refs/rad/{actor}/cobs/{}/{CREATE_REF_PLACEHOLDER}", profile.cob_type)
        }
        _ => String::new(),
    };
    let attachment_id = prepared_action
        .as_ref()
        .and_then(|action| action.attachment.as_ref())
        .map(|attachment| attachment.attachment_id.clone());
    let plan = MutationPlan {
        schema: MUTATION_PLAN_SCHEMA.to_owned(),
        valid: issues.is_empty() && prepared_action.is_some(),
        issues,
        mutated: false,
        actor: request.actor.clone(),
        operation: request.action.operation(),
        target: request.target.clone(),
        target_key,
        profile_hash: profile_identity(profile),
        sidecar_candidates,
        selected_sidecar_object_id: selected,
        intended_ref,
        attachment_id,
    };
    let (event, attachment, embed) =
        prepared_action.map_or((None, None, None), |action| (Some(action.event), action.attachment, action.embed));
    Ok(PreparedMutation {
        plan,
        request,
        event,
        attachment,
        embed,
    })
}

pub fn execute_mutation<G>(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    signer: &G,
    prepared: PreparedMutation,
) -> Result<MutationResult, AdapterError>
where
    G: radicle_crypto::signature::Signer<ExtendedSignature> + Signer,
{
    if !prepared.plan.valid {
        return Err(AdapterError::Input("mutation plan is invalid".to_owned()));
    }
    if signer.public_key().to_string() != prepared.request.actor {
        return Err(AdapterError::Input("signer does not match planned actor".to_owned()));
    }
    let event = prepared
        .event
        .as_ref()
        .ok_or_else(|| AdapterError::Input("validated mutation lacks event".to_owned()))?;
    let event_bytes = serde_json::to_vec(event)?;
    let embeds = store_embeds(store, prepared.embed.as_ref())?;
    let type_name = TypeName::from_str(&profile.cob_type).map_err(|error| AdapterError::Input(error.to_string()))?;
    let target_oid =
        Oid::from_str(&prepared.request.target.object_id).map_err(|error| AdapterError::Input(error.to_string()))?;
    let (sidecar_object_id, entry_oid) = match &prepared.plan.selected_sidecar_object_id {
        Some(object_id) => {
            let object_id = ObjectId::from_str(object_id).map_err(|error| AdapterError::Input(error.to_string()))?;
            let updated = radicle_cob::update::<PhysicalSidecar, _, _>(
                store,
                signer,
                None,
                vec![target_oid],
                signer.public_key(),
                Update {
                    changes: nonempty(vec![event_bytes])?,
                    object_id,
                    type_name: type_name.clone(),
                    message: MUTATION_MESSAGE.to_owned(),
                    embeds,
                },
            )
            .map_err(|error| AdapterError::Cob(error.to_string()))?;
            (object_id.to_string(), updated.head.to_string())
        }
        None => {
            let root_event = root_event(profile, &prepared.request.target)?;
            let root_bytes = serde_json::to_vec(&root_event)?;
            let created = radicle_cob::create::<PhysicalSidecar, _, _>(
                store,
                signer,
                None,
                vec![target_oid],
                signer.public_key(),
                Create {
                    contents: nonempty(vec![root_bytes, event_bytes])?,
                    type_name: type_name.clone(),
                    message: MUTATION_MESSAGE.to_owned(),
                    embeds,
                    version: Version::default(),
                },
            )
            .map_err(|error| AdapterError::Cob(error.to_string()))?;
            (created.id.to_string(), created.id.to_string())
        }
    };
    verify_written_entry(store, &entry_oid, event)?;
    let binding_receipt = prepared
        .attachment
        .as_ref()
        .map(|attachment| {
            let embed_oid = prepared.embed.as_ref().map(|embed| embed.git_oid.clone());
            build_binding_receipt(profile, attachment, &entry_oid, embed_oid)
                .map_err(|issues| AdapterError::Input(issue_codes(&issues).join(",")))
        })
        .transpose()?;
    let updated_ref = intended_ref(profile, signer.public_key(), &sidecar_object_id)?;
    let mut executed_plan = prepared.plan;
    executed_plan.mutated = true;
    executed_plan.selected_sidecar_object_id = Some(sidecar_object_id.clone());
    executed_plan.intended_ref = updated_ref.clone();
    Ok(MutationResult {
        schema: MUTATION_RESULT_SCHEMA.to_owned(),
        plan: executed_plan,
        mutated: true,
        sidecar_object_id,
        entry_oid,
        updated_ref,
        binding_receipt,
    })
}

pub fn materialize_snapshot(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    authorization: &CobAuthorizationProfile,
    artifact_root: &cap_std::fs::Dir,
    target: CobTargetRef,
) -> Result<SnapshotMaterialization, AdapterError> {
    let current_target_state = load_target_state(store, &target)?;
    let loaded = load_target_sidecars(store, profile, &target)?;
    let sidecars = loaded
        .iter()
        .map(|sidecar| SidecarObservation {
            object_id: sidecar.object_id.clone(),
            tips: sidecar.tips.clone(),
        })
        .collect::<Vec<_>>();
    let events = loaded
        .iter()
        .flat_map(|sidecar| sidecar.events.iter().map(|event| event.observed.clone()))
        .collect::<Vec<_>>();
    let (resolutions, mut adapter_issues) = resolve_attachments(store, artifact_root, &loaded);
    let report = reduce_logical_sidecar(&CobReductionInput {
        profile: profile.clone(),
        authorization: authorization.clone(),
        target,
        current_target_state,
        sidecars,
        events,
        resolutions,
    });
    let snapshot_artifact = report
        .snapshot
        .as_ref()
        .map(|snapshot| snapshot_evidence_artifact(profile, authorization, snapshot))
        .transpose()
        .map_err(|issues| AdapterError::Input(issue_codes(&issues).join(",")))?;
    adapter_issues.sort();
    adapter_issues.dedup();
    Ok(SnapshotMaterialization {
        report,
        snapshot_artifact,
        adapter_issues,
    })
}

#[must_use]
pub fn verify_saved_binding(profile: &CobEvidenceProfile, receipt: &BindingReceipt) -> Vec<String> {
    issue_codes(&verify_binding_receipt(profile, receipt))
}

#[must_use]
pub fn verify_saved_snapshot(
    profile: &CobEvidenceProfile,
    authorization: &CobAuthorizationProfile,
    snapshot: &CobSnapshot,
) -> Vec<String> {
    issue_codes(&verify_snapshot(profile, authorization, snapshot))
}

fn prepare_action(
    profile: &CobEvidenceProfile,
    artifact_root: &cap_std::fs::Dir,
    request: &MutationRequest,
    observed_target_state: Option<&ObservedTargetState>,
    issues: &mut Vec<String>,
) -> Result<Option<PreparedAction>, AdapterError> {
    match &request.action {
        MutationRequestAction::Attach(attach) => {
            prepare_attach(profile, artifact_root, request, attach, observed_target_state, issues)
        }
        MutationRequestAction::Supersede(supersede) => Ok(Some(PreparedAction {
            event: CobEvent {
                schema: EVENT_SCHEMA.to_owned(),
                action: CobEventAction::Supersede(SupersedeAction {
                    profile_hash: profile_identity(profile),
                    attachment_id: supersede.attachment_id.clone(),
                    replacement_attachment_id: supersede.replacement_attachment_id.clone(),
                    verification_role: VerificationRole::RecordedOnly,
                    non_claims: profile.required_non_claims.clone(),
                }),
            },
            attachment: None,
            embed: None,
        })),
        MutationRequestAction::Revoke(revoke) => Ok(Some(PreparedAction {
            event: CobEvent {
                schema: EVENT_SCHEMA.to_owned(),
                action: CobEventAction::Revoke(RevokeAction {
                    profile_hash: profile_identity(profile),
                    attachment_id: revoke.attachment_id.clone(),
                    reason: revoke.reason.clone(),
                    verification_role: VerificationRole::RecordedOnly,
                    non_claims: profile.required_non_claims.clone(),
                }),
            },
            attachment: None,
            embed: None,
        })),
    }
}

fn prepare_attach(
    profile: &CobEvidenceProfile,
    artifact_root: &cap_std::fs::Dir,
    request: &MutationRequest,
    attach: &AttachRequest,
    observed_target_state: Option<&ObservedTargetState>,
    issues: &mut Vec<String>,
) -> Result<Option<PreparedAction>, AdapterError> {
    let bytes = match artifact_root.read(&attach.artifact_locator) {
        Ok(bytes) => bytes,
        Err(error) => {
            issues.push(format!("artifact_read_failed:{error}"));
            return Ok(None);
        }
    };
    let artifact = match serde_json::from_slice::<EvidenceArtifact>(&bytes) {
        Ok(artifact) => artifact,
        Err(error) => {
            issues.push(format!("artifact_json_invalid:{error}"));
            return Ok(None);
        }
    };
    let report =
        check_evidence_artifact(&serde_json::to_string(&artifact).expect("typed evidence serialization is infallible"));
    if !report.valid {
        issues.push(format!("artifact_invalid:{}", report.issue_codes.join(",")));
        return Ok(None);
    }
    let size_bytes = u64::try_from(bytes.len()).map_err(|error| AdapterError::Input(error.to_string()))?;
    let raw_blake3 = format!("b3:{}", blake3::hash(&bytes).to_hex());
    let (payload, embed) = match attach.payload_mode {
        RequestedPayloadMode::ReferenceOnly => (
            AttachmentPayload::ReferenceOnly {
                locator: attach.artifact_locator.clone(),
                media_type: JSON_MEDIA_TYPE.to_owned(),
                size_bytes,
                raw_blake3,
            },
            None,
        ),
        RequestedPayloadMode::Embedded => {
            let git_oid = git2::Oid::hash_object(git2::ObjectType::Blob, &bytes)?.to_string();
            (
                AttachmentPayload::Embedded {
                    name: DEFAULT_EMBED_NAME.to_owned(),
                    git_oid: git_oid.clone(),
                    media_type: JSON_MEDIA_TYPE.to_owned(),
                    size_bytes,
                    raw_blake3,
                },
                Some(PreparedEmbed {
                    name: DEFAULT_EMBED_NAME.to_owned(),
                    bytes,
                    git_oid,
                }),
            )
        }
    };
    let Some(observed_target_state) = observed_target_state.cloned() else {
        return Ok(None);
    };
    let attachment = match build_attachment(
        profile,
        &request.target,
        attach.relationship,
        EvidenceReference::from_artifact(&artifact),
        observed_target_state,
        payload,
        attach.confidentiality,
        profile.required_non_claims.clone(),
    ) {
        Ok(attachment) => attachment,
        Err(attachment_issues) => {
            issues.extend(issue_codes(&attachment_issues));
            return Ok(None);
        }
    };
    Ok(Some(PreparedAction {
        event: CobEvent {
            schema: EVENT_SCHEMA.to_owned(),
            action: CobEventAction::Attach(attachment.clone()),
        },
        attachment: Some(attachment),
        embed,
    }))
}

fn load_target_state(store: &GitCobStore, target: &CobTargetRef) -> Result<ObservedTargetState, AdapterError> {
    let type_name = TypeName::from_str(&target.type_name).map_err(|error| AdapterError::Input(error.to_string()))?;
    let object_id = ObjectId::from_str(&target.object_id).map_err(|error| AdapterError::Input(error.to_string()))?;
    let objects = store.objects(&type_name, &object_id).map_err(|error| AdapterError::Cob(error.to_string()))?;
    let tips = objects.iter().map(|reference| reference.target.id.to_string()).collect::<Vec<_>>();
    build_observed_target_state(target, &tips).map_err(|issues| AdapterError::Input(issue_codes(&issues).join(",")))
}

fn load_target_sidecars(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    target: &CobTargetRef,
) -> Result<Vec<LoadedSidecar>, AdapterError> {
    let expected_target_key =
        target_key(target).map_err(|issues| AdapterError::Input(issue_codes(&issues).join(",")))?;
    let type_name = TypeName::from_str(&profile.cob_type).map_err(|error| AdapterError::Input(error.to_string()))?;
    let objects = radicle_cob::list::<PhysicalSidecar, _>(store, &type_name)
        .map_err(|error| AdapterError::Cob(error.to_string()))?;
    let mut loaded = objects
        .into_iter()
        .filter_map(|object| {
            let object_id = object.id.to_string();
            let mut events = object.object.events;
            for event in &mut events {
                event.observed.sidecar_object_id = object_id.clone();
            }
            let matches_target = events.iter().any(|event| {
                matches!(
                    &event.observed.event,
                    Some(CobEvent {
                        action: CobEventAction::Initialize(root),
                        ..
                    }) if root.target_key == expected_target_key
                ) || (event.observed.entry_oid == object_id
                    && event.observed.related_oids.iter().any(|related| related == &target.object_id))
            });
            matches_target.then(|| LoadedSidecar {
                object_id,
                tips: object.history.tips().into_iter().map(|tip| tip.to_string()).collect(),
                events,
            })
        })
        .collect::<Vec<_>>();
    loaded.sort_by(|left, right| left.object_id.cmp(&right.object_id));
    Ok(loaded)
}

#[allow(clippy::too_many_arguments)]
fn validate_mutation_predecessors(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    authorization: &CobAuthorizationProfile,
    artifact_root: &cap_std::fs::Dir,
    request: &MutationRequest,
    selected: Option<&str>,
    sidecars: &[LoadedSidecar],
    issues: &mut Vec<String>,
) -> Result<(), AdapterError> {
    let required_ids = match &request.action {
        MutationRequestAction::Attach(_) => return Ok(()),
        MutationRequestAction::Supersede(action) => {
            vec![action.attachment_id.as_str(), action.replacement_attachment_id.as_str()]
        }
        MutationRequestAction::Revoke(action) => vec![action.attachment_id.as_str()],
    };
    let Some(selected) = selected else {
        issues.push("existing_sidecar_required".to_owned());
        return Ok(());
    };
    let materialization = materialize_snapshot(store, profile, authorization, artifact_root, request.target.clone())?;
    if !materialization.report.valid || !materialization.adapter_issues.is_empty() {
        issues.push("current_snapshot_invalid_or_unresolved".to_owned());
        return Ok(());
    }
    let Some(snapshot) = materialization.report.snapshot else {
        issues.push("current_snapshot_missing".to_owned());
        return Ok(());
    };
    let accepted = snapshot
        .accepted_attachments
        .iter()
        .map(|attachment| {
            (
                attachment.attachment.attachment_id.as_str(),
                attachment.source_entries.iter().map(String::as_str).collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let selected_entries = sidecars
        .iter()
        .find(|sidecar| sidecar.object_id == selected)
        .map(|sidecar| sidecar.events.iter().map(|event| event.observed.entry_oid.as_str()).collect::<BTreeSet<_>>())
        .unwrap_or_default();
    for attachment_id in required_ids {
        match accepted.get(attachment_id) {
            Some(source_entries) if !source_entries.is_disjoint(&selected_entries) => {}
            Some(_) => issues.push("attachment_not_causal_in_selected_sidecar".to_owned()),
            None => issues.push("accepted_attachment_predecessor_missing".to_owned()),
        }
    }
    Ok(())
}

fn select_sidecar(candidates: &[String], requested: Option<&str>, issues: &mut Vec<String>) -> Option<String> {
    match (candidates, requested) {
        ([], None) => None,
        ([], Some(_)) => {
            issues.push("requested_sidecar_not_found".to_owned());
            None
        }
        ([only], None) => Some(only.clone()),
        (_, None) => {
            issues.push("sidecar_selection_required".to_owned());
            None
        }
        (_, Some(requested)) if candidates.iter().any(|candidate| candidate == requested) => Some(requested.to_owned()),
        (_, Some(_)) => {
            issues.push("requested_sidecar_not_found".to_owned());
            None
        }
    }
}

fn root_event(profile: &CobEvidenceProfile, target: &CobTargetRef) -> Result<CobEvent, AdapterError> {
    let root = SidecarRoot {
        schema: ROOT_SCHEMA.to_owned(),
        target: target.clone(),
        target_key: target_key(target).map_err(|issues| AdapterError::Input(issue_codes(&issues).join(",")))?,
        profile_hash: profile_identity(profile),
        verification_role: VerificationRole::RecordedOnly,
        non_claims: profile.required_non_claims.clone(),
    };
    Ok(CobEvent {
        schema: EVENT_SCHEMA.to_owned(),
        action: CobEventAction::Initialize(root),
    })
}

fn store_embeds(store: &GitCobStore, embed: Option<&PreparedEmbed>) -> Result<Vec<Embed<Oid>>, AdapterError> {
    let Some(embed) = embed else {
        return Ok(Vec::new());
    };
    let written = store.as_raw().blob(&embed.bytes)?;
    if written.to_string() != embed.git_oid {
        return Err(AdapterError::Input("written embed OID differs from plan".to_owned()));
    }
    Ok(vec![Embed {
        name: embed.name.clone(),
        content: Oid::from(written),
    }])
}

fn verify_written_entry(store: &GitCobStore, entry_oid: &str, expected: &CobEvent) -> Result<(), AdapterError> {
    let oid = Oid::from_str(entry_oid).map_err(|error| AdapterError::Input(error.to_string()))?;
    let entry = store.load(oid).map_err(|error| AdapterError::Cob(error.to_string()))?;
    if !entry.valid_signatures() {
        return Err(AdapterError::Input("written entry signature is invalid".to_owned()));
    }
    let found = entry
        .contents()
        .iter()
        .any(|bytes| serde_json::from_slice::<CobEvent>(bytes).is_ok_and(|event| event == *expected));
    if !found {
        return Err(AdapterError::Input("written entry does not contain planned event".to_owned()));
    }
    Ok(())
}

fn resolve_attachments(
    store: &GitCobStore,
    artifact_root: &cap_std::fs::Dir,
    sidecars: &[LoadedSidecar],
) -> (Vec<valence_core::cob_evidence::ResolvedArtifact>, Vec<String>) {
    let mut resolutions = BTreeMap::new();
    let mut issues = Vec::new();
    for transport in sidecars.iter().flat_map(|sidecar| &sidecar.events) {
        let Some(CobEvent {
            action: CobEventAction::Attach(attachment),
            ..
        }) = &transport.observed.event
        else {
            continue;
        };
        match resolve_attachment(store, artifact_root, transport, attachment) {
            Ok(resolution) => {
                resolutions.entry(attachment.attachment_id.clone()).or_insert(resolution);
            }
            Err(error) => issues.push(format!("attachment_resolution_failed:{}:{error}", attachment.attachment_id)),
        }
    }
    (resolutions.into_values().collect(), issues)
}

fn resolve_attachment(
    store: &GitCobStore,
    artifact_root: &cap_std::fs::Dir,
    transport: &TransportObservedEvent,
    attachment: &CobEvidenceAttachment,
) -> Result<valence_core::cob_evidence::ResolvedArtifact, AdapterError> {
    let (bytes, git_oid, locator) = match &attachment.payload {
        AttachmentPayload::ReferenceOnly { locator, .. } => (artifact_root.read(locator)?, None, Some(locator.clone())),
        AttachmentPayload::Embedded { name, .. } => {
            let revision = git2::Oid::from_str(&transport.revision_oid)?;
            let tree = store.as_raw().find_tree(revision)?;
            let path = Path::new("embeds").join(name);
            let entry = tree.get_path(&path)?;
            let blob = entry.to_object(store.as_raw())?.peel_to_blob()?;
            let oid = blob.id().to_string();
            (blob.content().to_vec(), Some(oid), None)
        }
    };
    let artifact = serde_json::from_slice::<EvidenceArtifact>(&bytes)?;
    Ok(valence_core::cob_evidence::ResolvedArtifact {
        attachment_id: attachment.attachment_id.clone(),
        artifact,
        size_bytes: u64::try_from(bytes.len()).map_err(|error| AdapterError::Input(error.to_string()))?,
        raw_blake3: format!("b3:{}", blake3::hash(&bytes).to_hex()),
        git_oid,
        locator,
    })
}

fn intended_ref(profile: &CobEvidenceProfile, actor: &PublicKey, object_id: &str) -> Result<String, AdapterError> {
    let type_name = TypeName::from_str(&profile.cob_type).map_err(|error| AdapterError::Input(error.to_string()))?;
    let object_id = ObjectId::from_str(object_id).map_err(|error| AdapterError::Input(error.to_string()))?;
    Ok(GitCobStore::cob_ref_name(actor, &type_name, &object_id))
}

fn nonempty(values: Vec<Vec<u8>>) -> Result<NonEmpty<Vec<u8>>, AdapterError> {
    NonEmpty::collect(values).ok_or_else(|| AdapterError::Input("COB mutation requires content".to_owned()))
}

fn issue_codes(issues: &[valence_core::cob_evidence::CobIssue]) -> Vec<String> {
    let mut codes = issues.iter().map(|issue| issue.code.clone()).collect::<Vec<_>>();
    codes.sort();
    codes.dedup();
    codes
}

#[cfg(test)]
mod tests;
