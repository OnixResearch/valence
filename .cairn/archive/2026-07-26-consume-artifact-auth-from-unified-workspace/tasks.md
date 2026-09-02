## Phase 1: Source and behavior baseline

- [x] [serial] Record focused core, shell, and historical-source checks before migration. r[valence.artifact_auth_adoption.behavior]

## Phase 2: Receiver-local migration

- [x] [serial] Pin Cargo and Nix to the immutable unified Artifact revision and regenerate both locks with their owning tools. r[valence.artifact_auth_adoption.source] r[valence.artifact_auth_adoption.lock_agreement]
- [x] [parallel] Add pure source admission with positive and negative repository, package-graph, NAR, license, and source-byte checks. r[valence.artifact_auth_adoption.radicle_transport] r[valence.artifact_auth_adoption.fallback]
- [x] [parallel] Add typed migration evidence and source documentation with bounded rollback, historical receipt status, and explicit non-claims. r[valence.artifact_auth_adoption.radicle_evidence]

## Phase 3: Validation and closeout

- [x] [serial] Run focused Cargo, strict workspace, no-std, Nix, Cairn, and traceability checks without changing Rust implementation behavior. r[valence.artifact_auth_adoption.behavior]
- [x] [serial] Sync the accepted specification and archive only after all receiver evidence agrees. r[valence.artifact_auth_adoption.radicle_evidence]
