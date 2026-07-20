use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use valence_core::artifact_auth::ValenceArtifactAuthObservation;
use valence_core::artifact_auth::ValenceArtifactAuthReport;
use valence_core::artifact_auth::ValenceArtifactAuthStatementInput;
use valence_core::artifact_auth::evaluate_valence_artifact_auth;
use valence_core::artifact_auth::map_valence_artifact_auth_statement;
use valence_core::cob_evidence::CobAuthorizationProfile;
use valence_core::cob_evidence::CobEvidenceProfile;
use valence_core::cob_evidence::CobOperation;
use valence_core::cob_evidence::ObservedCobEvent;
use valence_core::cob_evidence::actor_is_authorized;
use valence_core::cob_evidence::validate_authorization_profile;
use valence_core::evidence_ir::EvidenceArtifact;
use valence_core::evidence_ir::REQUIRED_NON_CLAIM;
use valence_core::evidence_ir::VerificationRole;
use valence_core::evidence_ir::build_evidence_artifact;
use valence_core::evidence_ir::check_evidence_artifact;

use crate::AdapterError;

const BLAKE3_REF_PREFIX: &str = "blake3:";
const BLAKE3_HEX_CHARS: usize = 64;
const HEX_DIGIT_COUNT: usize = 16;
const HEX_DIGITS: &[u8; HEX_DIGIT_COUNT] = b"0123456789abcdef";
const BITS_PER_NIBBLE: u32 = 4;
const LOW_NIBBLE_MASK: u8 = 0x0f;
const HEX_CHARS_PER_BYTE: usize = 2;
const GIT_OID_HEX_CHARS: usize = 40;
const OPERATIONAL_RECEIPT_SCHEMA: &str = "valence-artifact-auth-operational-receipt-v1";
const OPERATIONAL_RECEIPT_KIND: &str = "valence-artifact-auth-operational-receipt-v1";
const OPERATIONAL_RECEIPT_NON_CLAIM: &str = "persisted replay proves exact standalone bytes, bounded authorization identity, and signed COB linkage only; it does not replace Radicle protocol verification, actor authorization, COB reduction, Git or SSH effects, lifecycle handoff, release interpretation, or runtime authority";

#[derive(Debug, Clone, Copy)]
pub struct ValenceArtifactAuthShellInput<'a> {
    pub statement: ValenceArtifactAuthStatementInput<'a>,
    pub evidence_profile: &'a CobEvidenceProfile,
    pub authorization: &'a CobAuthorizationProfile,
    pub operation: CobOperation,
    pub actor_authorization_ref: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedValenceArtifactAuthStatement {
    pub statement_ref: String,
    pub public_key: String,
    pub public_key_ref: String,
    pub signature_ref: String,
    pub signature_hex: String,
    pub signature_bytes: Vec<u8>,
}

#[cfg(test)]
impl SignedValenceArtifactAuthStatement {
    pub(crate) fn replace_signature_bytes_for_test(&mut self, signature_bytes: Vec<u8>) {
        self.signature_ref = content_ref(&signature_bytes);
        self.signature_hex = bytes_to_lower_hex(&signature_bytes);
        self.signature_bytes = signature_bytes;
    }

