## Phase 1: Corpus model

- [ ] [serial] r[valence.evidence_ir.malformed_input_corpus] Add checked-in malformed-input corpus layout and fixture metadata.
- [ ] [serial] r[valence.evidence_ir.malformed_input_corpus.runner] Add deterministic corpus runner over pure validation cores where possible.
- [ ] [parallel] r[valence.evidence_ir.malformed_input_corpus.diagnostics] Normalize issue-code ordering and unexpected-success diagnostics.

## Phase 2: Fixture coverage

- [ ] [parallel] r[valence.evidence_ir.malformed_input_corpus.digest_and_graph] Add malformed digest, duplicate node, and dangling edge fixtures.
- [ ] [parallel] r[valence.evidence_ir.malformed_input_corpus.bindings_and_capabilities] Add stale binding, ambiguous alias, unknown capability, and denied capability fixtures.
- [ ] [parallel] r[valence.evidence_ir.malformed_input_corpus.boundary_and_storage] Add weakened non-claim and durable-envelope mismatch fixtures.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.malformed_input_corpus] Run malformed corpus tests, stack-provenance tests if touched, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
