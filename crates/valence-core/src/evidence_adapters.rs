//! Pure adapter validators for external evidence rows that map into Valence
//! Evidence IR identity/linkage surfaces.
//!
//! These adapters validate parsed in-memory records only. Producers remain
//! responsible for measuring bytes, source analysis, replay/VMM semantics,
//! lifecycle policy, and release-readiness decisions.
//!
//! r[impl valence.evidence_ir.cairn_content_addressed_compat.manifest]
//! r[impl valence.evidence_ir.cairn_content_addressed_compat.validation]
//! r[impl valence.evidence_ir.chaoscontrol_export.contract]
//! r[impl valence.evidence_ir.chaoscontrol_export.validation]
//! r[impl valence.evidence_ir.state_machine_evidence.contract]
//! r[impl valence.evidence_ir.state_machine_evidence.validation]
//! r[impl valence.provenance.canonical_core_manifest]
//! r[impl valence.provenance.canonical_core_drift]

use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::shared_identity::DigestRole;
use crate::shared_identity::MIXED_BLAKE3_PROFILE;
use crate::shared_identity::blake3_hex;
use crate::shared_identity::contains_overclaim;
use crate::shared_identity::parse_digest;

const FIELD_ARTIFACT_KIND: &str = "artifact_kind";
const FIELD_DIGEST: &str = "digest";
const FIELD_DISPOSITION: &str = "compatibility_disposition";
const FIELD_EVIDENCE_TIER: &str = "evidence_tier";
const FIELD_FAMILY: &str = "family";
const FIELD_HASH: &str = "hash";
const FIELD_HASH_MATERIAL: &str = "hash_material";
const FIELD_NON_CLAIMS: &str = "non_claims";
const FIELD_POINTER_TARGET: &str = "pointer_target";
const FIELD_PROOF_LINK: &str = "proof_link";
const FIELD_SCHEMA: &str = "schema";
const FIELD_SOURCE_SPAN: &str = "source_span";
const FIELD_STATUS: &str = "status";
const FIELD_VERIFICATION_ROLE: &str = "verification_role";

pub const CAIRN_COMPAT_REQUIRED_NON_CLAIM: &str =
    "Cairn compatibility validation proves identity/linkage compatibility only";
pub const CHAOSCONTROL_EXPORT_REQUIRED_NON_CLAIM: &str =
    "ChaosControl export validation proves evidence identity/linkage only";
pub const STATE_MACHINE_REQUIRED_NON_CLAIM: &str =
    "state-machine evidence validation proves receipt shape, identity, linkage, and non-claims only";
pub const CANONICAL_CORE_REQUIRED_NON_CLAIM: &str =
    "canonical core cutover proves ownership and identity compatibility only";

const CAIRN_COMPAT_SUPPORTED_CLAIM: &str = "Cairn compatibility validation proves identity/linkage compatibility only and does not prove Cairn lifecycle readiness, source-code correctness, verifier soundness, semantic equivalence, release eligibility, or whole-program safety";
const CHAOSCONTROL_SUPPORTED_CLAIM: &str = "ChaosControl export validation proves evidence identity/linkage only and does not prove universal determinism, hosted-product parity, replay completeness, or VMM correctness";
const STATE_MACHINE_SUPPORTED_CLAIM: &str = "state-machine evidence validation proves receipt shape, identity, linkage, and non-claims only, not Rust semantics, protocol completeness, liveness, verifier soundness, exhaustive Rust coverage, or release readiness";
const CANONICAL_CORE_SUPPORTED_CLAIM: &str = "canonical core cutover proves ownership and identity compatibility only, not behavioral correctness, semantic equivalence, release eligibility, verifier soundness, or requirement satisfaction";

const COMPATIBLE_DISPOSITION: &str = "compatible";
const MIGRATION_EXCEPTION_DISPOSITION: &str = "migration-exception";
const CANONICAL_DISPOSITION: &str = "canonical";
const LEGACY_COMPATIBLE_DISPOSITION: &str = "legacy-compatible";
const RETIRED_DISPOSITION: &str = "retired";

const SUPPORTED_CAIRN_FAMILIES: &[&str] = &["function-object-v1", "name-pointer", "export-bundle"];