    pub(crate) fn replace_public_key_for_test(&mut self, public_key: radicle_crypto::PublicKey) {
        self.public_key_ref = content_ref(&public_key.to_byte_array());
        self.public_key = public_key.to_string();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValenceArtifactAuthShellReport {
    pub statement_ref: String,
    pub public_key_ref: String,
    pub signature_ref: String,
    pub signature_hex: String,
    pub signing_authorization_observation_ref: String,
    pub actor_authorization_observed: bool,
    pub cryptographic_failure_code: Option<String>,
    pub dual_run: ValenceArtifactAuthReport,
}

#[derive(Debug, Clone, Copy)]
pub struct ValenceArtifactAuthOperationalInput<'a> {
    pub observed: &'a ObservedCobEvent,
    pub related_target: &'a str,
    pub profile_id: &'a str,
    pub profile_context: &'a str,
    pub protocol_key_identity_blake3: &'a str,
    pub evidence_profile: &'a CobEvidenceProfile,
    pub authorization: &'a CobAuthorizationProfile,
    pub operation: CobOperation,
    pub authorization_observation_ref: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValenceArtifactAuthOperationalReceipt {
    pub schema: String,
    pub statement_ref: String,
    pub public_key_ref: String,
    pub signature_ref: String,
    pub signing_authorization_ref: String,
    pub authorization_hash: String,
    pub operation: String,
    pub entry_oid: String,
    pub generation: u64,
    pub currentness: String,
    pub signed: SignedValenceArtifactAuthStatement,
    pub standalone_passed: bool,
    pub compatibility_case_explained: bool,
    pub legacy_authoritative: bool,
    pub standalone_authority_admitted: bool,
    pub rollback_available: bool,
    pub non_claim: String,
    pub receipt_blake3: String,
}

#[derive(Serialize)]
struct ValenceArtifactAuthReceiptMaterial<'a> {
    schema: &'a str,
    statement_ref: &'a str,
    public_key_ref: &'a str,
    signature_ref: &'a str,
    signing_authorization_ref: &'a str,
    authorization_hash: &'a str,
    operation: &'a str,
    entry_oid: &'a str,
    generation: u64,
    currentness: &'a str,
    signed: &'a SignedValenceArtifactAuthStatement,
    standalone_passed: bool,
    compatibility_case_explained: bool,
    legacy_authoritative: bool,
    standalone_authority_admitted: bool,
    rollback_available: bool,
    non_claim: &'a str,
}

// r[impl valence.artifact_auth_operational_receipt.currentness]
pub fn derive_valence_artifact_auth_operational_shell_input<'a>(
    input: &'a ValenceArtifactAuthOperationalInput<'a>,
) -> Result<ValenceArtifactAuthShellInput<'a>, AdapterError> {
    let authorization_issues = validate_authorization_profile(input.evidence_profile, input.authorization);
    if !authorization_issues.is_empty() {
        return Err(AdapterError::Input(format!(
            "artifact-auth operational authorization profile is invalid: {}",
            authorization_issues.into_iter().map(|issue| issue.code).collect::<Vec<_>>().join(",")
        )));
    }
    if !input.observed.signature_valid {
        return Err(AdapterError::Input(
            "artifact-auth operational currentness requires protocol verification".to_owned(),
        ));
    }
    if !actor_is_authorized(input.evidence_profile, input.authorization, &input.observed.signer, input.operation) {
        return Err(AdapterError::Input(
            "artifact-auth operational currentness requires actor authorization".to_owned(),
        ));
    }
    let generation = input
        .observed
        .operation_index
        .checked_add(1)
        .map(u64::from)
        .ok_or_else(|| AdapterError::Input("artifact-auth operational generation overflow".to_owned()))?;
    let expected_authorization_ref = valence_authorization_observation_ref(input.authorization);
    require_carrier_identity("authorization", input.authorization_observation_ref, &expected_authorization_ref)?;
    let statement = ValenceArtifactAuthStatementInput {
        observed: input.observed,
        related_target: input.related_target,
        profile_id: input.profile_id,
        profile_context: input.profile_context,
        generation,
        currentness: artifact_auth_core::KeyCurrentness::Current,
        currentness_context: &input.authorization.authorization_hash,
        protocol_key_identity_blake3: input.protocol_key_identity_blake3,
    };
    debug_assert!(generation > 0);
    debug_assert_eq!(statement.currentness_context, input.authorization.authorization_hash);
    Ok(ValenceArtifactAuthShellInput {
        statement,
        evidence_profile: input.evidence_profile,
        authorization: input.authorization,
        operation: input.operation,
        actor_authorization_ref: input.authorization_observation_ref,
    })
}

// r[impl valence.artifact_auth_operational_receipt.identity]
pub fn capture_valence_artifact_auth_operational_artifact<G>(
    signer: &G,
    input: &ValenceArtifactAuthOperationalInput<'_>,
) -> Result<EvidenceArtifact, AdapterError>
where
    G: radicle_crypto::Signer,
{
    let shell_input = derive_valence_artifact_auth_operational_shell_input(input)?;
    let signed = sign_valence_artifact_auth_statement(signer, &shell_input)?;
    let shell_report = evaluate_valence_artifact_auth_shell(&shell_input, &signed)?;
    build_valence_artifact_auth_operational_artifact(&shell_input, &signed, &shell_report)
}

