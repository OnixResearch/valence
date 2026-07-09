//! Pure Preserves evidence identity and Preserves-to-Valence bridge validation.
//!
//! This module validates already-loaded Preserves text/bytes and envelope
//! metadata. It does not parse files, execute Nickel, decide release/lifecycle
//! policy, or prove receipt truth.
//!
//! r[impl valence.evidence_ir.preserves_evidence_core.contract]
//! r[impl valence.evidence_ir.preserves_evidence_core.validation]
//! r[impl valence.evidence_ir.preserves_evidence_core.adapters]
//! r[impl valence.evidence_ir.preserves_bridge.contract]
//! r[impl valence.evidence_ir.preserves_bridge.validation]

use serde::Deserialize;
use serde::Serialize;

use crate::shared_identity::DigestRole;
use crate::shared_identity::MIXED_BLAKE3_PROFILE;
use crate::shared_identity::blake3_hex;
use crate::shared_identity::contains_overclaim;
use crate::shared_identity::parse_digest;

const FIELD_ARTIFACT_ROLE: &str = "artifact_role";
const FIELD_CONTENT_HASH: &str = "preserves_content_hash";
const FIELD_EXCLUSION_PROFILE: &str = "self_hash_exclusion_profile";
const FIELD_NON_CLAIMS: &str = "non_claims";
const FIELD_PRESERVES_TEXT: &str = "preserves_text";
const FIELD_SCHEMA: &str = "schema";
const FIELD_VERIFICATION_ROLE: &str = "verification_role";

pub const PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM: &str =
    "Preserves evidence validation proves only canonical Preserves content identity and bounded envelope compatibility";
pub const PRESERVES_BRIDGE_REQUIRED_NON_CLAIM: &str =
    "Preserves bridge validation proves only identity, schema, role, and non-claim mapping";

const PRESERVES_SUPPORTED_CLAIM: &str = "Preserves evidence validation proves only canonical Preserves content identity and bounded envelope compatibility; it does not prove receipt truth, release eligibility, lifecycle readiness, semantic equivalence, behavioral correctness, verifier soundness, or whole-program safety";
const PRESERVES_BRIDGE_SUPPORTED_CLAIM: &str = "Preserves bridge validation proves identity, schema, role, and non-claim mapping only and does not prove Molten runtime authority, transport correctness, or Preserves application semantics";

const PRESERVES_TEXT_PROFILE: &str = "preserves-text-v1";
const RECEIPT_EXCLUSION_PROFILE: &str = "receipt-self-hash";
const CONTENT_HASH_EXCLUSION_PROFILE: &str = "content-hash-self";
const NO_SELF_HASH_EXCLUSION_PROFILE: &str = "none";

const SUPPORTED_PRESERVES_SCHEMAS: &[&str] = &[
    "valence.preserves-evidence.v1",
    "cairn.preserves-receipt.v1",
    "mantle.release-evidence.preserves.v1",
    "octet.evidence-receipt.preserves.v1",
    "molten.preserves-envelope.v1",
];

const SUPPORTED_ARTIFACT_ROLES: &[&str] = &[
    "artifact_identity",
    "receipt_identity",
    "preserves_metadata",
    "compatibility_reference",
];

const SUPPORTED_VERIFICATION_ROLES: &[&str] = &["property", "recorded_only", "boundary"];

/// In-memory Preserves evidence envelope candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesEvidenceEnvelope {
    pub schema: String,
    pub canonicalization_profile: String,
    pub preserves_text: String,
    pub preserves_content_hash: String,
    pub self_hash_exclusion_profile: String,
    pub non_claims: Vec<String>,
}

/// In-memory Preserves-to-Valence bridge row.
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

/// Field-scoped deterministic Preserves issue.
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

/// Validation report for Preserves evidence and bridge checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PreservesEvidenceReport {
    pub valid: bool,
    pub normalized_content_hash: Option<String>,
    pub issues: Vec<PreservesEvidenceIssue>,
    pub supported_claim: String,
}

/// Validate a Preserves evidence envelope without I/O.
#[must_use]
pub fn validate_preserves_evidence(envelope: &PreservesEvidenceEnvelope) -> PreservesEvidenceReport {
    let mut issues = Vec::new();
    validate_schema(&envelope.schema, &mut issues);
    validate_profile(&envelope.canonicalization_profile, &mut issues);
    validate_preserves_text(&envelope.preserves_text, &mut issues);
    validate_exclusion_profile(&envelope.self_hash_exclusion_profile, &mut issues);
    validate_non_claims(&envelope.non_claims, PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM, &mut issues);

    let expected = preserves_content_hash(&envelope.preserves_text, &envelope.self_hash_exclusion_profile);
    validate_declared_hash(
        &envelope.preserves_content_hash,
        expected.as_deref(),
        DigestRole::PreservesMetadata,
        &mut issues,
    );

    finish_report(expected, issues, PRESERVES_SUPPORTED_CLAIM)
}

