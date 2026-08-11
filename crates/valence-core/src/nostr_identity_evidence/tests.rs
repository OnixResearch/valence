use super::identity::authentication_receipt_identity;
use super::identity::composition_receipt_identity;
use super::*;

const TEST_HEX_LENGTH: usize = 64;
const TEST_REVISION_LENGTH: usize = 40;
const KEY: &str = "f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9";
const ROTATED_KEY: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const AUDIENCE: &str = "service:operator";
const PURPOSE: &str = "authenticate-control-plane";

fn digest(role: DigestRole, value: char) -> TypedDigest {
    TypedDigest::new(role, value.to_string().repeat(TEST_HEX_LENGTH))
}

fn basalt_non_claims() -> Vec<String> {
    BASALT_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect()
}

fn valence_non_claims() -> Vec<String> {
    REQUIRED_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect()
}

fn pins() -> ProducerPins {
    ProducerPins {
        ucan_source_revision: UCAN_SOURCE_REVISION.to_owned(),
        ucan_credential_revision: UCAN_CREDENTIAL_REVISION.to_owned(),
        basalt_source_revision: BASALT_SOURCE_REVISION.to_owned(),
        basalt_schema_identity: TypedDigest::new(DigestRole::BasaltBlake3, BASALT_SCHEMA_BLAKE3),
        basalt_implementation_identity: TypedDigest::new(DigestRole::BasaltBlake3, BASALT_IMPLEMENTATION_BLAKE3),
        basalt_policy_identity: TypedDigest::new(DigestRole::BasaltBlake3, BASALT_POLICY_BLAKE3),
    }
}

fn credential() -> UcanNostrCredentialRecord {
    UcanNostrCredentialRecord {
        schema: UCAN_CREDENTIAL_RECORD_SCHEMA.to_owned(),
        source_revision: UCAN_SOURCE_REVISION.to_owned(),
        credential_revision: UCAN_CREDENTIAL_REVISION.to_owned(),
        credential_contract: UCAN_CREDENTIAL_CONTRACT.to_owned(),
        event_identity: digest(DigestRole::NostrEventSha256, 'a'),
        public_key: KEY.to_owned(),
        adapter_profile_identity: digest(DigestRole::UcanAdapterBlake3, 'b'),
        adapter_implementation_identity: digest(DigestRole::UcanAdapterBlake3, 'c'),
        request_identity: digest(DigestRole::UcanAdapterBlake3, 'd'),
        audience: AUDIENCE.to_owned(),
        purpose: PURPOSE.to_owned(),
        freshness: "fresh".to_owned(),
        replay: "fresh".to_owned(),
    }
}

fn authentication(credential: &UcanNostrCredentialRecord) -> BasaltAuthenticationReceipt {
    let mut receipt = BasaltAuthenticationReceipt {
        schema: BASALT_AUTHENTICATION_SCHEMA.to_owned(),
        ucan_source_revision: UCAN_SOURCE_REVISION.to_owned(),
        credential_contract: UCAN_CREDENTIAL_CONTRACT.to_owned(),
        producer_schema_identity: pins().basalt_schema_identity,
        producer_implementation_identity: pins().basalt_implementation_identity,
        event_identity: credential.event_identity.clone(),
        public_key: credential.public_key.clone(),
        adapter_profile_identity: credential.adapter_profile_identity.clone(),
        adapter_implementation_identity: credential.adapter_implementation_identity.clone(),
        request_identity: credential.request_identity.clone(),
        audience_identity: super::identity::public_field_identity("audience", AUDIENCE),
        purpose_identity: super::identity::public_field_identity("purpose", PURPOSE),
        policy_identity: pins().basalt_policy_identity,
        subject_id: Some("subject:operator".to_owned()),
        binding_id: Some("binding:operator-current".to_owned()),
        binding_identity: Some(digest(DigestRole::BasaltBlake3, 'e')),
        decision: AuthenticationDecision::Authenticated,
        reason: "authenticated".to_owned(),
        verification_role: RecordedRole::RecordedOnly,
        non_claims: basalt_non_claims(),
        receipt_identity: digest(DigestRole::BasaltBlake3, '0'),
    };
    receipt.receipt_identity = authentication_receipt_identity(&receipt);
    receipt
}

fn composition(auth: &BasaltAuthenticationReceipt) -> BasaltCompositionReceipt {
    let mut receipt = BasaltCompositionReceipt {
        schema: BASALT_COMPOSITION_SCHEMA.to_owned(),
        producer_schema_identity: pins().basalt_schema_identity,
        producer_implementation_identity: pins().basalt_implementation_identity,
        authentication_receipt_identity: auth.receipt_identity.clone(),
        authorization_receipt_identity: digest(DigestRole::BasaltBlake3, 'f'),
        subject_id: auth.subject_id.clone().expect("subject"),
        request_identity: auth.request_identity.clone(),
        policy_identity: auth.policy_identity.clone(),
        resource_identity: Some(digest(DigestRole::BasaltBlake3, '1')),
        ability_identity: Some(digest(DigestRole::BasaltBlake3, '2')),
        decision: CompositionDecision::ComposedAllow,
        reason: "composed-allow".to_owned(),
        verification_role: RecordedRole::RecordedOnly,
        non_claims: basalt_non_claims(),
        receipt_identity: digest(DigestRole::BasaltBlake3, '0'),
    };
    receipt.receipt_identity = composition_receipt_identity(&receipt);
    receipt
}