const SUPPORTED_CHAOSCONTROL_FAMILIES: &[&str] = &[
    "replay-verdict",
    "dogfood-receipt",
    "assertion-summary",
    "readiness-report",
];

const ALLOWED_CHAOSCONTROL_ROLES: &[&str] = &["recorded_only", "boundary"];
const ALLOWED_STATE_MACHINE_TIERS: &[&str] = &["source_shape", "bounded_source_shape"];
const ALLOWED_STATUS_VALUES: &[&str] = &["pass", "fail", "skipped"];
const ALLOWED_CORE_DISPOSITIONS: &[&str] = &[
    CANONICAL_DISPOSITION,
    LEGACY_COMPATIBLE_DISPOSITION,
    MIGRATION_EXCEPTION_DISPOSITION,
    RETIRED_DISPOSITION,
];

/// Field-scoped deterministic adapter issue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct AdapterIssue {
    pub field: &'static str,
    pub code: &'static str,
}

impl AdapterIssue {
    #[must_use]
    pub const fn new(field: &'static str, code: &'static str) -> Self {
        Self { field, code }
    }
}

/// Generic adapter report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdapterReport {
    pub valid: bool,
    pub issues: Vec<AdapterIssue>,
    pub supported_claim: String,
    pub mapped_identity: Option<String>,
    pub migration_exception: bool,
}

/// Manifest row for Cairn content-addressed Rust evidence compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CairnContentAddressedManifestRow {
    pub family: String,
    pub schema: String,
    pub hash_field: String,
    pub hash_material_profile: String,
    pub valence_artifact_kind: String,
    pub verification_role: String,
    pub disposition: String,
    pub non_claims: Vec<String>,
}

/// Parsed Cairn content-addressed receipt candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CairnContentAddressedReceipt {
    pub family: String,
    pub schema: String,
    pub stable_id: String,
    pub declared_hash: String,
    pub hash_material: Vec<u8>,
    pub pointer_targets: Vec<String>,
    pub known_targets: Vec<String>,
    pub ambient_entropy_fields: Vec<String>,
    pub non_claims: Vec<String>,
}

/// ChaosControl export adapter row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChaosControlExportRow {
    pub family: String,
    pub schema: String,
    pub stable_id: String,
    pub canonical_hash: String,
    pub canonical_material: Vec<u8>,
    pub payload_digest: String,
    pub verification_role: String,
    pub non_claims: Vec<String>,
}

/// Octet-produced state-machine evidence receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateMachineEvidenceReceipt {
    pub schema: String,
    pub transition_core_id: String,
    pub source_span_id: String,
    pub state_type_id: String,
    pub event_type_id: String,
    pub variant_inventory_digest: String,
    pub coverage_summary_digest: String,
    pub coverage_summary_material: Vec<u8>,
    pub status: String,
    pub diagnostics: Vec<String>,
    pub artifact_identity: String,
    pub parent_export_identity: String,
    pub evidence_tier: String,
    pub proof_links: Vec<String>,
    pub available_proof_receipts: Vec<String>,
    pub non_claims: Vec<String>,
}

/// Canonical Valence core manifest row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCoreManifestRow {
    pub module_or_schema: String,
    pub canonical_valence_path: String,
    pub legacy_octet_path: Option<String>,
    pub schema_identity: String,
    pub compatibility_disposition: String,
    pub fixture_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

/// Drift-check input for a canonical/legacy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalCoreDriftRow {
    pub row_id: String,
    pub canonical_digest: String,
    pub observed_digest: String,
    pub compatibility_disposition: String,
    pub migration_exception_owner: Option<String>,
    pub non_claims: Vec<String>,
}

