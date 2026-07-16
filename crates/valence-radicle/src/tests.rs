// r[verify valence.radicle_cob_evidence.adapter_boundary]
// r[verify valence.radicle_cob_evidence.repository_safety]
// r[verify valence.radicle_cob_evidence.final_validation]
use std::path::Path;

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use radicle_crypto::ssh::keystore::MemorySigner;
use valence_core::cob_evidence::AttachmentStatus;
use valence_core::cob_evidence::ConfidentialityClass;
use valence_core::cob_evidence::EVIDENCE_NON_CLAIM;
use valence_core::evidence_ir::REQUIRED_NON_CLAIM;
use valence_core::evidence_ir::VerificationRole;
use valence_core::evidence_ir::build_evidence_artifact;

use super::*;

const PROFILE_BYTES: &[u8] = include_bytes!("../../../contracts/radicle-cob-evidence/generated/profile.json");
const GIT_BLOB_FILE_MODE: i32 = 0o100644;
const TARGET_TYPE: &str = "xyz.radicle.issue";
const ARTIFACT_LOCATOR: &str = "lint-receipt.json";

struct Fixture {
    _temp: tempfile::TempDir,
    repository_path: std::path::PathBuf,
    artifact_root_path: std::path::PathBuf,
    target: CobTargetRef,
    signer: MemorySigner,
    profile: CobEvidenceProfile,
    authorization: CobAuthorizationProfile,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().expect("create fixture root");
        let repository_path = temp.path().join("repository");
        let artifact_root_path = temp.path().join("artifacts");
        std::fs::create_dir_all(&artifact_root_path).expect("create artifact root");
        let signer = MemorySigner::r#gen();
        let actor = signer.public_key().to_string();
        let repository = git2::Repository::init(&repository_path).expect("initialize repository");
        configure_repository(&repository);
        let target_oid = commit_target(&repository);
        let target_ref = format!("refs/rad/{actor}/cobs/{TARGET_TYPE}/{target_oid}");
        repository
            .reference(&target_ref, target_oid, true, "fixture target")
            .expect("create target COB ref");
        let artifact = build_evidence_artifact(
            "ast-grep-lint-receipt-v1",
            "lint:durable:unwrap",
            None,
            VerificationRole::Property,
            serde_json::json!({"finding": "unwrap", "triaged": true}),
            vec![REQUIRED_NON_CLAIM.to_owned()],
        )
        .expect("build fixture evidence");
        std::fs::write(
            artifact_root_path.join(ARTIFACT_LOCATOR),
            serde_json::to_vec_pretty(&artifact).expect("serialize fixture evidence"),
        )
        .expect("write fixture evidence");
        let profile = parse_profile(PROFILE_BYTES).expect("parse generated profile");
        let authorization = build_authorization_for_actor(&profile, actor, "maintainer".to_owned());
        Self {
            _temp: temp,
            repository_path,
            artifact_root_path,
            target: CobTargetRef {
                repository_rid: "rad:z3gqcJUoA1n9HaHKufZs5FCSGazv5".to_owned(),
                type_name: TARGET_TYPE.to_owned(),
                object_id: target_oid.to_string(),
            },
            signer,
            profile,
            authorization,
        }
    }

    fn store(&self) -> GitCobStore {
        GitCobStore::open(&self.repository_path).expect("open fixture store")
    }

    fn artifact_root(&self) -> Dir {
        Dir::open_ambient_dir(&self.artifact_root_path, ambient_authority()).expect("open artifact root")
    }

    fn attach_request(&self, payload_mode: RequestedPayloadMode) -> MutationRequest {
        MutationRequest {
            schema: MUTATION_REQUEST_SCHEMA.to_owned(),
            target: self.target.clone(),
            actor: self.signer.public_key().to_string(),
            sidecar_object_id: None,
            action: MutationRequestAction::Attach(AttachRequest {
                relationship: AttachmentRelationship::EvidenceFor,
                artifact_locator: ARTIFACT_LOCATOR.to_owned(),
                payload_mode,
                confidentiality: ConfidentialityClass::Public,
            }),
        }
    }
}

