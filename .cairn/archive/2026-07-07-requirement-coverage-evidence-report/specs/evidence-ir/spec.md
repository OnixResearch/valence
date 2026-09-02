# Evidence IR Specification

## Purpose

Extends `evidence-ir` with a deterministic requirement coverage evidence report for testing harness traceability.

## Requirements

### Requirement: Requirement coverage evidence report
r[valence.evidence_ir.requirement_coverage_report] Valence MUST provide a deterministic report that maps accepted requirement IDs to implementation and verification evidence markers.

#### Scenario: Coverage rows are deterministic
r[valence.evidence_ir.requirement_coverage_report.rows]
- GIVEN accepted requirement IDs and discovered evidence markers
- WHEN Valence builds the coverage report
- THEN report rows MUST be sorted deterministically by requirement identity.

#### Scenario: Implementation and verification markers are distinct
r[valence.evidence_ir.requirement_coverage_report.markers]
- GIVEN source or test markers use `r[impl ...]` or `r[verify ...]`
- WHEN Valence classifies coverage evidence
- THEN implementation evidence and verification evidence MUST be reported separately.

#### Scenario: Coverage computation core is pure
r[valence.evidence_ir.requirement_coverage_report.shell]
- GIVEN in-memory requirement and marker sets
- WHEN coverage computation runs
- THEN it MUST produce report rows without filesystem or process I/O.

#### Scenario: Covered requirements report positive evidence
r[valence.evidence_ir.requirement_coverage_report.positive]
- GIVEN a requirement has both implementation and verification markers
- WHEN the coverage report is built
- THEN the report SHOULD mark that requirement as covered by traceability evidence.

#### Scenario: Missing coverage is reported deterministically
r[valence.evidence_ir.requirement_coverage_report.missing]
- GIVEN an accepted requirement lacks implementation or verification markers
- WHEN the coverage report is built
- THEN the missing evidence MUST be reported with deterministic diagnostics.

#### Scenario: Coverage claims remain bounded
r[valence.evidence_ir.requirement_coverage_report.non_claims]
- GIVEN coverage reports describe requirement evidence
- WHEN the supported claim is stated
- THEN the report MUST state that marker coverage proves traceability only, not correctness, verifier soundness, semantic equivalence, or requirement satisfaction.
