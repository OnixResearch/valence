//! Evidence IR v1: canonical, serializable, type-checked intermediate
//! representation for evidence artifacts with span/provenance threading and
//! non-claim boundary enforcement.
//!
//! This module is the spine of the Valence Evidence IR. It defines the
//! unified canonical form that Function Objects, evidence index records, and
//! receipts can be carried in, plus a checker and a span explainer.
//!
//! # Design
//!
//! The IR is a typed Rust data model, not a language. It gives the stack
//! canonical-evidence-like discipline (canonical machine form + human projection +
//! span survival through the chain) without a compiler, without backends, and
//! without mechanized type-soundness.
//!
//! # Verification role
//!
//! Every artifact carries a [`VerificationRole`]. In Phase 0 the schema exists
//! and the checker validates it. In Phase 1, cairn `release-readiness` and
//! `evidence verify` count only [`VerificationRole::Property`] artifacts toward
//! "verified," so a "0 verified" receipt cannot be misread as "all good."
//!
//! # Non-claims
//!
//! Every artifact carries an explicit `non_claims` boundary. The checker
//! fails closed if the boundary is missing or weakened, mirroring the existing
//! [`crate::evidence_codebase_index::INDEX_BOUNDARY`] discipline.

use serde::Deserialize;
use serde::Serialize;

/// The Evidence IR v1 schema identifier.
pub const EVIDENCE_IR_SCHEMA: &str = "valence.evidence-ir.v1";

/// Required non-claim boundary text that every artifact must carry.
pub const REQUIRED_NON_CLAIM: &str =
    "does not claim behavioral correctness, semantic equivalence, or whole-program verification";

/// Overclaim strings the checker rejects if present in `non_claims` or `payload`.
const OVERCLAIM_MARKERS: &[&str] = &[
    "proves semantic equivalence",
    "proves whole-program correctness",
    "proves exhaustive coverage",
    "certified correct",
    "fully verified runtime",
];

/// The verification role of an evidence artifact.
///
/// Schema lands in Phase 0; enforcement (counting only `Property` as verified)
/// lands in Phase 1 via cairn `release-readiness` and `evidence verify`.
///
/// - `Property`: this artifact verifies a checkable predicate.
/// - `RecordedOnly`: this artifact records structure/identity only, proves nothing. A "0 passed, 0
///   failed" on recorded-only artifacts means "nothing was verified," not "all good."
/// - `Boundary`: non-claim text artifact, not a verification artifact at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationRole {
    Property,
    RecordedOnly,
    Boundary,
}

impl VerificationRole {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Property => "property",
            Self::RecordedOnly => "recorded_only",
            Self::Boundary => "boundary",
        }
    }
}

/// Extended source span carrying the stable identifier that threads through
/// the whole chain (source -> receipt -> binary).
///
/// The `source_id` is the stable identifier that
/// `valence explain --id <source_id>` resolves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceSpan {
    /// Stable identifier threaded through the chain.
    pub source_id: String,
    /// Path to the source file.
    pub source_path: String,
    /// Zero-based byte offset of the span start.
    pub start_byte: u64,
    /// Zero-based byte offset of the span end (exclusive).
    pub end_byte: u64,
    /// One-based line number of the span start.
    pub start_line: u32,
    /// One-based column number of the span start.
    pub start_column: u32,
    /// One-based line number of the span end.
    pub end_line: u32,
    /// One-based column number of the span end.
    pub end_column: u32,
    /// Equation/requirement label if present.
    pub label: Option<String>,
}

/// A canonical Evidence IR artifact: the unified form for Function Objects,
/// evidence index records, and receipts.
///
/// The `canonical_hash` is computed over [`EvidenceArtifactHashPayload`], which
/// excludes `canonical_hash` and `source_span` so identity is stable across
/// path relocation and independent of its own hash output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceArtifact {
    /// Schema identifier, always [`EVIDENCE_IR_SCHEMA`].
    pub schema: String,
    /// Kind of artifact: "function-object-v1", "evidence-index-record", "receipt", etc.
    pub artifact_kind: String,
    /// The stable ID that threads through the chain.
    pub stable_id: String,
    /// Canonical BLAKE3 hash of the hash payload.
    pub canonical_hash: String,
    /// Provenance back to source, if known.
    pub source_span: Option<EvidenceSpan>,
    /// What this artifact verifies (or does not).
    pub verification_role: VerificationRole,
    /// Kind-specific payload.
    pub payload: serde_json::Value,
    /// Explicit non-claim boundary strings.
    pub non_claims: Vec<String>,
}

