//! Stack-provenance adapter for bounded Valence/Octet/Mantle traceability.
//!
//! The adapter turns verified Octet provenance exports into a Valence sidecar
//! that Mantle can bundle opaquely. Validation proves only identity and linkage:
//! artifact digests, evidence tiers, source/function/requirement IDs, and the
//! Mantle external-evidence digest. It does not prove behavioral correctness,
//! semantic equivalence, whole-program safety, or requirement satisfaction.

use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::provenance::PROVENANCE_BOUNDARY;
use crate::provenance::ProvenanceChain;
use crate::provenance::ProvenanceLink;
use crate::provenance::check_provenance_chain;

/// Canonical stack-provenance sidecar schema.
pub const STACK_PROVENANCE_SIDECAR_SCHEMA: &str = "valence.stack-provenance-sidecar.v1";
/// Octet-to-Valence export schema consumed by this adapter.
pub const OCTET_VALENCE_EXPORT_SCHEMA: &str = "octet-valence-provenance-export/v1";
/// Mantle external evidence role used for the sidecar.
pub const MANTLE_STACK_PROVENANCE_ROLE: &str = "stack-provenance-trace";
/// Claim scope recorded in the sidecar and Mantle external evidence metadata.
pub const STACK_PROVENANCE_CLAIM_SCOPE: &str = "identity-linkage-sidecar";
/// Required sidecar non-claim.
pub const STACK_PROVENANCE_NON_CLAIM: &str = "stack provenance proves identity and linkage only; it does not prove behavioral correctness, semantic equivalence, whole-program safety, or requirement satisfaction";
/// Required built-binary thread non-claim.
pub const BUILT_BINARY_THREAD_NON_CLAIM: &str = "built-binary thread verification proves measured-byte identity and linkage only; it does not prove behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, verifier soundness, or whole-program safety";
const FORMAL_PROOF_TIER: &str = "formal_proof";
const MANUAL_REVIEW_TIER: &str = "manual_review_required";
const EXTERNAL_TRELLIS_ORIGIN: &str = "external_trellis_import";
const VALID_VERIFICATION_STATUS: &str = "valid";
const BUILT_BINARY_OVERCLAIM_PATTERNS: &[&str] = &[
    "proves behavioral correctness",
    "proves semantic equivalence",
    "proves requirement satisfaction",
    "proves release eligibility",
    "proves whole-program safety",
    "certifies release eligibility",
    "guarantees behavioral correctness",
    "guarantees whole-program safety",
];
const BLAKE3_HEX_LENGTH: usize = 64;

/// Verified Octet export consumed by the adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OctetValenceProvenanceExport {
    pub schema_version: String,
    pub artifact_dir: String,
    pub verification_status: String,
    pub artifacts: Vec<StackArtifactIdentity>,
    pub provenance_records: Vec<OctetProvenanceRecord>,
    #[serde(default)]
    pub proof_cards: Vec<OctetProofCard>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
    #[serde(default)]
    pub non_claims: Vec<String>,
}

/// Generic artifact identity used by Octet, Cairn, Trellis, and Mantle adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackArtifactIdentity {
    pub artifact: String,
    pub blake3: String,
}

/// Normalized Octet provenance record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OctetProvenanceRecord {
    pub source_id: String,
    pub checker_id: String,
    pub evidence_tier: String,
    pub artifact: String,
    pub blake3: String,
    pub checked_scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verifier_identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_artifact_identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub human_action: Option<String>,
    #[serde(default)]
    pub parent_artifacts: Vec<String>,
}

/// Normalized proof-card summary from Octet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OctetProofCard {
    pub proof_or_property_id: String,
    pub source_requirement_or_policy_id: String,
    pub policy_id: String,
    pub rail_index: u32,
    pub receipt_path: String,
    pub artifact_blake3_identities: Vec<StackArtifactIdentity>,
    pub evidence_tier: String,
    pub evidence_origin: String,
    pub status: String,
    #[serde(default)]
    pub non_claims: Vec<String>,
    #[serde(default)]
    pub diagnostics: Vec<String>,
}

/// Normalized Trellis evidence record for future adapter inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrellisProofEvidenceRecord {
    pub proof_id: String,
    pub repository_revision: String,
    pub artifact: StackArtifactIdentity,
    pub evidence_tier: String,
}

/// Normalized Cairn receipt identity for future adapter inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CairnReceiptRecord {
    pub requirement_id: String,
    pub receipt_hash: String,
    pub stage: String,
}

/// Mantle external evidence record parsed from release manifests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MantleExternalEvidenceRecord {
    pub role: String,
    pub schema: String,
    pub relative_path: String,
    pub digest_blake3: String,
    pub claim_scope: String,
    #[serde(default)]
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct MantleManifestView {
    #[serde(default)]
    external_evidence: Vec<MantleExternalEvidenceRecord>,
}

