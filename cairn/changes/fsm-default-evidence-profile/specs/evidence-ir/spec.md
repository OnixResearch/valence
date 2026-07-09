# Evidence Ir Delta

## ADDED Requirements

### Requirement: FSM default-exhaustive evidence profile
r[valence.evidence_ir.fsm_default_profile.contract] Valence MUST define an `fsm-default-exhaustive` Evidence IR profile for bounded state-machine evidence that binds closed state/event universes, transition coverage, invalid-transition denial coverage, producer identities, source spans, artifact identities, roles, and required non-claims.

#### Scenario: Valid profile preserves producer boundaries
r[valence.evidence_ir.fsm_default_profile.contract.boundaries]
- GIVEN an FSM default-exhaustive evidence record names Octet lint/provenance evidence and optionally Kamacite FSM spec evidence
- WHEN Valence validates the record
- THEN validation MUST preserve Octet as source-shape producer, Kamacite as portable schema/receipt producer when present, and Valence as identity/linkage/non-claim verifier.

#### Scenario: Closed universes are bound
r[valence.evidence_ir.fsm_default_profile.contract.universes]
- GIVEN an FSM default-exhaustive evidence record declares closed state and event universe identities
- WHEN Valence validates the record
- THEN validation MUST require state universe identity, event universe identity, variant inventory identity, transition coverage identity, and invalid-transition denial coverage identity.

### Requirement: Pure FSM evidence validation
r[valence.evidence_ir.fsm_default_profile.validation] Valence MUST validate FSM default-exhaustive evidence through pure deterministic logic over parsed in-memory values.

#### Scenario: Core has no shell side effects
r[valence.evidence_ir.fsm_default_profile.validation.pure]
- GIVEN parsed FSM default-exhaustive evidence values
- WHEN validation runs
- THEN the pure core MUST decide acceptance or rejection without filesystem, process, network, clock, environment, or stdout side effects.

### Requirement: Positive FSM default-exhaustive fixtures
r[valence.evidence_ir.fsm_default_profile.fixtures.positive] Valence MUST include positive fixtures for valid FSM default-exhaustive evidence records.

#### Scenario: Octet-only receipt passes
r[valence.evidence_ir.fsm_default_profile.fixtures.positive.octet_only]
- GIVEN a fixture contains an Octet-produced FSM default-exhaustive receipt with source spans, closed universe identities, transition coverage, denial coverage, artifact BLAKE3 identity, parent export identity, and non-claims
- WHEN the FSM default-exhaustive fixture suite runs
- THEN the fixture MUST pass and produce deterministic Evidence IR output.

#### Scenario: Octet plus Kamacite linked receipt passes
r[valence.evidence_ir.fsm_default_profile.fixtures.positive.kamacite_link]
- GIVEN a fixture contains Octet source-shape evidence linked to a Kamacite FSM spec and receipt identity with preserved non-claims
- WHEN Valence validates the linked evidence
- THEN validation MUST preserve both producer identities and report the linkage without treating Kamacite or Octet evidence as runtime correctness proof.

### Requirement: Negative FSM default-exhaustive fixtures
r[valence.evidence_ir.fsm_default_profile.fixtures.negative] Valence MUST reject malformed, stale, unsupported, or overclaiming FSM default-exhaustive evidence records.

#### Scenario: Missing denial coverage fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.denial_coverage]
- GIVEN an FSM default-exhaustive record omits invalid-transition denial coverage identity
- WHEN Valence validates the record
- THEN validation MUST fail with a deterministic missing-denial-coverage diagnostic.

#### Scenario: Stale universe or coverage identity fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.stale]
- GIVEN an FSM default-exhaustive record carries malformed or stale universe, inventory, coverage, denial coverage, artifact, or parent export identity
- WHEN Valence validates the record
- THEN validation MUST fail with deterministic digest or identity-domain diagnostics.

#### Scenario: Missing non-claims fail
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.non_claims]
- GIVEN an FSM default-exhaustive record omits the required bounded evidence non-claims
- WHEN Valence validates the record
- THEN validation MUST fail with a deterministic non-claim diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.overclaim]
- GIVEN an FSM default-exhaustive record claims behavioral correctness, protocol completeness, liveness, verifier soundness, release eligibility, or exhaustive Rust coverage
- WHEN Valence validates the record
- THEN validation MUST reject the record with a deterministic overclaim diagnostic.

### Requirement: Formal proof links remain separate
r[valence.evidence_ir.fsm_default_profile.proof_links] Valence MUST preserve optional formal proof links for FSM invariants as separate evidence records and MUST NOT promote the FSM default-exhaustive receipt tier.

#### Scenario: Linked proof does not promote FSM receipt
r[valence.evidence_ir.fsm_default_profile.proof_links.no_promotion]
- GIVEN a valid FSM default-exhaustive record links to a Verus, Kani, or Trellis proof receipt
- WHEN Valence builds or verifies the evidence graph
- THEN the FSM receipt MUST remain bounded source-shape/schema evidence while the proof receipt retains its own verifier identity and tier.

### Requirement: FSM default-exhaustive documentation
r[valence.evidence_ir.fsm_default_profile.docs] Valence documentation MUST describe Octet, Kamacite, and Valence ownership boundaries for FSM default-exhaustive evidence.

#### Scenario: Consumer boundary is visible
r[valence.evidence_ir.fsm_default_profile.docs.boundary]
- GIVEN a consumer reads Valence documentation for FSM default-exhaustive evidence
- WHEN the supported claim is stated
- THEN the documentation MUST say Valence validates identity, linkage, role, digest, coverage-record shape, and non-claims only, not Rust semantics, FSM semantics, liveness, or release readiness.

### Requirement: FSM default-exhaustive validation evidence
r[valence.evidence_ir.fsm_default_profile.final_validation] The FSM default-exhaustive profile change MUST include focused positive and negative tests plus Cairn validation evidence before archive.

#### Scenario: Focused validation covers pass and fail paths
r[valence.evidence_ir.fsm_default_profile.final_validation.focused]
- GIVEN implementation is complete
- WHEN the change is prepared for archive
- THEN saved evidence MUST identify focused core tests, positive and negative fixture coverage, and Cairn proposal/design/tasks validation results.
