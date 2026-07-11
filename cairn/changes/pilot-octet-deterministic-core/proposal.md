## Why

Valence's `shared_identity` module is a pure deterministic BLAKE3 identity core and is the intended first downstream pilot for Octet's opt-in `deterministic-core` profile. Valence currently has no canonical Octet consumer configuration, reviewed function-address baseline, or replayable profile-bound admission evidence, so the profile cannot yet be dogfooded without ad hoc operator state.

## What Changes

- Add a non-publishable workspace harness that compiles the exact production shared-identity source under a focused Cargo target.
- Add canonical Valence workspace metadata and `dylint.toml` configuration that select that harness and declare the shared-identity implementation files as reviewed deterministic cores.
- Replace mutable issue accumulators and an infallible-assumption panic with returned pure-core outcomes.
- Bootstrap and check in a reviewed function-address baseline for the selected source files.
- Run Octet's `deterministic-core` profile, verify the emitted artifact bundle, and record stable profile/config identities plus the admitted function-address set.
- Exercise a negative stale-source case and document producer findings, limitations, replay commands, advisory debt, and explicit non-claims.

## Impact

- **Files**: workspace Cargo metadata and pilot target, shared-identity pure-core APIs/tests, root `dylint.toml`, Octet pilot evidence/baseline, operator documentation, and native Cairn lifecycle artifacts.
- **Testing**: existing Valence core tests and Clippy, Octet observe/bootstrap, deterministic-core admission, artifact verification, repeat replay, stale-source rejection, core-purity smoke, and Cairn validation/gates.
- **Compatibility**: ordinary Cargo and Rust behavior remains unchanged; strict admission is opt-in and depends on an operator-provided Octet binary.
