# Evidence IR Specification

## Purpose

Extends `evidence-ir` with explicit fast, deep, and nightly testing harness lanes.

## Requirements

### Requirement: Fast and deep test suite partitioning
r[valence.evidence_ir.test_suite_partition] Valence MUST define explicit test suite lanes for fast deterministic checks, deeper optional integration checks, and expensive/nightly validation.

#### Scenario: Fast lane is hermetic
r[valence.evidence_ir.test_suite_partition.fast]
- GIVEN the fast test lane
- WHEN developers run ordinary local validation
- THEN the lane MUST use hermetic checked-in fixtures and MUST NOT require live sibling repositories, network access, or host-specific sandbox enforcement.

#### Scenario: Deep lane contains optional integrations
r[valence.evidence_ir.test_suite_partition.deep]
- GIVEN cross-repo stack checks or sandbox enforcement probes are requested
- WHEN the deep lane runs
- THEN those checks SHOULD run there or in the nightly lane with explicit pass, fail, skipped, or unavailable reporting.

#### Scenario: Lane reporting is stable
r[valence.evidence_ir.test_suite_partition.reporting]
- GIVEN any suite lane completes
- WHEN the harness renders its summary
- THEN the summary MUST use deterministic ordering and stable status labels for pass, fail, skipped, and unavailable checks.

#### Scenario: CI and local commands are documented
r[valence.evidence_ir.test_suite_partition.ci]
- GIVEN maintainers need to run a suite lane locally or in CI
- WHEN they inspect the harness documentation
- THEN each lane MUST have a stable command or documented invocation.

#### Scenario: Suite partition claims remain bounded
r[valence.evidence_ir.test_suite_partition.non_claims]
- GIVEN suite-lane reports describe validation evidence
- WHEN the supported claim is stated
- THEN the report MUST state that lanes schedule evidence gates only and do not prove correctness, verifier soundness, semantic equivalence, or requirement satisfaction.