/// Validate Cairn content-addressed compatibility manifest rows.
#[must_use]
pub fn validate_cairn_content_addressed_manifest(rows: &[CairnContentAddressedManifestRow]) -> AdapterReport {
    let mut issues = Vec::new();
    for row in rows {
        if !SUPPORTED_CAIRN_FAMILIES.contains(&row.family.as_str()) {
            issues.push(AdapterIssue::new(FIELD_FAMILY, "unsupported_family"));
        }
        if row.schema.trim().is_empty() {
            issues.push(AdapterIssue::new(FIELD_SCHEMA, "schema_missing"));
        }
        if row.hash_field.trim().is_empty() || row.hash_material_profile.trim().is_empty() {
            issues.push(AdapterIssue::new(FIELD_HASH_MATERIAL, "hash_material_profile_missing"));
        }
        if row.valence_artifact_kind.trim().is_empty() {
            issues.push(AdapterIssue::new(FIELD_ARTIFACT_KIND, "artifact_kind_missing"));
        }
        if row.verification_role.trim().is_empty() {
            issues.push(AdapterIssue::new(FIELD_VERIFICATION_ROLE, "verification_role_missing"));
        }
        if row.disposition != COMPATIBLE_DISPOSITION && row.disposition != MIGRATION_EXCEPTION_DISPOSITION {
            issues.push(AdapterIssue::new(FIELD_DISPOSITION, "unsupported_disposition"));
        }
        validate_non_claims(&row.non_claims, CAIRN_COMPAT_REQUIRED_NON_CLAIM, &mut issues);
    }
    finish_report(issues, CAIRN_COMPAT_SUPPORTED_CLAIM, None, false)
}

/// Validate and map one Cairn content-addressed receipt into bounded Valence identity/linkage.
#[must_use]
pub fn validate_cairn_content_addressed_receipt(
    receipt: &CairnContentAddressedReceipt,
    manifest: &[CairnContentAddressedManifestRow],
) -> AdapterReport {
    let Some(row) = manifest.iter().find(|row| row.family == receipt.family) else {
        return finish_report(
            vec![AdapterIssue::new(FIELD_FAMILY, "unsupported_family")],
            CAIRN_COMPAT_SUPPORTED_CLAIM,
            None,
            false,
        );
    };

    let mut issues = Vec::new();
    if row.schema != receipt.schema {
        issues.push(AdapterIssue::new(FIELD_SCHEMA, "schema_drift"));
    }
    validate_hash(&receipt.declared_hash, &receipt.hash_material, DigestRole::ReceiptIdentity, &mut issues);
    validate_non_claims(&receipt.non_claims, CAIRN_COMPAT_REQUIRED_NON_CLAIM, &mut issues);
    if !receipt.ambient_entropy_fields.is_empty() {
        issues.push(AdapterIssue::new(FIELD_HASH_MATERIAL, "ambient_entropy_in_hash_material"));
    }
    let known_targets = receipt.known_targets.iter().collect::<BTreeSet<_>>();
    if receipt.pointer_targets.iter().any(|target| !known_targets.contains(target)) {
        issues.push(AdapterIssue::new(FIELD_POINTER_TARGET, "unknown_pointer_target"));
    }
    let migration_exception = row.disposition == MIGRATION_EXCEPTION_DISPOSITION;
    finish_report(issues, CAIRN_COMPAT_SUPPORTED_CLAIM, Some(receipt.stable_id.clone()), migration_exception)
}

/// Validate a ChaosControl export row without claiming VMM/replay semantics.
#[must_use]
pub fn validate_chaoscontrol_export(row: &ChaosControlExportRow) -> AdapterReport {
    let mut issues = Vec::new();
    if !SUPPORTED_CHAOSCONTROL_FAMILIES.contains(&row.family.as_str()) {
        issues.push(AdapterIssue::new(FIELD_FAMILY, "unsupported_family"));
    }
    if row.schema.trim().is_empty() {
        issues.push(AdapterIssue::new(FIELD_SCHEMA, "schema_missing"));
    }
    if row.stable_id.trim().is_empty() {
        issues.push(AdapterIssue::new("stable_id", "stable_id_missing"));
    }
    validate_hash(&row.canonical_hash, &row.canonical_material, DigestRole::ArtifactIdentity, &mut issues);
    validate_digest_syntax(&row.payload_digest, DigestRole::ArtifactIdentity, &mut issues);
    if !ALLOWED_CHAOSCONTROL_ROLES.contains(&row.verification_role.as_str()) {
        issues.push(AdapterIssue::new(FIELD_VERIFICATION_ROLE, "role_promotion"));
    }
    validate_non_claims(&row.non_claims, CHAOSCONTROL_EXPORT_REQUIRED_NON_CLAIM, &mut issues);
    finish_report(issues, CHAOSCONTROL_SUPPORTED_CLAIM, Some(row.stable_id.clone()), false)
}

