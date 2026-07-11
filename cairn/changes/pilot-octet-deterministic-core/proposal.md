## Why

Valence's `shared_identity` module is a pure deterministic BLAKE3 identity core and is the intended first downstream pilot for Octet's opt-in `deterministic-core` profile. Valence currently has no canonical Octet consumer configuration, reviewed function-address baseline, or replayable profile-bound admission evidence, so the profile cannot yet be dogfooded without ad hoc operator state.

## What Changes

- Add canonical Valence workspace metadata and `dylint.toml` configuration that scope Cargo to the production `valence-core` library and declare `shared_identity.rs` as the reviewed deterministic core.
- Bootstrap and check in a reviewed function-address baseline for that source file.
- Run Octet's `deterministic-core` profile, verify the emitted artifact bundle, and record stable profile/config identities plus the admitted function-address set.
- Exercise a negative stale-source case and document findings, limitations, replay commands, and explicit non-claims.

## Impact

- **Files**: workspace Cargo metadata, root `dylint.toml`, Octet pilot evidence/baseline, operator documentation, and native Cairn lifecycle artifacts.
- **Testing**: existing Valence core tests and Clippy, Octet observe/bootstrap, deterministic-core admission, artifact verification, repeat replay, stale-source rejection, core-purity smoke, and Cairn validation/gates.
- **Compatibility**: ordinary Cargo and Rust behavior remains unchanged; strict admission is opt-in and depends on an operator-provided Octet binary.
