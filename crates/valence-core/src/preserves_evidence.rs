//! Pure Preserves evidence identity, migration, and Preserves-to-Valence bridge
//! validation.
//!
//! This module validates already-loaded Preserves values, canonical binary
//! payloads, historical private-format v2 reproductions, and typed claim
//! boundaries. It performs no I/O and grants no behavioral, authority, or
//! release claims.
//!
//! Profile history:
//! - `preserves-text-v1` (legacy, rejected): vulnerable line-filtering text hashing.
//! - `preserves-packed-structural-v2` (historical private format, rejected for new writes): private
//!   count-prefixed S-expression list encoding retained only for explicit historical receipt
//!   reproduction and migration.
//! - `preserves-canonical-binary-v3` (active): bounded upstream Preserves 0.996.3 canonical binary
//!   encoding with BLAKE3 content identities and structural self-hash exclusion.

use serde::Deserialize;
use serde::Serialize;

use crate::preserves_canonical::PreservesLimits;
use crate::preserves_canonical::PreservesValue;
use crate::preserves_canonical::decode_canonical_binary;
use crate::preserves_canonical::encode_canonical_binary;
use crate::preserves_canonical::parse_preserves_text;
use crate::shared_identity::MIXED_BLAKE3_PROFILE;
use crate::shared_identity::PreservesMetadataDigest;
use crate::shared_identity::blake3_hex;

const FIELD_ARTIFACT_ROLE: &str = "artifact_role";
const FIELD_CANONICAL_BYTES: &str = "canonical_bytes";
const FIELD_CLAIM_BOUNDARY: &str = "claim_boundary";
const FIELD_CONTENT_HASH: &str = "preserves_content_hash";
const FIELD_EXCLUSION_PROFILE: &str = "self_hash_exclusion_profile";
const FIELD_NON_CLAIMS: &str = "non_claims";
const FIELD_PRESERVES_TEXT: &str = "preserves_text";
const FIELD_PROFILE: &str = "canonicalization_profile";
const FIELD_SCHEMA: &str = "schema";
const FIELD_VERIFICATION_ROLE: &str = "verification_role";
const HASH_PREFIX: &str = "b3:";

// Historical private-format v2 tags (NOT upstream Preserves binary tags).
const HISTORICAL_V2_LIST_TAG: u8 = 0x80;
const HISTORICAL_V2_STRING_TAG: u8 = 0xb1;
const HISTORICAL_V2_SYMBOL_TAG: u8 = 0xb0;

pub const PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM: &str =
    "Preserves evidence validation proves only canonical Preserves content identity and bounded envelope compatibility";
pub const PRESERVES_BRIDGE_REQUIRED_NON_CLAIM: &str =
    "Preserves bridge validation proves only identity, schema, role, and non-claim mapping";

/// Active upstream-compatible Preserves 0.996.3 canonical binary profile.
pub const UPSTREAM_CANONICAL_BINARY_PROFILE: &str = "preserves-canonical-binary-v3";
/// Active alias pointing to the upstream-compatible canonical binary profile.
pub const CORRECTED_CANONICALIZATION_PROFILE: &str = UPSTREAM_CANONICAL_BINARY_PROFILE;

/// Active v3 structural self-hash exclusion profiles.
pub const UPSTREAM_RECEIPT_EXCLUSION_PROFILE: &str = "receipt-self-hash-canonical-v3";
pub const UPSTREAM_CONTENT_EXCLUSION_PROFILE: &str = "content-hash-self-canonical-v3";
pub const UPSTREAM_NO_EXCLUSION_PROFILE: &str = "none-canonical-v3";

/// Active aliases pointing to v3 exclusion profiles.
pub const CORRECTED_RECEIPT_EXCLUSION_PROFILE: &str = UPSTREAM_RECEIPT_EXCLUSION_PROFILE;
pub const CORRECTED_CONTENT_EXCLUSION_PROFILE: &str = UPSTREAM_CONTENT_EXCLUSION_PROFILE;
pub const NO_SELF_HASH_EXCLUSION_PROFILE: &str = UPSTREAM_NO_EXCLUSION_PROFILE;

/// Explicitly identified historical private-format v2 profile (NOT upstream
/// Preserves binary). Retained only for reproducing historical receipts and
/// authoring versioned migration receipts.
pub const HISTORICAL_PRIVATE_PACKED_PROFILE: &str = "preserves-packed-structural-v2";
pub const HISTORICAL_RECEIPT_EXCLUSION_PROFILE: &str = "receipt-self-hash-structural-v2";
pub const HISTORICAL_CONTENT_EXCLUSION_PROFILE: &str = "content-hash-self-structural-v2";
pub const HISTORICAL_NO_EXCLUSION_PROFILE: &str = "none-structural-v2";

/// Vulnerable legacy line-filtering text profile.
pub const LEGACY_CANONICALIZATION_PROFILE: &str = "preserves-text-v1";

/// Historical v1-to-v2 migration receipt profile.
pub const LEGACY_MIGRATION_PROFILE: &str = "preserves-integrity-migration-v1";
/// Active versioned migration receipt profile into `preserves-canonical-binary-v3`.
pub const PRESERVES_PROFILE_MIGRATION_V2: &str = "preserves-integrity-migration-v2";
pub const REQUIRED_IDENTITY_BOUNDARY: &str = "valence.identity-linkage-only.v1";

const PRESERVES_SUPPORTED_CLAIM: &str = "Preserves evidence validation proves only canonical Preserves 0.996.3 content identity and bounded envelope compatibility; it does not prove receipt truth, release eligibility, lifecycle readiness, semantic equivalence, behavioral correctness, verifier soundness, execution authority, or whole-program safety";
const HISTORICAL_V2_SUPPORTED_CLAIM: &str = "Historical preserves-packed-structural-v2 validation reproduces private-format v2 digest identity only; it is NOT upstream canonical Preserves and does not prove receipt truth, behavioral correctness, execution authority, or release eligibility";
const PRESERVES_BRIDGE_OPAQUE_SUPPORTED_CLAIM: &str = "Preserves opaque-byte bridge validation proves opaque-byte digest binding, schema, role, and non-claim mapping only; it does not parse or verify canonical Preserves binary syntax, Molten runtime authority, transport correctness, or Preserves application semantics";
const PRESERVES_BRIDGE_CANONICAL_SUPPORTED_CLAIM: &str = "Preserves canonical bridge validation proves single-value upstream Preserves 0.996.3 canonical binary syntax, BLAKE3 digest binding, schema, role, and non-claim mapping only; it does not prove receipt truth, behavioral correctness, execution authority, or release eligibility";

