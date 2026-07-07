## Phase 1: Durable envelope

- [ ] [serial] r[valence.evidence_ir.typed_durable_storage] Define typed durable evidence storage envelope fields.
- [ ] [serial] r[valence.evidence_ir.typed_durable_storage.decode] Add fail-closed validation before decoding persisted evidence values.

## Phase 2: Migration and tests

- [ ] [serial] r[valence.evidence_ir.typed_durable_storage.migration] Define explicit migration receipt metadata for legacy evidence values.
- [ ] [parallel] r[valence.evidence_ir.typed_durable_storage.tests] Add positive tests for valid envelopes and migrations.
- [ ] [parallel] r[valence.evidence_ir.typed_durable_storage.fail_closed] Add negative tests for schema/type/codec/content mismatches and missing migrations.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.typed_durable_storage.docs] Document durable storage claims and non-claims.
- [ ] [serial] r[valence.evidence_ir.typed_durable_storage] Run storage/migration tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
