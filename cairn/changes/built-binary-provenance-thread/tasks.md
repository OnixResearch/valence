## Phase 1: Core verifier

- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread] Define `BuiltBinaryThreadInput`, `BuiltBinaryThreadReport`, and parsed Mantle binary identity views for pure verification.
- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.binary_digest] Implement BLAKE3 measurement over supplied binary bytes and compare it to the sidecar `binary_hash`, every provenance link `binary_hash`, and Mantle release binary identity metadata.
- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.octet_export] Cross-check the Octet Valence provenance export against the sidecar parent artifact identities, verification status, diagnostics, evidence tiers, and source/requirement/artifact links.
- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.non_claims] Preserve the identity-and-linkage-only non-claim and reject report wording or input claims that promote traceability to behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, or whole-program safety.

## Phase 2: CLI shell

- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.cli] Add `valence stack-provenance verify-built-binary --binary <PATH> --octet-export <FILE> --sidecar <FILE> --manifest <FILE>` as the thin shell for the pure verifier.
- [ ] [parallel] r[valence.stack_provenance_adapter.built_binary_thread.cli] Print deterministic diagnostics for stale binary bytes, missing Mantle binary identity, sidecar digest mismatch, Octet export mismatch, and tier-promotion failures.
- [ ] [parallel] r[valence.stack_provenance_adapter.built_binary_thread.cli] Support a machine-readable report shape for release automation without changing the bounded claim.

## Phase 3: End-to-end evidence

- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.e2e] Add a focused positive integration test that builds or materializes a local binary artifact, hashes its bytes, emits/uses an Octet Valence export, builds a Valence sidecar, records it in Mantle external-evidence metadata, and verifies the whole source/provenance/artifact/binary thread.
- [ ] [parallel] r[valence.stack_provenance_adapter.built_binary_thread.e2e] Add negative tests for changed binary bytes, mismatched sidecar binary hash, missing Mantle binary identity, stale sidecar digest, missing Octet parent artifact, and imported Trellis/manual-review evidence promoted to `formal_proof`.
- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread.non_claims] Document the operator command and explicitly state that success proves only measured-byte identity and linkage.
- [ ] [serial] r[valence.stack_provenance_adapter.built_binary_thread] Run focused validation: `cargo fmt --check`, `cargo test --workspace`, the E2E built-binary thread test, and relevant Cairn validation/gates.