const SUPPORTED_PRESERVES_SCHEMAS: &[&str] = &[
    "valence.preserves-evidence.v1",
    "cairn.preserves-receipt.v1",
    "mantle.release-evidence.preserves.v1",
    "octet.evidence-receipt.preserves.v1",
    "molten.preserves-envelope.v1",
    "kamacite.preserves-record.v1",
];
const SUPPORTED_ARTIFACT_ROLES: &[&str] = &[
    "artifact_identity",
    "receipt_identity",
    "preserves_metadata",
    "compatibility_reference",
];
const SUPPORTED_VERIFICATION_ROLES: &[&str] = &["property", "recorded_only", "boundary"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesEvidenceEnvelope {
    pub schema: String,
    pub canonicalization_profile: String,
    pub preserves_text: String,
    pub preserves_content_hash: String,
    pub self_hash_exclusion_profile: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesBridgeRow {
    pub schema_label: String,
    pub canonical_bytes: Vec<u8>,
    pub preserves_content_hash: String,
    pub artifact_role: String,
    pub valence_artifact_kind: String,
    pub verification_role: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedClaimClass {
    Identity,
    Linkage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedVerificationRole {
    RecordedOnly,
    Boundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProhibitedCapability {
    BehavioralCorrectness,
    SemanticEquivalence,
    Authority,
    VerifierSoundness,
    ReleaseEligibility,
}

impl ProhibitedCapability {
    const fn phrase(self) -> &'static str {
        match self {
            Self::BehavioralCorrectness => "behavioral correctness",
            Self::SemanticEquivalence => "semantic equivalence",
            Self::Authority => "authority",
            Self::VerifierSoundness => "verifier soundness",
            Self::ReleaseEligibility => "release eligibility",
        }
    }
}

const PROHIBITED_CAPABILITIES: &[ProhibitedCapability] = &[
    ProhibitedCapability::BehavioralCorrectness,
    ProhibitedCapability::SemanticEquivalence,
    ProhibitedCapability::Authority,
    ProhibitedCapability::VerifierSoundness,
    ProhibitedCapability::ReleaseEligibility,
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedClaimBoundary {
    pub claim_class: SupportedClaimClass,
    pub verification_role: TypedVerificationRole,
    pub required_boundary_ids: Vec<String>,
    pub asserted_capabilities: Vec<ProhibitedCapability>,
    pub display_metadata: Vec<String>,
}

/// Historical v1-to-v2 migration receipt retained for archived migration records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesMigrationReceipt {
    pub migration_profile: String,
    pub original_bytes_hash: String,
    pub legacy_identity: String,
    pub corrected_identity: String,
    pub non_claims: Vec<String>,
}

/// Versioned migration receipt binding exact original bytes, source encoding
/// profile, source exclusion rule, source identity, target upstream canonical
/// Preserves profile (`preserves-canonical-binary-v3`), target exclusion rule,
/// target identity, and required non-claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesProfileMigrationReceipt {
    pub migration_profile: String,
    pub original_bytes_hash: String,
    pub source_profile: String,
    pub source_exclusion_profile: String,
    pub source_identity: String,
    pub target_profile: String,
    pub target_exclusion_profile: String,
    pub target_identity: String,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PreservesEvidenceIssue {
    pub field: &'static str,
    pub code: &'static str,
}

impl PreservesEvidenceIssue {
    #[must_use]
    pub const fn new(field: &'static str, code: &'static str) -> Self {
        Self { field, code }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PreservesEvidenceReport {
    pub valid: bool,
    pub normalized_content_hash: Option<String>,
    pub issues: Vec<PreservesEvidenceIssue>,
    pub supported_claim: String,
}

/// Validate an active Preserves evidence envelope under the upstream-compatible
/// `preserves-canonical-binary-v3` profile.
///
/// Rejects both `preserves-text-v1` and historical private-format
/// `preserves-packed-structural-v2` so new writes cannot silently advertise a
/// legacy or private encoding as upstream canonical Preserves.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.profile]
/// r[impl valence.evidence_ir.preserves_canonical_v3.self_hash_exclusion]
#[must_use]
pub fn validate_preserves_evidence(envelope: &PreservesEvidenceEnvelope) -> PreservesEvidenceReport {
    validate_preserves_evidence_with_limits(envelope, PreservesLimits::default())
}

/// Validate an active `preserves-canonical-binary-v3` envelope with explicit
/// [`PreservesLimits`].
#[must_use]
pub fn validate_preserves_evidence_with_limits(
    envelope: &PreservesEvidenceEnvelope,
    limits: PreservesLimits,
) -> PreservesEvidenceReport {
    let mut issues = Vec::new();
    validate_schema(&envelope.schema, &mut issues);
    match envelope.canonicalization_profile.as_str() {
        UPSTREAM_CANONICAL_BINARY_PROFILE => {}
        HISTORICAL_PRIVATE_PACKED_PROFILE => {
            issues.push(PreservesEvidenceIssue::new(FIELD_PROFILE, "historical_private_profile_rejected"));
        }
        LEGACY_CANONICALIZATION_PROFILE => {
            issues.push(PreservesEvidenceIssue::new(FIELD_PROFILE, "legacy_profile_rejected"));
        }
        _ => {
            issues.push(PreservesEvidenceIssue::new(FIELD_PROFILE, "unsupported_canonicalization_profile"));
        }
    }
    if !is_supported_v3_exclusion_profile(&envelope.self_hash_exclusion_profile) {
        issues.push(PreservesEvidenceIssue::new(FIELD_EXCLUSION_PROFILE, "unknown_self_hash_exclusion_profile"));
    }
    validate_non_claim_metadata(&envelope.non_claims, PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM, &mut issues);

    let expected = match canonical_preserves_bytes_with_limits(
        &envelope.preserves_text,
        &envelope.self_hash_exclusion_profile,
        limits,
    ) {
        Ok(bytes) => Some(blake3_hex(&bytes)),
        Err(code) => {
            issues.push(PreservesEvidenceIssue::new(FIELD_PRESERVES_TEXT, code));
            None
        }
    };
    validate_declared_hash(&envelope.preserves_content_hash, expected.as_deref(), &mut issues);
    finish_report(expected, issues, PRESERVES_SUPPORTED_CLAIM)
}

/// Explicitly validate a historical `preserves-packed-structural-v2` receipt
/// under its original private encoding.
///
/// Never reinterprets historical v2 hashes as upstream canonical Preserves
/// hashes; callers must opt into this entry point explicitly.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.historical_v2]
#[must_use]
pub fn validate_historical_preserves_v2_evidence(envelope: &PreservesEvidenceEnvelope) -> PreservesEvidenceReport {
    let mut issues = Vec::new();
    validate_schema(&envelope.schema, &mut issues);
    if envelope.canonicalization_profile != HISTORICAL_PRIVATE_PACKED_PROFILE {
        issues.push(PreservesEvidenceIssue::new(FIELD_PROFILE, "expected_historical_private_profile"));
    }
    if !is_supported_v2_exclusion_profile(&envelope.self_hash_exclusion_profile) {
        issues.push(PreservesEvidenceIssue::new(FIELD_EXCLUSION_PROFILE, "unknown_self_hash_exclusion_profile"));
    }
    validate_non_claim_metadata(&envelope.non_claims, PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM, &mut issues);

    let expected =
        match historical_private_packed_v2_bytes(&envelope.preserves_text, &envelope.self_hash_exclusion_profile) {
            Ok(bytes) => Some(blake3_hex(&bytes)),
            Err(code) => {
                issues.push(PreservesEvidenceIssue::new(FIELD_PRESERVES_TEXT, code));
                None
            }
        };
    validate_declared_hash(&envelope.preserves_content_hash, expected.as_deref(), &mut issues);
    finish_report(expected, issues, HISTORICAL_V2_SUPPORTED_CLAIM)
}

/// Validate a Preserves bridge row as an **opaque-byte carrier**.
///
/// This check validates BLAKE3 digest binding over `row.canonical_bytes`,
/// schema, artifact/verification roles, and non-claims only. It deliberately
/// does NOT parse or validate `row.canonical_bytes` as canonical Preserves
/// binary syntax; use [`validate_preserves_bridge_canonical`] when canonical
/// Preserves parsing and validation are required.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.opaque_vs_canonical]
#[must_use]
pub fn validate_preserves_bridge(row: &PreservesBridgeRow) -> PreservesEvidenceReport {
    let mut issues = validate_bridge_metadata(row);
    let expected = Some(blake3_hex(&row.canonical_bytes));
    validate_declared_hash(&row.preserves_content_hash, expected.as_deref(), &mut issues);
    finish_report(expected, issues, PRESERVES_BRIDGE_OPAQUE_SUPPORTED_CLAIM)
}

/// Validate a Preserves bridge row WITH strict upstream Preserves 0.996.3
/// canonical binary parsing and validation.
///
/// Ensures `row.canonical_bytes` decodes as exactly one canonical upstream
/// Preserves value with no trailing bytes, minimal varints/integers, canonical
/// set/dictionary ordering, and conformance to `limits`.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.opaque_vs_canonical]
#[must_use]
pub fn validate_preserves_bridge_canonical(
    row: &PreservesBridgeRow,
    limits: PreservesLimits,
) -> PreservesEvidenceReport {
    let mut issues = validate_bridge_metadata(row);
    let decoded = decode_canonical_binary(&row.canonical_bytes, limits);
    let expected = match decoded {
        Ok(value) => match encode_canonical_binary(&value, limits) {
            Ok(reencoded) if reencoded == row.canonical_bytes => Some(blake3_hex(&row.canonical_bytes)),
            Ok(_) => {
                issues.push(PreservesEvidenceIssue::new(FIELD_CANONICAL_BYTES, "noncanonical_binary"));
                None
            }
            Err(err) => {
                issues.push(PreservesEvidenceIssue::new(FIELD_CANONICAL_BYTES, err.code()));
                None
            }
        },
        Err(err) => {
            issues.push(PreservesEvidenceIssue::new(FIELD_CANONICAL_BYTES, err.code()));
            None
        }
    };
    validate_declared_hash(&row.preserves_content_hash, expected.as_deref(), &mut issues);
    finish_report(expected, issues, PRESERVES_BRIDGE_CANONICAL_SUPPORTED_CLAIM)
}

fn validate_bridge_metadata(row: &PreservesBridgeRow) -> Vec<PreservesEvidenceIssue> {
    let mut issues = Vec::new();
    validate_schema(&row.schema_label, &mut issues);
    if row.valence_artifact_kind.trim().is_empty() {
        issues.push(PreservesEvidenceIssue::new(FIELD_SCHEMA, "artifact_kind_missing"));
    }
    if !SUPPORTED_ARTIFACT_ROLES.contains(&row.artifact_role.as_str()) {
        issues.push(PreservesEvidenceIssue::new(FIELD_ARTIFACT_ROLE, "unsupported_artifact_role"));
    }
    if !SUPPORTED_VERIFICATION_ROLES.contains(&row.verification_role.as_str()) {
        issues.push(PreservesEvidenceIssue::new(FIELD_VERIFICATION_ROLE, "unsupported_verification_role"));
    }
    validate_non_claim_metadata(&row.non_claims, PRESERVES_BRIDGE_REQUIRED_NON_CLAIM, &mut issues);
    issues
}

/// Compute the BLAKE3 hex content identity over upstream Preserves 0.996.3
/// canonical binary bytes under `preserves-canonical-binary-v3`.
#[must_use]
pub fn preserves_content_hash(text: &str, exclusion_profile: &str) -> Option<String> {
    canonical_preserves_bytes(text, exclusion_profile).ok().as_deref().map(blake3_hex)
}

/// Parse upstream Preserves text, apply the named v3 structural self-hash
/// exclusion profile, and encode canonical Preserves 0.996.3 binary bytes.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.profile]
/// r[impl valence.evidence_ir.preserves_canonical_v3.self_hash_exclusion]
pub fn canonical_preserves_bytes(text: &str, exclusion_profile: &str) -> Result<Vec<u8>, &'static str> {
    canonical_preserves_bytes_with_limits(text, exclusion_profile, PreservesLimits::default())
}

/// Same as [`canonical_preserves_bytes`] with explicit [`PreservesLimits`].
pub fn canonical_preserves_bytes_with_limits(
    text: &str,
    exclusion_profile: &str,
    limits: PreservesLimits,
) -> Result<Vec<u8>, &'static str> {
    if !is_supported_v3_exclusion_profile(exclusion_profile) {
        return Err("unknown_self_hash_exclusion_profile");
    }
    let parsed = parse_preserves_text(text, limits).map_err(|err| err.code())?;
    let transformed = apply_v3_self_hash_exclusion(parsed, exclusion_profile)?;
    encode_canonical_binary(&transformed, limits).map_err(|err| err.code())
}

/// Strictly decode upstream Preserves 0.996.3 canonical binary input, apply the
/// named v3 self-hash exclusion profile, and re-encode canonically.
pub fn canonical_preserves_binary_bytes(
    packed_bytes: &[u8],
    exclusion_profile: &str,
    limits: PreservesLimits,
) -> Result<Vec<u8>, &'static str> {
    if !is_supported_v3_exclusion_profile(exclusion_profile) {
        return Err("unknown_self_hash_exclusion_profile");
    }
    let decoded = decode_canonical_binary(packed_bytes, limits).map_err(|err| err.code())?;
    let transformed = apply_v3_self_hash_exclusion(decoded, exclusion_profile)?;
    encode_canonical_binary(&transformed, limits).map_err(|err| err.code())
}

fn apply_v3_self_hash_exclusion(value: PreservesValue, profile: &str) -> Result<PreservesValue, &'static str> {
    let excluded = match profile {
        UPSTREAM_RECEIPT_EXCLUSION_PROFILE => &["receipt_hash", "preserves_content_hash"][..],
        UPSTREAM_CONTENT_EXCLUSION_PROFILE => &["preserves_content_hash"][..],
        UPSTREAM_NO_EXCLUSION_PROFILE => return Ok(value),
        _ => return Err("unknown_self_hash_exclusion_profile"),
    };
    match value {
        PreservesValue::Dictionary(entries) => {
            filter_top_level_dictionary(entries, excluded).map(PreservesValue::Dictionary)
        }
        PreservesValue::Record { label, mut fields } => {
            if !matches!(*label, PreservesValue::Symbol(_)) {
                return Err("top_level_envelope_required");
            }
            if fields.len() != 1 {
                return Err("top_level_envelope_required");
            }
            let Some(PreservesValue::Dictionary(entries)) = fields.pop() else {
                return Err("top_level_envelope_required");
            };
            let retained = filter_top_level_dictionary(entries, excluded)?;
            Ok(PreservesValue::Record {
                label,
                fields: vec![PreservesValue::Dictionary(retained)],
            })
        }
        _ => Err("top_level_envelope_required"),
    }
}

fn filter_top_level_dictionary(
    entries: Vec<(PreservesValue, PreservesValue)>,
    excluded: &[&str],
) -> Result<Vec<(PreservesValue, PreservesValue)>, &'static str> {
    if entries.is_empty() {
        return Err("empty_envelope");
    }
    let mut retained = Vec::with_capacity(entries.len());
    let mut removed: Vec<String> = Vec::new();
    for (key, value) in entries {
        if let PreservesValue::Symbol(ref name) = key
            && excluded.contains(&name.as_str())
        {
            if removed.iter().any(|seen| seen == name) {
                return Err("duplicate_self_hash_field");
            }
            removed.push(name.clone());
            continue;
        }
        if contains_nested_v3_self_hash(&key, excluded) || contains_nested_v3_self_hash(&value, excluded) {
            return Err("misplaced_self_hash_field");
        }
        retained.push((key, value));
    }
    Ok(retained)
}

/// Detect misplaced structural self-hash fields in nested positions.
///
/// Only exact structural symbol keys in nested dictionaries or nested records
/// labeled with an excluded field name fail closed. Payload strings, byte
/// strings, string keys, and symbol keys whose names merely contain the
/// substring remain hash-bearing.
fn contains_nested_v3_self_hash(value: &PreservesValue, excluded: &[&str]) -> bool {
    match value {
        PreservesValue::Boolean(_)
        | PreservesValue::SignedInteger(_)
        | PreservesValue::String(_)
        | PreservesValue::ByteString(_)
        | PreservesValue::Symbol(_) => false,
        PreservesValue::Sequence(items) | PreservesValue::Set(items) => {
            items.iter().any(|item| contains_nested_v3_self_hash(item, excluded))
        }
        PreservesValue::Record { label, fields } => {
            if let PreservesValue::Symbol(name) = label.as_ref()
                && excluded.contains(&name.as_str())
            {
                return true;
            }
            contains_nested_v3_self_hash(label, excluded)
                || fields.iter().any(|field| contains_nested_v3_self_hash(field, excluded))
        }
        PreservesValue::Dictionary(entries) => entries.iter().any(|(key, val)| {
            if let PreservesValue::Symbol(name) = key
                && excluded.contains(&name.as_str())
            {
                return true;
            }
            contains_nested_v3_self_hash(key, excluded) || contains_nested_v3_self_hash(val, excluded)
        }),
    }
}

/// Reproduce the historical private-format `preserves-packed-structural-v2`
/// byte sequence for archived v2 receipts and legacy-format regressions.
///
/// IMPORTANT: This encoding is a historical private count-prefixed list format
/// and is NOT upstream Preserves binary. Never use it for new canonical
/// Preserves writes.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.historical_v2]
pub fn historical_private_packed_v2_bytes(text: &str, exclusion_profile: &str) -> Result<Vec<u8>, &'static str> {
    if !is_supported_v2_exclusion_profile(exclusion_profile) {
        return Err("unknown_self_hash_exclusion_profile");
    }
    let parsed = HistoricalV2Parser::new(text).parse().map_err(|_| "malformed_preserves_structure")?;
    let transformed = remove_historical_v2_self_hash_fields(parsed, exclusion_profile)?;
    let mut bytes = Vec::new();
    encode_historical_v2_packed(&transformed, &mut bytes);
    Ok(bytes)
}