/// Validate an Octet state-machine exhaustiveness receipt.
#[must_use]
pub fn validate_state_machine_evidence(receipt: &StateMachineEvidenceReceipt) -> AdapterReport {
    let mut issues = Vec::new();
    validate_required_text(&receipt.schema, FIELD_SCHEMA, "schema_missing", &mut issues);
    validate_required_text(&receipt.transition_core_id, "transition_core_id", "transition_core_missing", &mut issues);
    validate_required_text(&receipt.source_span_id, FIELD_SOURCE_SPAN, "source_span_missing", &mut issues);
    validate_required_text(&receipt.state_type_id, "state_type_id", "state_type_missing", &mut issues);
    validate_required_text(&receipt.event_type_id, "event_type_id", "event_type_missing", &mut issues);
    validate_digest_syntax(&receipt.variant_inventory_digest, DigestRole::ArtifactIdentity, &mut issues);
    validate_hash(
        &receipt.coverage_summary_digest,
        &receipt.coverage_summary_material,
        DigestRole::ReceiptIdentity,
        &mut issues,
    );
    validate_digest_syntax(&receipt.artifact_identity, DigestRole::ArtifactIdentity, &mut issues);
    validate_digest_syntax(&receipt.parent_export_identity, DigestRole::ArtifactIdentity, &mut issues);
    if !ALLOWED_STATUS_VALUES.contains(&receipt.status.as_str()) {
        issues.push(AdapterIssue::new(FIELD_STATUS, "unknown_status"));
    }
    if receipt.status == "pass" && !receipt.diagnostics.is_empty() {
        issues.push(AdapterIssue::new(FIELD_STATUS, "passing_receipt_has_diagnostics"));
    }
    if !ALLOWED_STATE_MACHINE_TIERS.contains(&receipt.evidence_tier.as_str()) {
        issues.push(AdapterIssue::new(FIELD_EVIDENCE_TIER, "unknown_or_promoted_tier"));
    }
    let available = receipt.available_proof_receipts.iter().collect::<BTreeSet<_>>();
    if receipt.proof_links.iter().any(|link| !available.contains(link)) {
        issues.push(AdapterIssue::new(FIELD_PROOF_LINK, "missing_linked_proof"));
    }
    validate_non_claims(&receipt.non_claims, STATE_MACHINE_REQUIRED_NON_CLAIM, &mut issues);

    finish_report(issues, STATE_MACHINE_SUPPORTED_CLAIM, Some(receipt.transition_core_id.clone()), false)
}

/// Validate canonical Valence core manifest rows.
#[must_use]
pub fn validate_canonical_core_manifest(rows: &[CanonicalCoreManifestRow]) -> AdapterReport {
    let mut issues = Vec::new();
    for row in rows {
        validate_required_text(&row.module_or_schema, "module_or_schema", "module_missing", &mut issues);
        validate_required_text(
            &row.canonical_valence_path,
            "canonical_valence_path",
            "canonical_path_missing",
            &mut issues,
        );
        validate_digest_syntax(&row.schema_identity, DigestRole::ArtifactIdentity, &mut issues);
        validate_core_disposition(&row.compatibility_disposition, &mut issues);
        if row.fixture_refs.is_empty() {
            issues.push(AdapterIssue::new("fixture_refs", "fixture_refs_missing"));
        }
        validate_non_claims(&row.non_claims, CANONICAL_CORE_REQUIRED_NON_CLAIM, &mut issues);
    }
    finish_report(issues, CANONICAL_CORE_SUPPORTED_CLAIM, None, false)
}

