## Phase 1: Identity model

- [ ] [serial] r[valence.evidence_ir.content_addressed_definitions] Define typed canonical evidence-definition identity and BLAKE3 hashing rules.
- [ ] [serial] r[valence.evidence_ir.content_addressed_definitions.dependencies] Represent dependency references by content identity where available.
- [ ] [serial] r[valence.evidence_ir.content_addressed_definitions.metadata_boundary] Mark names, paths, and aliases as metadata unless an existing compatibility contract requires otherwise.

## Phase 2: Validation and tests

- [ ] [parallel] r[valence.evidence_ir.content_addressed_definitions.rename_stability] Add positive tests showing display-name and path changes do not alter content identity when canonical content is unchanged.
- [ ] [parallel] r[valence.evidence_ir.content_addressed_definitions.dependency_change] Add negative tests showing dependency-content changes alter identity.
- [ ] [parallel] r[valence.evidence_ir.content_addressed_definitions.non_claims] Add docs/tests preserving the bounded identity-only claim.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.content_addressed_definitions] Run focused Evidence IR tests, stack-provenance tests if touched, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
