use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use radicle_crypto::PublicKey;
use radicle_crypto::Signer;
use radicle_crypto::ssh::keystore::MemorySigner;
use serde::Deserialize;
use serde::Serialize;
use valence_core::cob_evidence::AttachmentRelationship;
use valence_core::cob_evidence::CobAuthorizationProfile;
use valence_core::cob_evidence::CobEvent;
use valence_core::cob_evidence::CobEventAction;
use valence_core::cob_evidence::CobEvidenceProfile;
use valence_core::cob_evidence::CobOperation;
use valence_core::cob_evidence::CobTargetRef;
use valence_core::cob_evidence::ConfidentialityClass;
use valence_core::cob_evidence::ObservedCobEvent;
use valence_core::cob_evidence::build_authorization_profile;
use valence_core::cob_evidence::profile_identity;
use valence_core::evidence_ir::EvidenceArtifact;
use valence_core::evidence_ir::REQUIRED_NON_CLAIM;
use valence_core::evidence_ir::VerificationRole;
use valence_core::evidence_ir::build_evidence_artifact;
use valence_radicle::AttachRequest;
use valence_radicle::GitCobStore;
use valence_radicle::MUTATION_REQUEST_SCHEMA;
use valence_radicle::MutationRequest;
use valence_radicle::MutationRequestAction;
use valence_radicle::PhysicalSidecar;
use valence_radicle::RequestedPayloadMode;
use valence_radicle::ValenceArtifactAuthOperationalInput;
use valence_radicle::build_authorization_for_actor;
use valence_radicle::capture_valence_artifact_auth_operational_artifact;
use valence_radicle::derive_valence_artifact_auth_operational_shell_input;
use valence_radicle::execute_mutation;
use valence_radicle::materialize_snapshot;
use valence_radicle::parse_authorization;
use valence_radicle::parse_profile;
use valence_radicle::prepare_mutation;
use valence_radicle::replay_valence_artifact_auth_operational_artifact;
use valence_radicle::valence_authorization_observation_ref;

const PROFILE_BYTES: &[u8] = include_bytes!("../../../contracts/radicle-cob-evidence/generated/profile.json");
const TARGET_TYPE: &str = "xyz.radicle.issue";
const REPOSITORY_RID: &str = "rad:z3gqcJUoA1n9HaHKufZs5FCSGazv5";
const INITIAL_ARTIFACT_LOCATOR: &str = "lint-receipt.json";
const OPERATIONAL_ARTIFACT_LOCATOR: &str = "artifact-auth-operational.json";
const CONTEXT_FILE: &str = "context.json";
const AUTHORIZATION_FILE: &str = "authorization.json";
const GIT_BLOB_FILE_MODE: i32 = 0o100644;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanaryContext {
    actor: String,
    target: CobTargetRef,
    initial_entry_oid: String,
    operational_entry_oid: String,
    receipt_stable_id: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = std::env::args().collect::<Vec<_>>();
    if arguments.len() != 4 {
        return Err("usage: artifact_auth_canary <capture|replay|unauthorize> <state-dir> <evidence-dir>".into());
    }
    let state_dir = PathBuf::from(&arguments[2]);
    let evidence_dir = PathBuf::from(&arguments[3]);
    std::fs::create_dir_all(&evidence_dir)?;
    match arguments[1].as_str() {
        "capture" => capture(&state_dir, &evidence_dir),
        "replay" => replay(&state_dir, &evidence_dir),
        "unauthorize" => unauthorize(&state_dir, &evidence_dir),
        mode => Err(format!("unknown canary mode: {mode}").into()),
    }
}

