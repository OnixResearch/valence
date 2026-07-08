## Phase 1: Fixture runner

- [x] [serial] r[valence.evidence_ir.transcript_fixture_runner] Add pass/fail transcript fixture directories and typed fixture loading.
- [x] [serial] r[valence.evidence_ir.transcript_fixture_runner.pure] Route fixture execution through pure transcript evaluators only.
- [x] [serial] r[valence.evidence_ir.transcript_fixture_runner.refresh] Add an explicit refresh plan or command that does not run during normal validation.

## Phase 2: Coverage

- [x] [parallel] r[valence.evidence_ir.transcript_fixture_runner.positive] Add positive transcript fixtures for valid Evidence IR and stack-provenance checks.
- [x] [parallel] r[valence.evidence_ir.transcript_fixture_runner.negative] Add negative transcript fixtures for malformed input, overclaims, stale hashes, and denied capabilities.
- [x] [parallel] r[valence.evidence_ir.transcript_fixture_runner.stability] Add deterministic report/golden checks for reordered fixture discovery.

## Phase 3: Evidence

- [x] [serial] r[valence.evidence_ir.transcript_fixture_runner] Run transcript fixture tests, stack-provenance tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
