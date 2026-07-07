## Phase 1: Graph model

- [ ] [serial] r[valence.evidence_ir.queryable_graph] Define typed graph nodes and edges for evidence identities, requirements, verifiers, receipts, binaries, sidecars, and aliases.
- [ ] [serial] r[valence.evidence_ir.queryable_graph.export] Add deterministic graph export/import helpers.

## Phase 2: Queries

- [ ] [parallel] r[valence.evidence_ir.queryable_graph.queries] Add pure query helpers for dependency closure, reverse dependencies, requirement coverage, binary traceability, verifier evidence, and alias resolution.
- [ ] [parallel] r[valence.evidence_ir.queryable_graph.determinism] Add positive determinism fixtures for equivalent graph inputs in different orders.
- [ ] [parallel] r[valence.evidence_ir.queryable_graph.fail_closed] Add negative tests for duplicate node identities, dangling edges, and ambiguous aliases.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.queryable_graph.docs] Document graph claims and non-claims.
- [ ] [serial] r[valence.evidence_ir.queryable_graph] Run graph tests, provenance tests, stack-provenance tests if touched, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