/// Minimal Mantle binary identity consumed by the Valence semantic verifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MantleBinaryIdentity {
    pub relative_path: String,
    pub digest_blake3: String,
}

/// Minimal Mantle provenance coverage view consumed by built-binary threading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MantleProvenanceCoverageView {
    pub binary_hash: String,
}

/// Minimal Mantle release manifest view for stack-specific Valence verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MantleReleaseManifestView {
    #[serde(default)]
    pub external_evidence: Vec<MantleExternalEvidenceRecord>,
    #[serde(default)]
    pub binaries: Vec<MantleBinaryIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance_coverage: Option<MantleProvenanceCoverageView>,
}

/// Pure input for verifying that stack provenance reaches measured binary bytes.
pub struct BuiltBinaryThreadInput<'a> {
    pub binary_bytes: &'a [u8],
    pub octet_export: &'a OctetValenceProvenanceExport,
    pub sidecar: &'a StackProvenanceSidecar,
    pub sidecar_bytes: &'a [u8],
    pub mantle_manifest: &'a MantleReleaseManifestView,
}

/// Deterministic report for built-binary thread verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuiltBinaryThreadReport {
    pub valid: bool,
    pub issue_codes: Vec<String>,
    pub binary_hash: String,
    pub sidecar_digest_blake3: String,
    pub source_ids: Vec<String>,
    pub requirement_ids: Vec<String>,
    pub artifact_identities: Vec<StackArtifactIdentity>,
    pub non_claims: Vec<String>,
}

/// Canonical Valence stack-provenance sidecar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackProvenanceSidecar {
    pub schema: String,
    pub claim_scope: String,
    pub binary_hash: String,
    pub octet_export_artifact_dir: String,
    pub chain: ProvenanceChain,
    pub parent_artifacts: Vec<StackArtifactIdentity>,
    pub diagnostics: Vec<String>,
    pub non_claims: Vec<String>,
}

/// Validation report for stack-provenance sidecars.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackProvenanceReport {
    pub valid: bool,
    pub issue_codes: Vec<String>,
}

/// Build a sidecar from a verified Octet export and a release binary hash.
#[must_use]
pub fn build_stack_provenance_sidecar(
    octet_export: &OctetValenceProvenanceExport,
    binary_hash: &str,
) -> StackProvenanceSidecar {
    let chain = ProvenanceChain::from_links(
        octet_export
            .provenance_records
            .iter()
            .map(|record| provenance_link_from_octet(record, &octet_export.proof_cards, binary_hash))
            .collect(),
    );
    StackProvenanceSidecar {
        schema: STACK_PROVENANCE_SIDECAR_SCHEMA.to_owned(),
        claim_scope: STACK_PROVENANCE_CLAIM_SCOPE.to_owned(),
        binary_hash: binary_hash.to_owned(),
        octet_export_artifact_dir: octet_export.artifact_dir.clone(),
        chain,
        parent_artifacts: octet_export.artifacts.clone(),
        diagnostics: octet_export.diagnostics.clone(),
        non_claims: vec![STACK_PROVENANCE_NON_CLAIM.to_owned(), PROVENANCE_BOUNDARY.to_owned()],
    }
}

/// Validate a sidecar without any filesystem access.
#[must_use]
pub fn validate_stack_provenance_sidecar(sidecar: &StackProvenanceSidecar) -> StackProvenanceReport {
    let mut issue_codes = Vec::new();
    if sidecar.schema != STACK_PROVENANCE_SIDECAR_SCHEMA {
        issue_codes.push("schema_mismatch".to_owned());
    }
    if sidecar.claim_scope != STACK_PROVENANCE_CLAIM_SCOPE {
        issue_codes.push("claim_scope_mismatch".to_owned());
    }
    if !is_blake3ish(&sidecar.binary_hash) {
        issue_codes.push("binary_hash_invalid".to_owned());
    }
    if !sidecar.non_claims.iter().any(|claim| claim == STACK_PROVENANCE_NON_CLAIM) {
        issue_codes.push("non_claim_boundary_missing".to_owned());
    }
    if !sidecar.diagnostics.is_empty() {
        issue_codes.push("diagnostics_present".to_owned());
    }
    if sidecar.parent_artifacts.is_empty() {
        issue_codes.push("parent_artifact_missing".to_owned());
    }
    issue_codes.extend(validate_chain(&sidecar.chain, &sidecar.binary_hash));
    issue_codes.sort();
    issue_codes.dedup();
    StackProvenanceReport {
        valid: issue_codes.is_empty(),
        issue_codes,
    }
}

