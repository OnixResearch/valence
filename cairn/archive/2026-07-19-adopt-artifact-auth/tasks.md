## Phase 1: Consumer baseline and source admission

- [x] [serial] I1 Capture current Valence Radicle verification, actor/evidence identities, parent/related refs, issue ordering, retained authority, and positive/negative fixtures; compare them with the immutable published Valence mapping profile. r[valence.artifact_auth_adoption.source] r[valence.artifact_auth_adoption.authority]
- [x] [serial] I2 Pin Cargo and Nix to `ssh://git@github.com/OnixResearch/artifact-auth.git` revision `799459346d5416fbd7b9f55840a7371441b55afa`, generate locks only with owning tools, and reject floating, duplicate, mismatched, sibling-path, product-dependent, or license-incompatible sources. r[valence.artifact_auth_adoption.source]

## Phase 2: Adapter and cutover

- [x] [depends:valence.artifact_auth_adoption.source] I3 Add a pure one-way adapter from already protocol-verified actor key, entry subject, parent OIDs, related target, and profile context while keeping Radicle verification, actor authorization, Evidence IR, COB/Git/SSH effects, lifecycle, and release interpretation in Valence. r[valence.artifact_auth_adoption.authority]
- [x] [depends:valence.artifact_auth_adoption.authority] I4 Dual-run legacy and standalone projections over identical positive and tamper observations, classify every difference, reject unrelated-failure false parity, and retain legacy authority plus rollback until admission. r[valence.artifact_auth_adoption.cutover]
- [x] [depends:valence.artifact_auth_adoption.cutover] I5 Admit or reject cutover from durable compatibility evidence, update migration/rollback documentation, and remove duplicate canonical ownership only after the bounded rollback period. r[valence.artifact_auth_adoption.cutover]

## Phase 3: Verification

- [x] [parallel] V1 Add positive tests for protocol-verified mapping and bounded non-claims plus negative tests for wrong actor/entry/parent/related/profile refs, malformed or unverified protocol input, authorization promotion, Evidence IR role/identity loss, duplicate keys, unsafe diagnostics, and weakened non-claims. r[valence.artifact_auth_adoption.authority] r[valence.artifact_auth_adoption.cutover]
- [x] [serial] V2 Run focused adapter/protocol tests, exact-source checks, workspace tests, strict Clippy, Nix checks, Cairn validation/gates, accepted-spec sync, and archive with exact bounded evidence. r[valence.artifact_auth_adoption.source] r[valence.artifact_auth_adoption.cutover]