#[test]
fn real_radicle_cob_round_trip_is_preview_first_and_ref_confined() {
    // r[verify valence.radicle_cob_evidence.adapter_boundary.valid]
    // r[verify valence.radicle_cob_evidence.repository_safety.valid]
    let fixture = Fixture::new();
    let store = fixture.store();
    let artifact_root = fixture.artifact_root();
    let before_refs = store.all_reference_targets().expect("snapshot refs");
    let before_head = store.as_raw().head().expect("head").target().expect("direct head");
    let before_file = std::fs::read(fixture.repository_path.join("target.txt")).expect("read tracked file");
    let prepared = prepare_mutation(
        &store,
        &fixture.profile,
        &fixture.authorization,
        &artifact_root,
        fixture.attach_request(RequestedPayloadMode::ReferenceOnly),
    )
    .expect("prepare attach");
    assert!(prepared.plan.valid, "{:?}", prepared.plan.issues);
    assert!(!prepared.plan.mutated);
    assert_eq!(store.all_reference_targets().expect("refs after preview"), before_refs);

    let result = execute_mutation(&store, &fixture.profile, &fixture.signer, prepared).expect("execute attach");
    assert!(result.mutated);
    let binding = result.binding_receipt.as_ref().expect("attach binding receipt");
    assert!(verify_saved_binding(&fixture.profile, binding).is_empty());
    assert!(result.updated_ref.starts_with("refs/rad/"));
    assert_eq!(store.as_raw().head().expect("head after").target(), Some(before_head));
    assert_eq!(std::fs::read(fixture.repository_path.join("target.txt")).expect("tracked file after"), before_file);
    let after_refs = store.all_reference_targets().expect("refs after execute");
    for (name, target) in &before_refs {
        assert_eq!(after_refs.get(name), Some(target), "pre-existing ref changed: {name}");
    }
    let new_refs = after_refs.keys().filter(|name| !before_refs.contains_key(*name)).collect::<Vec<_>>();
    assert_eq!(new_refs.len(), 1);
    assert_eq!(new_refs[0].as_str(), result.updated_ref.as_str());

    let snapshot =
        materialize_snapshot(&store, &fixture.profile, &fixture.authorization, &artifact_root, fixture.target.clone())
            .expect("materialize snapshot");
    assert!(snapshot.adapter_issues.is_empty(), "{:?}", snapshot.adapter_issues);
    assert!(snapshot.report.valid, "{:?}", snapshot.report.issues);
    let snapshot_value = snapshot.report.snapshot.expect("snapshot");
    assert_eq!(snapshot_value.accepted_attachments.len(), 1);
    assert_eq!(snapshot_value.verification_role, VerificationRole::RecordedOnly);
    assert!(snapshot_value.non_claims.contains(&EVIDENCE_NON_CLAIM.to_owned()));
    assert_eq!(
        snapshot.snapshot_artifact.expect("snapshot artifact").verification_role,
        VerificationRole::RecordedOnly
    );
}

#[test]
fn embedded_attach_then_revoke_preserves_history_and_changes_only_accepted_state() {
    // r[verify valence.radicle_cob_evidence.payload_modes.valid]
    // r[verify valence.radicle_cob_evidence.privacy_retention.valid]
    let fixture = Fixture::new();
    let store = fixture.store();
    let artifact_root = fixture.artifact_root();
    let attach = prepare_mutation(
        &store,
        &fixture.profile,
        &fixture.authorization,
        &artifact_root,
        fixture.attach_request(RequestedPayloadMode::Embedded),
    )
    .expect("prepare embedded attach");
    let attachment_id = attach.plan.attachment_id.clone().expect("attachment id");
    let attached =
        execute_mutation(&store, &fixture.profile, &fixture.signer, attach).expect("execute embedded attach");
    assert!(attached.binding_receipt.as_ref().expect("binding").embed_oid.is_some());

    let revoke_request = MutationRequest {
        schema: MUTATION_REQUEST_SCHEMA.to_owned(),
        target: fixture.target.clone(),
        actor: fixture.signer.public_key().to_string(),
        sidecar_object_id: Some(attached.sidecar_object_id.clone()),
        action: MutationRequestAction::Revoke(RevokeRequest {
            attachment_id: attachment_id.clone(),
            reason: "producer withdrew this evidence".to_owned(),
        }),
    };
    let revoke = prepare_mutation(&store, &fixture.profile, &fixture.authorization, &artifact_root, revoke_request)
        .expect("prepare revoke");
    assert!(revoke.plan.valid, "{:?}", revoke.plan.issues);
    let revoked = execute_mutation(&store, &fixture.profile, &fixture.signer, revoke).expect("execute revoke");
    assert!(revoked.binding_receipt.is_none());
    assert_ne!(revoked.entry_oid, attached.entry_oid);

    let snapshot =
        materialize_snapshot(&store, &fixture.profile, &fixture.authorization, &artifact_root, fixture.target)
            .expect("snapshot after revoke");
    let accepted = snapshot
        .report
        .snapshot
        .expect("snapshot")
        .accepted_attachments
        .into_iter()
        .find(|accepted| accepted.attachment.attachment_id == attachment_id)
        .expect("historical attachment retained");
    assert!(matches!(accepted.status, AttachmentStatus::Revoked { .. }));
}