fn capture(state_dir: &Path, evidence_dir: &Path) -> Result<(), Box<dyn Error>> {
    if state_dir.exists() {
        return Err("Valence canary state already exists".into());
    }
    let repository_path = state_dir.join("repository");
    let artifact_root_path = evidence_dir.join("artifacts");
    std::fs::create_dir_all(&artifact_root_path)?;
    let signer = MemorySigner::r#gen();
    let actor = signer.public_key().to_string();
    let repository = git2::Repository::init(&repository_path)?;
    configure_repository(&repository)?;
    let target_oid = commit_target(&repository)?;
    let target_ref = format!("refs/rad/{actor}/cobs/{TARGET_TYPE}/{target_oid}");
    repository.reference(&target_ref, target_oid, true, "canary target")?;
    let target = CobTargetRef {
        repository_rid: REPOSITORY_RID.to_owned(),
        type_name: TARGET_TYPE.to_owned(),
        object_id: target_oid.to_string(),
    };
    let initial_artifact = build_evidence_artifact(
        "ast-grep-lint-receipt-v1",
        "canary:lint:durable-unwrap",
        None,
        VerificationRole::Property,
        serde_json::json!({"finding": "unwrap", "triaged": true, "canary": true}),
        vec![REQUIRED_NON_CLAIM.to_owned()],
    )?;
    write_json(&artifact_root_path.join(INITIAL_ARTIFACT_LOCATOR), &initial_artifact)?;
    let profile = parse_profile(PROFILE_BYTES)?;
    let authorization = build_authorization_for_actor(&profile, actor.clone(), "maintainer".to_owned());
    write_json(&evidence_dir.join("profile.json"), &profile)?;
    write_json(&evidence_dir.join("authorization-before.json"), &authorization)?;
    write_json(&state_dir.join(AUTHORIZATION_FILE), &authorization)?;
    let store = GitCobStore::open(&repository_path)?;
    let artifact_root = open_dir(&artifact_root_path)?;
    let initial = prepare_mutation(
        &store,
        &profile,
        &authorization,
        &artifact_root,
        attach_request(&target, &actor, INITIAL_ARTIFACT_LOCATOR),
    )?;
    let initial_result = execute_mutation(&store, &profile, &signer, initial)?;
    let observed = reload_observed_event(&store, &profile, &initial_result.entry_oid)?;
    let key_identity = artifact_auth_ed25519::public_key_identity(&signer.public_key().to_byte_array());
    let authorization_observation_ref = valence_authorization_observation_ref(&authorization);
    let operational_input = ValenceArtifactAuthOperationalInput {
        observed: &observed,
        related_target: &target.object_id,
        profile_id: "valence-radicle-cob-artifact-auth-canary-v1",
        profile_context: "org.onixresearch.valence.evidence:v1",
        protocol_key_identity_blake3: &key_identity.digest_hex,
        evidence_profile: &profile,
        authorization: &authorization,
        operation: CobOperation::Attach,
        authorization_observation_ref: &authorization_observation_ref,
    };
    let artifact = capture_valence_artifact_auth_operational_artifact(&signer, &operational_input)?;
    artifact_root.write(OPERATIONAL_ARTIFACT_LOCATOR, &serde_json::to_vec_pretty(&artifact)?)?;
    let operational = prepare_mutation(
        &store,
        &profile,
        &authorization,
        &artifact_root,
        attach_request(&target, &actor, OPERATIONAL_ARTIFACT_LOCATOR),
    )?;
    let operational_result = execute_mutation(&store, &profile, &signer, operational)?;
    let materialized = materialize_snapshot(&store, &profile, &authorization, &artifact_root, target.clone())?;
    if !materialized.adapter_issues.is_empty() || !materialized.report.valid {
        return Err(format!("materialization failed: {:?}", materialized.adapter_issues).into());
    }
    let snapshot = materialized.report.snapshot.ok_or("materialization produced no snapshot")?;
    if !snapshot
        .accepted_attachments
        .iter()
        .any(|accepted| accepted.attachment.artifact.stable_id == artifact.stable_id)
    {
        return Err("materialized snapshot omitted operational receipt".into());
    }
    let context = CanaryContext {
        actor,
        target,
        initial_entry_oid: initial_result.entry_oid,
        operational_entry_oid: operational_result.entry_oid,
        receipt_stable_id: artifact.stable_id.clone(),
    };
    write_json(&evidence_dir.join(CONTEXT_FILE), &context)?;
    write_json(
        &evidence_dir.join("capture-summary.json"),
        &serde_json::json!({
            "schema": "valence-artifact-auth-canary-summary-v1",
            "phase": "capture",
            "result": "pass",
            "initial_entry_oid": context.initial_entry_oid,
            "operational_entry_oid": context.operational_entry_oid,
            "receipt_stable_id": context.receipt_stable_id,
            "verification_role": "recorded_only",
            "legacy_authoritative": true,
            "standalone_authority_admitted": false,
            "rollback_available": true,
            "signer_source": "ephemeral-memory-ed25519-non-production",
            "non_claim": "non-production signed COB canary evidence does not grant Radicle membership, global revocation, lifecycle, or release authority"
        }),
    )?;
    Ok(())
}

