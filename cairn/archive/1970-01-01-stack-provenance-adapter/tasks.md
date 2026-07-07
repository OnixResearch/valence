## Phase 1: Core adapter model

- [x] [serial] r[valence.stack_provenance_adapter] Define normalized adapter input structs for Octet provenance records, proof cards, Trellis proof-evidence records, Cairn receipts, and Mantle release external evidence records.
- [x] [serial] r[valence.stack_provenance_adapter] Define a canonical stack-provenance sidecar schema that wraps a `ProvenanceChain` with source artifact identities, diagnostics, claim scope, and non-claims.
- [x] [serial] r[valence.stack_provenance_adapter.validation] Implement pure validation for sidecar schema, non-claim boundary, binary hash binding, source/function/requirement IDs, parent artifact identities, and evidence-tier boundaries.

## Phase 2: CLI shell

- [x] [serial] r[valence.stack_provenance_adapter.cli] Add `valence stack-provenance build` to read Octet, Trellis, Cairn, and Mantle inputs and write the canonical sidecar.
- [x] [serial] r[valence.stack_provenance_adapter.cli] Add `valence stack-provenance verify` to locate a Mantle external-evidence sidecar by role and run the Valence semantic verifier.
- [x] [parallel] r[valence.stack_provenance_adapter.cli] Add human and JSON diagnostics that distinguish missing evidence, stale digest, manual-review-only evidence, and overclaim failures.

## Phase 3: Tests and docs

- [x] [parallel] r[valence.stack_provenance_adapter.validation] Add positive tests for a minimal valid release trace and a valid trace with Octet proof-card parent artifacts.
- [x] [parallel] r[valence.stack_provenance_adapter.validation] Add negative tests for missing Mantle role, sidecar digest mismatch, binary hash mismatch, duplicate source ID, missing parent artifact, weakened boundary, and manual-review evidence promoted to formal proof.
- [x] [serial] r[valence.stack_provenance_adapter.boundary] Document that Mantle bundles adapter sidecars opaquely and Valence owns stack-specific semantic verification.
- [x] [serial] r[valence.stack_provenance_adapter] Run focused validation: `cargo test --workspace`, `cargo fmt --check`, and the relevant Cairn validation/gates.
