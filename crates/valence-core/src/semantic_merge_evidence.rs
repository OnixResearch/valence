//! Typed evidence-chain profile and consumer applicability checks for
//! four-version semantic merge checks (base O, variants A/B, candidate M).
//!
//! This profile plugs into [`crate::evidence_chain`] under
//! `evidence_kind = "semantic-merge"`. It reuses generic canonical envelope
//! identity, profile/policy roots, typed parent links, projection drift checks,
//! and graph roots, adding only the narrowly required four-version subject,
//! provider-native provenance, claim/observable/assumption, six-state outcome,
//! artifact-role separation, and consumer applicability surface.
//!
//! Valence performs no merge synthesis, source mutation, or verifier execution.

use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::evidence_chain::EvidenceChainDigest;
use crate::evidence_chain::EvidenceChainEnvelope;
use crate::evidence_chain::EvidenceChainGraphRow;
use crate::evidence_chain::validate_evidence_chain;
use crate::evidence_ir::VerificationRole;
use crate::shared_identity::blake3_hex;
use crate::shared_identity::canonical_json_bytes;

pub const SEMANTIC_MERGE_PROFILE_VERSION: &str = "valence.semantic-merge-evidence.v1";
pub const SEMANTIC_MERGE_PROJECTION_SCHEMA: &str = "valence.semantic-merge-projection.v1";
pub const SEMANTIC_MERGE_ROLE_BINDING_DOMAIN: &str = "semantic-merge-role-binding.v1";

pub const DOMAIN_SOURCE_TREE: &str = "source-tree";
pub const DOMAIN_SEMANTIC_SUBJECT: &str = "semantic-subject";
pub const DOMAIN_CORRESPONDENCE: &str = "correspondence-evidence";
pub const DOMAIN_PIJUL_CHANNEL_STATE: &str = "pijul-channel-state";
pub const DOMAIN_PIJUL_CHANGE_HASH: &str = "pijul-change-hash";
pub const DOMAIN_PIJUL_CHANGE_SET: &str = "pijul-change-set";
pub const DOMAIN_VCS_NATIVE_REVISION: &str = "vcs-native-revision";
pub const DOMAIN_PROPERTY_SPEC: &str = "property-spec";
pub const DOMAIN_OBSERVABLE_DOMAIN: &str = "observable-domain";
pub const DOMAIN_SEMANTIC_MODEL: &str = "semantic-model";
pub const DOMAIN_DEPENDENCY_CLOSURE: &str = "dependency-closure";
pub const DOMAIN_SUMMARY_IDENTITY: &str = "summary-identity";
pub const DOMAIN_ASSUMPTION: &str = "assumption";
pub const DOMAIN_CHECKER_TOOLCHAIN: &str = "checker-toolchain";
pub const DOMAIN_EXTRACTION_TRANSCRIPT: &str = "extraction-transcript";
pub const DOMAIN_TEST_RESULT: &str = "test-result";
pub const DOMAIN_SOLVER_OUTPUT: &str = "solver-output";
pub const DOMAIN_CHECKED_PROOF: &str = "checked-proof";
pub const DOMAIN_COUNTEREXAMPLE_WITNESS: &str = "counterexample-witness";

pub const SEMANTIC_MERGE_REQUIRED_NON_CLAIMS: [&str; 3] = [
    "semantic-merge evidence binds four-version identity, linkage, role, outcome, and applicability only",
    "does not prove evidence truth, checker soundness, or semantic correctness beyond the producer's checked relation",
    "grants no authority to synthesize merges, mutate source, or authorize release",
];

const SEMANTIC_MERGE_SUPPORTED_CLAIM: &str = "semantic-merge evidence validation proves four-version canonical identity, digest-domain separation, typed linkage, outcome/role separation, projection conformance, and consumer applicability only; it does not prove evidence truth, checker soundness, semantic correctness beyond the producer's checked relation, source mutation authority, or release eligibility";

const OVERCLAIM_MARKERS: &[&str] = &[
    "proves universal correctness",
    "proves checker soundness",
    "proves semantic equivalence for all programs",
    "grants merge authority",
    "grants source mutation authority",
    "grants release eligibility",
    "proves developer intent",
];

const BLAKE3_HEX_LENGTH: usize = 64;

/// Four explicit subject roles for a three-way semantic merge check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticMergeRole {
    Base,
    VariantA,
    VariantB,
    Candidate,
}

impl SemanticMergeRole {
    pub const ALL: [Self; 4] = [Self::Base, Self::VariantA, Self::VariantB, Self::Candidate];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::VariantA => "variant_a",
            Self::VariantB => "variant_b",
            Self::Candidate => "candidate",
        }
    }
}

/// Provider-native provenance kept strictly separate from measured source-tree
/// and language semantic-subject identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeProvenanceBinding {
    pub provider: String,
    pub channel_state: Option<EvidenceChainDigest>,
    pub change_set: Option<EvidenceChainDigest>,
    pub change_hashes: Vec<EvidenceChainDigest>,
    pub native_revision: Option<EvidenceChainDigest>,
}

/// Role-tagged binding for one of O, A, B, or M.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeSubjectBinding {
    pub role: SemanticMergeRole,
    pub source_tree: EvidenceChainDigest,
    pub semantic_subject: EvidenceChainDigest,
    pub correspondence: EvidenceChainDigest,
    pub native_provenance: Option<NativeProvenanceBinding>,
}

/// Declared observable, precondition/domain identity, and admitted-input count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservableSelection {
    pub logical_id: String,
    pub domain_and_precondition: EvidenceChainDigest,
    pub admitted_inputs: u64,
    pub covered: bool,
}

/// Optional shared-fragment summary with checked premises and dependency binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedSummaryBinding {
    pub logical_id: String,
    pub summary_identity: EvidenceChainDigest,
    pub premises: Vec<EvidenceChainDigest>,
    pub dependency_identity: EvidenceChainDigest,
}

/// Exact producer-selected claim, model, dependency, assumption, and toolchain bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProducerClaimSpec {
    pub language: String,
    pub producer_profile: String,
    pub producer_profile_version: String,
    pub property_id: String,
    pub property_spec: EvidenceChainDigest,
    pub semantic_model: EvidenceChainDigest,
    pub model_version: String,
    pub dependency_closure: EvidenceChainDigest,
    pub observables: Vec<ObservableSelection>,
    pub summaries: Vec<SharedSummaryBinding>,
    pub assumptions: Vec<EvidenceChainDigest>,
    pub checker_toolchain: EvidenceChainDigest,
    pub checker_id: String,
    pub checker_version: String,
    pub solver_id: Option<String>,
    pub configuration_digest: EvidenceChainDigest,
}

/// Explicit artifact role preventing extraction transcripts, tests, or raw
/// solver output from being conflated with independently checked proof artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticMergeArtifactRole {
    ExtractionTranscript,
    TestResult,
    SolverOutput,
    IndependentlyCheckedProof,
    ExplicitAssumption,
}