/// Validate an Octet export before building a sidecar.
#[must_use]
pub fn validate_octet_export(export: &OctetValenceProvenanceExport) -> StackProvenanceReport {
    let mut issue_codes = Vec::new();
    if export.schema_version != OCTET_VALENCE_EXPORT_SCHEMA {
        issue_codes.push("octet_schema_mismatch".to_owned());
    }
    if export.verification_status != VALID_VERIFICATION_STATUS {
        issue_codes.push("octet_verification_not_valid".to_owned());
    }
    if !export.diagnostics.is_empty() {
        issue_codes.push("octet_diagnostics_present".to_owned());
    }
    if export.provenance_records.is_empty() {
        issue_codes.push("octet_provenance_missing".to_owned());
    }
    issue_codes.extend(validate_parent_artifacts(export));
    issue_codes.extend(validate_evidence_tiers(export));
    issue_codes.sort();
    issue_codes.dedup();
    StackProvenanceReport {
        valid: issue_codes.is_empty(),
        issue_codes,
    }
}

/// Parse, build, validate, and render a stack sidecar as pretty JSON.
pub fn build_sidecar_json_from_octet_export_json(
    octet_export_json: &str,
    binary_hash: &str,
) -> Result<String, StackProvenanceReport> {
    let export = match serde_json::from_str::<OctetValenceProvenanceExport>(octet_export_json) {
        Ok(export) => export,
        Err(_) => {
            return Err(StackProvenanceReport {
                valid: false,
                issue_codes: vec!["octet_export_malformed".to_owned()],
            });
        }
    };
    let export_report = validate_octet_export(&export);
    if !export_report.valid {
        return Err(export_report);
    }
    let sidecar = build_stack_provenance_sidecar(&export, binary_hash);
    let sidecar_report = validate_stack_provenance_sidecar(&sidecar);
    if !sidecar_report.valid {
        return Err(sidecar_report);
    }
    serde_json::to_string_pretty(&sidecar).map_err(|_| StackProvenanceReport {
        valid: false,
        issue_codes: vec!["sidecar_serialize_failed".to_owned()],
    })
}

/// Verify a sidecar against a Mantle release manifest external-evidence record.
#[must_use]
pub fn verify_mantle_external_evidence_sidecar(
    mantle_manifest_json: &str,
    sidecar_json: &str,
) -> StackProvenanceReport {
    let manifest = match serde_json::from_str::<MantleManifestView>(mantle_manifest_json) {
        Ok(manifest) => manifest,
        Err(_) => {
            return StackProvenanceReport {
                valid: false,
                issue_codes: vec!["mantle_manifest_malformed".to_owned()],
            };
        }
    };
    let sidecar = match serde_json::from_str::<StackProvenanceSidecar>(sidecar_json) {
        Ok(sidecar) => sidecar,
        Err(_) => {
            return StackProvenanceReport {
                valid: false,
                issue_codes: vec!["sidecar_malformed".to_owned()],
            };
        }
    };
    let mut issue_codes = validate_stack_provenance_sidecar(&sidecar).issue_codes;
    let digest = blake3::hash(sidecar_json.as_bytes()).to_hex().to_string();
    match manifest.external_evidence.iter().find(|record| record.role == MANTLE_STACK_PROVENANCE_ROLE) {
        Some(record) => {
            if record.schema != STACK_PROVENANCE_SIDECAR_SCHEMA {
                issue_codes.push("mantle_schema_mismatch".to_owned());
            }
            if record.claim_scope != STACK_PROVENANCE_CLAIM_SCOPE {
                issue_codes.push("mantle_claim_scope_mismatch".to_owned());
            }
            if record.digest_blake3 != digest {
                issue_codes.push("sidecar_digest_mismatch".to_owned());
            }
            if !record.non_claims.iter().any(|claim| claim == STACK_PROVENANCE_NON_CLAIM) {
                issue_codes.push("mantle_non_claim_missing".to_owned());
            }
        }
        None => issue_codes.push("mantle_role_missing".to_owned()),
    }
    issue_codes.sort();
    issue_codes.dedup();
    StackProvenanceReport {
        valid: issue_codes.is_empty(),
        issue_codes,
    }
}

