## Phase 1: Suite lanes

- [ ] [serial] r[valence.evidence_ir.test_suite_partition] Define fast, deep, and nightly harness lanes with stable command names or documented invocations.
- [ ] [serial] r[valence.evidence_ir.test_suite_partition.fast] Keep fast tests hermetic and independent of live sibling repositories and host-specific enforcement backends.
- [ ] [serial] r[valence.evidence_ir.test_suite_partition.deep] Place optional cross-repo and sandbox enforcement checks in deep or nightly lanes.

## Phase 2: Reporting and docs

- [ ] [parallel] r[valence.evidence_ir.test_suite_partition.reporting] Add stable lane summaries with pass, fail, skipped, and unavailable states.
- [ ] [parallel] r[valence.evidence_ir.test_suite_partition.ci] Document or wire CI/local commands for each lane.
- [ ] [parallel] r[valence.evidence_ir.test_suite_partition.non_claims] Document that suite partitioning schedules evidence gates and does not broaden verifier claims.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.test_suite_partition] Run fast lane tests, documented deep lane smoke where available, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