// r[impl valence.artifact_auth_operational_receipt.persistence]
pub fn replay_valence_artifact_auth_operational_artifact(
    input: &ValenceArtifactAuthOperationalInput<'_>,
    artifact: &EvidenceArtifact,
) -> Result<ValenceArtifactAuthShellReport, AdapterError> {
    let receipt = validate_valence_artifact_auth_operational_artifact(artifact)?;
    let shell_input = derive_valence_artifact_auth_operational_shell_input(input)?;
    let shell_report = evaluate_valence_artifact_auth_shell(&shell_input, &receipt.signed)?;
    let expected = build_valence_artifact_auth_operational_artifact(&shell_input, &receipt.signed, &shell_report)?;
    let expected_value = serde_json::to_value(&expected).map_err(|error| AdapterError::Input(error.to_string()))?;
    let observed_value = serde_json::to_value(artifact).map_err(|error| AdapterError::Input(error.to_string()))?;
    if expected_value != observed_value {
        return Err(AdapterError::Input("artifact-auth operational replay drifted".to_owned()));
    }
    debug_assert!(shell_report.dual_run.compatibility.legacy_authoritative);
    debug_assert!(!shell_report.dual_run.compatibility.standalone_authority_admitted);
    Ok(shell_report)
}

fn build_valence_artifact_auth_operational_artifact(
    input: &ValenceArtifactAuthShellInput<'_>,
    signed: &SignedValenceArtifactAuthStatement,
    report: &ValenceArtifactAuthShellReport,
) -> Result<EvidenceArtifact, AdapterError> {
    let is_standalone_passed = report.dual_run.standalone.as_ref().is_some_and(|decision| decision.passed);
    let compatibility = &report.dual_run.compatibility;
    if report.cryptographic_failure_code.is_some()
        || !report.actor_authorization_observed
        || !is_standalone_passed
        || !compatibility.case_explained
    {
        return Err(AdapterError::Input(
            "artifact-auth operational receipt requires passing explained authorized evidence".to_owned(),
        ));
    }
    if report.statement_ref != signed.statement_ref
        || report.public_key_ref != signed.public_key_ref
        || report.signature_ref != signed.signature_ref
        || report.signing_authorization_observation_ref != input.actor_authorization_ref
    {
        return Err(AdapterError::Input("artifact-auth operational shell carrier drifted".to_owned()));
    }
    let mut receipt = ValenceArtifactAuthOperationalReceipt {
        schema: OPERATIONAL_RECEIPT_SCHEMA.to_owned(),
        statement_ref: signed.statement_ref.clone(),
        public_key_ref: signed.public_key_ref.clone(),
        signature_ref: signed.signature_ref.clone(),
        signing_authorization_ref: input.actor_authorization_ref.to_owned(),
        authorization_hash: input.authorization.authorization_hash.clone(),
        operation: input.operation.as_str().to_owned(),
        entry_oid: input.statement.observed.entry_oid.clone(),
        generation: input.statement.generation,
        currentness: "current".to_owned(),
        signed: signed.clone(),
        standalone_passed: is_standalone_passed,
        compatibility_case_explained: compatibility.case_explained,
        legacy_authoritative: compatibility.legacy_authoritative,
        standalone_authority_admitted: compatibility.standalone_authority_admitted,
        rollback_available: compatibility.rollback_available,
        non_claim: OPERATIONAL_RECEIPT_NON_CLAIM.to_owned(),
        receipt_blake3: String::new(),
    };
    receipt.receipt_blake3 = operational_receipt_identity(&receipt)?;
    validate_operational_receipt(&receipt)?;
    let payload = serde_json::to_value(&receipt).map_err(|error| AdapterError::Input(error.to_string()))?;
    let mut non_claims = input.evidence_profile.required_non_claims.clone();
    non_claims.push(REQUIRED_NON_CLAIM.to_owned());
    non_claims.push(OPERATIONAL_RECEIPT_NON_CLAIM.to_owned());
    non_claims.sort();
    non_claims.dedup();
    let artifact = build_evidence_artifact(
        OPERATIONAL_RECEIPT_KIND,
        &receipt.receipt_blake3,
        None,
        VerificationRole::RecordedOnly,
        payload,
        non_claims,
    )
    .map_err(|error| AdapterError::Input(error.to_string()))?;
    validate_valence_artifact_auth_operational_artifact(&artifact)?;
    Ok(artifact)
}

