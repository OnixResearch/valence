//! r[verify valence.nostr_identity_evidence.final_validation]
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

use valence_core::nostr_identity_evidence as nostr;

const KEY: &str = "f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9";
const AUDIENCE: &str = "service:operator";
const PURPOSE: &str = "authenticate-control-plane";

fn digest(role: nostr::DigestRole, value: char) -> nostr::TypedDigest {
    nostr::TypedDigest {
        role,
        hex: value.to_string().repeat(64),
    }
}

fn basalt_non_claims() -> Vec<String> {
    nostr::BASALT_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect()
}

fn pins() -> nostr::ProducerPins {
    nostr::ProducerPins {
        ucan_source_revision: nostr::UCAN_SOURCE_REVISION.to_owned(),
        ucan_credential_revision: nostr::UCAN_CREDENTIAL_REVISION.to_owned(),
        basalt_source_revision: nostr::BASALT_SOURCE_REVISION.to_owned(),
        basalt_schema_identity: nostr::TypedDigest {
            role: nostr::DigestRole::BasaltBlake3,
            hex: nostr::BASALT_SCHEMA_BLAKE3.to_owned(),
        },
        basalt_implementation_identity: nostr::TypedDigest {
            role: nostr::DigestRole::BasaltBlake3,
            hex: nostr::BASALT_IMPLEMENTATION_BLAKE3.to_owned(),
        },
        basalt_policy_identity: nostr::TypedDigest {
            role: nostr::DigestRole::BasaltBlake3,
            hex: nostr::BASALT_POLICY_BLAKE3.to_owned(),
        },
    }
}

fn valid_input() -> nostr::NostrIdentityEvidenceInput {
    let credential = nostr::UcanNostrCredentialRecord {
        schema: nostr::UCAN_CREDENTIAL_RECORD_SCHEMA.to_owned(),
        source_revision: nostr::UCAN_SOURCE_REVISION.to_owned(),
        credential_revision: nostr::UCAN_CREDENTIAL_REVISION.to_owned(),
        credential_contract: nostr::UCAN_CREDENTIAL_CONTRACT.to_owned(),
        event_identity: digest(nostr::DigestRole::NostrEventSha256, 'a'),
        public_key: KEY.to_owned(),
        adapter_profile_identity: digest(nostr::DigestRole::UcanAdapterBlake3, 'b'),
        adapter_implementation_identity: digest(nostr::DigestRole::UcanAdapterBlake3, 'c'),
        request_identity: digest(nostr::DigestRole::UcanAdapterBlake3, 'd'),
        audience: AUDIENCE.to_owned(),
        purpose: PURPOSE.to_owned(),
        freshness: "fresh".to_owned(),
        replay: "fresh".to_owned(),
    };
    let producer_pins = pins();
    let mut authentication = nostr::BasaltAuthenticationReceipt {
        schema: nostr::BASALT_AUTHENTICATION_SCHEMA.to_owned(),
        ucan_source_revision: nostr::UCAN_SOURCE_REVISION.to_owned(),
        credential_contract: nostr::UCAN_CREDENTIAL_CONTRACT.to_owned(),
        producer_schema_identity: producer_pins.basalt_schema_identity.clone(),
        producer_implementation_identity: producer_pins.basalt_implementation_identity.clone(),
        event_identity: credential.event_identity.clone(),
        public_key: KEY.to_owned(),
        adapter_profile_identity: credential.adapter_profile_identity.clone(),
        adapter_implementation_identity: credential.adapter_implementation_identity.clone(),
        request_identity: credential.request_identity.clone(),
        audience_identity: nostr::public_field_identity("audience", AUDIENCE),
        purpose_identity: nostr::public_field_identity("purpose", PURPOSE),
        policy_identity: producer_pins.basalt_policy_identity.clone(),
        subject_id: Some("subject:operator".to_owned()),
        binding_id: Some("binding:operator-current".to_owned()),
        binding_identity: Some(digest(nostr::DigestRole::BasaltBlake3, 'e')),
        decision: nostr::AuthenticationDecision::Authenticated,
        reason: "authenticated".to_owned(),
        verification_role: nostr::RecordedRole::RecordedOnly,
        non_claims: basalt_non_claims(),
        receipt_identity: digest(nostr::DigestRole::BasaltBlake3, '0'),
    };
    authentication.receipt_identity = nostr::authentication_receipt_identity(&authentication);
    nostr::NostrIdentityEvidenceInput {
        schema: nostr::NOSTR_EVIDENCE_SCHEMA.to_owned(),
        producer_pins,
        credential,
        authentication,
        composition: None,
        verification_role: nostr::RecordedRole::RecordedOnly,
        public_metadata: BTreeMap::new(),
        non_claims: nostr::REQUIRED_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect(),
    }
}

fn valence_binary() -> &'static str {
    env!("CARGO_BIN_EXE_valence")
}

#[test]
fn valid_input_writes_a_deterministic_recorded_only_artifact() {
    let input_root = tempfile::tempdir().expect("input root");
    let output_root = tempfile::tempdir().expect("output root");
    fs::write(input_root.path().join("input.json"), serde_json::to_vec_pretty(&valid_input()).expect("input JSON"))
        .expect("write input");

    let output = Command::new(valence_binary())
        .args([
            "nostr-identity",
            "validate",
            "--artifact-root",
            input_root.path().to_str().expect("input path"),
            "--input",
            "input.json",
            "--output-root",
            output_root.path().to_str().expect("output path"),
            "--out",
            "artifact.json",
        ])
        .output()
        .expect("run CLI");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let artifact: nostr::NostrIdentityEvidenceArtifact =
        serde_json::from_slice(&fs::read(output_root.path().join("artifact.json")).expect("artifact"))
            .expect("artifact JSON");
    assert_eq!(artifact.verification_role, nostr::RecordedRole::RecordedOnly);
}

#[test]
fn invalid_role_fails_without_writing_output() {
    let input_root = tempfile::tempdir().expect("input root");
    let output_root = tempfile::tempdir().expect("output root");
    let mut input = valid_input();
    input.verification_role = nostr::RecordedRole::Property;
    fs::write(input_root.path().join("input.json"), serde_json::to_vec_pretty(&input).expect("input JSON"))
        .expect("write input");
    let output = Command::new(valence_binary())
        .args([
            "nostr-identity",
            "validate",
            "--artifact-root",
            input_root.path().to_str().expect("input path"),
            "--input",
            "input.json",
            "--output-root",
            output_root.path().to_str().expect("output path"),
            "--out",
            "artifact.json",
        ])
        .output()
        .expect("run CLI");
    assert_eq!(output.status.code(), Some(1));
    assert!(!output_root.path().join("artifact.json").exists());
}

#[test]
fn traversal_is_rejected_before_read() {
    let root = tempfile::tempdir().expect("root");
    let output = Command::new(valence_binary())
        .args([
            "nostr-identity",
            "validate",
            "--artifact-root",
            root.path().to_str().expect("root path"),
            "--input",
            "../outside.json",
        ])
        .output()
        .expect("run CLI");
    assert_eq!(output.status.code(), Some(2));
}
