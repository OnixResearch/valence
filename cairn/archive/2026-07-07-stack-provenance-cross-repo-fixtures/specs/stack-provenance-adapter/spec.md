# Stack Provenance Adapter Specification

## Purpose

Extends `stack-provenance-adapter` with deterministic cross-repo fixture coverage for the bounded provenance thread.

## Requirements

### Requirement: Stack provenance cross-repo fixtures
r[valence.stack_provenance_adapter.cross_repo_fixtures] Valence MUST provide minimal checked-in fixtures that exercise the Octet export to Valence sidecar to Mantle manifest to measured binary hash provenance thread.

#### Scenario: Fast fixtures avoid live sibling repositories
r[valence.stack_provenance_adapter.cross_repo_fixtures.fast]
- GIVEN the fast stack-provenance fixture suite
- WHEN it runs in ordinary workspace tests
- THEN it MUST use frozen local fixtures and MUST NOT require live Octet, Mantle, Trellis, or Cairn repository state.

#### Scenario: Positive fixture verifies identity linkage
r[valence.stack_provenance_adapter.cross_repo_fixtures.positive]
- GIVEN matching Octet export, Valence sidecar, Mantle manifest, and measured binary byte fixtures
- WHEN Valence verifies the built-binary provenance thread
- THEN verification MUST accept the measured-byte identity and linkage.

#### Scenario: Negative fixtures fail closed
r[valence.stack_provenance_adapter.cross_repo_fixtures.negative]
- GIVEN fixtures with stale binary bytes, mismatched sidecar digest, missing Mantle role, parent artifact mismatch, tier promotion, or weakened boundaries
- WHEN Valence verifies the provenance thread
- THEN verification MUST fail closed with deterministic diagnostics.

#### Scenario: Deep sibling integration is separated
r[valence.stack_provenance_adapter.cross_repo_fixtures.deep]
- GIVEN maintainers run a deep or nightly integration suite
- WHEN live sibling repositories are used
- THEN that suite MUST be opt-in and MUST NOT be required for the fast fixture gate.

#### Scenario: Fixture claims remain bounded
r[valence.stack_provenance_adapter.cross_repo_fixtures.non_claims]
- GIVEN fixture reports describe stack-provenance success
- WHEN the supported claim is stated
- THEN the report MUST state that success proves measured-byte identity and linkage only, not correctness, release eligibility, semantic equivalence, verifier soundness, or requirement satisfaction.
