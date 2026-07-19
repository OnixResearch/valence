## Phase 1: Baseline and mapping

- [x] [serial] I1 Record focused core/Radicle baselines, exact source, completion contract, approach registry, false-completion cases, and retained authority. r[valence.artifact_auth_shell.exact_verification] r[valence.artifact_auth_shell.authority]
- [x] [serial] I2 Expose pure deterministic standalone statement mapping and disjoint rejection-cause classification without consuming protocol signature validity as standalone proof. r[valence.artifact_auth_shell.exact_verification]

## Phase 2: Radicle shell pilot

- [x] [depends:valence.artifact_auth_shell.exact_verification] I3 Add protocol-gated actor-matched dedicated signing of exact standalone bytes through `radicle_crypto::Signer` and verification through pinned `artifact-auth-ed25519`. r[valence.artifact_auth_shell.exact_verification] r[valence.artifact_auth_shell.authority]
- [x] [depends:valence.artifact_auth_shell.exact_verification] I4 Emit bounded public statement/key/signature refs, lowercase signature hex, stable cryptographic failure class, and dual-run evidence without private or repository authority. r[valence.artifact_auth_shell.evidence]
- [x] [depends:valence.artifact_auth_shell.evidence] I5 Document integration, rollback, retained protocol/product authority, and continued rejection of standalone authority admission. r[valence.artifact_auth_shell.authority]

## Phase 3: Verification

- [x] [parallel] V1 Add real positive and negative tests for dedicated signature parity, entry-signature non-reuse, tampered statement/signature, wrong key, malformed length, carrier drift, unknown currentness, protocol rejection, false parity, secret exclusion, and authority non-promotion. r[valence.artifact_auth_shell.exact_verification] r[valence.artifact_auth_shell.evidence] r[valence.artifact_auth_shell.authority]
- [x] [serial] V2 Run focused and workspace tests, strict Clippy, exact-source/Nix checks, Cairn validation/gates, accepted-spec sync, and archive with bounded evidence. r[valence.artifact_auth_shell.evidence] r[valence.artifact_auth_shell.authority]
