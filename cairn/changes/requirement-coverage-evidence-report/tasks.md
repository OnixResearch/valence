## Phase 1: Coverage report model

- [ ] [serial] r[valence.evidence_ir.requirement_coverage_report] Define deterministic requirement-to-marker coverage report rows.
- [ ] [serial] r[valence.evidence_ir.requirement_coverage_report.markers] Document `r[impl ...]` and `r[verify ...]` marker expectations.
- [ ] [serial] r[valence.evidence_ir.requirement_coverage_report.shell] Keep file discovery/report writing in the shell and coverage computation in a pure core.

## Phase 2: Policy and tests

- [ ] [parallel] r[valence.evidence_ir.requirement_coverage_report.positive] Add positive coverage fixtures with implementation and verification markers.
- [ ] [parallel] r[valence.evidence_ir.requirement_coverage_report.missing] Add negative/advisory fixtures for missing implementation or verification markers.
- [ ] [parallel] r[valence.evidence_ir.requirement_coverage_report.non_claims] Document that marker coverage is traceability evidence only.

## Phase 3: Evidence

- [ ] [serial] r[valence.evidence_ir.requirement_coverage_report] Run coverage report tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