/// Reviewable typed hash material for Evidence IR artifact identity.
///
/// Exactly these fields are included in the canonical BLAKE3 hash. Display
/// fields (`source_span`) and the computed `canonical_hash` itself are
/// deliberately excluded so identity is stable across relocation.
#[derive(Debug, Clone, Serialize)]
struct EvidenceArtifactHashPayload<'a> {
    schema: &'a str,
    artifact_kind: &'a str,
    stable_id: &'a str,
    verification_role: &'a str,
    payload: &'a serde_json::Value,
    non_claims: &'a [String],
}

/// Check report returned by [`check_evidence_artifact`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCheckReport {
    pub valid: bool,
    pub issue_codes: Vec<&'static str>,
}

/// Build a canonical Evidence IR artifact from typed inputs.
///
/// # Errors
///
/// Returns [`EvidenceIrError`] if the payload cannot be serialized into
/// canonical JSON for hashing.
pub fn build_evidence_artifact(
    artifact_kind: &str,
    stable_id: &str,
    source_span: Option<EvidenceSpan>,
    verification_role: VerificationRole,
    payload: serde_json::Value,
    non_claims: Vec<String>,
) -> Result<EvidenceArtifact, EvidenceIrError> {
    let mut artifact = EvidenceArtifact {
        schema: EVIDENCE_IR_SCHEMA.to_owned(),
        artifact_kind: artifact_kind.to_owned(),
        stable_id: stable_id.to_owned(),
        canonical_hash: String::new(),
        source_span,
        verification_role,
        payload,
        non_claims,
    };
    artifact.canonical_hash = canonical_hash(&artifact)?;
    Ok(artifact)
}

/// Computes the canonical BLAKE3 identity for an Evidence IR artifact.
///
/// # Errors
///
/// Returns [`EvidenceIrError`] if the hash payload cannot be serialized.
pub fn canonical_hash(artifact: &EvidenceArtifact) -> Result<String, EvidenceIrError> {
    let payload = EvidenceArtifactHashPayload {
        schema: &artifact.schema,
        artifact_kind: &artifact.artifact_kind,
        stable_id: &artifact.stable_id,
        verification_role: artifact.verification_role.as_str(),
        payload: &artifact.payload,
        non_claims: &artifact.non_claims,
    };
    let value = serde_json::to_value(&payload).map_err(|error| EvidenceIrError::CanonicalSerialization {
        detail: error.to_string(),
    })?;
    let bytes = serde_json::to_string(&value).map_err(|error| EvidenceIrError::CanonicalSerialization {
        detail: error.to_string(),
    })?;
    Ok(full_hash(&bytes))
}

