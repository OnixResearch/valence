# Evidence IR Specification

## Purpose

Extends `evidence-ir` with stable golden report normalization for the testing harness.

## Requirements

### Requirement: Stable golden report normalization
r[valence.evidence_ir.golden_report_normalization] Valence MUST compare harness report goldens through canonical normalized values instead of incidental raw formatting.

#### Scenario: Equivalent reports normalize identically
r[valence.evidence_ir.golden_report_normalization.stability]
- GIVEN equivalent report values with different object field orders or non-semantic fixture discovery orders
- WHEN Valence normalizes them for golden comparison
- THEN the normalized values MUST be identical.

#### Scenario: Semantic diffs are reviewable
r[valence.evidence_ir.golden_report_normalization.semantic_diff]
- GIVEN a normalized report differs from its golden expectation
- WHEN the test fails
- THEN the harness SHOULD render a reviewable semantic diff rather than a noisy raw stdout diff.

#### Scenario: Raw stdout snapshots stay at CLI boundary
r[valence.evidence_ir.golden_report_normalization.raw_stdout_boundary]
- GIVEN a test does not assert user-facing CLI formatting
- WHEN it compares report output
- THEN it SHOULD use normalized semantic values rather than raw stdout snapshots.

#### Scenario: Refresh is explicit
r[valence.evidence_ir.golden_report_normalization.refresh]
- GIVEN expected report output intentionally changes
- WHEN maintainers refresh goldens
- THEN refresh MUST be explicit, reviewable, and MUST NOT run during normal validation.

#### Scenario: Normalization failures fail closed
r[valence.evidence_ir.golden_report_normalization.fail_closed]
- GIVEN a report cannot be parsed or normalized into the expected report type
- WHEN golden comparison runs
- THEN the harness MUST fail closed with deterministic normalization diagnostics.
