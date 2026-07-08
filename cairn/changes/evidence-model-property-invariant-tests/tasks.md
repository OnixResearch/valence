## Phase 1: Invariant harness

- [ ] [serial] r[valence.evidence_ir.property_invariants] Add deterministic property/table harness helpers for pure evidence-model values.
- [ ] [parallel] r[valence.evidence_ir.property_invariants.rename_stability] Add invariant tests for name/path/alias churn preserving content identity.
- [ ] [parallel] r[valence.evidence_ir.property_invariants.dependency_sensitivity] Add invariant tests for canonical content and dependency changes altering identity.

## Phase 2: Fail-closed invariants

- [ ] [parallel] r[valence.evidence_ir.property_invariants.graph_determinism] Add graph ordering determinism invariants.
- [ ] [parallel] r[valence.evidence_ir.property_invariants.alias_fail_closed] Add ambiguous alias and stale binding invariants.
- [ ] [parallel] r[valence.evidence_ir.property_invariants.cache_key_equality] Add cache hit/miss/block invariants across every key field and capability class.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.property_invariants] Run invariant tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