/// Compute the historical private-format `preserves-packed-structural-v2`
/// BLAKE3 hex digest for archived receipt reproduction and migration.
#[must_use]
pub fn historical_private_packed_v2_hash(text: &str, exclusion_profile: &str) -> Option<String> {
    historical_private_packed_v2_bytes(text, exclusion_profile).ok().as_deref().map(blake3_hex)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HistoricalV2Value {
    Symbol(String),
    String(String),
    List(Vec<Self>),
}

struct HistoricalV2Parser<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> HistoricalV2Parser<'a> {
    const fn new(input: &'a str) -> Self {
        Self {
            input: input.as_bytes(),
            cursor: 0,
        }
    }

    fn parse(mut self) -> Result<HistoricalV2Value, &'static str> {
        self.skip_space();
        let value = self.parse_value()?;
        self.skip_space();
        if self.cursor != self.input.len() {
            return Err("trailing_input");
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<HistoricalV2Value, &'static str> {
        match self.peek() {
            Some(b'(') => self.parse_list(),
            Some(b'\"') => self.parse_string(),
            Some(_) => self.parse_symbol(),
            None => Err("unexpected_end"),
        }
    }

    fn parse_list(&mut self) -> Result<HistoricalV2Value, &'static str> {
        self.cursor += 1;
        let mut values = Vec::new();
        loop {
            self.skip_space();
            match self.peek() {
                Some(b')') => {
                    self.cursor += 1;
                    return Ok(HistoricalV2Value::List(values));
                }
                Some(_) => values.push(self.parse_value()?),
                None => return Err("unterminated_list"),
            }
        }
    }

    fn parse_string(&mut self) -> Result<HistoricalV2Value, &'static str> {
        self.cursor += 1;
        let mut value = String::new();
        loop {
            let byte = self.take().ok_or("unterminated_string")?;
            match byte {
                b'\"' => return Ok(HistoricalV2Value::String(value)),
                b'\\' => {
                    let escaped = self.take().ok_or("unterminated_escape")?;
                    match escaped {
                        b'\"' => value.push('\"'),
                        b'\\' => value.push('\\'),
                        b'n' => value.push('\n'),
                        b'r' => value.push('\r'),
                        b't' => value.push('\t'),
                        _ => return Err("unsupported_escape"),
                    }
                }
                byte if byte.is_ascii_control() => return Err("control_in_string"),
                byte => value.push(char::from(byte)),
            }
        }
    }

    fn parse_symbol(&mut self) -> Result<HistoricalV2Value, &'static str> {
        let start = self.cursor;
        while let Some(byte) = self.peek() {
            if byte.is_ascii_whitespace() || matches!(byte, b'(' | b')' | b'\"') {
                break;
            }
            self.cursor += 1;
        }
        if start == self.cursor {
            return Err("empty_symbol");
        }
        let bytes = &self.input[start..self.cursor];
        let symbol = std::str::from_utf8(bytes).map_err(|_| "invalid_utf8")?;
        Ok(HistoricalV2Value::Symbol(symbol.to_owned()))
    }

    fn skip_space(&mut self) {
        while self.peek().is_some_and(|byte| byte.is_ascii_whitespace()) {
            self.cursor += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.cursor).copied()
    }

    fn take(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.cursor += 1;
        Some(byte)
    }
}

