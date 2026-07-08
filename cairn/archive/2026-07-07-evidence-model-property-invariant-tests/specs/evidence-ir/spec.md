# Evidence IR Specification

## Purpose

Extends `evidence-ir` with deterministic property/invariant tests for evidence-model identity and fail-closed behavior.

## Requirements

### Requirement: Evidence model property invariants
r[valence.evidence_ir.property_invariants] Valence MUST test core Evidence IR model invariants with deterministic generated or table-driven cases.

#### Scenario: Rename metadata does not change content identity
r[valence.evidence_ir.property_invariants.rename_stability]
- GIVEN generated or table-driven cases that vary display name, source path, or alias metadata only
- WHEN Valence recomputes evidence-definition identity
- THEN the identity MUST remain stable.

#### Scenario: Canonical content changes alter identity
r[valence.evidence_ir.property_invariants.dependency_sensitivity]
- GIVEN generated or table-driven cases that change canonical content or dependency content identity
- WHEN Valence recomputes evidence-definition identity
- THEN the identity MUST change or the test MUST fail closed.

#### Scenario: Graph determinism survives input reordering
r[valence.evidence_ir.property_invariants.graph_determinism]
- GIVEN equivalent evidence graphs with nodes, edges, and bindings in different input orders
- WHEN Valence exports or hashes the graph
- THEN graph output and graph identity MUST remain stable.

#### Scenario: Alias ambiguity fails closed
r[valence.evidence_ir.property_invariants.alias_fail_closed]
- GIVEN an alias bound to multiple current content identities or a stale content identity
- WHEN identity-level resolution or graph validation runs
- THEN Valence MUST return deterministic fail-closed diagnostics.

#### Scenario: Cache equality checks every key field
r[valence.evidence_ir.property_invariants.cache_key_equality]
- GIVEN cached verifier results and current verifier contexts that vary one cache-key field or capability class at a time
- WHEN Valence decides cache reuse
- THEN reuse MUST hit only for fully equal replay-safe keys and MUST miss or block otherwise.

#### Scenario: Invariant claims remain bounded
r[valence.evidence_ir.property_invariants.non_claims]
- GIVEN invariant test reports describe passing identity, graph, binding, or cache checks
- WHEN the supported claim is stated
- THEN the report MUST state that invariants cover modeled identity/linkage/cache behavior only, not correctness, verifier soundness, semantic equivalence, or requirement satisfaction.