fn replay(state_dir: &Path, evidence_dir: &Path) -> Result<(), Box<dyn Error>> {
    let context = read_context(evidence_dir)?;
    let profile = read_profile(evidence_dir)?;
    let authorization = read_authorization(state_dir)?;
    let store = GitCobStore::open(&state_dir.join("repository"))?;
    let artifact_root_path = evidence_dir.join("artifacts");
    let artifact_root = open_dir(&artifact_root_path)?;
    let observed = reload_observed_event(&store, &profile, &context.initial_entry_oid)?;
    let actor = PublicKey::from_str(&context.actor)?;
    let key_identity = artifact_auth_ed25519::public_key_identity(&actor.to_byte_array());
    let authorization_observation_ref = valence_authorization_observation_ref(&authorization);
    let input = ValenceArtifactAuthOperationalInput {
        observed: &observed,
        related_target: &context.target.object_id,
        profile_id: "valence-radicle-cob-artifact-auth-canary-v1",
        profile_context: "org.onixresearch.valence.evidence:v1",
        protocol_key_identity_blake3: &key_identity.digest_hex,
        evidence_profile: &profile,
        authorization: &authorization,
        operation: CobOperation::Attach,
        authorization_observation_ref: &authorization_observation_ref,
    };
    let artifact_bytes = artifact_root.read(OPERATIONAL_ARTIFACT_LOCATOR)?;
    let artifact = serde_json::from_slice::<EvidenceArtifact>(&artifact_bytes)?;
    let report = replay_valence_artifact_auth_operational_artifact(&input, &artifact)?;
    let materialized = materialize_snapshot(&store, &profile, &authorization, &artifact_root, context.target.clone())?;
    if !materialized.adapter_issues.is_empty() || !materialized.report.valid {
        return Err("reopened materialization is invalid".into());
    }
    write_json(
        &evidence_dir.join("replay-summary.json"),
        &serde_json::json!({
            "schema": "valence-artifact-auth-canary-summary-v1",
            "phase": "restart-replay",
            "result": "pass",
            "receipt_stable_id": artifact.stable_id,
            "standalone_passed": report.dual_run.standalone.as_ref().is_some_and(|decision| decision.passed),
            "legacy_authoritative": report.dual_run.compatibility.legacy_authoritative,
            "standalone_authority_admitted": report.dual_run.compatibility.standalone_authority_admitted,
            "rollback_available": report.dual_run.compatibility.rollback_available
        }),
    )?;
    Ok(())
}

fn unauthorize(state_dir: &Path, evidence_dir: &Path) -> Result<(), Box<dyn Error>> {
    let context = read_context(evidence_dir)?;
    let profile = read_profile(evidence_dir)?;
    let denied = build_authorization_profile(&profile_identity(&profile), Vec::new());
    write_json(&state_dir.join(AUTHORIZATION_FILE), &denied)?;
    write_json(&evidence_dir.join("authorization-after-removal.json"), &denied)?;
    let store = GitCobStore::open(&state_dir.join("repository"))?;
    let observed = reload_observed_event(&store, &profile, &context.initial_entry_oid)?;
    let actor = PublicKey::from_str(&context.actor)?;
    let key_identity = artifact_auth_ed25519::public_key_identity(&actor.to_byte_array());
    let authorization_observation_ref = valence_authorization_observation_ref(&denied);
    let input = ValenceArtifactAuthOperationalInput {
        observed: &observed,
        related_target: &context.target.object_id,
        profile_id: "valence-radicle-cob-artifact-auth-canary-v1",
        profile_context: "org.onixresearch.valence.evidence:v1",
        protocol_key_identity_blake3: &key_identity.digest_hex,
        evidence_profile: &profile,
        authorization: &denied,
        operation: CobOperation::Attach,
        authorization_observation_ref: &authorization_observation_ref,
    };
    let error = derive_valence_artifact_auth_operational_shell_input(&input)
        .expect_err("removed actor authorization must deny operational replay");
    write_json(
        &evidence_dir.join("authorization-removal-summary.json"),
        &serde_json::json!({
            "schema": "valence-artifact-auth-canary-summary-v1",
            "phase": "authorization-removal",
            "result": "expected-denial",
            "error": error.to_string(),
            "standalone_authority_admitted": false,
            "rollback_available": true
        }),
    )?;
    Ok(())
}