impl SemanticMergeArtifactRole {
    #[must_use]
    pub const fn expected_domain(self) -> &'static str {
        match self {
            Self::ExtractionTranscript => DOMAIN_EXTRACTION_TRANSCRIPT,
            Self::TestResult => DOMAIN_TEST_RESULT,
            Self::SolverOutput => DOMAIN_SOLVER_OUTPUT,
            Self::IndependentlyCheckedProof => DOMAIN_CHECKED_PROOF,
            Self::ExplicitAssumption => DOMAIN_ASSUMPTION,
        }
    }

    #[must_use]
    pub const fn edge_kind(self) -> &'static str {
        match self {
            Self::ExtractionTranscript => "recorded-extraction-transcript",
            Self::TestResult => "recorded-test-result",
            Self::SolverOutput => "recorded-solver-output",
            Self::IndependentlyCheckedProof => "checked-proof-artifact",
            Self::ExplicitAssumption => "explicit-assumption-artifact",
        }
    }
}

/// Typed evidence artifact attached to a semantic-merge result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeArtifact {
    pub role: SemanticMergeArtifactRole,
    pub digest: EvidenceChainDigest,
    pub verification_role: VerificationRole,
}

/// Six-state semantic-merge outcome lattice. Never collapsed into a Boolean;
/// well-formed envelopes may validly record any nonverified outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SemanticMergeOutcome {
    Verified {
        checked_observables: usize,
        checked_inputs: u64,
    },
    ValidatedCounterexample {
        observable: String,
        witness: EvidenceChainDigest,
    },
    Unknown {
        reason: String,
    },
    Unsupported {
        reason: String,
    },
    TimeoutExhausted {
        limit_kind: String,
        limit_value: u64,
    },
    ExecutionError {
        backend: String,
        diagnostic: String,
    },
}

impl SemanticMergeOutcome {
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }
}

/// Producer-reported observable coverage and claim scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeCoverage {
    pub covered_observables: Vec<String>,
    pub missing_observables: Vec<String>,
    pub scope_label: String,
}

/// Payload validated under `evidence_kind = "semantic-merge"`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeEvidence {
    pub profile_version: String,
    pub subjects: Vec<SemanticMergeSubjectBinding>,
    pub claim: ProducerClaimSpec,
    pub outcome: SemanticMergeOutcome,
    pub verification_role: VerificationRole,
    pub artifacts: Vec<SemanticMergeArtifact>,
    pub coverage: SemanticMergeCoverage,
    pub non_claims: Vec<String>,
}

/// Deterministic compatibility projection preserving identities, role tags,
/// outcomes, and claim scope through serialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeProjection {
    pub schema: String,
    pub profile_version: String,
    pub role_bindings: Vec<RoleBindingSummary>,
    pub language: String,
    pub producer_profile: String,
    pub property_id: String,
    pub property_spec: EvidenceChainDigest,
    pub semantic_model: EvidenceChainDigest,
    pub dependency_closure: EvidenceChainDigest,
    pub outcome: SemanticMergeOutcome,
    pub verification_role: VerificationRole,
    pub scope_label: String,
    pub covered_observables: Vec<String>,
    pub missing_observables: Vec<String>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleBindingSummary {
    pub role: SemanticMergeRole,
    pub role_binding_digest: String,
    pub source_tree: EvidenceChainDigest,
    pub semantic_subject: EvidenceChainDigest,
    pub pijul_channel_state: Option<EvidenceChainDigest>,
}

/// Consumer policy mode for semantic-merge applicability checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectationMode {
    RequiredVerifiedProperty,
    Advisory,
}

/// Consumer-expected binding for one subject role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedRoleSubject {
    pub role: SemanticMergeRole,
    pub source_tree: EvidenceChainDigest,
    pub semantic_subject: EvidenceChainDigest,
    pub correspondence: EvidenceChainDigest,
    pub pijul_channel_state: Option<EvidenceChainDigest>,
}

/// Consumer-supplied expectation (e.g. Kiln CI gate context) required when
/// checking whether a validated semantic-merge envelope is applicable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticMergeExpectation {
    pub mode: ExpectationMode,
    pub expected_profile_version: String,
    pub expected_producer_profile: String,
    pub expected_language: String,
    pub expected_property_id: String,
    pub expected_property_spec: EvidenceChainDigest,
    pub expected_semantic_model: EvidenceChainDigest,
    pub expected_dependency_closure: EvidenceChainDigest,
    pub expected_checker_toolchain: EvidenceChainDigest,
    pub expected_assumptions: Vec<EvidenceChainDigest>,
    pub expected_required_observables: Vec<String>,
    pub expected_subjects: Vec<ExpectedRoleSubject>,
    pub expected_proof_digest: Option<EvidenceChainDigest>,
    pub expected_policy_identity: EvidenceChainDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct SemanticMergeIssue {
    pub field: &'static str,
    pub code: &'static str,
}

