## Phase 1: Contract model

- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts] Add typed Nickel contracts for Octet Valence provenance exports, Valence stack sidecars, Mantle release manifest views, and built-binary thread reports.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.static_invariants] Encode fixed schema strings, role names, claim scope, BLAKE3 digest shape, required non-claims, and allowed evidence tiers in contracts.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.thread_input] Add a bundled built-binary thread input contract that validates static metadata before Rust measures bytes.

## Phase 2: Fixtures and gates

- [x] [parallel] r[valence.stack_provenance_adapter.nickel_contracts.fixtures] Add positive contract fixtures for valid Octet export, sidecar, Mantle manifest view, built-binary thread input, and verification report metadata.
- [x] [parallel] r[valence.stack_provenance_adapter.nickel_contracts.fixtures] Add negative contract fixtures for bad schema strings, malformed BLAKE3 digests, missing required non-claims, missing sidecar role, manual-review evidence promoted to `formal_proof`, and overclaim wording.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.gate] Add a focused validation command or test target that evaluates positive and negative Nickel fixtures deterministically.

## Phase 3: Integration and documentation

- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.rust_boundary] Keep Rust as the authority for binary byte reads, BLAKE3 measurement, and cross-artifact digest comparison; do not add Nickel execution to `valence-core` runtime.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.cli] Optionally expose contract preflight through the CLI or test harness without changing the existing Rust verifier's bounded claim.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts.docs] Document that Nickel proves metadata shape/static invariants only and does not prove measured-byte identity, behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, or whole-program safety.
- [x] [serial] r[valence.stack_provenance_adapter.nickel_contracts] Run focused validation: Nickel fixture checks, `cargo fmt --check`, `cargo test --workspace`, and relevant Cairn validation/gates.
