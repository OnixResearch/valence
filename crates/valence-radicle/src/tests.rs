// r[verify valence.radicle_cob_evidence.adapter_boundary]
// r[verify valence.radicle_cob_evidence.repository_safety]
// r[verify valence.radicle_cob_evidence.final_validation]
// r[verify valence.radicle_cob_evidence.adapter_boundary.invalid]
// r[verify valence.radicle_cob_evidence.authority.valid]
use std::path::Path;

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use radicle_crypto::ssh::keystore::MemorySigner;
use valence_core::cob_evidence::AttachmentStatus;
use valence_core::cob_evidence::CobOperation;
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

// r[verify valence.artifact_auth_shell.exact_verification]
// r[verify valence.artifact_auth_shell.evidence]
// r[verify valence.artifact_auth_shell.authority]
#[test]
fn dedicated_artifact_auth_signature_uses_real_radicle_signer_without_authority_promotion() {
    const ENTRY_OID: &str = "1111111111111111111111111111111111111111";
    const PARENT_OID: &str = "2222222222222222222222222222222222222222";
    const RELATED_OID: &str = "3333333333333333333333333333333333333333";
    const GENERATION_ONE: u64 = 1;

    let signer = MemorySigner::r#gen();
    let evidence_profile = parse_profile(PROFILE_BYTES).expect("evidence profile");
    let authorization =
        valence_core::cob_evidence::build_authorization_profile(&profile_identity(&evidence_profile), vec![
            ActorBinding {
                signer: signer.public_key().to_string(),
                role: "maintainer".to_owned(),
            },
        ]);
    let event = valence_core::cob_evidence::ObservedCobEvent {
        sidecar_object_id: "sidecar-object".to_owned(),
        entry_oid: ENTRY_OID.to_owned(),
        operation_index: 0,
        parent_oids: vec![PARENT_OID.to_owned()],
        related_oids: vec![RELATED_OID.to_owned()],
        signer: signer.public_key().to_string(),
        signature_valid: true,
        event: None,
        decode_issue: None,
    };
    let key_identity = artifact_auth_ed25519::public_key_identity(&signer.public_key().to_byte_array());
    let statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        observed: &event,
        related_target: RELATED_OID,
        profile_id: "valence-radicle-cob-artifact-auth-v1",
        profile_context: "org.onixresearch.valence.evidence:v1",
        generation: GENERATION_ONE,
        currentness: artifact_auth_core::KeyCurrentness::Current,
        currentness_context: "reviewed-radicle-key-currentness",
        protocol_key_identity_blake3: &key_identity.digest_hex,
    };
    let shell_input = ValenceArtifactAuthShellInput {
        statement,
        evidence_profile: &evidence_profile,
        authorization: &authorization,
        operation: CobOperation::Attach,
        actor_authorization_ref: "blake3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    };
    let signed = sign_valence_artifact_auth_statement(&signer, &shell_input).expect("dedicated signature");
    let report = evaluate_valence_artifact_auth_shell(&shell_input, &signed).expect("dedicated verification");

    assert!(report.cryptographic_failure_code.is_none());
    assert!(report.dual_run.protocol_signature_valid);
    assert!(report.dual_run.standalone.as_ref().is_some_and(|decision| decision.passed));
    assert!(report.dual_run.compatibility.case_explained);
    assert!(report.dual_run.compatibility.legacy_authoritative);
    assert!(!report.dual_run.compatibility.standalone_authority_admitted);
    assert!(report.dual_run.compatibility.rollback_available);
    assert!(report.actor_authorization_observed);
    assert_eq!(report.public_key_ref, format!("blake3:{}", key_identity.digest_hex));
    assert_eq!(signed.signature_bytes.len(), artifact_auth_ed25519::ED25519_SIGNATURE_BYTES);
    assert!(signed.signature_hex.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    assert!(!format!("{signed:?}{report:?}").contains("MemorySigner"));
}

// r[verify valence.artifact_auth_shell.exact_verification]
// r[verify valence.artifact_auth_shell.evidence]
// r[verify valence.artifact_auth_shell.authority]
#[test]
fn dedicated_artifact_auth_signature_rejects_protocol_tamper_key_currentness_and_false_parity() {
    use radicle_crypto::signature::Signer as _;

    const ENTRY_OID: &str = "1111111111111111111111111111111111111111";
    const OTHER_ENTRY_OID: &str = "4444444444444444444444444444444444444444";
    const PARENT_OID: &str = "2222222222222222222222222222222222222222";
    const RELATED_OID: &str = "3333333333333333333333333333333333333333";
    const GENERATION_ONE: u64 = 1;
    const MALFORMED_SIGNATURE_BYTES: usize = 1;

    let signer = MemorySigner::r#gen();
    let evidence_profile = parse_profile(PROFILE_BYTES).expect("evidence profile");
    let authorization =
        valence_core::cob_evidence::build_authorization_profile(&profile_identity(&evidence_profile), vec![
            ActorBinding {
                signer: signer.public_key().to_string(),
                role: "maintainer".to_owned(),
            },
        ]);
    let event = valence_core::cob_evidence::ObservedCobEvent {
        sidecar_object_id: "sidecar-object".to_owned(),
        entry_oid: ENTRY_OID.to_owned(),
        operation_index: 0,
        parent_oids: vec![PARENT_OID.to_owned()],
        related_oids: vec![RELATED_OID.to_owned()],
        signer: signer.public_key().to_string(),
        signature_valid: true,
        event: None,
        decode_issue: None,
    };
    let key_identity = artifact_auth_ed25519::public_key_identity(&signer.public_key().to_byte_array());
    let statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        observed: &event,
        related_target: RELATED_OID,
        profile_id: "valence-radicle-cob-artifact-auth-v1",
        profile_context: "org.onixresearch.valence.evidence:v1",
        generation: GENERATION_ONE,
        currentness: artifact_auth_core::KeyCurrentness::Current,
        currentness_context: "reviewed-radicle-key-currentness",
        protocol_key_identity_blake3: &key_identity.digest_hex,
    };
    let shell_input = ValenceArtifactAuthShellInput {
        statement,
        evidence_profile: &evidence_profile,
        authorization: &authorization,
        operation: CobOperation::Attach,
        actor_authorization_ref: "blake3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    };
    let signed = sign_valence_artifact_auth_statement(&signer, &shell_input).expect("dedicated signature");

    let mut protocol_rejected_event = event.clone();
    protocol_rejected_event.signature_valid = false;
    let rejected_statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        observed: &protocol_rejected_event,
        ..statement
    };
    let rejected_signing = sign_valence_artifact_auth_statement(&signer, &ValenceArtifactAuthShellInput {
        statement: rejected_statement,
        ..shell_input
    })
    .expect_err("protocol rejection blocks dedicated signing");
    assert!(rejected_signing.to_string().contains("requires protocol verification"));

    let unauthorized_signer = MemorySigner::r#gen();
    let unauthorized_profile =
        valence_core::cob_evidence::build_authorization_profile(&profile_identity(&evidence_profile), vec![
            ActorBinding {
                signer: unauthorized_signer.public_key().to_string(),
                role: "maintainer".to_owned(),
            },
        ]);
    let unauthorized = sign_valence_artifact_auth_statement(&signer, &ValenceArtifactAuthShellInput {
        authorization: &unauthorized_profile,
        ..shell_input
    })
    .expect_err("actor authorization blocks dedicated signing");
    assert!(unauthorized.to_string().contains("requires actor authorization"));

    let wrong_signer = MemorySigner::r#gen();
    let signer_mismatch = sign_valence_artifact_auth_statement(&wrong_signer, &shell_input)
        .expect_err("actor mismatch blocks dedicated signing");
    assert!(signer_mismatch.to_string().contains("does not match the protocol actor"));

    let protocol_preimage_signature: radicle_crypto::Signature = signer.sign(b"radicle-protocol-preimage");
    let mut protocol_preimage_carrier = signed.clone();
    protocol_preimage_carrier.replace_signature_bytes_for_test(protocol_preimage_signature.as_ref().to_vec());
    let protocol_preimage = evaluate_valence_artifact_auth_shell(&shell_input, &protocol_preimage_carrier)
        .expect("protocol preimage decision");
    assert_eq!(protocol_preimage.cryptographic_failure_code.as_deref(), Some("ed25519.signature_invalid"));
    assert!(protocol_preimage.dual_run.compatibility.decision_drift);

    let mut wrong_preimage_event = event.clone();
    wrong_preimage_event.entry_oid = OTHER_ENTRY_OID.to_owned();
    let wrong_preimage_statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        observed: &wrong_preimage_event,
        ..statement
    };
    let mapped = valence_core::artifact_auth::map_valence_artifact_auth_statement(&wrong_preimage_statement)
        .expect("wrong preimage statement");
    let mapped_bytes = artifact_auth_core::canonical_statement_bytes(&mapped).expect("canonical statement");
    let mut wrong_preimage_carrier = signed.clone();
    wrong_preimage_carrier.statement_ref = format!("blake3:{}", blake3::hash(&mapped_bytes).to_hex());
    let wrong_preimage = evaluate_valence_artifact_auth_shell(
        &ValenceArtifactAuthShellInput {
            statement: wrong_preimage_statement,
            ..shell_input
        },
        &wrong_preimage_carrier,
    )
    .expect("wrong preimage decision");
    assert_eq!(wrong_preimage.cryptographic_failure_code.as_deref(), Some("ed25519.signature_invalid"));

    let mut wrong_key_carrier = signed.clone();
    wrong_key_carrier.replace_public_key_for_test(*wrong_signer.public_key());
    let wrong_key = evaluate_valence_artifact_auth_shell(&shell_input, &wrong_key_carrier).expect("wrong key decision");
    assert!(!wrong_key.dual_run.compatibility.identity_drift_explained);
    assert!(wrong_key.dual_run.compatibility.blockers.contains(&"identity-drift".to_owned()));

    let mut malformed = signed.clone();
    malformed.replace_signature_bytes_for_test(vec![0_u8; MALFORMED_SIGNATURE_BYTES]);
    let malformed_report =
        evaluate_valence_artifact_auth_shell(&shell_input, &malformed).expect("malformed signature decision");
    assert_eq!(malformed_report.cryptographic_failure_code.as_deref(), Some("ed25519.signature_length"));

    let mut carrier_drift = signed.clone();
    carrier_drift.signature_ref = "blake3:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned();
    let carrier_error =
        evaluate_valence_artifact_auth_shell(&shell_input, &carrier_drift).expect_err("carrier drift denied");
    assert!(carrier_error.to_string().contains("carrier signature identity mismatch"));

    let unknown_statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        currentness: artifact_auth_core::KeyCurrentness::Unknown,
        ..statement
    };
    let unknown = evaluate_valence_artifact_auth_shell(
        &ValenceArtifactAuthShellInput {
            statement: unknown_statement,
            ..shell_input
        },
        &signed,
    )
    .expect("unknown currentness decision");
    assert!(unknown.dual_run.compatibility.decision_drift);
    assert!(!unknown.dual_run.compatibility.standalone_authority_admitted);

    let false_parity = evaluate_valence_artifact_auth_shell(
        &ValenceArtifactAuthShellInput {
            statement: rejected_statement,
            ..shell_input
        },
        &signed,
    )
    .expect("protocol rejection and standalone pass decision");
    assert!(!false_parity.dual_run.protocol_signature_valid);
    assert!(false_parity.dual_run.standalone.as_ref().is_some_and(|decision| decision.passed));
    assert!(!false_parity.dual_run.compatibility.case_explained);

    let mut decode_failed_event = protocol_rejected_event;
    decode_failed_event.decode_issue = Some("event_json_invalid".to_owned());
    let decode_statement = valence_core::artifact_auth::ValenceArtifactAuthStatementInput {
        observed: &decode_failed_event,
        ..statement
    };
    let decode_false_parity = evaluate_valence_artifact_auth_shell(
        &ValenceArtifactAuthShellInput {
            statement: decode_statement,
            ..shell_input
        },
        &protocol_preimage_carrier,
    )
    .expect("decode false parity decision");
    assert!(!decode_false_parity.dual_run.compatibility.case_explained);
    assert!(
        decode_false_parity
            .dual_run
            .compatibility
            .blockers
            .contains(&"unrelated-protocol-and-decode-failures".to_owned())
    );
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
