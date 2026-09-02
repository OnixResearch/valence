## Phase 1: Frozen fixture set

- [x] [serial] r[valence.stack_provenance_adapter.cross_repo_fixtures] Add minimal Octet export, Valence sidecar, Mantle manifest, and binary-byte/hash fixtures.
- [x] [serial] r[valence.stack_provenance_adapter.cross_repo_fixtures.fast] Keep the fast suite independent of live sibling repositories.
- [x] [parallel] r[valence.stack_provenance_adapter.cross_repo_fixtures.positive] Add positive identity/linkage fixture tests.

## Phase 2: Negative and deep checks

- [x] [parallel] r[valence.stack_provenance_adapter.cross_repo_fixtures.negative] Add negative fixtures for stale bytes, sidecar digest mismatch, missing role, parent mismatch, tier promotion, and weakened boundaries.
- [x] [serial] r[valence.stack_provenance_adapter.cross_repo_fixtures.deep] Add or document an optional deep/nightly sibling-repo integration gate.
- [x] [parallel] r[valence.stack_provenance_adapter.cross_repo_fixtures.non_claims] Document the identity/linkage-only boundary for fixture reports.

## Phase 3: Evidence

- [x] [serial] r[valence.stack_provenance_adapter.cross_repo_fixtures] Run stack-provenance fixture tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