/// Validate a Preserves-to-Valence bridge row without I/O.
#[must_use]
pub fn validate_preserves_bridge(row: &PreservesBridgeRow) -> PreservesEvidenceReport {
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
    validate_non_claims(&row.non_claims, PRESERVES_BRIDGE_REQUIRED_NON_CLAIM, &mut issues);

    let expected = Some(blake3_hex(&row.canonical_bytes));
    validate_declared_hash(
        &row.preserves_content_hash,
        expected.as_deref(),
        DigestRole::PreservesMetadata,
        &mut issues,
    );

    finish_report(expected, issues, PRESERVES_BRIDGE_SUPPORTED_CLAIM)
}

/// Compute the normalized raw BLAKE3 Preserves content hash after named self-hash exclusion.
#[must_use]
pub fn preserves_content_hash(preserves_text: &str, self_hash_exclusion_profile: &str) -> Option<String> {
    if !is_supported_exclusion_profile(self_hash_exclusion_profile) {
        return None;
    }
    let material = preserves_hash_material(preserves_text, self_hash_exclusion_profile);
    Some(blake3_hex(material.as_bytes()))
}

fn preserves_hash_material(preserves_text: &str, self_hash_exclusion_profile: &str) -> String {
    if self_hash_exclusion_profile == NO_SELF_HASH_EXCLUSION_PROFILE {
        return preserves_text.trim().to_owned();
    }
    preserves_text
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.contains("preserves_content_hash") && !trimmed.contains("receipt_hash")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}

fn validate_declared_hash(
    declared: &str,
    expected: Option<&str>,
    role: DigestRole,
    issues: &mut Vec<PreservesEvidenceIssue>,
) {
    let mut shared_issues = Vec::new();
    let parsed = parse_digest(declared, role, MIXED_BLAKE3_PROFILE, FIELD_CONTENT_HASH, &mut shared_issues);
    if parsed.is_none() {
        issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "malformed_blake3"));
        return;
    }
    let Some(expected) = expected else {
        issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "hash_material_unavailable"));
        return;
    };
    if parsed.expect("checked parsed digest").normalized_hex != expected {
        issues.push(PreservesEvidenceIssue::new(FIELD_CONTENT_HASH, "stale_content_hash"));
    }
}

fn validate_schema(schema: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if !SUPPORTED_PRESERVES_SCHEMAS.contains(&schema) {
        issues.push(PreservesEvidenceIssue::new(FIELD_SCHEMA, "unsupported_schema"));
    }
}

fn validate_profile(profile: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if profile != PRESERVES_TEXT_PROFILE {
        issues.push(PreservesEvidenceIssue::new(FIELD_SCHEMA, "unsupported_canonicalization_profile"));
    }
}

fn validate_exclusion_profile(profile: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if !is_supported_exclusion_profile(profile) {
        issues.push(PreservesEvidenceIssue::new(FIELD_EXCLUSION_PROFILE, "unknown_self_hash_exclusion_profile"));
    }
}

fn is_supported_exclusion_profile(profile: &str) -> bool {
    matches!(profile, RECEIPT_EXCLUSION_PROFILE | CONTENT_HASH_EXCLUSION_PROFILE | NO_SELF_HASH_EXCLUSION_PROFILE)
}

fn validate_preserves_text(text: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        issues.push(PreservesEvidenceIssue::new(FIELD_PRESERVES_TEXT, "preserves_text_empty"));
        return;
    }
    if !trimmed.starts_with('(') && !trimmed.starts_with('#') {
        issues.push(PreservesEvidenceIssue::new(FIELD_PRESERVES_TEXT, "malformed_preserves_text"));
        return;
    }
    if !parentheses_are_balanced(trimmed) {
        issues.push(PreservesEvidenceIssue::new(FIELD_PRESERVES_TEXT, "malformed_preserves_text"));
    }
}

fn parentheses_are_balanced(text: &str) -> bool {
    let mut depth = 0_u32;
    for character in text.chars() {
        match character {
            '(' => depth = depth.saturating_add(1),
            ')' => {
                let Some(next_depth) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next_depth;
            }
            _ => {}
        }
    }
    depth == 0
}