/// Parse local artifacts and verify that provenance reaches measured binary bytes.
#[must_use]
pub fn verify_built_binary_thread_json(
    binary_bytes: &[u8],
    octet_export_json: &str,
    sidecar_json: &str,
    mantle_manifest_json: &str,
) -> BuiltBinaryThreadReport {
    let binary_hash = blake3_hex(binary_bytes);
    let sidecar_digest_blake3 = blake3_hex(sidecar_json.as_bytes());
    let mut issue_codes = Vec::new();
    let octet_export = match serde_json::from_str::<OctetValenceProvenanceExport>(octet_export_json) {
        Ok(export) => export,
        Err(_) => {
            issue_codes.push("octet_export_malformed".to_owned());
            return built_binary_report(issue_codes, binary_hash, sidecar_digest_blake3, None, None);
        }
    };
    let sidecar = match serde_json::from_str::<StackProvenanceSidecar>(sidecar_json) {
        Ok(sidecar) => sidecar,
        Err(_) => {
            issue_codes.push("sidecar_malformed".to_owned());
            return built_binary_report(issue_codes, binary_hash, sidecar_digest_blake3, Some(&octet_export), None);
        }
    };
    let mantle_manifest = match serde_json::from_str::<MantleReleaseManifestView>(mantle_manifest_json) {
        Ok(manifest) => manifest,
        Err(_) => {
            issue_codes.push("mantle_manifest_malformed".to_owned());
            return built_binary_report(
                issue_codes,
                binary_hash,
                sidecar_digest_blake3,
                Some(&octet_export),
                Some(&sidecar),
            );
        }
    };
    verify_built_binary_thread(BuiltBinaryThreadInput {
        binary_bytes,
        octet_export: &octet_export,
        sidecar: &sidecar,
        sidecar_bytes: sidecar_json.as_bytes(),
        mantle_manifest: &mantle_manifest,
    })
}

/// Verify the measured built binary against Octet, Valence, and Mantle identities.
#[must_use]
pub fn verify_built_binary_thread(input: BuiltBinaryThreadInput<'_>) -> BuiltBinaryThreadReport {
    let binary_hash = blake3_hex(input.binary_bytes);
    let sidecar_digest_blake3 = blake3_hex(input.sidecar_bytes);
    let mut issue_codes = Vec::new();
    issue_codes.extend(validate_stack_provenance_sidecar(input.sidecar).issue_codes);
    issue_codes.extend(validate_octet_export(input.octet_export).issue_codes);
    issue_codes.extend(validate_sidecar_external_evidence(
        input.mantle_manifest.external_evidence.as_slice(),
        &sidecar_digest_blake3,
    ));
    issue_codes.extend(validate_measured_binary_identity(&binary_hash, input.sidecar, input.mantle_manifest));
    issue_codes.extend(validate_sidecar_matches_octet_export(input.octet_export, input.sidecar, &binary_hash));
    issue_codes.extend(validate_built_binary_claim_boundary(input.octet_export, input.sidecar));
    issue_codes.sort();
    issue_codes.dedup();
    built_binary_report(issue_codes, binary_hash, sidecar_digest_blake3, Some(input.octet_export), Some(input.sidecar))
}

fn built_binary_report(
    mut issue_codes: Vec<String>,
    binary_hash: String,
    sidecar_digest_blake3: String,
    octet_export: Option<&OctetValenceProvenanceExport>,
    sidecar: Option<&StackProvenanceSidecar>,
) -> BuiltBinaryThreadReport {
    issue_codes.sort();
    issue_codes.dedup();
    let source_ids = sidecar
        .map(|sidecar| {
            let mut ids = sidecar.chain.links.iter().map(|link| link.source_id.clone()).collect::<Vec<_>>();
            ids.sort();
            ids.dedup();
            ids
        })
        .unwrap_or_default();
    let requirement_ids = sidecar
        .map(|sidecar| {
            let mut ids =
                sidecar.chain.links.iter().flat_map(|link| link.requirement_ids.iter().cloned()).collect::<Vec<_>>();
            ids.sort();
            ids.dedup();
            ids
        })
        .unwrap_or_default();
    let artifact_identities = octet_export.map(|export| export.artifacts.clone()).unwrap_or_default();
    BuiltBinaryThreadReport {
        valid: issue_codes.is_empty(),
        issue_codes,
        binary_hash,
        sidecar_digest_blake3,
        source_ids,
        requirement_ids,
        artifact_identities,
        non_claims: vec![
            BUILT_BINARY_THREAD_NON_CLAIM.to_owned(),
            STACK_PROVENANCE_NON_CLAIM.to_owned(),
        ],
    }
}

fn validate_sidecar_external_evidence(
    external_evidence: &[MantleExternalEvidenceRecord],
    sidecar_digest_blake3: &str,
) -> Vec<String> {
    let mut issues = Vec::new();
    match external_evidence.iter().find(|record| record.role == MANTLE_STACK_PROVENANCE_ROLE) {
        Some(record) => {
            if record.schema != STACK_PROVENANCE_SIDECAR_SCHEMA {
                issues.push("mantle_schema_mismatch".to_owned());
            }
            if record.claim_scope != STACK_PROVENANCE_CLAIM_SCOPE {
                issues.push("mantle_claim_scope_mismatch".to_owned());
            }
            if !hash_matches(&record.digest_blake3, sidecar_digest_blake3) {
                issues.push("sidecar_digest_mismatch".to_owned());
            }
            if !record.non_claims.iter().any(|claim| claim == STACK_PROVENANCE_NON_CLAIM) {
                issues.push("mantle_non_claim_missing".to_owned());
            }
        }
        None => issues.push("mantle_role_missing".to_owned()),
    }
    issues
}