fn remove_historical_v2_self_hash_fields(
    value: HistoricalV2Value,
    profile: &str,
) -> Result<HistoricalV2Value, &'static str> {
    let excluded = match profile {
        HISTORICAL_RECEIPT_EXCLUSION_PROFILE => &["receipt_hash", "preserves_content_hash"][..],
        HISTORICAL_CONTENT_EXCLUSION_PROFILE => &["preserves_content_hash"][..],
        HISTORICAL_NO_EXCLUSION_PROFILE => &[][..],
        _ => return Err("unknown_self_hash_exclusion_profile"),
    };
    let HistoricalV2Value::List(children) = value else {
        return Err("top_level_envelope_required");
    };
    if children.is_empty() {
        return Err("empty_envelope");
    }
    let mut retained = Vec::with_capacity(children.len());
    let mut removed: Vec<String> = Vec::new();
    for (index, child) in children.into_iter().enumerate() {
        let field = direct_historical_v2_field_name(&child).map(str::to_owned);
        if index > 0 && field.as_deref().is_some_and(|name| excluded.contains(&name)) {
            let name = field.expect("field was checked");
            if removed.contains(&name) {
                return Err("duplicate_self_hash_field");
            }
            removed.push(name);
        } else {
            if contains_nested_historical_v2_field(&child, excluded, index == 0) {
                return Err("misplaced_self_hash_field");
            }
            retained.push(child);
        }
    }
    Ok(HistoricalV2Value::List(retained))
}

