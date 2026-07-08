## Phase 1: Invariant harness

- [x] [serial] r[valence.evidence_ir.property_invariants] Add deterministic property/table harness helpers for pure evidence-model values.
- [x] [parallel] r[valence.evidence_ir.property_invariants.rename_stability] Add invariant tests for name/path/alias churn preserving content identity.
- [x] [parallel] r[valence.evidence_ir.property_invariants.dependency_sensitivity] Add invariant tests for canonical content and dependency changes altering identity.

## Phase 2: Fail-closed invariants

- [x] [parallel] r[valence.evidence_ir.property_invariants.graph_determinism] Add graph ordering determinism invariants.
- [x] [parallel] r[valence.evidence_ir.property_invariants.alias_fail_closed] Add ambiguous alias and stale binding invariants.
- [x] [parallel] r[valence.evidence_ir.property_invariants.cache_key_equality] Add cache hit/miss/block invariants across every key field and capability class.

## Phase 3: Evidence

- [x] [serial] r[valence.evidence_ir.property_invariants] Run invariant tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