fn validate_measured_binary_identity(
    binary_hash: &str,
    sidecar: &StackProvenanceSidecar,
    mantle_manifest: &MantleReleaseManifestView,
) -> Vec<String> {
    let mut issues = Vec::new();
    if !hash_matches(&sidecar.binary_hash, binary_hash) {
        issues.push("binary_digest_mismatch".to_owned());
    }
    for link in &sidecar.chain.links {
        match link.binary_hash.as_deref() {
            Some(link_hash) if hash_matches(link_hash, binary_hash) => {}
            Some(_) | None => issues.push("provenance_link_binary_mismatch".to_owned()),
        }
    }
    if !mantle_manifest.binaries.iter().any(|binary| hash_matches(&binary.digest_blake3, binary_hash)) {
        issues.push("missing_mantle_binary_identity".to_owned());
    }
    if mantle_manifest
        .provenance_coverage
        .as_ref()
        .is_some_and(|coverage| !hash_matches(&coverage.binary_hash, binary_hash))
    {
        issues.push("provenance_coverage_binary_mismatch".to_owned());
    }
    issues
}

fn validate_sidecar_matches_octet_export(
    export: &OctetValenceProvenanceExport,
    sidecar: &StackProvenanceSidecar,
    binary_hash: &str,
) -> Vec<String> {
    let mut issues = Vec::new();
    if sidecar.octet_export_artifact_dir != export.artifact_dir {
        issues.push("octet_export_artifact_dir_mismatch".to_owned());
    }
    if artifact_identity_set(&sidecar.parent_artifacts) != artifact_identity_set(&export.artifacts) {
        issues.push("octet_parent_artifact_mismatch".to_owned());
    }
    let expected = build_stack_provenance_sidecar(export, binary_hash);
    if expected.chain.links != sidecar.chain.links {
        issues.push("sidecar_octet_link_mismatch".to_owned());
    }
    issues
}

fn validate_built_binary_claim_boundary(
    export: &OctetValenceProvenanceExport,
    sidecar: &StackProvenanceSidecar,
) -> Vec<String> {
    let overclaim_present = export
        .non_claims
        .iter()
        .chain(sidecar.non_claims.iter())
        .chain(export.proof_cards.iter().flat_map(|card| card.non_claims.iter()))
        .any(|claim| built_binary_overclaim_detected(claim));
    if overclaim_present {
        vec!["overclaim_detected".to_owned()]
    } else {
        Vec::new()
    }
}

fn built_binary_overclaim_detected(claim: &str) -> bool {
    let lower_claim = claim.to_ascii_lowercase();
    BUILT_BINARY_OVERCLAIM_PATTERNS.iter().any(|pattern| lower_claim.contains(pattern))
}