/// Validates a JSON-serialized Evidence IR artifact.
///
/// Checks: schema field, artifact_kind non-empty, stable_id non-empty,
/// canonical_hash matches recomputed hash, non_claims present and not
/// weakened, verification_role valid, no overclaim markers.
#[must_use]
pub fn check_evidence_artifact(json: &str) -> EvidenceCheckReport {
    let mut issue_codes = Vec::new();

    let artifact: EvidenceArtifact = match serde_json::from_str(json) {
        Ok(artifact) => artifact,
        Err(error) => {
            issue_codes.push("malformed_json");
            let _ = error;
            return EvidenceCheckReport {
                valid: false,
                issue_codes,
            };
        }
    };

    if artifact.schema != EVIDENCE_IR_SCHEMA {
        issue_codes.push("schema_mismatch");
    }
    if artifact.artifact_kind.trim().is_empty() {
        issue_codes.push("artifact_kind_empty");
    }
    if artifact.stable_id.trim().is_empty() {
        issue_codes.push("stable_id_empty");
    }

    let expected_hash = match canonical_hash(&artifact) {
        Ok(hash) => hash,
        Err(_) => {
            issue_codes.push("canonical_hash_compute_failed");
            String::new()
        }
    };
    if !expected_hash.is_empty() && artifact.canonical_hash != expected_hash {
        issue_codes.push("canonical_hash_mismatch");
    }

    if artifact.non_claims.is_empty() {
        issue_codes.push("non_claims_missing");
    } else {
        let joined = artifact.non_claims.join(" ");
        if !joined.contains(REQUIRED_NON_CLAIM) {
            issue_codes.push("non_claims_weakened");
        }
    }

    let full_json = serde_json::to_string(&artifact.payload).unwrap_or_default();
    let non_claims_joined = artifact.non_claims.join(" ");
    for marker in OVERCLAIM_MARKERS {
        if full_json.contains(marker) || non_claims_joined.contains(marker) {
            issue_codes.push("overclaim_detected");
            break;
        }
    }

    issue_codes.sort_unstable();
    issue_codes.dedup();
    EvidenceCheckReport {
        valid: issue_codes.is_empty(),
        issue_codes,
    }
}

/// Resolves a stable source ID to its span from a JSON manifest.
///
/// The manifest is a JSON array of [`EvidenceSpan`] objects.
/// Returns `None` if the ID is not found or the manifest is malformed.
#[must_use]
pub fn explain_span(manifest_json: &str, source_id: &str) -> Option<EvidenceSpan> {
    let spans: Vec<EvidenceSpan> = serde_json::from_str(manifest_json).ok()?;
    spans.into_iter().find(|span| span.source_id == source_id)
}

/// Construction and validation errors for Evidence IR artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceIrError {
    CanonicalSerialization { detail: String },
}

impl std::fmt::Display for EvidenceIrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CanonicalSerialization { detail } => {
                write!(f, "canonicalize evidence artifact: {detail}")
            }
        }
    }
}

impl std::error::Error for EvidenceIrError {}

