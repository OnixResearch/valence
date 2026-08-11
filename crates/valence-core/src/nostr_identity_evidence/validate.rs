use super::identity::artifact_identity;
use super::identity::authentication_receipt_identity;
use super::identity::composition_receipt_identity;
use super::identity::public_field_identity;
use super::*;

const DIGEST_HEX_BYTES: usize = 32;
const HEX_CHARS_PER_BYTE: usize = 2;
const DIGEST_HEX_CHARS: usize = DIGEST_HEX_BYTES * HEX_CHARS_PER_BYTE;
const MAX_PUBLIC_FIELD_BYTES: usize = 4_096;
const MAX_METADATA_FIELDS: usize = 32;

/// Parse one closed already-loaded producer envelope.
///
/// # Errors
///
/// Returns a JSON error for unknown, duplicate, missing, or mistyped fields.
// r[impl valence.nostr_identity_evidence.profile]
pub fn parse_nostr_identity_evidence(input: &str) -> Result<NostrIdentityEvidenceInput, serde_json::Error> {
    serde_json::from_str(input)
}

/// Validate exact producer linkage and construct one recorded-only artifact.
// r[impl valence.nostr_identity_evidence.producer_linkage]
// r[impl valence.nostr_identity_evidence.digest_domains]
// r[impl valence.nostr_identity_evidence.role_policy]
// r[impl valence.nostr_identity_evidence.privacy]
#[must_use]
pub fn validate_nostr_identity_evidence(input: &NostrIdentityEvidenceInput) -> NostrIdentityEvidenceReport {
    let mut issues = Vec::new();
    validate_profile_and_pins(input, &mut issues);
    validate_credential(input, &mut issues);
    validate_authentication(input, &mut issues);
    validate_composition(input, &mut issues);
    validate_privacy(input, &mut issues);
    if input.verification_role != RecordedRole::RecordedOnly {
        issues.push(issue("verification_role", "role_promotion_denied"));
    }
    if !exact_non_claims(&input.non_claims, REQUIRED_NON_CLAIMS) {
        issues.push(issue("non_claims", "required_non_claims_mismatch"));
    }
    issues.sort();
    issues.dedup();
    let artifact = issues.is_empty().then(|| build_artifact(input));
    NostrIdentityEvidenceReport {
        valid: issues.is_empty(),
        issues,
        artifact,
        property_count: 0,
        recorded_only_count: 1,
        non_claims: required_non_claims(),
    }
}

fn validate_profile_and_pins(input: &NostrIdentityEvidenceInput, issues: &mut Vec<NostrIdentityEvidenceIssue>) {
    if input.schema != NOSTR_EVIDENCE_SCHEMA {
        issues.push(issue("schema", "schema_mismatch"));
    }
    let pins = &input.producer_pins;
    if pins.ucan_source_revision != UCAN_SOURCE_REVISION
        || pins.ucan_credential_revision != UCAN_CREDENTIAL_REVISION
        || pins.basalt_source_revision != BASALT_SOURCE_REVISION
    {
        issues.push(issue("producer_pins.revision", "producer_revision_mismatch"));
    }
    for (digest, expected, field) in [
        (&pins.basalt_schema_identity, BASALT_SCHEMA_BLAKE3, "producer_pins.basalt_schema_identity"),
        (
            &pins.basalt_implementation_identity,
            BASALT_IMPLEMENTATION_BLAKE3,
            "producer_pins.basalt_implementation_identity",
        ),
        (&pins.basalt_policy_identity, BASALT_POLICY_BLAKE3, "producer_pins.basalt_policy_identity"),
    ] {
        if !valid_digest(digest, DigestRole::BasaltBlake3) || digest.hex != expected {
            issues.push(issue(field, "producer_identity_mismatch"));
        }
    }
}

fn validate_credential(input: &NostrIdentityEvidenceInput, issues: &mut Vec<NostrIdentityEvidenceIssue>) {
    let credential = &input.credential;
    if credential.schema != UCAN_CREDENTIAL_RECORD_SCHEMA
        || credential.source_revision != UCAN_SOURCE_REVISION
        || credential.credential_revision != UCAN_CREDENTIAL_REVISION
        || credential.credential_contract != UCAN_CREDENTIAL_CONTRACT
    {
        issues.push(issue("credential.producer", "credential_producer_mismatch"));
    }
    check_digest(&credential.event_identity, DigestRole::NostrEventSha256, "credential.event_identity", issues);
    for (digest, field) in [
        (&credential.adapter_profile_identity, "credential.adapter_profile_identity"),
        (&credential.adapter_implementation_identity, "credential.adapter_implementation_identity"),
        (&credential.request_identity, "credential.request_identity"),
    ] {
        check_digest(digest, DigestRole::UcanAdapterBlake3, field, issues);
    }
    if !canonical_lower_hex(&credential.public_key) {
        issues.push(issue("credential.public_key", "malformed_public_key"));
    }
    if credential.freshness != "fresh" || credential.replay != "fresh" {
        issues.push(issue("credential.freshness", "credential_not_fresh"));
    }
}