fn direct_historical_v2_field_name(value: &HistoricalV2Value) -> Option<&str> {
    let HistoricalV2Value::List(items) = value else {
        return None;
    };
    match items.first() {
        Some(HistoricalV2Value::Symbol(name)) => Some(name),
        _ => None,
    }
}

fn contains_nested_historical_v2_field(value: &HistoricalV2Value, excluded: &[&str], root_tag: bool) -> bool {
    match value {
        HistoricalV2Value::List(items) => {
            if !root_tag && direct_historical_v2_field_name(value).is_some_and(|name| excluded.contains(&name)) {
                return true;
            }
            items.iter().any(|item| contains_nested_historical_v2_field(item, excluded, false))
        }
        HistoricalV2Value::Symbol(_) | HistoricalV2Value::String(_) => false,
    }
}

fn encode_historical_v2_packed(value: &HistoricalV2Value, output: &mut Vec<u8>) {
    match value {
        HistoricalV2Value::Symbol(symbol) => {
            encode_historical_v2_atom(HISTORICAL_V2_SYMBOL_TAG, symbol.as_bytes(), output)
        }
        HistoricalV2Value::String(string) => {
            encode_historical_v2_atom(HISTORICAL_V2_STRING_TAG, string.as_bytes(), output)
        }
        HistoricalV2Value::List(items) => {
            output.push(HISTORICAL_V2_LIST_TAG);
            encode_historical_v2_length(items.len(), output);
            for item in items {
                encode_historical_v2_packed(item, output);
            }
        }
    }
}

fn encode_historical_v2_atom(tag: u8, bytes: &[u8], output: &mut Vec<u8>) {
    output.push(tag);
    encode_historical_v2_length(bytes.len(), output);
    output.extend_from_slice(bytes);
}

fn encode_historical_v2_length(length: usize, output: &mut Vec<u8>) {
    output.extend_from_slice(&u64::try_from(length).expect("usize fits in u64").to_be_bytes());
}

#[must_use]
pub fn validate_typed_claim_boundary(boundary: &TypedClaimBoundary) -> Vec<PreservesEvidenceIssue> {
    let mut issues = Vec::new();
    if !boundary.required_boundary_ids.iter().any(|id| id == REQUIRED_IDENTITY_BOUNDARY) {
        issues.push(PreservesEvidenceIssue::new(FIELD_CLAIM_BOUNDARY, "required_boundary_missing"));
    }
    if !boundary.asserted_capabilities.is_empty() {
        issues.push(PreservesEvidenceIssue::new(FIELD_CLAIM_BOUNDARY, "prohibited_capability_asserted"));
    }
    let prose_overclaim = boundary.display_metadata.iter().any(|text| {
        let normalized = text.to_ascii_lowercase();
        PROHIBITED_CAPABILITIES.iter().any(|capability| normalized.contains(capability.phrase()))
    });
    if prose_overclaim {
        issues.push(PreservesEvidenceIssue::new(FIELD_CLAIM_BOUNDARY, "prohibited_capability_in_metadata"));
    }
    issues.sort();
    issues.dedup();
    issues
}

/// Emit a versioned migration receipt (`preserves-integrity-migration-v2`)
/// binding exact original bytes, historical profile (`preserves-text-v1` or
/// `preserves-packed-structural-v2`), source exclusion rule, source identity,
/// target `preserves-canonical-binary-v3` profile, target exclusion rule,
/// target canonical Preserves identity, and required non-claims.
///
/// Missing original bytes, unsupported source/target profiles, mismatched
/// declared source identities, or invalid target Preserves values fail closed.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.migration]
pub fn migrate_preserves_profile(
    original_bytes: &[u8],
    source_profile: &str,
    source_exclusion_profile: &str,
    declared_source_identity: &str,
    target_preserves_text: &str,
    target_exclusion_profile: &str,
    limits: PreservesLimits,
) -> Result<PreservesProfileMigrationReceipt, &'static str> {
    if original_bytes.is_empty() {
        return Err("original_bytes_missing");
    }
    let source_text = std::str::from_utf8(original_bytes).map_err(|_| "original_bytes_invalid_utf8")?;
    let recomputed_source = match source_profile {
        LEGACY_CANONICALIZATION_PROFILE => {
            format!("{HASH_PREFIX}{}", legacy_rendered_line_hash(source_text, source_exclusion_profile)?)
        }
        HISTORICAL_PRIVATE_PACKED_PROFILE => {
            let hash = historical_private_packed_v2_hash(source_text, source_exclusion_profile)
                .ok_or("source_identity_unavailable")?;
            format!("{HASH_PREFIX}{hash}")
        }
        _ => return Err("unsupported_source_profile"),
    };
    if declared_source_identity != recomputed_source {
        return Err("source_identity_mismatch");
    }
    let target_bytes = canonical_preserves_bytes_with_limits(target_preserves_text, target_exclusion_profile, limits)?;
    let target_identity = format!("{HASH_PREFIX}{}", blake3_hex(&target_bytes));
    Ok(PreservesProfileMigrationReceipt {
        migration_profile: PRESERVES_PROFILE_MIGRATION_V2.to_owned(),
        original_bytes_hash: format!("{HASH_PREFIX}{}", blake3_hex(original_bytes)),
        source_profile: source_profile.to_owned(),
        source_exclusion_profile: source_exclusion_profile.to_owned(),
        source_identity: recomputed_source,
        target_profile: UPSTREAM_CANONICAL_BINARY_PROFILE.to_owned(),
        target_exclusion_profile: target_exclusion_profile.to_owned(),
        target_identity,
        non_claims: vec![PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM.to_owned()],
    })
}

/// Validate a [`PreservesProfileMigrationReceipt`] against the exact original
/// bytes and target upstream Preserves text.
///
/// r[impl valence.evidence_ir.preserves_canonical_v3.migration]
pub fn validate_preserves_profile_migration(
    receipt: &PreservesProfileMigrationReceipt,
    original_bytes: &[u8],
    target_preserves_text: &str,
    limits: PreservesLimits,
) -> Result<(), &'static str> {
    if receipt.migration_profile != PRESERVES_PROFILE_MIGRATION_V2 {
        return Err("unsupported_migration_profile");
    }
    if receipt.target_profile != UPSTREAM_CANONICAL_BINARY_PROFILE {
        return Err("unsupported_target_profile");
    }
    if !receipt.non_claims.iter().any(|claim| claim.contains(PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM)) {
        return Err("non_claims_weakened");
    }
    let expected = migrate_preserves_profile(
        original_bytes,
        &receipt.source_profile,
        &receipt.source_exclusion_profile,
        &receipt.source_identity,
        target_preserves_text,
        &receipt.target_exclusion_profile,
        limits,
    )?;
    if receipt.original_bytes_hash != expected.original_bytes_hash {
        return Err("original_bytes_hash_mismatch");
    }
    if receipt.target_identity != expected.target_identity {
        return Err("target_identity_mismatch");
    }
    Ok(())
}

