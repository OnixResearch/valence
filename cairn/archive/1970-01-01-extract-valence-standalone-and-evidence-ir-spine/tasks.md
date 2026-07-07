## Phase 1: Implementation

- [x] [serial] r[valence.evidence_ir.canonical_form] Create standalone `valence` repo by copying `octet/crates/valence-core/` source verbatim (function_object, evidence_index, evidence_codebase_index modules) into `crates/valence-core/`, and implement `evidence_ir` module v1: `EvidenceArtifact`, `EvidenceSpan`, `VerificationRole`, `build_evidence_artifact`, `check_evidence_artifact`, `explain_span`, with positive and negative tests.
- [x] [serial] r[valence.evidence_ir.cli] Implement `valence-cli` crate: `valence check <FILE>`, `valence explain --id <ID> --manifest <FILE>`, `valence version`, with CLI round-trip integration tests covering check-valid, check-overclaim, explain-known, explain-not-found, and version.
- [x] [serial] r[valence.evidence_ir.v1_contract_preservation] Add workspace scaffolding (Cargo.toml, rust-toolchain.toml, rustfmt.toml, clippy.toml, flake.nix, .gitignore, README.md, AGENTS.md) and initialize native cairn lifecycle with `valence-default` policy.
- [x] [serial] r[valence.evidence_ir.non_claim_boundary] Run validation: `cargo test --workspace` (47 passed), `cargo clippy --workspace --all-targets -- -D warnings` (clean), `cargo fmt --check` (clean), `cairn validate --root .` (valid), `cairn policy export --check` (fresh).

## Phase 2: Archive

- [x] [serial] r[valence.evidence_ir.canonical_form] Verify and archive the change via `cairn gate` (proposal/design/tasks), `cairn sync --execute`, `cairn archive --execute`.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| `cargo test --workspace` passes | 47 tests: 40 core + 2 CLI unit + 5 CLI integration, all pass |
| `cargo clippy --workspace --all-targets -- -D warnings` clean | clippy finished with no warnings |
| `cargo fmt --check` clean | fmt check passes |
| `cairn validate --root .` valid | `valid: true`, 0 issues |
| `cairn policy export --check` fresh | `policy fresh: cairn-policy/generated/cairn-policy.json` |
| Evidence IR artifact hash stable for same inputs | `evidence_ir::tests::artifact_hash_is_stable_for_same_core_inputs` |
| Hash excludes display fields (relocation, self-hash) | `evidence_ir::tests::hash_excludes_display_fields` |
| Checker passes valid artifact | `evidence_ir::tests::check_passes_for_valid_artifact` |
| Checker fails closed malformed JSON | `evidence_ir::tests::check_fails_closed_for_malformed_json` |
| Checker fails closed schema mismatch | `evidence_ir::tests::check_fails_closed_for_schema_mismatch` |
| Checker fails closed empty stable_id | `evidence_ir::tests::check_fails_closed_for_empty_stable_id` |
| Checker fails closed hash mismatch | `evidence_ir::tests::check_fails_closed_for_hash_mismatch` |
| Checker fails closed missing non_claims | `evidence_ir::tests::check_fails_closed_for_missing_non_claims` |
| Checker fails closed weakened non_claims | `evidence_ir::tests::check_fails_closed_for_weakened_non_claims` |
| Checker rejects overclaim in payload | `evidence_ir::tests::check_rejects_overclaim_in_payload` |
| Checker rejects overclaim in non_claims | `evidence_ir::tests::check_rejects_overclaim_in_non_claims` |
| VerificationRole serializes snake_case | `evidence_ir::tests::verification_role_serializes_to_snake_case` |
| explain resolves known id | `evidence_ir::tests::explain_resolves_known_id` |
| explain returns none for unknown id | `evidence_ir::tests::explain_returns_none_for_unknown_id` |
| explain returns none for malformed manifest | `evidence_ir::tests::explain_returns_none_for_malformed_manifest` |
| CLI check accepts library-emitted artifact | `cli_round_trip::check_accepts_library_emitted_artifact` |
| CLI check rejects overclaim | `cli_round_trip::check_rejects_overclaim_artifact` |
| CLI explain resolves known id | `cli_round_trip::explain_resolves_known_id_from_manifest` |
| CLI explain reports not_found | `cli_round_trip::explain_reports_not_found_for_unknown_id` |
| CLI version succeeds | `cli_round_trip::version_command_succeeds` |