fn validate_authentication(input: &NostrIdentityEvidenceInput, issues: &mut Vec<NostrIdentityEvidenceIssue>) {
    let auth = &input.authentication;
    if auth.schema != BASALT_AUTHENTICATION_SCHEMA
        || auth.ucan_source_revision != UCAN_SOURCE_REVISION
        || auth.credential_contract != UCAN_CREDENTIAL_CONTRACT
    {
        issues.push(issue("authentication.producer", "authentication_producer_mismatch"));
    }
    if auth.producer_schema_identity != input.producer_pins.basalt_schema_identity
        || auth.producer_implementation_identity != input.producer_pins.basalt_implementation_identity
        || auth.policy_identity != input.producer_pins.basalt_policy_identity
    {
        issues.push(issue("authentication.producer_identity", "producer_link_mismatch"));
    }
    let credential = &input.credential;
    if auth.event_identity != credential.event_identity
        || auth.public_key != credential.public_key
        || auth.adapter_profile_identity != credential.adapter_profile_identity
        || auth.adapter_implementation_identity != credential.adapter_implementation_identity
        || auth.request_identity != credential.request_identity
    {
        issues.push(issue("authentication.credential_link", "credential_link_mismatch"));
    }
    if auth.audience_identity != public_field_identity("audience", &credential.audience)
        || auth.purpose_identity != public_field_identity("purpose", &credential.purpose)
    {
        issues.push(issue("authentication.context_link", "context_link_mismatch"));
    }
    if let Some(binding_identity) = auth.binding_identity.as_ref() {
        check_digest(binding_identity, DigestRole::BasaltBlake3, "authentication.binding_identity", issues);
    }
    if auth.decision != AuthenticationDecision::Authenticated
        || auth.reason != "authenticated"
        || auth.subject_id.is_none()
        || auth.binding_id.is_none()
        || auth.binding_identity.is_none()
    {
        issues.push(issue("authentication.decision", "authentication_not_successful"));
    }
    if auth.verification_role != RecordedRole::RecordedOnly {
        issues.push(issue("authentication.verification_role", "role_promotion_denied"));
    }
    if !exact_non_claims(&auth.non_claims, BASALT_NON_CLAIMS) {
        issues.push(issue("authentication.non_claims", "producer_non_claims_mismatch"));
    }
    if auth.receipt_identity != authentication_receipt_identity(auth) {
        issues.push(issue("authentication.receipt_identity", "receipt_identity_mismatch"));
    }
}

fn validate_composition(input: &NostrIdentityEvidenceInput, issues: &mut Vec<NostrIdentityEvidenceIssue>) {
    let Some(composition) = input.composition.as_ref() else {
        return;
    };
    if composition.schema != BASALT_COMPOSITION_SCHEMA
        || composition.producer_schema_identity != input.producer_pins.basalt_schema_identity
        || composition.producer_implementation_identity != input.producer_pins.basalt_implementation_identity
    {
        issues.push(issue("composition.producer", "composition_producer_mismatch"));
    }
    let auth = &input.authentication;
    if composition.authentication_receipt_identity != auth.receipt_identity
        || composition.subject_id.as_str() != auth.subject_id.as_deref().unwrap_or_default()
        || composition.request_identity != auth.request_identity
        || composition.policy_identity != auth.policy_identity
    {
        issues.push(issue("composition.authentication_link", "composition_link_mismatch"));
    }
    check_digest(
        &composition.authorization_receipt_identity,
        DigestRole::BasaltBlake3,
        "composition.authorization_receipt_identity",
        issues,
    );
    for (digest, field) in [
        (composition.resource_identity.as_ref(), "composition.resource_identity"),
        (composition.ability_identity.as_ref(), "composition.ability_identity"),
    ] {
        if let Some(digest) = digest {
            check_digest(digest, DigestRole::BasaltBlake3, field, issues);
        }
    }
    if !valid_composition_shape(composition) {
        issues.push(issue("composition.decision", "composition_shape_mismatch"));
    }
    if composition.verification_role != RecordedRole::RecordedOnly {
        issues.push(issue("composition.verification_role", "role_promotion_denied"));
    }
    if !exact_non_claims(&composition.non_claims, BASALT_NON_CLAIMS) {
        issues.push(issue("composition.non_claims", "producer_non_claims_mismatch"));
    }
    if composition.receipt_identity != composition_receipt_identity(composition) {
        issues.push(issue("composition.receipt_identity", "receipt_identity_mismatch"));
    }
}

