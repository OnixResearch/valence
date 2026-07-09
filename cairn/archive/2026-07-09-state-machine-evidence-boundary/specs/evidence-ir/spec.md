# Evidence IR Specification Delta

## ADDED Requirements

### Requirement: State-machine exhaustiveness receipt contract
r[valence.evidence_ir.state_machine_evidence.contract] Valence MUST define a canonical receipt contract for Octet-produced state-machine exhaustiveness evidence.

#### Scenario: Valid receipt names the checked transition domain
r[valence.evidence_ir.state_machine_evidence.contract.domain]
- GIVEN an Octet state-machine exhaustiveness receipt names a transition core, source span, state type identity, event type identity, variant inventory digest, coverage summary digest, status, diagnostics, artifact identity, evidence tier, and required non-claims
- WHEN Valence validates the receipt
- THEN validation MUST preserve those identities as bounded source-shape evidence.

#### Scenario: Missing domain identity fails
r[valence.evidence_ir.state_machine_evidence.contract.missing_domain]
- GIVEN a receipt omits the transition core, state type, event type, source span, or coverage summary identity
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic missing-domain diagnostic.

### Requirement: Positive state-machine evidence fixtures
r[valence.evidence_ir.state_machine_evidence.fixtures.positive] Valence MUST include positive fixtures for valid Octet state-machine exhaustiveness receipts.

#### Scenario: Valid Octet receipt passes
r[valence.evidence_ir.state_machine_evidence.fixtures.positive.valid]
- GIVEN a fixture contains a valid Octet-produced state-machine exhaustiveness receipt with source spans, domain identities, coverage summary digest, passing status, artifact BLAKE3 identity, parent export identity, and non-claims
- WHEN the state-machine evidence fixture suite runs
- THEN the fixture MUST pass and produce deterministic Evidence IR output.

### Requirement: Negative state-machine evidence fixtures
r[valence.evidence_ir.state_machine_evidence.fixtures.negative] Valence MUST reject malformed, stale, unsupported, or overclaiming state-machine exhaustiveness receipts.

#### Scenario: Missing non-claims fail
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.non_claims]
- GIVEN a state-machine receipt omits the bounded source-shape non-claim boundary
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic non-claim diagnostic.

#### Scenario: Stale coverage summary fails
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.stale]
- GIVEN a state-machine receipt carries a malformed or stale coverage summary digest or artifact BLAKE3 identity
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic digest diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.overclaim]
- GIVEN a state-machine receipt claims behavioral correctness, semantic equivalence, verifier soundness, release eligibility, protocol completeness, liveness, or exhaustive Rust coverage
- WHEN Valence validates the receipt
- THEN validation MUST reject the receipt with a deterministic overclaim diagnostic.

### Requirement: Pure receipt validation core
r[valence.evidence_ir.state_machine_evidence.validation] Valence MUST validate state-machine exhaustiveness receipts through a pure core over parsed receipt values.

#### Scenario: Core owns validation decisions
r[valence.evidence_ir.state_machine_evidence.validation.core]
- GIVEN parsed state-machine receipt values
- WHEN validation runs
- THEN the pure core MUST decide acceptance or rejection without filesystem, process, network, clock, environment, or stdout side effects.

### Requirement: Formal proof links remain separate
r[valence.evidence_ir.state_machine_evidence.proof_links] Valence MUST preserve optional formal proof links for state-machine invariants as separate evidence records and MUST NOT promote the state-machine exhaustiveness receipt tier.

#### Scenario: Linked proof does not promote lint receipt
r[valence.evidence_ir.state_machine_evidence.proof_links.no_promotion]
- GIVEN a valid state-machine exhaustiveness receipt links to a separate Verus or Kani proof receipt
- WHEN Valence builds or verifies the evidence graph
- THEN the state-machine receipt MUST remain bounded source-shape evidence while the proof receipt retains its own verifier identity and tier.

#### Scenario: Missing linked proof fails only the link
r[valence.evidence_ir.state_machine_evidence.proof_links.missing]
- GIVEN a state-machine receipt references a proof receipt identity that is absent from the evidence graph
- WHEN Valence validates linked evidence
- THEN validation MUST report a missing-link diagnostic without treating the state-machine receipt as formal proof.

### Requirement: State-machine evidence documentation
r[valence.evidence_ir.state_machine_evidence.docs] Valence documentation MUST describe Octet producer responsibilities, Evidence IR validation boundaries, and consumer interpretation rules for state-machine exhaustiveness receipts.

#### Scenario: Consumer boundary is visible
r[valence.evidence_ir.state_machine_evidence.docs.boundary]
- GIVEN a consumer reads Valence documentation for state-machine evidence
- WHEN the supported claim is stated
- THEN the documentation MUST say Valence validates receipt shape, identity, linkage, and non-claims only, not Rust semantics or release readiness.

### Requirement: Final validation
r[valence.evidence_ir.state_machine_evidence.final_validation] The change MUST include focused core tests, positive and negative fixtures, formatting checks, and Cairn validation/gate evidence before archive.

#### Scenario: Validation covers receipt health
r[valence.evidence_ir.state_machine_evidence.final_validation.checks]
- GIVEN the receipt contract, fixtures, validation core, proof-link handling, and documentation are complete
- WHEN final validation runs
- THEN focused tests, fixture checks, formatting, and Cairn gates MUST pass or record explicit pre-existing blockers.