impl SemanticMergeIssue {
    #[must_use]
    pub const fn new(field: &'static str, code: &'static str) -> Self {
        Self { field, code }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticMergeReport {
    pub valid: bool,
    pub issues: Vec<SemanticMergeIssue>,
    pub outcome: SemanticMergeOutcome,
    pub verification_role: VerificationRole,
    pub role_bindings: Vec<RoleBindingSummary>,
    pub graph_rows: Vec<EvidenceChainGraphRow>,
    pub supported_claim: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SemanticMergeApplicabilityReport {
    pub applicable: bool,
    pub satisfies_required_property: bool,
    pub issues: Vec<SemanticMergeIssue>,
    pub outcome: Option<SemanticMergeOutcome>,
    pub verification_role: Option<VerificationRole>,
    pub supported_claim: &'static str,
}

/// Compute a deterministic role-tagged BLAKE3 digest for one subject binding so
/// identical source/subject digests in distinct roles (`A == O`, `A == B`)
/// remain distinct and non-interchangeable.
#[must_use]
pub fn role_binding_digest(subject: &SemanticMergeSubjectBinding) -> String {
    let payload = serde_json::json!({
        "domain": SEMANTIC_MERGE_ROLE_BINDING_DOMAIN,
        "role": subject.role.as_str(),
        "source_tree": normalize_digest_pair(&subject.source_tree),
        "semantic_subject": normalize_digest_pair(&subject.semantic_subject),
        "correspondence": normalize_digest_pair(&subject.correspondence),
        "native_provenance": subject.native_provenance.as_ref().map(|prov| serde_json::json!({
            "provider": prov.provider,
            "channel_state": prov.channel_state.as_ref().map(normalize_digest_pair),
            "change_set": prov.change_set.as_ref().map(normalize_digest_pair),
            "change_hashes": prov.change_hashes.iter().map(normalize_digest_pair).collect::<Vec<_>>(),
            "native_revision": prov.native_revision.as_ref().map(normalize_digest_pair),
        })),
    });
    let bytes = canonical_json_bytes(&payload).expect("semantic merge role binding serializes canonically");
    blake3_hex(&bytes)
}

/// Build the deterministic compatibility projection for a [`SemanticMergeEvidence`]
/// payload, preserving all four role-tagged identities, outcomes, and scope.
#[must_use]
pub fn project_semantic_merge(input: &SemanticMergeEvidence) -> SemanticMergeProjection {
    let role_bindings = build_role_summaries(&input.subjects);
    SemanticMergeProjection {
        schema: SEMANTIC_MERGE_PROJECTION_SCHEMA.to_owned(),
        profile_version: input.profile_version.clone(),
        role_bindings,
        language: input.claim.language.clone(),
        producer_profile: input.claim.producer_profile.clone(),
        property_id: input.claim.property_id.clone(),
        property_spec: input.claim.property_spec.clone(),
        semantic_model: input.claim.semantic_model.clone(),
        dependency_closure: input.claim.dependency_closure.clone(),
        outcome: input.outcome.clone(),
        verification_role: input.verification_role,
        scope_label: input.coverage.scope_label.clone(),
        covered_observables: input.coverage.covered_observables.clone(),
        missing_observables: input.coverage.missing_observables.clone(),
        non_claims: input.non_claims.clone(),
    }
}

/// Validate a four-version semantic-merge evidence payload.
///
/// r[impl valence.evidence_ir.semantic_merge_evidence_profile.registry]
/// r[impl valence.evidence_ir.semantic_merge_evidence_profile.subjects_and_domains]
/// r[impl valence.evidence_ir.semantic_merge_evidence_profile.outcomes_and_roles]
#[must_use]
pub fn validate_semantic_merge_evidence(input: &SemanticMergeEvidence) -> SemanticMergeReport {
    let mut issues = Vec::new();
    if input.profile_version != SEMANTIC_MERGE_PROFILE_VERSION {
        issues.push(SemanticMergeIssue::new("profile_version", "unsupported_profile_version"));
    }
    validate_subjects(&input.subjects, &mut issues);
    validate_claim(&input.claim, &mut issues);
    validate_artifacts(&input.artifacts, &mut issues);
    validate_outcome_and_roles(input, &mut issues);
    validate_coverage(input, &mut issues);
    validate_non_claims(&input.non_claims, &mut issues);
    issues.sort();
    issues.dedup();
    let valid = issues.is_empty();
    SemanticMergeReport {
        valid,
        issues,
        outcome: input.outcome.clone(),
        verification_role: input.verification_role,
        role_bindings: if valid {
            build_role_summaries(&input.subjects)
        } else {
            Vec::new()
        },
        graph_rows: if valid { build_graph_rows(input) } else { Vec::new() },
        supported_claim: SEMANTIC_MERGE_SUPPORTED_CLAIM,
    }
}

/// Check consumer-supplied applicability of a validated semantic-merge
/// [`EvidenceChainEnvelope`].
///
/// Rejects stale, swapped, omitted, duplicated-role, changed-proof, or
/// wrong-context bindings. Nonverified outcomes and recorded-only evidence
/// NEVER satisfy [`ExpectationMode::RequiredVerifiedProperty`].
///
/// r[impl valence.evidence_ir.semantic_merge_evidence_profile.applicability]
#[must_use]
pub fn check_semantic_merge_applicability(
    envelope: &EvidenceChainEnvelope,
    expectation: &SemanticMergeExpectation,
) -> SemanticMergeApplicabilityReport {
    let mut issues = Vec::new();
    let chain = validate_evidence_chain(envelope);
    if !chain.valid {
        issues.push(SemanticMergeIssue::new("envelope", "envelope_validation_failed"));
    }
    if envelope.evidence_kind != crate::evidence_chain::SEMANTIC_MERGE_CHAIN_KIND {
        issues.push(SemanticMergeIssue::new("evidence_kind", "unexpected_evidence_kind"));
    }
    if !digest_eq(&envelope.policy_identity, &expectation.expected_policy_identity) {
        issues.push(SemanticMergeIssue::new("policy_identity", "consumer_policy_mismatch"));
    }
    let Ok(payload) = serde_json::from_value::<SemanticMergeEvidence>(envelope.payload.clone()) else {
        issues.push(SemanticMergeIssue::new("payload", "profile_payload_malformed"));
        issues.sort();
        issues.dedup();
        return SemanticMergeApplicabilityReport {
            applicable: false,
            satisfies_required_property: false,
            issues,
            outcome: None,
            verification_role: None,
            supported_claim: SEMANTIC_MERGE_SUPPORTED_CLAIM,
        };
    };
    if payload.profile_version != expectation.expected_profile_version {
        issues.push(SemanticMergeIssue::new("profile_version", "expected_profile_mismatch"));
    }
    check_expected_subjects(&payload.subjects, &expectation.expected_subjects, &mut issues);
    check_expected_claim(&payload.claim, expectation, &mut issues);
    check_expected_proof(&payload.artifacts, expectation.expected_proof_digest.as_ref(), &mut issues);

    let has_checked_proof = payload.artifacts.iter().any(|artifact| {
        artifact.role == SemanticMergeArtifactRole::IndependentlyCheckedProof
            && artifact.verification_role == VerificationRole::Property
    });
    let verified_property =
        payload.outcome.is_verified() && payload.verification_role == VerificationRole::Property && has_checked_proof;

    if expectation.mode == ExpectationMode::RequiredVerifiedProperty && !verified_property {
        issues.push(SemanticMergeIssue::new("outcome", "required_verified_property_unsatisfied"));
    }

    issues.sort();
    issues.dedup();
    let applicable = issues.is_empty();
    SemanticMergeApplicabilityReport {
        applicable,
        satisfies_required_property: applicable && verified_property,
        issues,
        outcome: Some(payload.outcome),
        verification_role: Some(payload.verification_role),
        supported_claim: SEMANTIC_MERGE_SUPPORTED_CLAIM,
    }
}

fn validate_subjects(subjects: &[SemanticMergeSubjectBinding], issues: &mut Vec<SemanticMergeIssue>) {
    let mut seen = BTreeSet::new();
    for subject in subjects {
        if !seen.insert(subject.role) {
            issues.push(SemanticMergeIssue::new("subjects", "duplicate_subject_role"));
        }
        check_digest(&subject.source_tree, "subjects.source_tree", DOMAIN_SOURCE_TREE, issues);
        check_digest(&subject.semantic_subject, "subjects.semantic_subject", DOMAIN_SEMANTIC_SUBJECT, issues);
        check_digest(&subject.correspondence, "subjects.correspondence", DOMAIN_CORRESPONDENCE, issues);
        if let Some(prov) = &subject.native_provenance {
            validate_native_provenance(prov, issues);
        }
    }
    for required in SemanticMergeRole::ALL {
        if !seen.contains(&required) {
            issues.push(SemanticMergeIssue::new("subjects", "missing_subject_role"));
        }
    }
}

fn validate_native_provenance(prov: &NativeProvenanceBinding, issues: &mut Vec<SemanticMergeIssue>) {
    if prov.provider.trim().is_empty() {
        issues.push(SemanticMergeIssue::new("native_provenance", "provider_empty"));
    }
    if prov.provider == "pijul" && prov.channel_state.is_none() {
        issues.push(SemanticMergeIssue::new("native_provenance.channel_state", "pijul_channel_state_required"));
    }
    if let Some(state) = &prov.channel_state {
        check_digest(state, "native_provenance.channel_state", DOMAIN_PIJUL_CHANNEL_STATE, issues);
    }
    if let Some(change_set) = &prov.change_set {
        check_digest(change_set, "native_provenance.change_set", DOMAIN_PIJUL_CHANGE_SET, issues);
    }
    for change in &prov.change_hashes {
        check_digest(change, "native_provenance.change_hashes", DOMAIN_PIJUL_CHANGE_HASH, issues);
    }
    if let Some(rev) = &prov.native_revision {
        check_digest(rev, "native_provenance.native_revision", DOMAIN_VCS_NATIVE_REVISION, issues);
    }
}

fn validate_claim(claim: &ProducerClaimSpec, issues: &mut Vec<SemanticMergeIssue>) {
    if claim.language.trim().is_empty()
        || claim.producer_profile.trim().is_empty()
        || claim.producer_profile_version.trim().is_empty()
        || claim.property_id.trim().is_empty()
        || claim.model_version.trim().is_empty()
        || claim.checker_id.trim().is_empty()
        || claim.checker_version.trim().is_empty()
    {
        issues.push(SemanticMergeIssue::new("claim", "claim_metadata_incomplete"));
    }
    check_digest(&claim.property_spec, "claim.property_spec", DOMAIN_PROPERTY_SPEC, issues);
    check_digest(&claim.semantic_model, "claim.semantic_model", DOMAIN_SEMANTIC_MODEL, issues);
    check_digest(&claim.dependency_closure, "claim.dependency_closure", DOMAIN_DEPENDENCY_CLOSURE, issues);
    check_digest(&claim.checker_toolchain, "claim.checker_toolchain", DOMAIN_CHECKER_TOOLCHAIN, issues);
    check_digest(&claim.configuration_digest, "claim.configuration_digest", DOMAIN_CHECKER_TOOLCHAIN, issues);
    if claim.observables.is_empty() {
        issues.push(SemanticMergeIssue::new("claim.observables", "observables_missing"));
    }
    let mut seen_obs = BTreeSet::new();
    for obs in &claim.observables {
        if obs.logical_id.trim().is_empty() {
            issues.push(SemanticMergeIssue::new("claim.observables", "observable_id_empty"));
        }
        if !seen_obs.insert(obs.logical_id.as_str()) {
            issues.push(SemanticMergeIssue::new("claim.observables", "duplicate_observable"));
        }
        check_digest(
            &obs.domain_and_precondition,
            "claim.observables.domain_and_precondition",
            DOMAIN_OBSERVABLE_DOMAIN,
            issues,
        );
    }
    for summary in &claim.summaries {
        if summary.logical_id.trim().is_empty() {
            issues.push(SemanticMergeIssue::new("claim.summaries", "summary_id_empty"));
        }
        check_digest(&summary.summary_identity, "claim.summaries.summary_identity", DOMAIN_SUMMARY_IDENTITY, issues);
        check_digest(
            &summary.dependency_identity,
            "claim.summaries.dependency_identity",
            DOMAIN_DEPENDENCY_CLOSURE,
            issues,
        );
        if summary.premises.is_empty() {
            issues.push(SemanticMergeIssue::new("claim.summaries.premises", "summary_premises_missing"));
        }
        for premise in &summary.premises {
            check_digest(premise, "claim.summaries.premises", DOMAIN_ASSUMPTION, issues);
        }
    }
    if claim.assumptions.is_empty() {
        issues.push(SemanticMergeIssue::new("claim.assumptions", "assumptions_missing"));
    }
    for assumption in &claim.assumptions {
        check_digest(assumption, "claim.assumptions", DOMAIN_ASSUMPTION, issues);
    }
}

fn validate_artifacts(artifacts: &[SemanticMergeArtifact], issues: &mut Vec<SemanticMergeIssue>) {
    if artifacts.is_empty() {
        issues.push(SemanticMergeIssue::new("artifacts", "artifacts_missing"));
    }
    for artifact in artifacts {
        check_digest(&artifact.digest, "artifacts.digest", artifact.role.expected_domain(), issues);
        match artifact.role {
            SemanticMergeArtifactRole::ExtractionTranscript
            | SemanticMergeArtifactRole::TestResult
            | SemanticMergeArtifactRole::SolverOutput
            | SemanticMergeArtifactRole::ExplicitAssumption => {
                if artifact.verification_role == VerificationRole::Property {
                    issues.push(SemanticMergeIssue::new(
                        "artifacts.verification_role",
                        "recorded_to_property_promotion_denied",
                    ));
                }
            }
            SemanticMergeArtifactRole::IndependentlyCheckedProof => {}
        }
    }
}

fn validate_outcome_and_roles(input: &SemanticMergeEvidence, issues: &mut Vec<SemanticMergeIssue>) {
    match &input.outcome {
        SemanticMergeOutcome::Verified {
            checked_observables,
            checked_inputs,
        } => {
            if *checked_observables != input.claim.observables.len() || *checked_inputs == 0 {
                issues.push(SemanticMergeIssue::new("outcome", "verified_coverage_incomplete"));
            }
            if input.claim.observables.iter().any(|obs| !obs.covered || obs.admitted_inputs == 0) {
                issues.push(SemanticMergeIssue::new("outcome", "vacuous_verified_denied"));
            }
            if input.verification_role == VerificationRole::Property {
                let has_checked_proof = input.artifacts.iter().any(|artifact| {
                    artifact.role == SemanticMergeArtifactRole::IndependentlyCheckedProof
                        && artifact.verification_role == VerificationRole::Property
                });
                if !has_checked_proof {
                    issues.push(SemanticMergeIssue::new("verification_role", "recorded_to_property_promotion_denied"));
                }
            }
        }
        SemanticMergeOutcome::ValidatedCounterexample { observable, witness } => {
            if observable.trim().is_empty() {
                issues.push(SemanticMergeIssue::new("outcome", "counterexample_observable_empty"));
            }
            check_digest(witness, "outcome.witness", DOMAIN_COUNTEREXAMPLE_WITNESS, issues);
            deny_nonverified_property_role(input.verification_role, issues);
        }
        SemanticMergeOutcome::Unknown { reason } | SemanticMergeOutcome::Unsupported { reason } => {
            if reason.trim().is_empty() {
                issues.push(SemanticMergeIssue::new("outcome", "outcome_reason_empty"));
            }
            deny_nonverified_property_role(input.verification_role, issues);
        }
        SemanticMergeOutcome::TimeoutExhausted {
            limit_kind,
            limit_value,
        } => {
            if limit_kind.trim().is_empty() || *limit_value == 0 {
                issues.push(SemanticMergeIssue::new("outcome", "timeout_bounds_invalid"));
            }
            deny_nonverified_property_role(input.verification_role, issues);
        }
        SemanticMergeOutcome::ExecutionError { backend, diagnostic } => {
            if backend.trim().is_empty() || diagnostic.trim().is_empty() {
                issues.push(SemanticMergeIssue::new("outcome", "execution_error_incomplete"));
            }
            deny_nonverified_property_role(input.verification_role, issues);
        }
    }
}

fn deny_nonverified_property_role(role: VerificationRole, issues: &mut Vec<SemanticMergeIssue>) {
    if role == VerificationRole::Property {
        issues.push(SemanticMergeIssue::new("verification_role", "nonverified_property_role_denied"));
    }
}

fn validate_coverage(input: &SemanticMergeEvidence, issues: &mut Vec<SemanticMergeIssue>) {
    if input.coverage.scope_label.trim().is_empty() {
        issues.push(SemanticMergeIssue::new("coverage.scope_label", "scope_label_empty"));
    }
    if input.outcome.is_verified() && !input.coverage.missing_observables.is_empty() {
        issues.push(SemanticMergeIssue::new("coverage.missing_observables", "verified_with_missing_coverage"));
    }
    let declared: BTreeSet<&str> = input.claim.observables.iter().map(|o| o.logical_id.as_str()).collect();
    if input.coverage.covered_observables.iter().any(|id| !declared.contains(id.as_str())) {
        issues.push(SemanticMergeIssue::new("coverage.covered_observables", "undeclared_observable_in_coverage"));
    }
    if input.outcome.is_verified() {
        let covered: BTreeSet<&str> = input.coverage.covered_observables.iter().map(String::as_str).collect();
        if declared != covered {
            issues.push(SemanticMergeIssue::new("coverage.covered_observables", "verified_coverage_mismatch"));
        }
    }
}

fn validate_non_claims(non_claims: &[String], issues: &mut Vec<SemanticMergeIssue>) {
    for required in SEMANTIC_MERGE_REQUIRED_NON_CLAIMS {
        if !non_claims.iter().any(|claim| claim == required) {
            issues.push(SemanticMergeIssue::new("non_claims", "non_claims_weakened"));
        }
    }
    let has_overclaim = non_claims.iter().any(|claim| {
        let lower = claim.to_ascii_lowercase();
        OVERCLAIM_MARKERS.iter().any(|marker| lower.contains(marker))
    });
    if has_overclaim {
        issues.push(SemanticMergeIssue::new("non_claims", "overclaim_denied"));
    }
}

fn check_expected_subjects(
    actual: &[SemanticMergeSubjectBinding],
    expected: &[ExpectedRoleSubject],
    issues: &mut Vec<SemanticMergeIssue>,
) {
    let mut seen_expected = BTreeSet::new();
    for exp in expected {
        if !seen_expected.insert(exp.role) {
            issues.push(SemanticMergeIssue::new("expected_subjects", "duplicate_expected_role"));
            continue;
        }
        let Some(found) = actual.iter().find(|subject| subject.role == exp.role) else {
            issues.push(SemanticMergeIssue::new("subjects", "expected_subject_missing"));
            continue;
        };
        if !digest_eq(&found.source_tree, &exp.source_tree) {
            issues.push(SemanticMergeIssue::new("subjects.source_tree", "expected_source_tree_mismatch"));
        }
        if !digest_eq(&found.semantic_subject, &exp.semantic_subject) {
            issues.push(SemanticMergeIssue::new("subjects.semantic_subject", "expected_semantic_subject_mismatch"));
        }
        if !digest_eq(&found.correspondence, &exp.correspondence) {
            issues.push(SemanticMergeIssue::new("subjects.correspondence", "expected_correspondence_mismatch"));
        }
        if let Some(expected_state) = &exp.pijul_channel_state {
            let actual_state = found.native_provenance.as_ref().and_then(|prov| prov.channel_state.as_ref());
            if actual_state.is_none_or(|state| !digest_eq(state, expected_state)) {
                issues.push(SemanticMergeIssue::new(
                    "subjects.native_provenance",
                    "expected_pijul_channel_state_mismatch",
                ));
            }
        }
    }
    for required in SemanticMergeRole::ALL {
        if !seen_expected.contains(&required) {
            issues.push(SemanticMergeIssue::new("expected_subjects", "missing_expected_role"));
        }
    }
}

fn check_expected_claim(
    claim: &ProducerClaimSpec,
    expectation: &SemanticMergeExpectation,
    issues: &mut Vec<SemanticMergeIssue>,
) {
    if claim.producer_profile != expectation.expected_producer_profile {
        issues.push(SemanticMergeIssue::new("claim.producer_profile", "expected_producer_profile_mismatch"));
    }
    if claim.language != expectation.expected_language {
        issues.push(SemanticMergeIssue::new("claim.language", "expected_language_mismatch"));
    }
    if claim.property_id != expectation.expected_property_id
        || !digest_eq(&claim.property_spec, &expectation.expected_property_spec)
    {
        issues.push(SemanticMergeIssue::new("claim.property_spec", "expected_property_mismatch"));
    }
    if !digest_eq(&claim.semantic_model, &expectation.expected_semantic_model) {
        issues.push(SemanticMergeIssue::new("claim.semantic_model", "expected_model_mismatch"));
    }
    if !digest_eq(&claim.dependency_closure, &expectation.expected_dependency_closure) {
        issues.push(SemanticMergeIssue::new("claim.dependency_closure", "expected_dependency_closure_mismatch"));
    }
    if !digest_eq(&claim.checker_toolchain, &expectation.expected_checker_toolchain) {
        issues.push(SemanticMergeIssue::new("claim.checker_toolchain", "expected_checker_toolchain_mismatch"));
    }
    for req_assumption in &expectation.expected_assumptions {
        if !claim.assumptions.iter().any(|actual| digest_eq(actual, req_assumption)) {
            issues.push(SemanticMergeIssue::new("claim.assumptions", "expected_assumption_missing"));
        }
    }
    for req_obs in &expectation.expected_required_observables {
        if !claim.observables.iter().any(|obs| &obs.logical_id == req_obs) {
            issues.push(SemanticMergeIssue::new("claim.observables", "expected_observable_missing"));
        }
    }
}

fn check_expected_proof(
    artifacts: &[SemanticMergeArtifact],
    expected_proof: Option<&EvidenceChainDigest>,
    issues: &mut Vec<SemanticMergeIssue>,
) {
    let Some(expected) = expected_proof else {
        return;
    };
    let matched = artifacts.iter().any(|artifact| {
        artifact.role == SemanticMergeArtifactRole::IndependentlyCheckedProof && digest_eq(&artifact.digest, expected)
    });
    if !matched {
        issues.push(SemanticMergeIssue::new("artifacts", "expected_proof_digest_mismatch"));
    }
}

fn build_role_summaries(subjects: &[SemanticMergeSubjectBinding]) -> Vec<RoleBindingSummary> {
    let mut rows: Vec<RoleBindingSummary> = subjects
        .iter()
        .map(|subject| RoleBindingSummary {
            role: subject.role,
            role_binding_digest: role_binding_digest(subject),
            source_tree: subject.source_tree.clone(),
            semantic_subject: subject.semantic_subject.clone(),
            pijul_channel_state: subject.native_provenance.as_ref().and_then(|prov| prov.channel_state.clone()),
        })
        .collect();
    rows.sort_by_key(|row| row.role);
    rows
}

fn build_graph_rows(input: &SemanticMergeEvidence) -> Vec<EvidenceChainGraphRow> {
    let mut rows = Vec::new();
    for subject in &input.subjects {
        let edge_subject = format!("role-{}-semantic-subject", subject.role.as_str());
        let edge_corr = format!("role-{}-correspondence", subject.role.as_str());
        rows.push(graph_row(&subject.source_tree, &subject.semantic_subject, &edge_subject));
        rows.push(graph_row(&subject.semantic_subject, &subject.correspondence, &edge_corr));
        if let Some(prov) = &subject.native_provenance
            && let Some(state) = &prov.channel_state
        {
            let edge_prov = format!("role-{}-pijul-channel-state", subject.role.as_str());
            rows.push(graph_row(state, &subject.source_tree, &edge_prov));
        }
    }
    rows.push(graph_row(&input.claim.property_spec, &input.claim.semantic_model, "checked-under-model"));
    rows.push(graph_row(&input.claim.property_spec, &input.claim.dependency_closure, "bound-to-dependencies"));
    rows.push(graph_row(&input.claim.property_spec, &input.claim.checker_toolchain, "checked-by-toolchain"));
    for assumption in &input.claim.assumptions {
        rows.push(graph_row(&input.claim.property_spec, assumption, "assumes"));
    }
    for artifact in &input.artifacts {
        rows.push(graph_row(&input.claim.property_spec, &artifact.digest, artifact.role.edge_kind()));
    }
    rows.sort_by(|left, right| {
        (&left.from_domain, &left.from_digest, &left.to_domain, &left.to_digest, &left.edge_kind).cmp(&(
            &right.from_domain,
            &right.from_digest,
            &right.to_domain,
            &right.to_digest,
            &right.edge_kind,
        ))
    });
    rows.dedup();
    rows
}

fn graph_row(from: &EvidenceChainDigest, to: &EvidenceChainDigest, edge_kind: &str) -> EvidenceChainGraphRow {
    EvidenceChainGraphRow {
        from_domain: from.domain.clone(),
        from_digest: normalize_hex(&from.digest),
        to_domain: to.domain.clone(),
        to_digest: normalize_hex(&to.digest),
        edge_kind: edge_kind.to_owned(),
    }
}

fn check_digest(
    value: &EvidenceChainDigest,
    field: &'static str,
    expected_domain: &str,
    issues: &mut Vec<SemanticMergeIssue>,
) {
    if value.domain != expected_domain {
        issues.push(SemanticMergeIssue::new(field, "digest_domain_substitution"));
    }
    let normalized = normalize_hex(&value.digest);
    if normalized.len() != BLAKE3_HEX_LENGTH || !normalized.chars().all(|c| c.is_ascii_hexdigit()) {
        issues.push(SemanticMergeIssue::new(field, "malformed_blake3"));
    }
}

fn digest_eq(left: &EvidenceChainDigest, right: &EvidenceChainDigest) -> bool {
    left.domain == right.domain && normalize_hex(&left.digest) == normalize_hex(&right.digest)
}

fn normalize_hex(value: &str) -> String {
    value.strip_prefix("b3:").unwrap_or(value).to_ascii_lowercase()
}

fn normalize_digest_pair(value: &EvidenceChainDigest) -> serde_json::Value {
    serde_json::json!({
        "domain": value.domain,
        "digest": normalize_hex(&value.digest),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_chain::EVIDENCE_CHAIN_NON_CLAIM;
    use crate::evidence_chain::EVIDENCE_CHAIN_PROFILE_VERSION;
    use crate::evidence_chain::EvidenceChainLink;
    use crate::evidence_chain::SEMANTIC_MERGE_CHAIN_KIND;

    fn d(domain: &str, ch: char) -> EvidenceChainDigest {
        EvidenceChainDigest {
            domain: domain.to_owned(),
            digest: std::iter::repeat_n(ch, 64).collect(),
        }
    }

    fn subject(role: SemanticMergeRole, tag: char) -> SemanticMergeSubjectBinding {
        SemanticMergeSubjectBinding {
            role,
            source_tree: d(DOMAIN_SOURCE_TREE, tag),
            semantic_subject: d(DOMAIN_SEMANTIC_SUBJECT, tag),
            correspondence: d(DOMAIN_CORRESPONDENCE, tag),
            native_provenance: Some(NativeProvenanceBinding {
                provider: "pijul".to_owned(),
                channel_state: Some(d(DOMAIN_PIJUL_CHANNEL_STATE, tag)),
                change_set: Some(d(DOMAIN_PIJUL_CHANGE_SET, tag)),
                change_hashes: vec![d(DOMAIN_PIJUL_CHANGE_HASH, tag)],
                native_revision: None,
            }),
        }
    }

    fn sample_evidence() -> SemanticMergeEvidence {
        SemanticMergeEvidence {
            profile_version: SEMANTIC_MERGE_PROFILE_VERSION.to_owned(),
            subjects: vec![
                subject(SemanticMergeRole::Base, '1'),
                subject(SemanticMergeRole::VariantA, '2'),
                subject(SemanticMergeRole::VariantB, '3'),
                subject(SemanticMergeRole::Candidate, '4'),
            ],
            claim: ProducerClaimSpec {
                language: "rust".to_owned(),
                producer_profile: "octet-merge-rail-bounded-v1".to_owned(),
                producer_profile_version: "1".to_owned(),
                property_id: "bounded-four-version-observable-preservation".to_owned(),
                property_spec: d(DOMAIN_PROPERTY_SPEC, 'a'),
                semantic_model: d(DOMAIN_SEMANTIC_MODEL, 'b'),
                model_version: "pure-scalar-observable-model/1".to_owned(),
                dependency_closure: d(DOMAIN_DEPENDENCY_CLOSURE, 'c'),
                observables: vec![ObservableSelection {
                    logical_id: "compute_score".to_owned(),
                    domain_and_precondition: d(DOMAIN_OBSERVABLE_DOMAIN, 'd'),
                    admitted_inputs: 16,
                    covered: true,
                }],
                summaries: Vec::new(),
                assumptions: vec![d(DOMAIN_ASSUMPTION, 'e')],
                checker_toolchain: d(DOMAIN_CHECKER_TOOLCHAIN, 'f'),
                checker_id: "octet-merge-rail-lean".to_owned(),
                checker_version: "1".to_owned(),
                solver_id: Some("lean-kernel-4".to_owned()),
                configuration_digest: d(DOMAIN_CHECKER_TOOLCHAIN, '9'),
            },
            outcome: SemanticMergeOutcome::Verified {
                checked_observables: 1,
                checked_inputs: 16,
            },
            verification_role: VerificationRole::Property,
            artifacts: vec![
                SemanticMergeArtifact {
                    role: SemanticMergeArtifactRole::ExtractionTranscript,
                    digest: d(DOMAIN_EXTRACTION_TRANSCRIPT, '5'),
                    verification_role: VerificationRole::RecordedOnly,
                },
                SemanticMergeArtifact {
                    role: SemanticMergeArtifactRole::IndependentlyCheckedProof,
                    digest: d(DOMAIN_CHECKED_PROOF, '6'),
                    verification_role: VerificationRole::Property,
                },
            ],
            coverage: SemanticMergeCoverage {
                covered_observables: vec!["compute_score".to_owned()],
                missing_observables: Vec::new(),
                scope_label: "bounded-four-version-observable-preservation".to_owned(),
            },
            non_claims: SEMANTIC_MERGE_REQUIRED_NON_CLAIMS.iter().map(|s| (*s).to_owned()).collect(),
        }
    }

    fn wrap_envelope(evidence: &SemanticMergeEvidence) -> EvidenceChainEnvelope {
        let payload = serde_json::to_value(evidence).expect("serialize payload");
        let canonical_bytes = canonical_json_bytes(&payload).expect("canonical bytes");
        let projection = serde_json::to_value(project_semantic_merge(evidence)).expect("projection");
        let projection_bytes = canonical_json_bytes(&projection).expect("projection bytes");
        EvidenceChainEnvelope {
            profile_version: EVIDENCE_CHAIN_PROFILE_VERSION.to_owned(),
            evidence_kind: SEMANTIC_MERGE_CHAIN_KIND.to_owned(),
            envelope_identity: EvidenceChainDigest {
                domain: "kamacite-envelope".to_owned(),
                digest: blake3_hex(&canonical_bytes),
            },
            canonical_bytes,
            profile_root: d("profile-root", '7'),
            policy_identity: d("policy-profile", '8'),
            parent_links: vec![EvidenceChainLink {
                from: d(DOMAIN_SOURCE_TREE, '4'),
                to: d(DOMAIN_CHECKED_PROOF, '6'),
                edge_kind: "checked-candidate".to_owned(),
            }],
            verification_role: evidence.verification_role,
            compatibility_projection: Some(projection),
            projection_identity: Some(EvidenceChainDigest {
                domain: "compatibility-projection".to_owned(),
                digest: blake3_hex(&projection_bytes),
            }),
            payload,
            non_claims: vec![EVIDENCE_CHAIN_NON_CLAIM.to_owned()],
        }
    }

    fn expectation_for(evidence: &SemanticMergeEvidence, mode: ExpectationMode) -> SemanticMergeExpectation {
        SemanticMergeExpectation {
            mode,
            expected_profile_version: SEMANTIC_MERGE_PROFILE_VERSION.to_owned(),
            expected_producer_profile: evidence.claim.producer_profile.clone(),
            expected_language: evidence.claim.language.clone(),
            expected_property_id: evidence.claim.property_id.clone(),
            expected_property_spec: evidence.claim.property_spec.clone(),
            expected_semantic_model: evidence.claim.semantic_model.clone(),
            expected_dependency_closure: evidence.claim.dependency_closure.clone(),
            expected_checker_toolchain: evidence.claim.checker_toolchain.clone(),
            expected_assumptions: evidence.claim.assumptions.clone(),
            expected_required_observables: vec!["compute_score".to_owned()],
            expected_subjects: evidence
                .subjects
                .iter()
                .map(|s| ExpectedRoleSubject {
                    role: s.role,
                    source_tree: s.source_tree.clone(),
                    semantic_subject: s.semantic_subject.clone(),
                    correspondence: s.correspondence.clone(),
                    pijul_channel_state: s.native_provenance.as_ref().and_then(|p| p.channel_state.clone()),
                })
                .collect(),
            expected_proof_digest: Some(d(DOMAIN_CHECKED_PROOF, '6')),
            expected_policy_identity: d("policy-profile", '8'),
        }
    }

    #[test]
    fn positive_four_version_and_identical_digests_with_distinct_roles_pass() {
        // r[verify valence.evidence_ir.semantic_merge_evidence_profile.subjects_and_domains]
        let ev = sample_evidence();
        let env = wrap_envelope(&ev);
        let chain_report = validate_evidence_chain(&env);
        assert!(chain_report.valid, "issues: {:?}", chain_report.issues);
        let exp = expectation_for(&ev, ExpectationMode::RequiredVerifiedProperty);
        let app = check_semantic_merge_applicability(&env, &exp);
        assert!(app.applicable, "issues: {:?}", app.issues);
        assert!(app.satisfies_required_property);

        // Identical digests across roles (A == O, and B == A) remain valid with distinct role bindings.
        let mut equal_roles = ev;
        equal_roles.subjects[1] = subject(SemanticMergeRole::VariantA, '1');
        equal_roles.subjects[2] = subject(SemanticMergeRole::VariantB, '1');
        let report = validate_semantic_merge_evidence(&equal_roles);
        assert!(report.valid, "issues: {:?}", report.issues);
        let base_binding = &report.role_bindings[0].role_binding_digest;
        let a_binding = &report.role_bindings[1].role_binding_digest;
        let b_binding = &report.role_bindings[2].role_binding_digest;
        assert_ne!(base_binding, a_binding);
        assert_ne!(a_binding, b_binding);
    }

    #[test]
    fn negative_fixtures_reject_all_required_failure_modes() {
        // r[verify valence.evidence_ir.semantic_merge_evidence_profile.negative]
        let base_ev = sample_evidence();
        let base_exp = expectation_for(&base_ev, ExpectationMode::RequiredVerifiedProperty);

        // 1. Role substitution (swap A and B) fails consumer applicability.
        let mut swapped = base_ev.clone();
        swapped.subjects[1] = subject(SemanticMergeRole::VariantA, '3');
        swapped.subjects[2] = subject(SemanticMergeRole::VariantB, '2');
        let swapped_app = check_semantic_merge_applicability(&wrap_envelope(&swapped), &base_exp);
        assert!(!swapped_app.applicable);
        assert!(swapped_app.issues.iter().any(|i| i.code == "expected_source_tree_mismatch"));

        // 2. Duplicate role and missing role fail validation.
        let mut dup_role = base_ev.clone();
        dup_role.subjects[2].role = SemanticMergeRole::VariantA;
        let dup_report = validate_semantic_merge_evidence(&dup_role);
        assert!(!dup_report.valid);
        assert!(dup_report.issues.iter().any(|i| i.code == "duplicate_subject_role"));
        assert!(dup_report.issues.iter().any(|i| i.code == "missing_subject_role"));

        // 3. Missing observables & missing assumptions fail closed.
        let mut missing = base_ev.clone();
        missing.claim.observables.clear();
        missing.claim.assumptions.clear();
        let missing_report = validate_semantic_merge_evidence(&missing);
        assert!(!missing_report.valid);
        assert!(missing_report.issues.iter().any(|i| i.code == "observables_missing"));
        assert!(missing_report.issues.iter().any(|i| i.code == "assumptions_missing"));

        // 4. Wrong candidate & wrong model fail applicability.
        let mut wrong_cm = base_ev.clone();
        wrong_cm.subjects[3] = subject(SemanticMergeRole::Candidate, '0');
        wrong_cm.claim.semantic_model = d(DOMAIN_SEMANTIC_MODEL, '0');
        let wrong_cm_app = check_semantic_merge_applicability(&wrap_envelope(&wrong_cm), &base_exp);
        assert!(!wrong_cm_app.applicable);
        assert!(wrong_cm_app.issues.iter().any(|i| i.code == "expected_model_mismatch"));

        // 5. Changed proof bytes fail applicability.
        let mut changed_proof = base_ev.clone();
        changed_proof.artifacts[1].digest = d(DOMAIN_CHECKED_PROOF, '0');
        let proof_app = check_semantic_merge_applicability(&wrap_envelope(&changed_proof), &base_exp);
        assert!(!proof_app.applicable);
        assert!(proof_app.issues.iter().any(|i| i.code == "expected_proof_digest_mismatch"));

        // 6. Stale consumer expectation (policy/dependency changed) fails applicability.
        let mut stale_exp = base_exp.clone();
        stale_exp.expected_policy_identity = d("policy-profile", '0');
        stale_exp.expected_dependency_closure = d(DOMAIN_DEPENDENCY_CLOSURE, '0');
        let stale_app = check_semantic_merge_applicability(&wrap_envelope(&base_ev), &stale_exp);
        assert!(!stale_app.applicable);
        assert!(stale_app.issues.iter().any(|i| i.code == "consumer_policy_mismatch"));
        assert!(stale_app.issues.iter().any(|i| i.code == "expected_dependency_closure_mismatch"));

        // 7. Unsupported version & dropped non-claims fail closed.
        let mut bad_meta = base_ev.clone();
        bad_meta.profile_version = "valence.semantic-merge-evidence.v99".to_owned();
        bad_meta.non_claims.clear();
        let bad_meta_report = validate_semantic_merge_evidence(&bad_meta);
        assert!(!bad_meta_report.valid);
        assert!(bad_meta_report.issues.iter().any(|i| i.code == "unsupported_profile_version"));
        assert!(bad_meta_report.issues.iter().any(|i| i.code == "non_claims_weakened"));

        // 8. Recorded-to-property promotion fails closed (both artifact-level and envelope-level).
        let mut promoted = base_ev.clone();
        promoted.artifacts = vec![SemanticMergeArtifact {
            role: SemanticMergeArtifactRole::SolverOutput,
            digest: d(DOMAIN_SOLVER_OUTPUT, '5'),
            verification_role: VerificationRole::RecordedOnly,
        }];
        let promoted_report = validate_semantic_merge_evidence(&promoted);
        assert!(!promoted_report.valid);
        assert!(promoted_report.issues.iter().any(|i| i.code == "recorded_to_property_promotion_denied"));

        // 9. Pijul change hash cannot substitute for Pijul channel state or source tree.
        let mut pijul_sub = base_ev;
        pijul_sub.subjects[0].native_provenance = Some(NativeProvenanceBinding {
            provider: "pijul".to_owned(),
            channel_state: Some(d(DOMAIN_PIJUL_CHANGE_HASH, '1')),
            change_set: None,
            change_hashes: vec![d(DOMAIN_PIJUL_CHANGE_HASH, '1')],
            native_revision: None,
        });
        let pijul_report = validate_semantic_merge_evidence(&pijul_sub);
        assert!(!pijul_report.valid);
        assert!(pijul_report.issues.iter().any(|i| i.code == "digest_domain_substitution"));
    }

    #[test]
    fn round_trip_projection_and_nonverified_outcomes_never_satisfy_required_property() {
        // r[verify valence.evidence_ir.semantic_merge_evidence_profile.outcomes_and_roles]
        // r[verify valence.evidence_ir.semantic_merge_evidence_profile.applicability]
        let base_ev = sample_evidence();
        let nonverified_outcomes = vec![
            SemanticMergeOutcome::ValidatedCounterexample {
                observable: "compute_score".to_owned(),
                witness: d(DOMAIN_COUNTEREXAMPLE_WITNESS, 'a'),
            },
            SemanticMergeOutcome::Unknown {
                reason: "summary premise unverified".to_owned(),
            },
            SemanticMergeOutcome::Unsupported {
                reason: "unsafe block outside bounded subset".to_owned(),
            },
            SemanticMergeOutcome::TimeoutExhausted {
                limit_kind: "evaluations".to_owned(),
                limit_value: 1000,
            },
            SemanticMergeOutcome::ExecutionError {
                backend: "lean-kernel-4".to_owned(),
                diagnostic: "nonzero exit".to_owned(),
            },
        ];

        for outcome in nonverified_outcomes {
            let mut ev = base_ev.clone();
            ev.outcome = outcome.clone();
            ev.verification_role = VerificationRole::RecordedOnly;
            ev.artifacts = vec![SemanticMergeArtifact {
                role: SemanticMergeArtifactRole::SolverOutput,
                digest: d(DOMAIN_SOLVER_OUTPUT, '5'),
                verification_role: VerificationRole::RecordedOnly,
            }];

            // Well-formed nonverified envelope passes structural validation & JSON round-trip.
            let env = wrap_envelope(&ev);
            let encoded = serde_json::to_string(&env).expect("encode json");
            let decoded: EvidenceChainEnvelope = serde_json::from_str(&encoded).expect("decode json");
            let chain_report = validate_evidence_chain(&decoded);
            assert!(chain_report.valid, "issues: {:?}", chain_report.issues);

            // Projection preserves exact outcome, role, and scope.
            let proj: SemanticMergeProjection =
                serde_json::from_value(decoded.compatibility_projection.clone().unwrap()).expect("projection");
            assert_eq!(proj.outcome, outcome);
            assert_eq!(proj.verification_role, VerificationRole::RecordedOnly);
            assert_eq!(proj.scope_label, ev.coverage.scope_label);
            assert_eq!(proj.role_bindings.len(), 4);

            // Advisory mode accepts a matching nonverified result while keeping satisfies_required_property =
            // false.
            let mut advisory_exp = expectation_for(&ev, ExpectationMode::Advisory);
            advisory_exp.expected_proof_digest = None;
            let advisory_app = check_semantic_merge_applicability(&decoded, &advisory_exp);
            assert!(advisory_app.applicable, "issues: {:?}", advisory_app.issues);
            assert!(!advisory_app.satisfies_required_property);

            // RequiredVerifiedProperty NEVER accepts any nonverified outcome!
            let mut req_exp = advisory_exp;
            req_exp.mode = ExpectationMode::RequiredVerifiedProperty;
            let req_app = check_semantic_merge_applicability(&decoded, &req_exp);
            assert!(!req_app.applicable);
            assert!(!req_app.satisfies_required_property);
            assert!(req_app.issues.iter().any(|i| i.code == "required_verified_property_unsatisfied"));
        }
    }

    #[test]
    fn frozen_end_to_end_octet_kiln_bounded_integration_fixture_validates() {
        // r[verify valence.evidence_ir.semantic_merge_evidence_profile.integration_fixture]
        let raw = include_str!("../../../fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json");
        let doc: serde_json::Value = serde_json::from_str(raw).expect("valid fixture json");
        assert_eq!(doc["classification"], "bounded_integration_evidence");
        assert_eq!(doc["producer_receipt"]["schema"], "octet-merge-rail-receipt/v1");

        let envelope: EvidenceChainEnvelope =
            serde_json::from_value(doc["valence_envelope"].clone()).expect("valence envelope");
        let expectation: SemanticMergeExpectation =
            serde_json::from_value(doc["kiln_expectation"].clone()).expect("kiln expectation");

        let chain_report = validate_evidence_chain(&envelope);
        assert!(chain_report.valid, "chain issues: {:?}", chain_report.issues);
        assert!(chain_report.graph_root.is_some());

        let app_report = check_semantic_merge_applicability(&envelope, &expectation);
        assert!(app_report.applicable, "applicability issues: {:?}", app_report.issues);
        assert!(app_report.satisfies_required_property);

        // Mutating consumer expectation (stale candidate) rejects the same frozen envelope.
        let mut stale = expectation;
        stale.expected_subjects[3].source_tree.digest = "0".repeat(64);
        let stale_report = check_semantic_merge_applicability(&envelope, &stale);
        assert!(!stale_report.applicable);
        assert!(!stale_report.satisfies_required_property);
    }
}