/// Validate canonical-vs-observed drift for legacy or consumer-visible rows.
#[must_use]
pub fn validate_canonical_core_drift(rows: &[CanonicalCoreDriftRow]) -> AdapterReport {
    let mut issues = Vec::new();
    for row in rows {
        validate_digest_syntax(&row.canonical_digest, DigestRole::ArtifactIdentity, &mut issues);
        validate_digest_syntax(&row.observed_digest, DigestRole::ArtifactIdentity, &mut issues);
        validate_core_disposition(&row.compatibility_disposition, &mut issues);
        validate_non_claims(&row.non_claims, CANONICAL_CORE_REQUIRED_NON_CLAIM, &mut issues);
        if row.canonical_digest != row.observed_digest {
            let has_exception = row.compatibility_disposition == MIGRATION_EXCEPTION_DISPOSITION
                && row.migration_exception_owner.as_deref().is_some_and(|owner| !owner.trim().is_empty());
            if !has_exception {
                issues.push(AdapterIssue::new(FIELD_DIGEST, "stale_copy"));
            }
        }
    }
    finish_report(issues, CANONICAL_CORE_SUPPORTED_CLAIM, None, false)
}

fn validate_core_disposition(disposition: &str, issues: &mut Vec<AdapterIssue>) {
    if !ALLOWED_CORE_DISPOSITIONS.contains(&disposition) {
        issues.push(AdapterIssue::new(FIELD_DISPOSITION, "unsupported_disposition"));
    }
}

fn validate_required_text(value: &str, field: &'static str, code: &'static str, issues: &mut Vec<AdapterIssue>) {
    if value.trim().is_empty() {
        issues.push(AdapterIssue::new(field, code));
    }
}

fn validate_hash(declared: &str, material: &[u8], role: DigestRole, issues: &mut Vec<AdapterIssue>) {
    validate_digest_syntax(declared, role, issues);
    let expected = blake3_hex(material);
    let mut shared_issues = Vec::new();
    let parsed = parse_digest(declared, role, MIXED_BLAKE3_PROFILE, FIELD_HASH, &mut shared_issues);
    if parsed.is_some_and(|digest| digest.normalized_hex != expected) {
        issues.push(AdapterIssue::new(FIELD_HASH, "stale_hash"));
    }
}

fn validate_digest_syntax(declared: &str, role: DigestRole, issues: &mut Vec<AdapterIssue>) {
    let mut shared_issues = Vec::new();
    if parse_digest(declared, role, MIXED_BLAKE3_PROFILE, FIELD_HASH, &mut shared_issues).is_none() {
        issues.push(AdapterIssue::new(FIELD_HASH, "malformed_blake3"));
    }
}

fn validate_non_claims(non_claims: &[String], required: &str, issues: &mut Vec<AdapterIssue>) {
    if non_claims.is_empty() {
        issues.push(AdapterIssue::new(FIELD_NON_CLAIMS, "non_claims_missing"));
        return;
    }
    if !non_claims.iter().any(|claim| claim.contains(required)) {
        issues.push(AdapterIssue::new(FIELD_NON_CLAIMS, "non_claims_weakened"));
    }
    if non_claims.iter().any(|claim| contains_overclaim(claim)) {
        issues.push(AdapterIssue::new(FIELD_NON_CLAIMS, "overclaim_detected"));
    }
}

