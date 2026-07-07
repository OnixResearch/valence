## Phase 1: Typed boundary model

- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values] Add pure typed boundary values for stack-provenance metadata, including BLAKE3 digest wrappers, evidence tier enum, verification status enum, claim-scope type, schema/version type, and Mantle external-evidence role type.
- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values.dto_parse] Add pure parsers that convert existing JSON DTO string fields into typed internal views without changing external serialization.
- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values.digest_roles] Distinguish raw binary digests, sidecar byte digests, artifact digests, and Preserves metadata hashes by type or explicit wrapper names so validation cannot accidentally compare unrelated roles without intent.

## Phase 2: Verification and projection integration

- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values.validation] Update stack-provenance validation to use typed values for schema, role, claim scope, status, digest shape, and evidence-tier checks before comparing cross-artifact identities.
- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values.preserves] Update Preserves projection/report comparison helpers to consume typed values or typed-normalized strings where available, keeping corpus changes intentional and reviewable.
- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values.boundary] Preserve the Nickel/Preserves/Rust boundary: Nickel validates metadata shape, Preserves canonicalizes metadata values, and Rust typed views still own raw binary measurement and digest comparison.

## Phase 3: Tests and docs

- [x] [parallel] r[valence.stack_provenance_adapter.typed_boundary_values.tests] Add positive tests for valid typed parsing of Octet export, sidecar, Mantle manifest view, built-binary thread input metadata, and verification reports.
- [x] [parallel] r[valence.stack_provenance_adapter.typed_boundary_values.tests] Add negative tests for malformed BLAKE3 digests, wrong schema/version strings, invalid claim scope, missing or wrong Mantle role, invalid verification status, manual-review evidence promoted to `formal_proof`, and unknown evidence tiers treated conservatively.
- [x] [parallel] r[valence.stack_provenance_adapter.typed_boundary_values.docs] Document that typed values are internal hardening and do not change the public JSON contracts or broaden identity-and-linkage-only claims.
- [x] [serial] r[valence.stack_provenance_adapter.typed_boundary_values] Run focused validation: typed boundary tests, stack-provenance tests, Preserves corpus tests, Nickel contract gate, `cargo fmt --check`, `cargo test --workspace`, and relevant Cairn validation/gates.