fn full_hash(input: &str) -> String {
    format!("b3:{}", blake3::hash(input.as_bytes()).to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_span() -> EvidenceSpan {
        EvidenceSpan {
            source_id: "eq:bs_001".to_owned(),
            source_path: "src/pricing.rs".to_owned(),
            start_byte: 142,
            end_byte: 168,
            start_line: 7,
            start_column: 12,
            end_line: 7,
            end_column: 38,
            label: Some("eq:bs".to_owned()),
        }
    }

    fn sample_payload() -> serde_json::Value {
        serde_json::json!({
            "function": "black_scholes_call",
            "body_hash": "b3:abc123",
        })
    }

    fn sample_non_claims() -> Vec<String> {
        vec![
            "evidence artifact records identity and linkage only".to_owned(),
            REQUIRED_NON_CLAIM.to_owned(),
        ]
    }

    fn sample_artifact() -> EvidenceArtifact {
        build_evidence_artifact(
            "function-object-v1",
            "eq:bs_001",
            Some(sample_span()),
            VerificationRole::RecordedOnly,
            sample_payload(),
            sample_non_claims(),
        )
        .expect("build sample artifact")
    }

    #[test]
    fn artifact_hash_is_stable_for_same_core_inputs() {
        let one = sample_artifact();
        let two = sample_artifact();
        assert_eq!(one.canonical_hash, two.canonical_hash);
        assert_eq!(one.schema, EVIDENCE_IR_SCHEMA);
    }

    #[test]
    fn hash_excludes_display_fields() {
        let artifact = sample_artifact();

        // Moving the source span should not change the hash.
        let mut relocated = artifact.clone();
        relocated.source_span = Some(EvidenceSpan {
            source_path: "/elsewhere/src/pricing.rs".to_owned(),
            start_byte: 0,
            ..sample_span()
        });
        assert_eq!(
            canonical_hash(&artifact).unwrap(),
            canonical_hash(&relocated).unwrap(),
            "hash must be stable across path relocation"
        );

        // Changing the canonical_hash field itself must not change the recomputed hash.
        let mut tampered = artifact.clone();
        tampered.canonical_hash = "b3:deadbeef".to_owned();
        assert_eq!(
            canonical_hash(&artifact).unwrap(),
            canonical_hash(&tampered).unwrap(),
            "hash must be independent of its own output"
        );
    }

    #[test]
    fn check_passes_for_valid_artifact() {
        let artifact = sample_artifact();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(report.valid);
        assert!(report.issue_codes.is_empty());
    }

    #[test]
    fn check_fails_closed_for_malformed_json() {
        let report = check_evidence_artifact("{ not json");
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"malformed_json"));
    }

    #[test]
    fn check_fails_closed_for_schema_mismatch() {
        let mut artifact = sample_artifact();
        artifact.schema = "wrong.schema".to_owned();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"schema_mismatch"));
    }

    #[test]
    fn check_fails_closed_for_empty_stable_id() {
        let artifact = build_evidence_artifact(
            "test",
            "",
            None,
            VerificationRole::RecordedOnly,
            serde_json::json!({}),
            sample_non_claims(),
        )
        .unwrap();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"stable_id_empty"));
    }

    #[test]
    fn check_fails_closed_for_hash_mismatch() {
        let mut artifact = sample_artifact();
        artifact.canonical_hash = "b3:deadbeef".to_owned();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"canonical_hash_mismatch"));
    }

    #[test]
    fn check_fails_closed_for_missing_non_claims() {
        let artifact =
            build_evidence_artifact("test", "id", None, VerificationRole::RecordedOnly, serde_json::json!({}), vec![])
                .unwrap();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"non_claims_missing"));
    }

    #[test]
    fn check_fails_closed_for_weakened_non_claims() {
        let artifact =
            build_evidence_artifact("test", "id", None, VerificationRole::RecordedOnly, serde_json::json!({}), vec![
                "this is fine".to_owned(),
            ])
            .unwrap();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"non_claims_weakened"));
    }

    #[test]
    fn check_rejects_overclaim_in_payload() {
        let artifact = build_evidence_artifact(
            "test",
            "id",
            None,
            VerificationRole::Property,
            serde_json::json!({"claim": "proves semantic equivalence"}),
            sample_non_claims(),
        )
        .unwrap();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"overclaim_detected"));
    }

    #[test]
    fn check_rejects_overclaim_in_non_claims() {
        let artifact =
            build_evidence_artifact("test", "id", None, VerificationRole::Property, serde_json::json!({}), vec![
                "proves whole-program correctness".to_owned(),
                REQUIRED_NON_CLAIM.to_owned(),
            ])
            .unwrap();
        let json = serde_json::to_string(&artifact).unwrap();
        let report = check_evidence_artifact(&json);
        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"overclaim_detected"));
    }

    #[test]
    fn verification_role_serializes_to_snake_case() {
        assert_eq!(VerificationRole::Property.as_str(), "property");
        assert_eq!(VerificationRole::RecordedOnly.as_str(), "recorded_only");
        assert_eq!(VerificationRole::Boundary.as_str(), "boundary");

        let json = serde_json::to_string(&VerificationRole::RecordedOnly).unwrap();
        assert_eq!(json, "\"recorded_only\"");
    }

    #[test]
    fn explain_resolves_known_id() {
        let manifest = serde_json::to_string(&[sample_span()]).unwrap();
        let resolved = explain_span(&manifest, "eq:bs_001");
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().source_path, "src/pricing.rs");
    }

    #[test]
    fn explain_returns_none_for_unknown_id() {
        let manifest = serde_json::to_string(&[sample_span()]).unwrap();
        let resolved = explain_span(&manifest, "nonexistent");
        assert!(resolved.is_none());
    }

    #[test]
    fn explain_returns_none_for_malformed_manifest() {
        let resolved = explain_span("{ not json", "eq:bs_001");
        assert!(resolved.is_none());
    }
}
