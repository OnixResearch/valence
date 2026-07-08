## ADDED Requirements

### Requirement: Required Kani non-claim boundaries
r[valence.kani_nonclaims.required_boundaries] Valence MUST require Kani artifacts to state non-claims for whole-program correctness, verifier soundness, aliasing models, concurrency or async scheduling, FFI and stubs, recursive termination, and bounded-only checks when applicable.

#### Scenario: Required non-claims pass
r[valence.kani_nonclaims.required_boundaries.present]
- GIVEN a Kani artifact includes all required non-claim boundaries for its role
- WHEN Valence validates the artifact
- THEN validation MUST preserve those boundaries.

#### Scenario: Missing required non-claim fails
r[valence.kani_nonclaims.required_boundaries.missing]
- GIVEN a Kani artifact omits a required non-claim boundary
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

### Requirement: Kani non-claim role binding
r[valence.kani_nonclaims.role_binding] Valence MUST bind Kani non-claim requirements to evidence roles and Kani facets so contract, bounded, verified-stub, and plain-stub evidence carry the correct boundary set.

#### Scenario: Contract boundary differs from bounded boundary
r[valence.kani_nonclaims.role_binding.contract]
- GIVEN a Kani contract artifact and a bounded harness artifact have different evidence roles
- WHEN Valence validates them
- THEN each artifact MUST carry the boundary set required for its role.

#### Scenario: Plain stub requires stub boundary
r[valence.kani_nonclaims.role_binding.stub]
- GIVEN a Kani artifact uses a plain stub
- WHEN Valence validates the artifact
- THEN the artifact MUST include a boundary saying the stub is a conditional assumption.

### Requirement: Weakened Kani boundary rejection
r[valence.kani_nonclaims.weakened_boundary_rejection] Valence MUST reject Kani artifacts that omit, weaken, or contradict required non-claim boundaries.

#### Scenario: Whole-program overclaim fails
r[valence.kani_nonclaims.weakened_boundary_rejection.whole_program]
- GIVEN a Kani artifact claims whole-program correctness
- WHEN Valence validates the artifact
- THEN validation MUST fail with an overclaim diagnostic.

#### Scenario: Bounded evidence cannot claim unbounded coverage
r[valence.kani_nonclaims.weakened_boundary_rejection.bounded]
- GIVEN a Kani bounded harness lacks passing unwind or contract evidence
- WHEN the artifact claims unbounded coverage
- THEN validation MUST fail with a bounded-overclaim diagnostic.

### Requirement: Kani non-claim positive fixtures
r[valence.kani_nonclaims.positive_fixtures] Valence MUST include positive fixtures for bounded harness, contract harness, and verified-stub evidence that carry required Kani non-claims.

#### Scenario: Positive non-claim fixtures pass
r[valence.kani_nonclaims.positive_fixtures.valid]
- GIVEN valid Kani fixtures include required non-claims
- WHEN the fixture suite runs
- THEN validation MUST accept the fixtures and preserve the boundaries.

### Requirement: Kani non-claim negative fixtures
r[valence.kani_nonclaims.negative_fixtures] Valence MUST include negative fixtures for whole-program, verifier-soundness, aliasing, concurrency, FFI, recursive-termination, and bounded-unbounded overclaims.

#### Scenario: Verifier-soundness overclaim fails
r[valence.kani_nonclaims.negative_fixtures.soundness]
- GIVEN a Kani artifact claims the Kani or solver implementation is sound
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

#### Scenario: Concurrency overclaim fails
r[valence.kani_nonclaims.negative_fixtures.concurrency]
- GIVEN a Kani artifact claims concurrency or async scheduling correctness without separate evidence
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

### Requirement: Kani non-claim report visibility
r[valence.kani_nonclaims.report_visibility] Valence reports MUST surface Kani non-claim boundaries so downstream operators can see the scoped claim and excluded claims.

#### Scenario: CLI report shows boundaries
r[valence.kani_nonclaims.report_visibility.cli]
- GIVEN a Kani artifact passes Valence validation
- WHEN the CLI renders a validation report
- THEN the report MUST include the Kani non-claim boundaries.