/// Historical v1-to-v2 migration helper retained for archived receipt
/// verification. New migrations into `preserves-canonical-binary-v3` MUST use
/// [`migrate_preserves_profile`].
pub fn migrate_legacy_preserves_identity(
    original_bytes: &[u8],
    declared_legacy_identity: &str,
    exclusion_profile: &str,
) -> Result<PreservesMigrationReceipt, &'static str> {
    if original_bytes.is_empty() {
        return Err("original_bytes_missing");
    }
    let text = std::str::from_utf8(original_bytes).map_err(|_| "original_bytes_invalid_utf8")?;
    let legacy_identity = format!("{HASH_PREFIX}{}", legacy_rendered_line_hash(text, exclusion_profile)?);
    if declared_legacy_identity != legacy_identity {
        return Err("legacy_identity_mismatch");
    }
    let corrected =
        historical_private_packed_v2_hash(text, exclusion_profile).ok_or("corrected_identity_unavailable")?;
    Ok(PreservesMigrationReceipt {
        migration_profile: LEGACY_MIGRATION_PROFILE.to_owned(),
        original_bytes_hash: format!("{HASH_PREFIX}{}", blake3_hex(original_bytes)),
        legacy_identity,
        corrected_identity: format!("{HASH_PREFIX}{corrected}"),
        non_claims: vec![PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM.to_owned()],
    })
}

fn legacy_rendered_line_hash(text: &str, exclusion_profile: &str) -> Result<String, &'static str> {
    if !is_supported_v2_exclusion_profile(exclusion_profile) {
        return Err("unknown_self_hash_exclusion_profile");
    }
    let material = if exclusion_profile == HISTORICAL_NO_EXCLUSION_PROFILE {
        text.trim().to_owned()
    } else {
        text.lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.contains("preserves_content_hash") && !trimmed.contains("receipt_hash")
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_owned()
    };
    Ok(blake3_hex(material.as_bytes()))
}

fn validate_declared_hash(declared: &str, expected: Option<&str>, issues: &mut Vec<PreservesEvidenceIssue>) {
    let parsed = match PreservesMetadataDigest::parse(declared, MIXED_BLAKE3_PROFILE, FIELD_CONTENT_HASH) {
        Ok(parsed) => parsed,
        Err(_) => {
            issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "malformed_blake3"));
            return;
        }
    };
    let Some(expected) = expected else {
        issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "hash_material_unavailable"));
        return;
    };
    if parsed.as_hex() != expected {
        issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "profile_digest_mismatch"));
    }
}

fn validate_schema(schema: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if !SUPPORTED_PRESERVES_SCHEMAS.contains(&schema) {
        issues.push(PreservesEvidenceIssue::new(FIELD_SCHEMA, "unsupported_schema"));
    }
}

fn is_supported_v3_exclusion_profile(profile: &str) -> bool {
    matches!(
        profile,
        UPSTREAM_RECEIPT_EXCLUSION_PROFILE | UPSTREAM_CONTENT_EXCLUSION_PROFILE | UPSTREAM_NO_EXCLUSION_PROFILE
    )
}

fn is_supported_v2_exclusion_profile(profile: &str) -> bool {
    matches!(
        profile,
        HISTORICAL_RECEIPT_EXCLUSION_PROFILE | HISTORICAL_CONTENT_EXCLUSION_PROFILE | HISTORICAL_NO_EXCLUSION_PROFILE
    )
}

fn validate_non_claim_metadata(non_claims: &[String], required: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if non_claims.is_empty() {
        issues.push(PreservesEvidenceIssue::new(FIELD_NON_CLAIMS, "non_claims_missing"));
    } else if !non_claims.iter().any(|claim| claim.contains(required)) {
        issues.push(PreservesEvidenceIssue::new(FIELD_NON_CLAIMS, "non_claims_weakened"));
    }
}

