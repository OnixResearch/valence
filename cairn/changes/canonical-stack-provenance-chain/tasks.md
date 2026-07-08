## Phase 1: Canonical model and policy

- [ ] [serial] r[valence.stack_provenance_adapter.canonical_chain_identity] Add pure canonical identity DTOs and BLAKE3 canonical hashing for stack provenance artifacts, requirements, spans, receipts, sidecars, release bundles, and lifecycle gates.
- [ ] [serial] r[valence.stack_provenance_adapter.typed_edges] Add typed edge validation for local proof, source-shape check, provenance export, release bundle inclusion, gate satisfaction, and manual-review input links.
- [ ] [serial] r[valence.stack_provenance_adapter.non_claim_policy] Enforce non-claim policy over producer roles, verification roles, edge kinds, and evidence tiers.

## Phase 2: Reports and fixtures

- [ ] [parallel] r[valence.stack_provenance_adapter.graph_report] Emit deterministic JSON graph reports with artifacts, edges, trust boundaries, manual nodes, unresolved links, diagnostics, and report hash.
- [ ] [parallel] r[valence.stack_provenance_adapter.graph_report] Emit compact operator Markdown from the same pure report DTO.
- [ ] [parallel] r[valence.stack_provenance_adapter.broken_chain_fixtures] Add positive complete-chain fixtures.
- [ ] [parallel] r[valence.stack_provenance_adapter.broken_chain_fixtures] Add negative fixtures for missing parent, digest mismatch, stale policy, unsupported edge kind, manual-review tier promotion, weakened non-claims, and span/source content drift.

## Phase 3: CLI/docs/validation

- [ ] [serial] r[valence.stack_provenance_adapter.graph_report] Add CLI flags or commands for graph report JSON and Markdown output.
- [ ] [serial] r[valence.stack_provenance_adapter.non_claim_policy] Update docs so Mantle and Cairn consumption paths identify Valence as the stack semantics verifier.
- [ ] [serial] r[valence.stack_provenance_adapter.canonical_chain_identity] Run focused stack-provenance tests, Nickel contract checks, workspace tests, formatting, clippy, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Complete chain verifies | Positive fixture test |
| Canonical IDs are deterministic | Unit/property test |
| Known typed edges pass | Fixture test |
| Unsupported edge fails closed | Negative fixture test |
| Missing parent fails closed | Negative fixture test |
| Digest mismatch fails closed | Negative fixture test |
| Manual-review tier promotion fails closed | Negative fixture test |
| Weakened non-claim fails closed | Negative fixture test |
| Graph JSON and Markdown share the same DTO | CLI/golden test |