fn validate_non_claims(non_claims: &[String], required: &str, issues: &mut Vec<PreservesEvidenceIssue>) {
    if non_claims.is_empty() {
        issues.push(PreservesEvidenceIssue::new(FIELD_NON_CLAIMS, "non_claims_missing"));
        return;
    }
    if !non_claims.iter().any(|claim| claim.contains(required)) {
        issues.push(PreservesEvidenceIssue::new(FIELD_NON_CLAIMS, "non_claims_weakened"));
    }
    if non_claims.iter().any(|claim| contains_overclaim(claim)) {
        issues.push(PreservesEvidenceIssue::new(FIELD_NON_CLAIMS, "overclaim_detected"));
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

    const VALID_PRESERVES_TEXT: &str = "(envelope (schema \"valence.preserves-evidence.v1\") (payload \"ok\"))";

    fn evidence_non_claims() -> Vec<String> {
        vec![PRESERVES_EVIDENCE_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn bridge_non_claims() -> Vec<String> {
        vec![PRESERVES_BRIDGE_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn envelope() -> PreservesEvidenceEnvelope {
        let hash = preserves_content_hash(VALID_PRESERVES_TEXT, NO_SELF_HASH_EXCLUSION_PROFILE)
            .expect("known profile should hash");
        PreservesEvidenceEnvelope {
            schema: "valence.preserves-evidence.v1".to_owned(),
            canonicalization_profile: PRESERVES_TEXT_PROFILE.to_owned(),
            preserves_text: VALID_PRESERVES_TEXT.to_owned(),
            preserves_content_hash: format!("b3:{hash}"),
            self_hash_exclusion_profile: NO_SELF_HASH_EXCLUSION_PROFILE.to_owned(),
            non_claims: evidence_non_claims(),
        }
    }

    #[test]
    fn valid_preserves_evidence_envelope_passes() {
        // r[verify valence.evidence_ir.preserves_evidence_core.fixtures.positive]
        let report = validate_preserves_evidence(&envelope());

        assert!(report.valid);
        assert!(report.normalized_content_hash.is_some());
        assert!(report.supported_claim.contains("canonical Preserves content identity"));
    }

    #[test]
    fn malformed_or_stale_preserves_evidence_fails_closed() {
        // r[verify valence.evidence_ir.preserves_evidence_core.fixtures.negative]
        let mut envelope = envelope();
        envelope.schema = "unknown.schema".to_owned();
        envelope.preserves_text = "not-preserves".to_owned();
        envelope.preserves_content_hash = "stale".to_owned();
        envelope.self_hash_exclusion_profile = "unknown".to_owned();
        envelope.non_claims.clear();

        let report = validate_preserves_evidence(&envelope);

        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "unsupported_schema"));
        assert!(report.issues.iter().any(|issue| issue.code == "malformed_preserves_text"));
        assert!(report.issues.iter().any(|issue| issue.code == "malformed_blake3"));
        assert!(report.issues.iter().any(|issue| issue.code == "unknown_self_hash_exclusion_profile"));
        assert!(report.issues.iter().any(|issue| issue.code == "non_claims_missing"));
    }

    #[test]
    fn self_hash_exclusion_changes_only_declared_lines() {
        // r[verify valence.evidence_ir.preserves_evidence_core.self_hash_exclusion.profile]
        let with_self_hash =
            "(receipt\n  (payload \"ok\")\n  (preserves_content_hash \"stale\")\n  (receipt_hash \"stale\")\n)";
        let without_self_hash = "(receipt\n  (payload \"ok\")\n)";

        let actual = preserves_hash_material(with_self_hash, RECEIPT_EXCLUSION_PROFILE);

        assert_eq!(actual, without_self_hash);
    }

    #[test]
    fn valid_preserves_bridge_row_passes() {
        // r[verify valence.evidence_ir.preserves_bridge.fixtures.positive]
        let bytes = VALID_PRESERVES_TEXT.as_bytes().to_vec();
        let hash = blake3_hex(&bytes);
        let row = PreservesBridgeRow {
            schema_label: "molten.preserves-envelope.v1".to_owned(),
            canonical_bytes: bytes,
            preserves_content_hash: hash,
            artifact_role: "preserves_metadata".to_owned(),
            valence_artifact_kind: "preserves-envelope".to_owned(),
            verification_role: "recorded_only".to_owned(),
            non_claims: bridge_non_claims(),
        };

        let report = validate_preserves_bridge(&row);

        assert!(report.valid);
        assert!(report.supported_claim.contains("does not prove Molten runtime authority"));
    }

    #[test]
    fn invalid_preserves_bridge_row_fails_closed() {
        // r[verify valence.evidence_ir.preserves_bridge.fixtures.negative]
        let row = PreservesBridgeRow {
            schema_label: "bad schema".to_owned(),
            canonical_bytes: VALID_PRESERVES_TEXT.as_bytes().to_vec(),
            preserves_content_hash: "b3:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
            artifact_role: "unsupported".to_owned(),
            valence_artifact_kind: String::new(),
            verification_role: "property_plus".to_owned(),
            non_claims: Vec::new(),
        };

        let report = validate_preserves_bridge(&row);

        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "unsupported_schema"));
        assert!(report.issues.iter().any(|issue| issue.code == "stale_content_hash"));
        assert!(report.issues.iter().any(|issue| issue.code == "unsupported_artifact_role"));
        assert!(report.issues.iter().any(|issue| issue.code == "unsupported_verification_role"));
        assert!(report.issues.iter().any(|issue| issue.code == "non_claims_missing"));
    }
}
