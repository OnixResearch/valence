## Why

The built-binary thread verifier now measures binary bytes and cross-checks Octet, Valence, and Mantle metadata, but malformed JSON and weakened metadata still reach Rust before shape errors become clear. Nickel contracts can harden the static side of this path: schema strings, digest shape, required non-claims, evidence-tier boundaries, and report shape.

This change adds typed Nickel contracts as a preflight and fixture-validation layer. Rust remains the authority for byte measurement and computed identity comparisons; Nickel rejects malformed or overbroad metadata before the pure verifier runs.

## What Changes

- Add Nickel contracts for the Octet Valence provenance export, Valence stack sidecar, Mantle release manifest view, and built-binary thread report.
- Add a bundled thread-input contract that validates the static metadata envelope used by `verify-built-binary` fixtures and automation.
- Add generated or checked-in JSON exports for Rust tests/CLI validation where needed, while keeping runtime Rust independent from Nickel execution unless an explicit preflight command is selected.
- Add positive and negative contract fixtures for valid metadata, bad schema versions, bad BLAKE3 shapes, missing required non-claims, manual-review evidence promoted to `formal_proof`, and overclaim wording.
- Document the boundary: Nickel proves metadata shape/static invariants only; Rust proves measured-byte identity and linkage.

## Impact

- **Files**: `contracts/`, `tests/fixtures/`, Valence CLI/core tests, README, and this Cairn package.
- **Testing**: Nickel contract evaluation for positive/negative fixtures, Rust workspace tests, `cargo fmt --check`, and Cairn validate/gates.
