## Phase 1: Canonical value model

- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values] Add pure helpers that project Octet export metadata, Valence stack sidecars, Mantle release manifest views, built-binary thread input envelopes, and verification reports into canonical Preserves values.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.hashes] Compute `preserves_content_hash` values with BLAKE3 over canonical Preserves bytes, using names that cannot be confused with raw binary or sidecar byte digests.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.boundary] Preserve the boundary that Preserves proves metadata value identity/canonicalization only and never measures executable bytes or broadens the built-binary claim.

## Phase 2: Corpus and report drift gates

- [ ] [parallel] r[valence.stack_provenance_adapter.preserves_values.corpus] Add positive Preserves corpus fixtures for valid Octet export, sidecar, Mantle manifest view, built-binary thread input, and verification report metadata.
- [ ] [parallel] r[valence.stack_provenance_adapter.preserves_values.corpus] Add negative or drift fixtures covering stale parent linkage, tier promotion, missing non-claims, overclaim wording, and report diagnostic changes.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.gate] Add a deterministic corpus freshness gate that fails when canonical Preserves values or hashes drift without an intentional fixture update.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.report_diff] Add a Preserves-backed report comparison path for built-binary thread reports so automation can compare semantic report values independent of JSON field order.

## Phase 3: Integration and documentation

- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.receipts] Include Preserves metadata hashes in release/test receipts where stack-provenance metadata identity matters, without replacing Mantle external-evidence sidecar byte digests.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values.docs] Document the Nickel/Preserves/Rust split: Nickel for contracts, Preserves for canonical metadata values, Rust for raw binary measurement and digest comparison.
- [ ] [serial] r[valence.stack_provenance_adapter.preserves_values] Run focused validation: Preserves corpus gate, Nickel contract gate, `cargo fmt --check`, `cargo test --workspace`, and relevant Cairn validation/gates.