fn finish_report(
    normalized_content_hash: Option<String>,
    mut issues: Vec<PreservesEvidenceIssue>,
    supported_claim: &str,
) -> PreservesEvidenceReport {
    issues.sort();
    issues.dedup();
    PreservesEvidenceReport {
        valid: issues.is_empty(),
        normalized_content_hash,
        issues,
        supported_claim: supported_claim.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preserves_canonical::PreservesCodecError;

    const VALID_V3_TEXT: &str = "<envelope {schema: \"valence.preserves-evidence.v1\", payload: \"ok\"}>";
    const VALID_HISTORICAL_V2_TEXT: &str = "(envelope (schema \"valence.preserves-evidence.v1\") (payload \"ok\"))";

    fn envelope_v3() -> PreservesEvidenceEnvelope {
        let hash = preserves_content_hash(VALID_V3_TEXT, UPSTREAM_NO_EXCLUSION_PROFILE).unwrap();
        PreservesEvidenceEnvelope {
            schema: "valence.preserves-evidence.v1".to_owned(),
            canonicalization_profile: UPSTREAM_CANONICAL_BINARY_PROFILE.to_owned(),
            preserves_text: VALID_V3_TEXT.to_owned(),
            preserves_content_hash: format!("{HASH_PREFIX}{hash}"),
            self_hash_exclusion_profile: UPSTREAM_NO_EXCLUSION_PROFILE.to_owned(),
            non_claims: vec![PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM.to_owned()],
        }
    }

    #[test]
    fn minimal_legacy_v2_case_is_retained_and_rejected_as_upstream_preserves() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.historical_v2]
        let legacy_bytes = historical_private_packed_v2_bytes("(x)", HISTORICAL_NO_EXCLUSION_PROFILE).unwrap();
        assert_eq!(legacy_bytes, vec![
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x78
        ]);
        // Upstream Preserves sees #f at byte 0 followed by 18 trailing bytes.
        assert_eq!(
            decode_canonical_binary(&legacy_bytes, PreservesLimits::default()),
            Err(PreservesCodecError::TrailingBytes)
        );
        // Upstream zero-field record <x> is valid under v3 and emits exact upstream bytes.
        let upstream_record = canonical_preserves_bytes("<x>", UPSTREAM_NO_EXCLUSION_PROFILE).unwrap();
        assert_eq!(upstream_record, vec![0xB4, 0xB3, 0x01, b'x', 0x84]);
        assert!(decode_canonical_binary(&upstream_record, PreservesLimits::default()).is_ok());
        // Private parenthesized S-expr "(x)" is rejected on the active v3 path.
        assert!(canonical_preserves_bytes("(x)", UPSTREAM_NO_EXCLUSION_PROFILE).is_err());
    }

    #[test]
    fn historical_v2_receipt_reproduces_only_under_explicit_historical_validator() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.historical_v2]
        let v2_hash =
            historical_private_packed_v2_hash(VALID_HISTORICAL_V2_TEXT, HISTORICAL_NO_EXCLUSION_PROFILE).unwrap();
        let v2_envelope = PreservesEvidenceEnvelope {
            schema: "valence.preserves-evidence.v1".to_owned(),
            canonicalization_profile: HISTORICAL_PRIVATE_PACKED_PROFILE.to_owned(),
            preserves_text: VALID_HISTORICAL_V2_TEXT.to_owned(),
            preserves_content_hash: format!("{HASH_PREFIX}{v2_hash}"),
            self_hash_exclusion_profile: HISTORICAL_NO_EXCLUSION_PROFILE.to_owned(),
            non_claims: vec![PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM.to_owned()],
        };
        // Reproducible under explicit historical validator.
        let historical_report = validate_historical_preserves_v2_evidence(&v2_envelope);
        assert!(historical_report.valid, "issues: {:?}", historical_report.issues);
        assert!(historical_report.supported_claim.contains("NOT upstream canonical Preserves"));

        // Rejected on active canonical validator so new writes cannot advertise v2.
        let active_report = validate_preserves_evidence(&v2_envelope);
        assert!(!active_report.valid);
        assert!(active_report.issues.iter().any(|issue| issue.code == "historical_private_profile_rejected"));
    }

    #[test]
    fn equivalent_v3_renderings_and_permuted_dicts_have_equal_identity() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.profile]
        let compact = "<receipt {payload: \"ok\", tags: #{1 -1 0}, receipt_hash: \"stale\"}>";
        let permuted = "<receipt\n  {\n    receipt_hash: \"other\",\n    tags: #{0 1 -1},\n    payload: \"ok\"\n  }\n>";
        let a = canonical_preserves_bytes(compact, UPSTREAM_RECEIPT_EXCLUSION_PROFILE).unwrap();
        let b = canonical_preserves_bytes(permuted, UPSTREAM_RECEIPT_EXCLUSION_PROFILE).unwrap();
        assert_eq!(a, b);
        assert!(decode_canonical_binary(&a, PreservesLimits::default()).is_ok());
    }

    #[test]
    fn payload_name_collisions_remain_hash_bearing_and_nested_or_duplicate_fail_closed() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.self_hash_exclusion]
        // String payload values containing self-hash names remain hash-bearing.
        let benign = "<receipt {payload: \"receipt_hash benign\"}>";
        let altered = "<receipt {payload: \"receipt_hash attacker\"}>";
        assert_ne!(
            preserves_content_hash(benign, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            preserves_content_hash(altered, UPSTREAM_RECEIPT_EXCLUSION_PROFILE)
        );
        // Payload string keys named "receipt_hash" or containing it remain hash-bearing.
        let string_key_a = "<receipt {payload: {\"receipt_hash\": \"a\", \"preserves_content_hash\": \"1\"}}>";
        let string_key_b = "<receipt {payload: {\"receipt_hash\": \"b\", \"preserves_content_hash\": \"1\"}}>";
        assert_ne!(
            preserves_content_hash(string_key_a, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            preserves_content_hash(string_key_b, UPSTREAM_RECEIPT_EXCLUSION_PROFILE)
        );
        // Symbol key merely containing the substring remains hash-bearing.
        let sym_sub_a = "<receipt {note_receipt_hash: \"a\"}>";
        let sym_sub_b = "<receipt {note_receipt_hash: \"b\"}>";
        assert_ne!(
            preserves_content_hash(sym_sub_a, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            preserves_content_hash(sym_sub_b, UPSTREAM_RECEIPT_EXCLUSION_PROFILE)
        );

        // Negative cases: duplicate self-hash key, duplicate payload key, nested symbol key, nested record,
        // non-envelope root.
        let duplicate = "<receipt {payload: \"ok\", receipt_hash: \"a\", receipt_hash: \"b\"}>";
        let duplicate_payload = "<receipt {payload: \"ok\", payload: \"dup\"}>";
        let nested_key = "<receipt {payload: {receipt_hash: \"hidden\"}}>";
        let nested_record = "<receipt {payload: <preserves_content_hash \"hidden\">}>";
        let wrong_root = "[receipt \"ok\"]";
        let empty_dict = "<receipt {}>";
        assert_eq!(
            canonical_preserves_bytes(duplicate, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            Err("duplicate_self_hash_field")
        );
        assert_eq!(
            canonical_preserves_bytes(duplicate_payload, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            Err("duplicate_dictionary_key")
        );
        assert_eq!(
            canonical_preserves_bytes(nested_key, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            Err("misplaced_self_hash_field")
        );
        assert_eq!(
            canonical_preserves_bytes(nested_record, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            Err("misplaced_self_hash_field")
        );
        assert_eq!(
            canonical_preserves_bytes(wrong_root, UPSTREAM_RECEIPT_EXCLUSION_PROFILE),
            Err("top_level_envelope_required")
        );
        assert_eq!(canonical_preserves_bytes(empty_dict, UPSTREAM_RECEIPT_EXCLUSION_PROFILE), Err("empty_envelope"));
    }

    #[test]
    fn valid_v3_envelope_passes_and_legacy_profiles_fail() {
        let valid = envelope_v3();
        assert!(validate_preserves_evidence(&valid).valid);
        let mut legacy = valid.clone();
        legacy.canonicalization_profile = LEGACY_CANONICALIZATION_PROFILE.to_owned();
        let report = validate_preserves_evidence(&legacy);
        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "legacy_profile_rejected"));
    }

    #[test]
    fn malformed_or_stale_v3_envelope_fails_closed() {
        let mut candidate = envelope_v3();
        candidate.preserves_text = "<unterminated".to_owned();
        candidate.preserves_content_hash = format!("{HASH_PREFIX}{}", "f".repeat(64));
        let report = validate_preserves_evidence(&candidate);
        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "unexpected_eof"));
    }

    #[test]
    fn typed_identity_boundary_passes_and_overclaims_fail() {
        let boundary = TypedClaimBoundary {
            claim_class: SupportedClaimClass::Identity,
            verification_role: TypedVerificationRole::RecordedOnly,
            required_boundary_ids: vec![REQUIRED_IDENTITY_BOUNDARY.to_owned()],
            asserted_capabilities: Vec::new(),
            display_metadata: vec!["records canonical identity only".to_owned()],
        };
        assert!(validate_typed_claim_boundary(&boundary).is_empty());

        let overclaim = TypedClaimBoundary {
            claim_class: SupportedClaimClass::Linkage,
            verification_role: TypedVerificationRole::Boundary,
            required_boundary_ids: Vec::new(),
            asserted_capabilities: vec![ProhibitedCapability::ReleaseEligibility],
            display_metadata: vec!["grants semantic equivalence".to_owned()],
        };
        let issues = validate_typed_claim_boundary(&overclaim);
        assert!(issues.iter().any(|issue| issue.code == "required_boundary_missing"));
        assert!(issues.iter().any(|issue| issue.code == "prohibited_capability_asserted"));
        assert!(issues.iter().any(|issue| issue.code == "prohibited_capability_in_metadata"));
    }

    #[test]
    fn versioned_v2_to_v3_migration_binds_originals_profiles_exclusion_and_identities() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.migration]
        let original_v2 = "(receipt (payload \"ok\") (receipt_hash \"stale\"))";
        let target_v3 = "<receipt {payload: \"ok\", receipt_hash: \"stale\"}>";
        let v2_identity = format!(
            "{HASH_PREFIX}{}",
            historical_private_packed_v2_hash(original_v2, HISTORICAL_RECEIPT_EXCLUSION_PROFILE).unwrap()
        );
        let receipt = migrate_preserves_profile(
            original_v2.as_bytes(),
            HISTORICAL_PRIVATE_PACKED_PROFILE,
            HISTORICAL_RECEIPT_EXCLUSION_PROFILE,
            &v2_identity,
            target_v3,
            UPSTREAM_RECEIPT_EXCLUSION_PROFILE,
            PreservesLimits::default(),
        )
        .unwrap();
        assert_eq!(receipt.migration_profile, PRESERVES_PROFILE_MIGRATION_V2);
        assert_eq!(receipt.source_profile, HISTORICAL_PRIVATE_PACKED_PROFILE);
        assert_eq!(receipt.source_exclusion_profile, HISTORICAL_RECEIPT_EXCLUSION_PROFILE);
        assert_eq!(receipt.source_identity, v2_identity);
        assert_eq!(receipt.target_profile, UPSTREAM_CANONICAL_BINARY_PROFILE);
        assert_eq!(receipt.target_exclusion_profile, UPSTREAM_RECEIPT_EXCLUSION_PROFILE);
        assert_ne!(receipt.target_identity, receipt.source_identity);
        assert_eq!(receipt.original_bytes_hash, format!("{HASH_PREFIX}{}", blake3_hex(original_v2.as_bytes())));
        assert!(
            validate_preserves_profile_migration(
                &receipt,
                original_v2.as_bytes(),
                target_v3,
                PreservesLimits::default()
            )
            .is_ok()
        );

        // Missing originals, unsupported source profile, or mismatched source identity fail closed.
        assert_eq!(
            migrate_preserves_profile(
                &[],
                HISTORICAL_PRIVATE_PACKED_PROFILE,
                HISTORICAL_RECEIPT_EXCLUSION_PROFILE,
                &v2_identity,
                target_v3,
                UPSTREAM_RECEIPT_EXCLUSION_PROFILE,
                PreservesLimits::default(),
            ),
            Err("original_bytes_missing")
        );
        assert_eq!(
            migrate_preserves_profile(
                original_v2.as_bytes(),
                "unknown-profile-v9",
                HISTORICAL_RECEIPT_EXCLUSION_PROFILE,
                &v2_identity,
                target_v3,
                UPSTREAM_RECEIPT_EXCLUSION_PROFILE,
                PreservesLimits::default(),
            ),
            Err("unsupported_source_profile")
        );
        assert_eq!(
            migrate_preserves_profile(
                original_v2.as_bytes(),
                HISTORICAL_PRIVATE_PACKED_PROFILE,
                HISTORICAL_RECEIPT_EXCLUSION_PROFILE,
                "b3:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                target_v3,
                UPSTREAM_RECEIPT_EXCLUSION_PROFILE,
                PreservesLimits::default(),
            ),
            Err("source_identity_mismatch")
        );
    }

    #[test]
    fn opaque_carrier_check_remains_distinct_from_canonical_validation_and_pinned_interop_passes() {
        // r[verify valence.evidence_ir.preserves_canonical_v3.opaque_vs_canonical]
        // 1. Historical private v2 bytes can pass opaque-byte carrier digest matching, but MUST fail
        //    canonical Preserves validation.
        let private_bytes = historical_private_packed_v2_bytes("(x)", HISTORICAL_NO_EXCLUSION_PROFILE).unwrap();
        let opaque_row = PreservesBridgeRow {
            schema_label: "molten.preserves-envelope.v1".to_owned(),
            preserves_content_hash: blake3_hex(&private_bytes),
            canonical_bytes: private_bytes,
            artifact_role: "preserves_metadata".to_owned(),
            valence_artifact_kind: "preserves-envelope".to_owned(),
            verification_role: "recorded_only".to_owned(),
            non_claims: vec![PRESERVES_BRIDGE_REQUIRED_NON_CLAIM.to_owned()],
        };
        let opaque_report = validate_preserves_bridge(&opaque_row);
        assert!(opaque_report.valid);
        assert!(opaque_report.supported_claim.contains("opaque-byte"));
        let canonical_report = validate_preserves_bridge_canonical(&opaque_row, PreservesLimits::default());
        assert!(!canonical_report.valid);
        assert!(canonical_report.issues.iter().any(|issue| issue.code == "trailing_bytes"));

        // 2. Pinned Kamacite producer -> Valence consumer interoperability fixture.
        let fixture_json = include_str!("../../../fixtures/preserves-canonical-v3/pinned-interop-v1.json");
        let fixture: serde_json::Value = serde_json::from_str(fixture_json).expect("valid interop fixture json");
        let text = fixture["preserves_text"].as_str().unwrap();
        let expected_hex = fixture["expected_canonical_binary_hex"].as_str().unwrap();
        let expected_blake3 = fixture["expected_blake3"].as_str().unwrap();
        let expected_bytes = hex_decode(expected_hex);

        let produced = canonical_preserves_bytes(text, UPSTREAM_NO_EXCLUSION_PROFILE).unwrap();
        assert_eq!(produced, expected_bytes);
        assert_eq!(blake3_hex(&produced), expected_blake3);

        let consumer_row = PreservesBridgeRow {
            schema_label: fixture["schema_label"].as_str().unwrap().to_owned(),
            canonical_bytes: produced,
            preserves_content_hash: format!("{HASH_PREFIX}{expected_blake3}"),
            artifact_role: "preserves_metadata".to_owned(),
            valence_artifact_kind: "kamacite-preserves-record".to_owned(),
            verification_role: "recorded_only".to_owned(),
            non_claims: vec![PRESERVES_BRIDGE_REQUIRED_NON_CLAIM.to_owned()],
        };
        let report = validate_preserves_bridge_canonical(&consumer_row, PreservesLimits::default());
        assert!(report.valid, "issues: {:?}", report.issues);

        // 3. Kamacite non-canonical integer fixture fails canonical bridge validation.
        let noncanonical_bytes = hex_decode(fixture["negative_nonminimal_integer_hex"].as_str().unwrap());
        let bad_row = PreservesBridgeRow {
            preserves_content_hash: format!("{HASH_PREFIX}{}", blake3_hex(&noncanonical_bytes)),
            canonical_bytes: noncanonical_bytes,
            ..consumer_row
        };
        let bad_report = validate_preserves_bridge_canonical(&bad_row, PreservesLimits::default());
        assert!(!bad_report.valid);
        assert!(bad_report.issues.iter().any(|issue| issue.code == "nonminimal_integer"));
    }

    fn hex_decode(hex: &str) -> Vec<u8> {
        hex.as_bytes()
            .chunks_exact(2)
            .map(|pair| {
                let s = std::str::from_utf8(pair).unwrap();
                u8::from_str_radix(s, 16).unwrap()
            })
            .collect()
    }
}
