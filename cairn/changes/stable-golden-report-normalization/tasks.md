## Phase 1: Normalization core

- [ ] [serial] r[valence.evidence_ir.golden_report_normalization] Add canonical report normalization helpers for test harness outputs.
- [ ] [serial] r[valence.evidence_ir.golden_report_normalization.semantic_diff] Add semantic diff rendering for normalized report mismatches.
- [ ] [serial] r[valence.evidence_ir.golden_report_normalization.raw_stdout_boundary] Restrict raw stdout snapshots to CLI boundary tests.

## Phase 2: Tests

- [ ] [parallel] r[valence.evidence_ir.golden_report_normalization.stability] Add positive tests showing equivalent report values normalize identically despite field/discovery order changes.
- [ ] [parallel] r[valence.evidence_ir.golden_report_normalization.fail_closed] Add negative tests for unnormalizable reports and stale golden expectations.
- [ ] [parallel] r[valence.evidence_ir.golden_report_normalization.refresh] Document or implement explicit golden refresh with reviewable diffs.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.golden_report_normalization] Run golden normalization tests, CLI snapshot tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
