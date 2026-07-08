## Phase 1: Graph model

- [x] [serial] r[valence.evidence_ir.queryable_graph] Define typed graph nodes and edges for evidence identities, requirements, verifiers, receipts, binaries, sidecars, and aliases.
- [x] [serial] r[valence.evidence_ir.queryable_graph.export] Add deterministic graph export/import helpers.

## Phase 2: Queries

- [x] [parallel] r[valence.evidence_ir.queryable_graph.queries] Add pure query helpers for dependency closure, reverse dependencies, requirement coverage, binary traceability, verifier evidence, and alias resolution.
- [x] [parallel] r[valence.evidence_ir.queryable_graph.determinism] Add positive determinism fixtures for equivalent graph inputs in different orders.
- [x] [parallel] r[valence.evidence_ir.queryable_graph.fail_closed] Add negative tests for duplicate node identities, dangling edges, and ambiguous aliases.

## Phase 3: Documentation and validation

- [x] [parallel] r[valence.evidence_ir.queryable_graph.docs] Document graph claims and non-claims.
- [x] [serial] r[valence.evidence_ir.queryable_graph] Run graph tests, provenance tests, stack-provenance tests if touched, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