fn finish_report(
    mut issues: Vec<AdapterIssue>,
    supported_claim: &str,
    mapped_identity: Option<String>,
    migration_exception: bool,
) -> AdapterReport {
    issues.sort();
    issues.dedup();
    AdapterReport {
        valid: issues.is_empty(),
        issues,
        supported_claim: supported_claim.to_owned(),
        mapped_identity,
        migration_exception,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STABLE_ID: &str = "fixture.identity";
    const DIGEST: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

    fn hash_for(bytes: &[u8]) -> String {
        blake3_hex(bytes)
    }

    fn cairn_non_claims() -> Vec<String> {
        vec![CAIRN_COMPAT_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn chaos_non_claims() -> Vec<String> {
        vec![CHAOSCONTROL_EXPORT_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn state_machine_non_claims() -> Vec<String> {
        vec![STATE_MACHINE_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn canonical_core_non_claims() -> Vec<String> {
        vec![CANONICAL_CORE_REQUIRED_NON_CLAIM.to_owned()]
    }

    fn cairn_manifest() -> CairnContentAddressedManifestRow {
        CairnContentAddressedManifestRow {
            family: "function-object-v1".to_owned(),
            schema: "cairn.content-addressed.function-object.v1".to_owned(),
            hash_field: "object_hash".to_owned(),
            hash_material_profile: "function-object-v1".to_owned(),
            valence_artifact_kind: "function_object".to_owned(),
            verification_role: "recorded_only".to_owned(),
            disposition: COMPATIBLE_DISPOSITION.to_owned(),
            non_claims: cairn_non_claims(),
        }
    }

    #[test]
    fn cairn_manifest_and_receipt_positive_fixture_pass() {
        // r[verify valence.evidence_ir.cairn_content_addressed_compat.fixtures.positive]
        let material = b"function-object".to_vec();
        let receipt = CairnContentAddressedReceipt {
            family: "function-object-v1".to_owned(),
            schema: "cairn.content-addressed.function-object.v1".to_owned(),
            stable_id: STABLE_ID.to_owned(),
            declared_hash: hash_for(&material),
            hash_material: material,
            pointer_targets: vec!["target-a".to_owned()],
            known_targets: vec!["target-a".to_owned()],
            ambient_entropy_fields: Vec::new(),
            non_claims: cairn_non_claims(),
        };

        let manifest_report = validate_cairn_content_addressed_manifest(&[cairn_manifest()]);
        let receipt_report = validate_cairn_content_addressed_receipt(&receipt, &[cairn_manifest()]);

        assert!(manifest_report.valid);
        assert!(receipt_report.valid);
        assert_eq!(receipt_report.mapped_identity.as_deref(), Some(STABLE_ID));
    }

    #[test]
    fn cairn_receipt_negative_fixture_fails_closed() {
        // r[verify valence.evidence_ir.cairn_content_addressed_compat.fixtures.negative]
        let receipt = CairnContentAddressedReceipt {
            family: "function-object-v1".to_owned(),
            schema: "drift".to_owned(),
            stable_id: STABLE_ID.to_owned(),
            declared_hash: DIGEST.to_owned(),
            hash_material: b"different".to_vec(),
            pointer_targets: vec!["missing".to_owned()],
            known_targets: vec!["present".to_owned()],
            ambient_entropy_fields: vec!["hostname".to_owned()],
            non_claims: vec!["proves semantic equivalence".to_owned()],
        };

        let report = validate_cairn_content_addressed_receipt(&receipt, &[cairn_manifest()]);

        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "schema_drift"));
        assert!(report.issues.iter().any(|issue| issue.code == "stale_hash"));
        assert!(report.issues.iter().any(|issue| issue.code == "ambient_entropy_in_hash_material"));
        assert!(report.issues.iter().any(|issue| issue.code == "unknown_pointer_target"));
        assert!(report.issues.iter().any(|issue| issue.code == "overclaim_detected"));
    }

    #[test]
    fn chaoscontrol_export_positive_and_negative_rows() {
        // r[verify valence.evidence_ir.chaoscontrol_export.fixtures.positive]
        // r[verify valence.evidence_ir.chaoscontrol_export.fixtures.negative]
        let material = b"replay-verdict".to_vec();
        let valid = ChaosControlExportRow {
            family: "replay-verdict".to_owned(),
            schema: "chaoscontrol.replay-verdict.v1".to_owned(),
            stable_id: STABLE_ID.to_owned(),
            canonical_hash: hash_for(&material),
            canonical_material: material,
            payload_digest: DIGEST.to_owned(),
            verification_role: "recorded_only".to_owned(),
            non_claims: chaos_non_claims(),
        };
        let mut invalid = valid.clone();
        invalid.family = "vmm-proof".to_owned();
        invalid.verification_role = "property".to_owned();
        invalid.canonical_hash = DIGEST.to_owned();
        invalid.non_claims.clear();

        let pass = validate_chaoscontrol_export(&valid);
        let fail = validate_chaoscontrol_export(&invalid);

        assert!(pass.valid);
        assert!(!fail.valid);
        assert!(fail.issues.iter().any(|issue| issue.code == "unsupported_family"));
        assert!(fail.issues.iter().any(|issue| issue.code == "role_promotion"));
        assert!(fail.issues.iter().any(|issue| issue.code == "non_claims_missing"));
    }

    fn state_machine_receipt() -> StateMachineEvidenceReceipt {
        let coverage = b"state-event coverage".to_vec();
        StateMachineEvidenceReceipt {
            schema: "octet.state-machine-exhaustiveness.v1".to_owned(),
            transition_core_id: "transition.apply".to_owned(),
            source_span_id: "span:transition.apply".to_owned(),
            state_type_id: "State".to_owned(),
            event_type_id: "Event".to_owned(),
            variant_inventory_digest: DIGEST.to_owned(),
            coverage_summary_digest: hash_for(&coverage),
            coverage_summary_material: coverage,
            status: "pass".to_owned(),
            diagnostics: Vec::new(),
            artifact_identity: DIGEST.to_owned(),
            parent_export_identity: DIGEST.to_owned(),
            evidence_tier: "source_shape".to_owned(),
            proof_links: vec!["proof:transition-preserves".to_owned()],
            available_proof_receipts: vec!["proof:transition-preserves".to_owned()],
            non_claims: state_machine_non_claims(),
        }
    }

    #[test]
    fn state_machine_receipt_positive_fixture_passes() {
        // r[verify valence.evidence_ir.state_machine_evidence.fixtures.positive]
        let report = validate_state_machine_evidence(&state_machine_receipt());

        assert!(report.valid);
        assert!(report.supported_claim.contains("not Rust semantics"));
    }

    #[test]
    fn state_machine_receipt_negative_fixture_fails_closed() {
        // r[verify valence.evidence_ir.state_machine_evidence.fixtures.negative]
        let mut receipt = state_machine_receipt();
        receipt.state_type_id.clear();
        receipt.coverage_summary_digest = DIGEST.to_owned();
        receipt.evidence_tier = "formal_proof".to_owned();
        receipt.proof_links = vec!["missing".to_owned()];
        receipt.non_claims = vec!["proves whole-program safety".to_owned()];

        let report = validate_state_machine_evidence(&receipt);

        assert!(!report.valid);
        assert!(report.issues.iter().any(|issue| issue.code == "state_type_missing"));
        assert!(report.issues.iter().any(|issue| issue.code == "stale_hash"));
        assert!(report.issues.iter().any(|issue| issue.code == "unknown_or_promoted_tier"));
        assert!(report.issues.iter().any(|issue| issue.code == "missing_linked_proof"));
        assert!(report.issues.iter().any(|issue| issue.code == "overclaim_detected"));
    }

    #[test]
    fn canonical_core_manifest_and_drift_checks_cover_pass_and_fail() {
        // r[verify valence.provenance.canonical_core_manifest.positive]
        // r[verify valence.provenance.canonical_core_manifest.negative]
        // r[verify valence.provenance.canonical_core_drift.match]
        // r[verify valence.provenance.canonical_core_drift.stale]
        let manifest = CanonicalCoreManifestRow {
            module_or_schema: "evidence_ir".to_owned(),
            canonical_valence_path: "crates/valence-core/src/evidence_ir.rs".to_owned(),
            legacy_octet_path: Some("octet/crates/valence-core/src/evidence_ir.rs".to_owned()),
            schema_identity: DIGEST.to_owned(),
            compatibility_disposition: LEGACY_COMPATIBLE_DISPOSITION.to_owned(),
            fixture_refs: vec!["fixtures/evidence_ir/pass.json".to_owned()],
            non_claims: canonical_core_non_claims(),
        };
        let stale = CanonicalCoreDriftRow {
            row_id: "evidence_ir".to_owned(),
            canonical_digest: DIGEST.to_owned(),
            observed_digest: hash_for(b"drift"),
            compatibility_disposition: LEGACY_COMPATIBLE_DISPOSITION.to_owned(),
            migration_exception_owner: None,
            non_claims: canonical_core_non_claims(),
        };
        let matching = CanonicalCoreDriftRow {
            observed_digest: DIGEST.to_owned(),
            ..stale.clone()
        };

        let manifest_report = validate_canonical_core_manifest(&[manifest]);
        let matching_report = validate_canonical_core_drift(&[matching]);
        let stale_report = validate_canonical_core_drift(&[stale]);

        assert!(manifest_report.valid);
        assert!(matching_report.valid);
        assert!(!stale_report.valid);
        assert!(stale_report.issues.iter().any(|issue| issue.code == "stale_copy"));
    }
}
