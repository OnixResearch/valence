## Phase 1: Coverage report model

- [x] [serial] r[valence.evidence_ir.requirement_coverage_report] Define deterministic requirement-to-marker coverage report rows.
- [x] [serial] r[valence.evidence_ir.requirement_coverage_report.markers] Document `r[impl ...]` and `r[verify ...]` marker expectations.
- [x] [serial] r[valence.evidence_ir.requirement_coverage_report.shell] Keep file discovery/report writing in the shell and coverage computation in a pure core.

## Phase 2: Policy and tests

- [x] [parallel] r[valence.evidence_ir.requirement_coverage_report.positive] Add positive coverage fixtures with implementation and verification markers.
- [x] [parallel] r[valence.evidence_ir.requirement_coverage_report.missing] Add negative/advisory fixtures for missing implementation or verification markers.
- [x] [parallel] r[valence.evidence_ir.requirement_coverage_report.non_claims] Document that marker coverage is traceability evidence only.

## Phase 3: Evidence

- [x] [serial] r[valence.evidence_ir.requirement_coverage_report] Run coverage report tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