// r[impl valence.artifact_auth_operational_receipt.identity]
pub fn validate_valence_artifact_auth_operational_artifact(
    artifact: &EvidenceArtifact,
) -> Result<ValenceArtifactAuthOperationalReceipt, AdapterError> {
    let encoded = serde_json::to_string(artifact).map_err(|error| AdapterError::Input(error.to_string()))?;
    let checked = check_evidence_artifact(&encoded);
    if !checked.valid {
        return Err(AdapterError::Input(format!(
            "artifact-auth operational Evidence IR is invalid: {}",
            checked.issue_codes.join(",")
        )));
    }
    if artifact.artifact_kind != OPERATIONAL_RECEIPT_KIND
        || artifact.verification_role != VerificationRole::RecordedOnly
        || !artifact.non_claims.iter().any(|claim| claim == OPERATIONAL_RECEIPT_NON_CLAIM)
    {
        return Err(AdapterError::Input("artifact-auth operational Evidence IR role or boundary drifted".to_owned()));
    }
    let receipt = serde_json::from_value::<ValenceArtifactAuthOperationalReceipt>(artifact.payload.clone())
        .map_err(|error| AdapterError::Input(format!("artifact-auth operational payload is invalid: {error}")))?;
    validate_operational_receipt(&receipt)?;
    if artifact.stable_id != receipt.receipt_blake3 {
        return Err(AdapterError::Input("artifact-auth operational stable identity does not match receipt".to_owned()));
    }
    Ok(receipt)
}

fn validate_operational_receipt(receipt: &ValenceArtifactAuthOperationalReceipt) -> Result<(), AdapterError> {
    if receipt.schema != OPERATIONAL_RECEIPT_SCHEMA
        || receipt.statement_ref != receipt.signed.statement_ref
        || receipt.public_key_ref != receipt.signed.public_key_ref
        || receipt.signature_ref != receipt.signed.signature_ref
    {
        return Err(AdapterError::Input("artifact-auth operational receipt carrier drifted".to_owned()));
    }
    for (label, value) in [
        ("statement", receipt.statement_ref.as_str()),
        ("public key", receipt.public_key_ref.as_str()),
        ("signature", receipt.signature_ref.as_str()),
        ("authorization observation", receipt.signing_authorization_ref.as_str()),
        ("receipt", receipt.receipt_blake3.as_str()),
    ] {
        require_blake3_ref(label, value)?;
    }
    if !valid_b3_ref(&receipt.authorization_hash)
        || receipt.entry_oid.len() != GIT_OID_HEX_CHARS
        || !receipt.entry_oid.bytes().all(|byte| byte.is_ascii_hexdigit())
        || receipt.generation == 0
        || receipt.currentness != "current"
        || !receipt.standalone_passed
        || !receipt.compatibility_case_explained
        || !receipt.legacy_authoritative
        || receipt.standalone_authority_admitted
        || !receipt.rollback_available
        || receipt.non_claim != OPERATIONAL_RECEIPT_NON_CLAIM
    {
        return Err(AdapterError::Input(
            "artifact-auth operational receipt key state or authority boundary drifted".to_owned(),
        ));
    }
    let public_key = radicle_crypto::PublicKey::from_str(&receipt.signed.public_key)
        .map_err(|_| AdapterError::Input("artifact-auth operational public key is malformed".to_owned()))?;
    require_carrier_identity("public key", &receipt.public_key_ref, &content_ref(&public_key.to_byte_array()))?;
    require_carrier_identity("signature", &receipt.signature_ref, &content_ref(&receipt.signed.signature_bytes))?;
    require_carrier_identity(
        "signature hex",
        &receipt.signed.signature_hex,
        &bytes_to_lower_hex(&receipt.signed.signature_bytes),
    )?;
    require_carrier_identity("receipt", &receipt.receipt_blake3, &operational_receipt_identity(receipt)?)?;
    Ok(())
}

pub fn valence_authorization_observation_ref(authorization: &CobAuthorizationProfile) -> String {
    content_ref(authorization.authorization_hash.as_bytes())
}