fn artifact_identity_set(artifacts: &[StackArtifactIdentity]) -> BTreeSet<(String, String)> {
    artifacts.iter().map(|artifact| (artifact.artifact.clone(), artifact.blake3.clone())).collect()
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn hash_matches(expected: &str, actual: &str) -> bool {
    normalize_hash(expected) == normalize_hash(actual)
}

fn normalize_hash(hash: &str) -> &str {
    hash.strip_prefix("b3:").unwrap_or(hash)
}

fn provenance_link_from_octet(
    record: &OctetProvenanceRecord,
    proof_cards: &[OctetProofCard],
    binary_hash: &str,
) -> ProvenanceLink {
    ProvenanceLink {
        source_id: record.source_id.clone(),
        function_object_id: record.artifact.clone(),
        requirement_ids: requirement_ids_for_record(record, proof_cards),
        artifact_hash: Some(record.blake3.clone()),
        binary_hash: Some(binary_hash.to_owned()),
    }
}

fn requirement_ids_for_record(record: &OctetProvenanceRecord, proof_cards: &[OctetProofCard]) -> Vec<String> {
    let mut requirements = proof_cards
        .iter()
        .filter(|card| card.receipt_path == record.artifact || card.policy_id == record.source_id)
        .map(|card| card.source_requirement_or_policy_id.clone())
        .collect::<Vec<_>>();
    requirements.sort();
    requirements.dedup();
    requirements
}

fn validate_chain(chain: &ProvenanceChain, binary_hash: &str) -> Vec<String> {
    let mut issues = check_provenance_chain(chain)
        .issue_codes
        .into_iter()
        .map(|code| format!("chain_{code}"))
        .collect::<Vec<_>>();
    for link in &chain.links {
        if link.binary_hash.as_deref() != Some(binary_hash) {
            issues.push("binary_hash_mismatch".to_owned());
        }
        if link.artifact_hash.as_deref().is_some_and(|hash| !is_blake3ish(hash)) {
            issues.push("artifact_hash_invalid".to_owned());
        }
    }
    issues
}

fn validate_parent_artifacts(export: &OctetValenceProvenanceExport) -> Vec<String> {
    let known = export.artifacts.iter().map(|artifact| artifact.artifact.as_str()).collect::<BTreeSet<_>>();
    let mut issues = Vec::new();
    for record in &export.provenance_records {
        if record.artifact.trim().is_empty() || !known.contains(record.artifact.as_str()) {
            issues.push("parent_artifact_missing".to_owned());
        }
        for parent in &record.parent_artifacts {
            if !known.contains(parent.as_str()) {
                issues.push("parent_artifact_missing".to_owned());
            }
        }
    }
    issues
}

fn validate_evidence_tiers(export: &OctetValenceProvenanceExport) -> Vec<String> {
    let mut issues = Vec::new();
    for card in &export.proof_cards {
        if !card.diagnostics.is_empty() {
            issues.push("proof_card_diagnostics_present".to_owned());
        }
        if card.evidence_origin == EXTERNAL_TRELLIS_ORIGIN && card.evidence_tier == FORMAL_PROOF_TIER {
            issues.push("manual_review_evidence_promoted".to_owned());
        }
    }
    for record in &export.provenance_records {
        if record.human_action.is_some() && record.evidence_tier == FORMAL_PROOF_TIER {
            issues.push("manual_review_evidence_promoted".to_owned());
        }
        if record.evidence_tier == MANUAL_REVIEW_TIER && record.human_action.as_deref().unwrap_or_default().is_empty() {
            issues.push("manual_review_action_missing".to_owned());
        }
    }
    issues
}

fn is_blake3ish(hash: &str) -> bool {
    let raw = hash.strip_prefix("b3:").unwrap_or(hash);
    raw.len() == BLAKE3_HEX_LENGTH && raw.chars().all(|character| character.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BINARY_HASH: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    const STATUS_HASH: &str = "2222222222222222222222222222222222222222222222222222222222222222";
    const RESULTS_HASH: &str = "3333333333333333333333333333333333333333333333333333333333333333";

    #[test]
    fn builds_and_verifies_minimal_release_trace() {
        let export = sample_export();
        let json = build_sidecar_json_from_octet_export_json(&serde_json::to_string(&export).unwrap(), BINARY_HASH)
            .expect("sidecar json");
        let digest = blake3::hash(json.as_bytes()).to_hex().to_string();
        let manifest = mantle_manifest(&digest);
        let report = verify_mantle_external_evidence_sidecar(&manifest, &json);

        assert!(report.valid, "unexpected issues: {:?}", report.issue_codes);
    }

    #[test]
    fn keeps_octet_proof_card_requirement_links() {
        let sidecar = build_stack_provenance_sidecar(&sample_export(), BINARY_HASH);
        let links = sidecar.chain.resolve_requirement("r[octet.policy.no_unwrap]");

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].source_id, "octet.standards.formal-proof");
    }

    #[test]
    fn rejects_missing_mantle_role_digest_mismatch_and_binary_mismatch() {
        let mut sidecar = build_stack_provenance_sidecar(&sample_export(), BINARY_HASH);
        sidecar.chain.links[0].binary_hash = Some(STATUS_HASH.to_owned());
        let json = serde_json::to_string_pretty(&sidecar).unwrap();
        let digest = blake3::hash(b"different bytes").to_hex().to_string();
        let manifest = mantle_manifest(&digest);
        let report = verify_mantle_external_evidence_sidecar(&manifest, &json);

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"binary_hash_mismatch".to_owned()));
        assert!(report.issue_codes.contains(&"sidecar_digest_mismatch".to_owned()));

        let missing_role = r#"{"external_evidence":[]}"#;
        let missing_role_report = verify_mantle_external_evidence_sidecar(missing_role, &json);
        assert!(missing_role_report.issue_codes.contains(&"mantle_role_missing".to_owned()));
    }

    #[test]
    fn rejects_stale_parent_and_trellis_tier_promotion() {
        let mut export = sample_export();
        export.provenance_records[0].parent_artifacts.push("missing.json".to_owned());
        export.proof_cards[0].evidence_origin = EXTERNAL_TRELLIS_ORIGIN.to_owned();
        export.proof_cards[0].evidence_tier = FORMAL_PROOF_TIER.to_owned();
        let report = validate_octet_export(&export);

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"parent_artifact_missing".to_owned()));
        assert!(report.issue_codes.contains(&"manual_review_evidence_promoted".to_owned()));
    }

    #[test]
    fn rejects_malformed_octet_export() {
        let report = build_sidecar_json_from_octet_export_json("not json", BINARY_HASH).unwrap_err();

        assert!(!report.valid);
        assert_eq!(report.issue_codes, vec!["octet_export_malformed".to_owned()]);
    }

    #[test]
    fn built_binary_thread_accepts_measured_binary_identity() {
        let binary_bytes = b"valence-built-binary-fixture";
        let binary_hash = blake3_hex(binary_bytes);
        let export = sample_export();
        let sidecar = build_stack_provenance_sidecar(&export, &binary_hash);
        let sidecar_json = serde_json::to_vec_pretty(&sidecar).unwrap();
        let sidecar_digest = blake3_hex(&sidecar_json);
        let manifest = mantle_release_manifest(&sidecar_digest, &binary_hash);
        let report = verify_built_binary_thread(BuiltBinaryThreadInput {
            binary_bytes,
            octet_export: &export,
            sidecar: &sidecar,
            sidecar_bytes: &sidecar_json,
            mantle_manifest: &manifest,
        });

        assert!(report.valid, "unexpected issues: {:?}", report.issue_codes);
        assert_eq!(report.binary_hash, binary_hash);
        assert_eq!(report.source_ids, vec!["octet.standards.formal-proof".to_owned()]);
        assert!(report.non_claims.contains(&BUILT_BINARY_THREAD_NON_CLAIM.to_owned()));
    }

    #[test]
    fn built_binary_thread_rejects_stale_bytes_and_missing_binary_identity() {
        let binary_bytes = b"valence-built-binary-fixture";
        let stale_binary_bytes = b"valence-built-binary-fixture-tampered";
        let binary_hash = blake3_hex(binary_bytes);
        let export = sample_export();
        let sidecar = build_stack_provenance_sidecar(&export, &binary_hash);
        let sidecar_json = serde_json::to_vec_pretty(&sidecar).unwrap();
        let sidecar_digest = blake3_hex(&sidecar_json);
        let manifest = mantle_release_manifest(&sidecar_digest, &binary_hash);
        let report = verify_built_binary_thread(BuiltBinaryThreadInput {
            binary_bytes: stale_binary_bytes,
            octet_export: &export,
            sidecar: &sidecar,
            sidecar_bytes: &sidecar_json,
            mantle_manifest: &manifest,
        });

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"binary_digest_mismatch".to_owned()));
        assert!(report.issue_codes.contains(&"provenance_link_binary_mismatch".to_owned()));
        assert!(report.issue_codes.contains(&"missing_mantle_binary_identity".to_owned()));
    }

    #[test]
    fn built_binary_thread_rejects_stale_sidecar_digest_and_octet_mismatch() {
        let binary_bytes = b"valence-built-binary-fixture";
        let binary_hash = blake3_hex(binary_bytes);
        let mut export = sample_export();
        let sidecar = build_stack_provenance_sidecar(&export, &binary_hash);
        export.artifacts[0].blake3 = RESULTS_HASH.to_owned();
        export.provenance_records[0].blake3 = RESULTS_HASH.to_owned();
        let sidecar_json = serde_json::to_vec_pretty(&sidecar).unwrap();
        let manifest = mantle_release_manifest(STATUS_HASH, &binary_hash);
        let report = verify_built_binary_thread(BuiltBinaryThreadInput {
            binary_bytes,
            octet_export: &export,
            sidecar: &sidecar,
            sidecar_bytes: &sidecar_json,
            mantle_manifest: &manifest,
        });

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"sidecar_digest_mismatch".to_owned()));
        assert!(report.issue_codes.contains(&"octet_parent_artifact_mismatch".to_owned()));
        assert!(report.issue_codes.contains(&"sidecar_octet_link_mismatch".to_owned()));
    }

    #[test]
    fn built_binary_thread_rejects_missing_parent_tier_promotion_and_overclaim() {
        let binary_bytes = b"valence-built-binary-fixture";
        let binary_hash = blake3_hex(binary_bytes);
        let mut export = sample_export();
        export.provenance_records[0].parent_artifacts.push("missing-parent.json".to_owned());
        export.proof_cards[0].evidence_origin = EXTERNAL_TRELLIS_ORIGIN.to_owned();
        export.proof_cards[0].evidence_tier = FORMAL_PROOF_TIER.to_owned();
        export.non_claims.push("proves semantic equivalence".to_owned());
        let sidecar = build_stack_provenance_sidecar(&export, &binary_hash);
        let sidecar_json = serde_json::to_vec_pretty(&sidecar).unwrap();
        let sidecar_digest = blake3_hex(&sidecar_json);
        let manifest = mantle_release_manifest(&sidecar_digest, &binary_hash);
        let report = verify_built_binary_thread(BuiltBinaryThreadInput {
            binary_bytes,
            octet_export: &export,
            sidecar: &sidecar,
            sidecar_bytes: &sidecar_json,
            mantle_manifest: &manifest,
        });

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"parent_artifact_missing".to_owned()));
        assert!(report.issue_codes.contains(&"manual_review_evidence_promoted".to_owned()));
        assert!(report.issue_codes.contains(&"overclaim_detected".to_owned()));
    }

    #[test]
    fn built_binary_thread_json_rejects_malformed_manifest() {
        let report = verify_built_binary_thread_json(b"binary", "not json", "not json", "not json");

        assert!(!report.valid);
        assert!(report.issue_codes.contains(&"octet_export_malformed".to_owned()));
    }

    fn sample_export() -> OctetValenceProvenanceExport {
        OctetValenceProvenanceExport {
            schema_version: OCTET_VALENCE_EXPORT_SCHEMA.to_owned(),
            artifact_dir: "target/octet-artifact".to_owned(),
            verification_status: VALID_VERIFICATION_STATUS.to_owned(),
            artifacts: vec![
                StackArtifactIdentity {
                    artifact: "status.json".to_owned(),
                    blake3: STATUS_HASH.to_owned(),
                },
                StackArtifactIdentity {
                    artifact: "results.jsonl".to_owned(),
                    blake3: RESULTS_HASH.to_owned(),
                },
            ],
            provenance_records: vec![OctetProvenanceRecord {
                source_id: "octet.standards.formal-proof".to_owned(),
                checker_id: "octet-artifact-verifier".to_owned(),
                evidence_tier: FORMAL_PROOF_TIER.to_owned(),
                artifact: "status.json".to_owned(),
                blake3: STATUS_HASH.to_owned(),
                checked_scope: "standards/proof-claims/catalog.ncl".to_owned(),
                verifier_identity: Some("octet-verus-rail".to_owned()),
                proof_artifact_identity: Some(RESULTS_HASH.to_owned()),
                human_action: None,
                parent_artifacts: vec!["results.jsonl".to_owned()],
            }],
            proof_cards: vec![OctetProofCard {
                proof_or_property_id: "octet.no-unwrap-proof".to_owned(),
                source_requirement_or_policy_id: "r[octet.policy.no_unwrap]".to_owned(),
                policy_id: "octet.standards.formal-proof".to_owned(),
                rail_index: 0,
                receipt_path: "status.json".to_owned(),
                artifact_blake3_identities: vec![StackArtifactIdentity {
                    artifact: "status.json".to_owned(),
                    blake3: STATUS_HASH.to_owned(),
                }],
                evidence_tier: FORMAL_PROOF_TIER.to_owned(),
                evidence_origin: "octet_verified_receipt".to_owned(),
                status: VALID_VERIFICATION_STATUS.to_owned(),
                non_claims: vec![STACK_PROVENANCE_NON_CLAIM.to_owned()],
                diagnostics: Vec::new(),
            }],
            diagnostics: Vec::new(),
            non_claims: vec![STACK_PROVENANCE_NON_CLAIM.to_owned()],
        }
    }

    fn mantle_manifest(digest: &str) -> String {
        serde_json::json!({
            "external_evidence": [
                {
                    "role": MANTLE_STACK_PROVENANCE_ROLE,
                    "schema": STACK_PROVENANCE_SIDECAR_SCHEMA,
                    "relative_path": "external-evidence/stack-provenance.json",
                    "digest_blake3": digest,
                    "claim_scope": STACK_PROVENANCE_CLAIM_SCOPE,
                    "non_claims": [STACK_PROVENANCE_NON_CLAIM]
                }
            ]
        })
        .to_string()
    }

    fn mantle_release_manifest(sidecar_digest: &str, binary_hash: &str) -> MantleReleaseManifestView {
        serde_json::from_value(serde_json::json!({
            "binaries": [
                {
                    "relative_path": "bin/valence-fixture",
                    "digest_blake3": binary_hash
                }
            ],
            "external_evidence": [
                {
                    "role": MANTLE_STACK_PROVENANCE_ROLE,
                    "schema": STACK_PROVENANCE_SIDECAR_SCHEMA,
                    "relative_path": "external-evidence/stack-provenance.json",
                    "digest_blake3": sidecar_digest,
                    "claim_scope": STACK_PROVENANCE_CLAIM_SCOPE,
                    "non_claims": [STACK_PROVENANCE_NON_CLAIM]
                }
            ],
            "provenance_coverage": {
                "binary_hash": binary_hash
            }
        }))
        .unwrap()
    }
}
