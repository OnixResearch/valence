use std::str::FromStr;

use valence_core::artifact_auth::ValenceArtifactAuthObservation;
use valence_core::artifact_auth::ValenceArtifactAuthReport;
use valence_core::artifact_auth::ValenceArtifactAuthStatementInput;
use valence_core::artifact_auth::evaluate_valence_artifact_auth;
use valence_core::artifact_auth::map_valence_artifact_auth_statement;
use valence_core::cob_evidence::CobAuthorizationProfile;
use valence_core::cob_evidence::CobEvidenceProfile;
use valence_core::cob_evidence::CobOperation;
use valence_core::cob_evidence::actor_is_authorized;

use crate::AdapterError;

const BLAKE3_REF_PREFIX: &str = "blake3:";
const BLAKE3_HEX_CHARS: usize = 64;
const HEX_DIGIT_COUNT: usize = 16;
const HEX_DIGITS: &[u8; HEX_DIGIT_COUNT] = b"0123456789abcdef";
const BITS_PER_NIBBLE: u32 = 4;
const LOW_NIBBLE_MASK: u8 = 0x0f;
const HEX_CHARS_PER_BYTE: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct ValenceArtifactAuthShellInput<'a> {
    pub statement: ValenceArtifactAuthStatementInput<'a>,
    pub evidence_profile: &'a CobEvidenceProfile,
    pub authorization: &'a CobAuthorizationProfile,
    pub operation: CobOperation,
    pub actor_authorization_ref: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