#[test]
fn independent_physical_sidecars_converge_and_require_explicit_future_selection() {
    // r[verify valence.radicle_cob_evidence.convergence.valid]
    // r[verify valence.radicle_cob_evidence.repository_safety.invalid]
    let fixture = Fixture::new();
    let store = fixture.store();
    let artifact_root = fixture.artifact_root();
    let first = prepare_mutation(
        &store,
        &fixture.profile,
        &fixture.authorization,
        &artifact_root,
        fixture.attach_request(RequestedPayloadMode::ReferenceOnly),
    )
    .expect("prepare first root");
    execute_mutation(&store, &fixture.profile, &fixture.signer, first).expect("create first root");

    let second_signer = MemorySigner::r#gen();
    let second_locator = "second-lint-receipt.json";
    let second_artifact = build_evidence_artifact(
        "ast-grep-lint-receipt-v1",
        "lint:durable:second",
        None,
        VerificationRole::Property,
        serde_json::json!({"finding": "second", "triaged": true}),
        vec![REQUIRED_NON_CLAIM.to_owned()],
    )
    .expect("build second evidence");
    std::fs::write(
        fixture.artifact_root_path.join(second_locator),
        serde_json::to_vec_pretty(&second_artifact).expect("serialize second evidence"),
    )
    .expect("write second evidence");
    let authorization =
        valence_core::cob_evidence::build_authorization_profile(&profile_identity(&fixture.profile), vec![
            ActorBinding {
                signer: fixture.signer.public_key().to_string(),
                role: "maintainer".to_owned(),
            },
            ActorBinding {
                signer: second_signer.public_key().to_string(),
                role: "maintainer".to_owned(),
            },
        ]);
    let second_request = MutationRequest {
        schema: MUTATION_REQUEST_SCHEMA.to_owned(),
        target: fixture.target.clone(),
        actor: second_signer.public_key().to_string(),
        sidecar_object_id: None,
        action: MutationRequestAction::Attach(AttachRequest {
            relationship: AttachmentRelationship::ContextFor,
            artifact_locator: second_locator.to_owned(),
            payload_mode: RequestedPayloadMode::ReferenceOnly,
            confidentiality: ConfidentialityClass::Public,
        }),
    };
    let second = prepare_mutation(&store, &fixture.profile, &authorization, &artifact_root, second_request)
        .expect("prepare second attachment");
    assert!(second.plan.valid, "{:?}", second.plan.issues);
    create_independent_sidecar(&store, &fixture.profile, &fixture.target, &second_signer, &second);

    let snapshot =
        materialize_snapshot(&store, &fixture.profile, &authorization, &artifact_root, fixture.target.clone())
            .expect("materialize converged roots");
    let snapshot = snapshot.report.snapshot.expect("snapshot");
    assert_eq!(snapshot.sidecars.len(), 2);
    assert_eq!(snapshot.accepted_attachments.len(), 2);

    let ambiguous = prepare_mutation(&store, &fixture.profile, &authorization, &artifact_root, MutationRequest {
        schema: MUTATION_REQUEST_SCHEMA.to_owned(),
        target: fixture.target.clone(),
        actor: fixture.signer.public_key().to_string(),
        sidecar_object_id: None,
        action: MutationRequestAction::Revoke(RevokeRequest {
            attachment_id: snapshot.accepted_attachments[0].attachment.attachment_id.clone(),
            reason: "withdraw selected evidence".to_owned(),
        }),
    })
    .expect("prepare ambiguous mutation");
    assert!(!ambiguous.plan.valid);
    assert!(ambiguous.plan.issues.contains(&"sidecar_selection_required".to_owned()));
}