fn valid_composition_shape(composition: &BasaltCompositionReceipt) -> bool {
    match composition.decision {
        CompositionDecision::ComposedAllow => {
            composition.reason == "composed-allow"
                && composition.resource_identity.is_some()
                && composition.ability_identity.is_some()
        }
        CompositionDecision::Denied => {
            let valid_reason = matches!(
                composition.reason.as_str(),
                "authorization-subject-mismatch"
                    | "authorization-policy-mismatch"
                    | "authorization-digest-invalid"
                    | "authorization-schema-mismatch"
                    | "authorization-shape-invalid"
                    | "authorization-request-mismatch"
                    | "authorization-missing"
                    | "authorization-stale"
                    | "authorization-denied"
            );
            valid_reason && composition.resource_identity.is_none() && composition.ability_identity.is_none()
        }
    }
}

fn validate_privacy(input: &NostrIdentityEvidenceInput, issues: &mut Vec<NostrIdentityEvidenceIssue>) {
    for (value, field) in [
        (&input.credential.audience, "credential.audience"),
        (&input.credential.purpose, "credential.purpose"),
    ] {
        if !valid_public_text(value) {
            issues.push(issue(field, "secret_or_unbounded_field"));
        }
    }
    if input.public_metadata.len() > MAX_METADATA_FIELDS {
        issues.push(issue("public_metadata", "metadata_limit_exceeded"));
    }
    for (key, value) in &input.public_metadata {
        if !valid_public_text(key) || !valid_public_text(value) {
            issues.push(issue("public_metadata", "secret_or_unbounded_field"));
        }
    }
}

fn build_artifact(input: &NostrIdentityEvidenceInput) -> NostrIdentityEvidenceArtifact {
    let auth = &input.authentication;
    let composition = input.composition.as_ref();
    let mut artifact = NostrIdentityEvidenceArtifact {
        schema: NOSTR_EVIDENCE_SCHEMA.to_owned(),
        producer_pins: input.producer_pins.clone(),
        event_identity: auth.event_identity.clone(),
        public_key: auth.public_key.clone(),
        request_identity: auth.request_identity.clone(),
        audience_identity: auth.audience_identity.clone(),
        purpose_identity: auth.purpose_identity.clone(),
        subject_id: auth.subject_id.clone().expect("validated subject"),
        binding_id: auth.binding_id.clone().expect("validated binding"),
        binding_identity: auth.binding_identity.clone().expect("validated binding identity"),
        policy_identity: auth.policy_identity.clone(),
        authentication_receipt_identity: auth.receipt_identity.clone(),
        composition_receipt_identity: composition.map(|value| value.receipt_identity.clone()),
        authorization_receipt_identity: composition.map(|value| value.authorization_receipt_identity.clone()),
        verification_role: RecordedRole::RecordedOnly,
        public_metadata: input.public_metadata.clone(),
        non_claims: required_non_claims(),
        artifact_identity: TypedDigest::new(DigestRole::ValenceBlake3, String::new()),
    };
    artifact.artifact_identity = artifact_identity(&artifact);
    artifact
}

fn check_digest(
    digest: &TypedDigest,
    role: DigestRole,
    field: &'static str,
    issues: &mut Vec<NostrIdentityEvidenceIssue>,
) {
    if !valid_digest(digest, role) {
        issues.push(issue(field, "digest_role_mismatch"));
    }
}

fn valid_digest(digest: &TypedDigest, role: DigestRole) -> bool {
    digest.role == role && canonical_lower_hex(&digest.hex)
}

fn canonical_lower_hex(value: &str) -> bool {
    value.len() == DIGEST_HEX_CHARS && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_public_text(value: &str) -> bool {
    if value.is_empty() || value.len() > MAX_PUBLIC_FIELD_BYTES || value.trim() != value {
        return false;
    }
    let lowered = value.to_ascii_lowercase();
    let forbidden = [
        "private_key",
        "private-key",
        "secret=",
        "credential=",
        "proof=",
        "challenge=",
        "signer_secret",
        "encrypted_payload",
        "nip46",
        "nip-46",
    ];
    !forbidden.iter().any(|marker| lowered.contains(marker))
}

fn exact_non_claims(values: &[String], required: &[&str]) -> bool {
    values.len() == required.len() && required.iter().all(|required| values.iter().any(|value| value == required))
}

fn required_non_claims() -> Vec<String> {
    REQUIRED_NON_CLAIMS.iter().map(|value| (*value).to_owned()).collect()
}

const fn issue(field: &'static str, code: &'static str) -> NostrIdentityEvidenceIssue {
    NostrIdentityEvidenceIssue::new(field, code)
}
