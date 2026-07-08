## Phase 1: Fixture runner

- [ ] [serial] r[valence.evidence_ir.transcript_fixture_runner] Add pass/fail transcript fixture directories and typed fixture loading.
- [ ] [serial] r[valence.evidence_ir.transcript_fixture_runner.pure] Route fixture execution through pure transcript evaluators only.
- [ ] [serial] r[valence.evidence_ir.transcript_fixture_runner.refresh] Add an explicit refresh plan or command that does not run during normal validation.

## Phase 2: Coverage

- [ ] [parallel] r[valence.evidence_ir.transcript_fixture_runner.positive] Add positive transcript fixtures for valid Evidence IR and stack-provenance checks.
- [ ] [parallel] r[valence.evidence_ir.transcript_fixture_runner.negative] Add negative transcript fixtures for malformed input, overclaims, stale hashes, and denied capabilities.
- [ ] [parallel] r[valence.evidence_ir.transcript_fixture_runner.stability] Add deterministic report/golden checks for reordered fixture discovery.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.transcript_fixture_runner] Run transcript fixture tests, stack-provenance tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