fn valid_b3_ref(value: &str) -> bool {
    value.strip_prefix("b3:").is_some_and(|digest| {
        digest.len() == BLAKE3_HEX_CHARS
            && digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
}

fn operational_receipt_identity(receipt: &ValenceArtifactAuthOperationalReceipt) -> Result<String, AdapterError> {
    let material = ValenceArtifactAuthReceiptMaterial {
        schema: &receipt.schema,
        statement_ref: &receipt.statement_ref,
        public_key_ref: &receipt.public_key_ref,
        signature_ref: &receipt.signature_ref,
        signing_authorization_ref: &receipt.signing_authorization_ref,
        authorization_hash: &receipt.authorization_hash,
        operation: &receipt.operation,
        entry_oid: &receipt.entry_oid,
        generation: receipt.generation,
        currentness: &receipt.currentness,
        signed: &receipt.signed,
        standalone_passed: receipt.standalone_passed,
        compatibility_case_explained: receipt.compatibility_case_explained,
        legacy_authoritative: receipt.legacy_authoritative,
        standalone_authority_admitted: receipt.standalone_authority_admitted,
        rollback_available: receipt.rollback_available,
        non_claim: &receipt.non_claim,
    };
    let bytes = serde_json::to_vec(&material).map_err(|error| AdapterError::Input(error.to_string()))?;
    Ok(content_ref(&bytes))
}

// r[impl valence.artifact_auth_shell.exact_verification]
// r[impl valence.artifact_auth_shell.evidence]
pub fn sign_valence_artifact_auth_statement<G>(
    signer: &G,
    input: &ValenceArtifactAuthShellInput<'_>,
) -> Result<SignedValenceArtifactAuthStatement, AdapterError>
where
    G: radicle_crypto::Signer,
{
    admit_signing_input(signer, input)?;
    let statement = map_statement(&input.statement)?;
    let statement_bytes = artifact_auth_core::canonical_statement_bytes(&statement)
        .map_err(|_| AdapterError::Input("artifact-auth statement is not canonicalizable".to_owned()))?;
    let signature: radicle_crypto::Signature = signer
        .try_sign(&statement_bytes)
        .map_err(|_| AdapterError::Input("dedicated artifact-auth signing failed".to_owned()))?;
    let public_key_bytes = signer.public_key().to_byte_array();
    let signature_bytes = signature.as_ref().to_vec();
    let result = SignedValenceArtifactAuthStatement {
        statement_ref: content_ref(&statement_bytes),
        public_key: signer.public_key().to_string(),
        public_key_ref: content_ref(&public_key_bytes),
        signature_ref: content_ref(&signature_bytes),
        signature_hex: bytes_to_lower_hex(&signature_bytes),
        signature_bytes,
    };
    debug_assert_eq!(result.signature_bytes.len(), artifact_auth_ed25519::ED25519_SIGNATURE_BYTES);
    debug_assert_eq!(result.public_key_ref, format!("{BLAKE3_REF_PREFIX}{}", statement.key_identity.digest_hex));
    Ok(result)
}

// r[impl valence.artifact_auth_shell.exact_verification]
// r[impl valence.artifact_auth_shell.evidence]
// r[impl valence.artifact_auth_shell.authority]
pub fn evaluate_valence_artifact_auth_shell(
    input: &ValenceArtifactAuthShellInput<'_>,
    signed: &SignedValenceArtifactAuthStatement,
) -> Result<ValenceArtifactAuthShellReport, AdapterError> {
    let statement = map_statement(&input.statement)?;
    let statement_bytes = artifact_auth_core::canonical_statement_bytes(&statement)
        .map_err(|_| AdapterError::Input("artifact-auth statement is not canonicalizable".to_owned()))?;
    require_carrier_identity("statement", &signed.statement_ref, &content_ref(&statement_bytes))?;
    require_blake3_ref("actor authorization", input.actor_authorization_ref)?;
    let public_key = radicle_crypto::PublicKey::from_str(&signed.public_key)
        .map_err(|_| AdapterError::Input("artifact-auth carrier public key is malformed".to_owned()))?;
    let public_key_bytes = public_key.to_byte_array();
    require_carrier_identity("public key", &signed.public_key_ref, &content_ref(&public_key_bytes))?;
    require_carrier_identity("signature", &signed.signature_ref, &content_ref(&signed.signature_bytes))?;
    require_carrier_identity("signature hex", &signed.signature_hex, &bytes_to_lower_hex(&signed.signature_bytes))?;
    let cryptographic = artifact_auth_ed25519::verify_statement(&statement, &public_key_bytes, &signed.signature_bytes);
    let cryptographic_failure_code = cryptographic.failure_code.clone();
    let dual_run = evaluate_valence_artifact_auth(&ValenceArtifactAuthObservation {
        observed: input.statement.observed,
        related_target: input.statement.related_target,
        profile_id: input.statement.profile_id,
        profile_context: input.statement.profile_context,
        generation: input.statement.generation,
        currentness: input.statement.currentness,
        currentness_context: input.statement.currentness_context,
        protocol_key_identity_blake3: input.statement.protocol_key_identity_blake3,
        standalone_cryptographic: cryptographic,
    });
    let result = ValenceArtifactAuthShellReport {
        statement_ref: signed.statement_ref.clone(),
        public_key_ref: signed.public_key_ref.clone(),
        signature_ref: signed.signature_ref.clone(),
        signature_hex: signed.signature_hex.clone(),
        signing_authorization_observation_ref: input.actor_authorization_ref.to_owned(),
        actor_authorization_observed: actor_is_authorized(
            input.evidence_profile,
            input.authorization,
            &input.statement.observed.signer,
            input.operation,
        ),
        cryptographic_failure_code,
        dual_run,
    };
    debug_assert!(result.dual_run.compatibility.legacy_authoritative);
    debug_assert!(!result.dual_run.compatibility.standalone_authority_admitted);
    Ok(result)
}

fn admit_signing_input<G>(signer: &G, input: &ValenceArtifactAuthShellInput<'_>) -> Result<(), AdapterError>
where G: radicle_crypto::Signer {
    if !input.statement.observed.signature_valid {
        return Err(AdapterError::Input("dedicated artifact-auth signing requires protocol verification".to_owned()));
    }
    if !actor_is_authorized(
        input.evidence_profile,
        input.authorization,
        &input.statement.observed.signer,
        input.operation,
    ) {
        return Err(AdapterError::Input("dedicated artifact-auth signing requires actor authorization".to_owned()));
    }
    require_blake3_ref("actor authorization", input.actor_authorization_ref)?;
    if signer.public_key().to_string() != input.statement.observed.signer {
        return Err(AdapterError::Input("dedicated artifact-auth signer does not match the protocol actor".to_owned()));
    }
    if input.statement.currentness != artifact_auth_core::KeyCurrentness::Current {
        return Err(AdapterError::Input("dedicated artifact-auth signing requires a current actor key".to_owned()));
    }
    let key_identity = artifact_auth_ed25519::public_key_identity(&signer.public_key().to_byte_array());
    if key_identity.digest_hex != input.statement.protocol_key_identity_blake3 {
        return Err(AdapterError::Input("dedicated artifact-auth signer full-key identity mismatch".to_owned()));
    }
    debug_assert!(input.statement.observed.signature_valid);
    debug_assert!(actor_is_authorized(
        input.evidence_profile,
        input.authorization,
        &input.statement.observed.signer,
        input.operation,
    ));
    Ok(())
}

fn map_statement(
    input: &ValenceArtifactAuthStatementInput<'_>,
) -> Result<artifact_auth_core::ArtifactStatement, AdapterError> {
    map_valence_artifact_auth_statement(input)
        .map_err(|issues| AdapterError::Input(format!("artifact-auth statement mapping failed: {}", issues.join(","))))
}

fn require_blake3_ref(label: &str, value: &str) -> Result<(), AdapterError> {
    let Some(digest_hex) = value.strip_prefix(BLAKE3_REF_PREFIX) else {
        return Err(AdapterError::Input(format!("{label} must be a BLAKE3 reference")));
    };
    if digest_hex.len() != BLAKE3_HEX_CHARS
        || !digest_hex.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(AdapterError::Input(format!("{label} BLAKE3 reference is malformed")));
    }
    Ok(())
}

fn require_carrier_identity(label: &str, observed: &str, expected: &str) -> Result<(), AdapterError> {
    if observed != expected {
        return Err(AdapterError::Input(format!("artifact-auth carrier {label} identity mismatch")));
    }
    Ok(())
}

fn content_ref(bytes: &[u8]) -> String {
    format!("{BLAKE3_REF_PREFIX}{}", blake3::hash(bytes).to_hex())
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len().saturating_mul(HEX_CHARS_PER_BYTE));
    for byte in bytes {
        let high = usize::from(byte >> BITS_PER_NIBBLE);
        let low = usize::from(byte & LOW_NIBBLE_MASK);
        encoded.push(char::from(HEX_DIGITS[high]));
        encoded.push(char::from(HEX_DIGITS[low]));
    }
    debug_assert_eq!(encoded.len(), bytes.len().saturating_mul(HEX_CHARS_PER_BYTE));
    encoded
}