fn attach_request(target: &CobTargetRef, actor: &str, artifact_locator: &str) -> MutationRequest {
    MutationRequest {
        schema: MUTATION_REQUEST_SCHEMA.to_owned(),
        target: target.clone(),
        actor: actor.to_owned(),
        sidecar_object_id: None,
        action: MutationRequestAction::Attach(AttachRequest {
            relationship: AttachmentRelationship::EvidenceFor,
            artifact_locator: artifact_locator.to_owned(),
            payload_mode: RequestedPayloadMode::ReferenceOnly,
            confidentiality: ConfidentialityClass::Public,
        }),
    }
}

fn reload_observed_event(
    store: &GitCobStore,
    profile: &CobEvidenceProfile,
    entry_oid: &str,
) -> Result<ObservedCobEvent, Box<dyn Error>> {
    let type_name = radicle_cob::TypeName::from_str(&profile.cob_type)?;
    let objects = radicle_cob::list::<PhysicalSidecar, _>(store, &type_name)?;
    for object in objects {
        let object_id = object.id.to_string();
        for mut event in object.object.events {
            event.observed.sidecar_object_id = object_id.clone();
            if event.observed.entry_oid == entry_oid
                && matches!(
                    event.observed.event,
                    Some(CobEvent {
                        action: CobEventAction::Attach(_),
                        ..
                    })
                )
            {
                if !event.observed.signature_valid {
                    return Err("reloaded COB event signature is invalid".into());
                }
                return Ok(event.observed);
            }
        }
    }
    Err(format!("signed COB event {entry_oid} was not found after reload").into())
}

fn configure_repository(repository: &git2::Repository) -> Result<(), Box<dyn Error>> {
    let mut config = repository.config()?;
    config.set_str("user.name", "Valence Artifact Auth Canary")?;
    config.set_str("user.email", "valence-canary@example.invalid")?;
    Ok(())
}

fn commit_target(repository: &git2::Repository) -> Result<git2::Oid, Box<dyn Error>> {
    let workdir = repository.workdir().ok_or("canary repository is bare")?;
    std::fs::write(workdir.join("target.txt"), "target\n")?;
    let blob = repository.blob(b"target\n")?;
    let mut builder = repository.treebuilder(None)?;
    builder.insert(Path::new("target.txt"), blob, GIT_BLOB_FILE_MODE)?;
    let tree_oid = builder.write()?;
    let tree = repository.find_tree(tree_oid)?;
    let signature = repository.signature()?;
    Ok(repository.commit(Some("HEAD"), &signature, &signature, "target COB canary", &tree, &[])?)
}

fn read_context(evidence_dir: &Path) -> Result<CanaryContext, Box<dyn Error>> {
    Ok(serde_json::from_slice(&std::fs::read(evidence_dir.join(CONTEXT_FILE))?)?)
}

fn read_profile(evidence_dir: &Path) -> Result<CobEvidenceProfile, Box<dyn Error>> {
    Ok(parse_profile(&std::fs::read(evidence_dir.join("profile.json"))?)?)
}

fn read_authorization(state_dir: &Path) -> Result<CobAuthorizationProfile, Box<dyn Error>> {
    Ok(parse_authorization(&std::fs::read(state_dir.join(AUTHORIZATION_FILE))?)?)
}

fn open_dir(path: &Path) -> Result<Dir, Box<dyn Error>> {
    Ok(Dir::open_ambient_dir(path, ambient_authority())?)
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), Box<dyn Error>> {
    std::fs::write(path, serde_json::to_vec_pretty(value)?)?;
    Ok(())
}