fn valid_input(with_composition: bool) -> NostrIdentityEvidenceInput {
    let credential = credential();
    let authentication = authentication(&credential);
    let composition = with_composition.then(|| composition(&authentication));
    NostrIdentityEvidenceInput {
        schema: NOSTR_EVIDENCE_SCHEMA.to_owned(),
        producer_pins: pins(),
        credential,
        authentication,
        composition,
        verification_role: RecordedRole::RecordedOnly,
        public_metadata: [("display_label".to_owned(), "Operator".to_owned())].into_iter().collect(),
        non_claims: valence_non_claims(),
    }
}

#[test]
fn exact_linkage_builds_deterministic_recorded_only_artifact() {
    let input = valid_input(true);
    let first = validate_nostr_identity_evidence(&input);
    let second = validate_nostr_identity_evidence(&input);
    assert!(first.valid, "issues: {:?}", first.issues);
    assert_eq!(first, second);
    assert_eq!(first.property_count, 0);
    assert_eq!(first.recorded_only_count, 1);
    let artifact = first.artifact.expect("artifact");
    assert_eq!(artifact.verification_role, RecordedRole::RecordedOnly);
    assert!(artifact.composition_receipt_identity.is_some());
}

#[test]
fn rotation_preserves_subject_and_changes_artifact_identity() {
    let original = valid_input(false);
    let original_report = validate_nostr_identity_evidence(&original);
    assert!(original_report.valid);

    let mut rotated = original.clone();
    rotated.credential.event_identity = digest(DigestRole::NostrEventSha256, 'b');
    rotated.credential.public_key = ROTATED_KEY.to_owned();
    rotated.authentication.event_identity = rotated.credential.event_identity.clone();
    rotated.authentication.public_key = rotated.credential.public_key.clone();
    rotated.authentication.receipt_identity = authentication_receipt_identity(&rotated.authentication);

    let rotated_report = validate_nostr_identity_evidence(&rotated);
    assert!(rotated_report.valid);
    assert_eq!(original.authentication.subject_id, rotated.authentication.subject_id);
    assert_ne!(
        original_report.artifact.expect("original artifact").artifact_identity,
        rotated_report.artifact.expect("rotated artifact").artifact_identity
    );
}

#[test]
fn optional_composition_can_be_absent() {
    let report = validate_nostr_identity_evidence(&valid_input(false));
    assert!(report.valid);
    assert!(report.artifact.expect("artifact").composition_receipt_identity.is_none());
}

#[test]
fn producer_schema_policy_and_revision_drift_fail() {
    let mut cases = vec![valid_input(false), valid_input(false), valid_input(false)];
    cases[0].producer_pins.basalt_source_revision = "0".repeat(TEST_REVISION_LENGTH);
    cases[1].producer_pins.basalt_schema_identity.hex = "0".repeat(TEST_HEX_LENGTH);
    cases[2].producer_pins.basalt_policy_identity.hex = "0".repeat(TEST_HEX_LENGTH);
    for case in cases {
        assert!(!validate_nostr_identity_evidence(&case).valid);
    }
}

#[test]
fn event_key_request_context_and_subject_mismatches_fail() {
    let mut cases = vec![
        valid_input(true),
        valid_input(true),
        valid_input(true),
        valid_input(true),
    ];
    cases[0].authentication.event_identity.hex = "0".repeat(TEST_HEX_LENGTH);
    cases[1].authentication.public_key = "0".repeat(TEST_HEX_LENGTH);
    cases[2].authentication.request_identity.hex = "0".repeat(TEST_HEX_LENGTH);
    cases[3].composition.as_mut().expect("composition").subject_id = "subject:other".to_owned();
    for case in cases {
        assert!(!validate_nostr_identity_evidence(&case).valid);
    }
}

#[test]
fn digest_substitution_and_role_promotion_fail() {
    let mut digest_role = valid_input(false);
    digest_role.credential.event_identity.role = DigestRole::ValenceBlake3;
    assert!(!validate_nostr_identity_evidence(&digest_role).valid);

    for role in [RecordedRole::Property, RecordedRole::Authority, RecordedRole::Boundary] {
        let mut promoted = valid_input(false);
        promoted.verification_role = role;
        assert!(!validate_nostr_identity_evidence(&promoted).valid);
    }
}

#[test]
fn secret_metadata_weakened_claims_and_tampered_receipts_fail() {
    let mut secret = valid_input(false);
    secret.public_metadata.insert("signer_secret".to_owned(), "secret=value".to_owned());
    assert!(!validate_nostr_identity_evidence(&secret).valid);

    let mut weakened = valid_input(false);
    weakened.non_claims.pop();
    assert!(!validate_nostr_identity_evidence(&weakened).valid);

    let mut malformed_composition = valid_input(true);
    let composition = malformed_composition.composition.as_mut().expect("composition");
    composition.reason = "secret=authorization".to_owned();
    composition.receipt_identity = composition_receipt_identity(composition);
    assert!(!validate_nostr_identity_evidence(&malformed_composition).valid);

    let mut tampered = valid_input(false);
    tampered.authentication.subject_id = Some("subject:other".to_owned());
    assert!(!validate_nostr_identity_evidence(&tampered).valid);
}

#[test]
fn closed_parser_rejects_unknown_secret_and_credential_fields() {
    let json = serde_json::to_value(valid_input(false)).expect("input value");
    let mut object = json.as_object().expect("object").clone();
    object.insert("private_key".to_owned(), serde_json::json!("secret"));
    assert!(parse_nostr_identity_evidence(&serde_json::to_string(&object).expect("json")).is_err());
}
