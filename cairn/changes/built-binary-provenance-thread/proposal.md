## Why

We have the bounded provenance plumbing, but we have not yet proven the thread reaches the exact bytes of a built binary. Octet can export verified provenance facts, Valence can build and verify the stack sidecar, and Mantle can bundle the sidecar opaquely, but the current checks still allow an operator to provide a `binary_hash` value without measuring a real build artifact in the same verification path.

This change closes that evidence gap without broadening the claim: Valence should report a built-binary provenance thread only after it hashes the built binary bytes and matches that digest against both the Valence sidecar and the Mantle release evidence identity.

## What Changes

- Add a Valence built-binary thread verifier that reads a built binary, an Octet Valence provenance export, a Valence stack sidecar, and a Mantle release manifest or bundle manifest view.
- Bind the actual BLAKE3 digest of the binary bytes to `StackProvenanceSidecar.binary_hash` and each provenance link's `binary_hash`.
- Cross-check the same digest against Mantle release binary identity metadata before accepting the sidecar as threaded through the release artifact.
- Add an end-to-end positive fixture that builds or materializes a local binary artifact and proves source/provenance/artifact/binary linkage.
- Add negative fixtures for stale binary bytes, mismatched sidecar binary hash, missing Mantle binary identity, stale sidecar digest, and tier-promotion or correctness overclaims.

## Impact

- **Files**: `crates/valence-core/src/stack_provenance_adapter.rs`, `crates/valence-cli/src/main.rs`, CLI integration tests, README, and this Cairn spec package.
- **Testing**: focused core tests, CLI positive/negative tests, an end-to-end built-binary thread test, `cargo fmt --check`, `cargo test --workspace`, and Cairn validate/gates.
