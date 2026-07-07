## Phase 1: Capability model

- [ ] [serial] r[valence.evidence_ir.verifier_capabilities] Add typed verifier capability labels and deterministic serialization.
- [ ] [serial] r[valence.evidence_ir.verifier_capabilities.receipts] Add capability sets to relevant verifier reports/receipts as additive metadata.

## Phase 2: Policy behavior

- [ ] [serial] r[valence.evidence_ir.verifier_capabilities.pure] Mark pure capability sets as pure-cache/replay eligible.
- [ ] [serial] r[valence.evidence_ir.verifier_capabilities.policy] Add policy checks for denied capabilities.
- [ ] [parallel] r[valence.evidence_ir.verifier_capabilities.tests] Add positive tests for pure/local capability sets and negative tests for unknown or denied effects.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.verifier_capabilities.docs] Document capability-label claims and non-claims.
- [ ] [serial] r[valence.evidence_ir.verifier_capabilities] Run focused capability tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
