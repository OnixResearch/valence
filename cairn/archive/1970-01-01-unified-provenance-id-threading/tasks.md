## Phase 1: Implementation

- [x] [serial] r[valence.provenance] Implement `provenance.rs` module: `ProvenanceLink` (source_id, function_object_id, requirement_ids, artifact_hash, binary_hash), `ProvenanceChain`, `check_provenance_chain`, `resolve`, `resolve_requirement`, `trace_binary`, with 15 positive and negative tests.
- [x] [serial] r[valence.provenance] Register the module in `lib.rs` and run validation: `cargo test --workspace` (62 passed), `cargo clippy` (clean), `cargo fmt --check` (clean).

## Phase 2: Archive

- [x] [serial] r[valence.provenance] Verify and archive the change.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| `cargo test --workspace` passes | 62 tests: 55 core + 2 CLI unit + 5 CLI integration |
| `cargo clippy --workspace --all-targets -- -D warnings` clean | clippy finished with no warnings |
| `cargo fmt --check` clean | fmt check passes |
| Chain resolves source ID to link | `provenance::tests::chain_resolves_source_id_to_link` |
| Chain resolves unknown source to none | `provenance::tests::chain_resolves_unknown_source_id_to_none` |
| Chain resolves requirement to function objects | `provenance::tests::chain_resolves_requirement_to_function_objects` |
| Chain resolves unknown requirement to empty | `provenance::tests::chain_resolves_unknown_requirement_to_empty` |
| Chain traces binary to source functions | `provenance::tests::chain_traces_binary_to_source_functions` |
| Chain traces unknown binary to empty | `provenance::tests::chain_traces_unknown_binary_to_empty` |
| Check passes valid chain | `provenance::tests::check_passes_for_valid_chain` |
| Check fails closed schema mismatch | `provenance::tests::check_fails_closed_for_schema_mismatch` |
| Check fails closed weakened boundary | `provenance::tests::check_fails_closed_for_weakened_boundary` |
| Check fails closed empty source_id | `provenance::tests::check_fails_closed_for_empty_source_id` |
| Check fails closed empty function_object_id | `provenance::tests::check_fails_closed_for_empty_function_object_id` |
| Check fails closed duplicate source_id | `provenance::tests::check_fails_closed_for_duplicate_source_id` |
| Multi-link chain resolves correctly | `provenance::tests::chain_with_multiple_links_resolves_correctly` |
| Link without artifact/binary is valid | `provenance::tests::link_without_artifact_or_binary_hash_is_valid` |
| Chain serializes to JSON with boundary | `provenance::tests::chain_serializes_to_json_with_boundary` |