#[test]
fn malformed_related_root_remains_visible_as_rejected_evidence() {
    // r[verify valence.radicle_cob_evidence.sidecar_profile.invalid]
    let fixture = Fixture::new();
    let store = fixture.store();
    let target_oid = Oid::from_str(&fixture.target.object_id).expect("target oid");
    let type_name = TypeName::from_str(&fixture.profile.cob_type).expect("sidecar type");
    radicle_cob::create::<PhysicalSidecar, _, _>(
        &store,
        &fixture.signer,
        None,
        vec![target_oid],
        fixture.signer.public_key(),
        Create {
            contents: nonempty(vec![b"{malformed".to_vec()]).expect("nonempty malformed event"),
            type_name,
            message: MUTATION_MESSAGE.to_owned(),
            embeds: Vec::new(),
            version: Version::default(),
        },
    )
    .expect("create malformed signed root");
    let snapshot = materialize_snapshot(
        &store,
        &fixture.profile,
        &fixture.authorization,
        &fixture.artifact_root(),
        fixture.target,
    )
    .expect("materialize malformed root");
    let snapshot = snapshot.report.snapshot.expect("bounded snapshot");
    assert!(snapshot.accepted_attachments.is_empty());
    assert!(
        snapshot
            .rejected_events
            .iter()
            .any(|event| { event.issue_codes.contains(&"event_decode_failed".to_owned()) })
    );
}

#[test]
fn unauthorized_and_unsafe_requests_do_not_mutate_repository() {
    // r[verify valence.radicle_cob_evidence.authority.invalid]
    // r[verify valence.radicle_cob_evidence.repository_safety.invalid]
    let fixture = Fixture::new();
    let store = fixture.store();
    let artifact_root = fixture.artifact_root();
    let before = store.all_reference_targets().expect("before refs");
    let unauthorized_signer = MemorySigner::r#gen();
    let mut unauthorized = fixture.attach_request(RequestedPayloadMode::ReferenceOnly);
    unauthorized.actor = unauthorized_signer.public_key().to_string();
    let plan = prepare_mutation(&store, &fixture.profile, &fixture.authorization, &artifact_root, unauthorized)
        .expect("prepare unauthorized request");
    assert!(!plan.plan.valid);
    assert!(plan.plan.issues.contains(&"operation_unauthorized".to_owned()));
    assert!(execute_mutation(&store, &fixture.profile, &unauthorized_signer, plan).is_err());

    let mut unsafe_request = fixture.attach_request(RequestedPayloadMode::ReferenceOnly);
    let MutationRequestAction::Attach(attach) = &mut unsafe_request.action else {
        panic!("fixture action is attach");
    };
    attach.artifact_locator = "../outside.json".to_owned();
    let unsafe_plan =
        prepare_mutation(&store, &fixture.profile, &fixture.authorization, &artifact_root, unsafe_request)
            .expect("prepare unsafe request");
    assert!(!unsafe_plan.plan.valid);
    assert!(!unsafe_plan.plan.mutated);
    assert_eq!(store.all_reference_targets().expect("after refs"), before);
}

fn create_independent_sidecar(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    target: &CobTargetRef,
    signer: &MemorySigner,
    prepared: &PreparedMutation,
) {
    let event = prepared.event.as_ref().expect("prepared event");
    let root = root_event(profile, target).expect("root event");
    let target_oid = Oid::from_str(&target.object_id).expect("target oid");
    let type_name = TypeName::from_str(&profile.cob_type).expect("sidecar type");
    radicle_cob::create::<PhysicalSidecar, _, _>(store, signer, None, vec![target_oid], signer.public_key(), Create {
        contents: nonempty(vec![
            serde_json::to_vec(&root).expect("serialize root"),
            serde_json::to_vec(event).expect("serialize event"),
        ])
        .expect("nonempty content"),
        type_name,
        message: MUTATION_MESSAGE.to_owned(),
        embeds: Vec::new(),
        version: Version::default(),
    })
    .expect("create independent sidecar");
}

fn configure_repository(repository: &git2::Repository) {
    let mut config = repository.config().expect("open repository config");
    config.set_str("user.name", "Valence Radicle Test").expect("set user name");
    config.set_str("user.email", "valence-radicle@example.invalid").expect("set user email");
}

fn commit_target(repository: &git2::Repository) -> git2::Oid {
    std::fs::write(repository.workdir().expect("non-bare fixture").join("target.txt"), "target\n")
        .expect("write target file");
    let blob = repository.blob(b"target\n").expect("write target blob");
    let mut builder = repository.treebuilder(None).expect("create tree builder");
    builder.insert(Path::new("target.txt"), blob, GIT_BLOB_FILE_MODE).expect("insert target blob");
    let tree_oid = builder.write().expect("write target tree");
    let tree = repository.find_tree(tree_oid).expect("load target tree");
    let signature = repository.signature().expect("repository signature");
    repository
        .commit(Some("HEAD"), &signature, &signature, "target COB fixture", &tree, &[])
        .expect("commit target")
}
