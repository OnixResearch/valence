# Evidence IR Specification

## Purpose

Extends `evidence-ir` with a first-class executable transcript fixture runner.

## Requirements

### Requirement: Executable transcript fixture runner
r[valence.evidence_ir.transcript_fixture_runner] Valence MUST provide a test harness that discovers checked-in executable transcript fixtures, evaluates them deterministically, and reports stable pass/fail diagnostics.

#### Scenario: Transcript fixtures load from pass and fail roots
r[valence.evidence_ir.transcript_fixture_runner.fixtures]
- GIVEN pass and fail transcript fixture directories
- WHEN the transcript fixture runner starts
- THEN it MUST load typed transcript fixtures with schema identity, scenario identity, inputs, verifier action, expected report fields, expected issue codes, expected hashes, and non-claims.

#### Scenario: Transcript execution stays pure
r[valence.evidence_ir.transcript_fixture_runner.pure]
- GIVEN a transcript fixture that targets a supported verifier action
- WHEN the runner evaluates the fixture
- THEN it MUST call pure in-memory verifier functions and MUST NOT perform filesystem, process, network, or clock I/O inside the evaluation core.

#### Scenario: Positive transcript fixtures pass
r[valence.evidence_ir.transcript_fixture_runner.positive]
- GIVEN positive transcript fixtures for valid Evidence IR or stack-provenance workflows
- WHEN the runner evaluates them
- THEN observed validity, issue codes, and expected hashes MUST match the fixture expectations.

#### Scenario: Negative transcript fixtures fail as expected
r[valence.evidence_ir.transcript_fixture_runner.negative]
- GIVEN negative transcript fixtures for malformed input, overclaims, stale hashes, or denied capabilities
- WHEN the runner evaluates them
- THEN each fixture MUST produce the expected deterministic failure diagnostics.

#### Scenario: Refresh is explicit
r[valence.evidence_ir.transcript_fixture_runner.refresh]
- GIVEN verifier output or expected hashes intentionally change
- WHEN maintainers refresh transcript expectations
- THEN the refresh MUST be explicit, reviewable, and MUST NOT update fixtures during normal validation.

#### Scenario: Fixture discovery order is stable
r[valence.evidence_ir.transcript_fixture_runner.stability]
- GIVEN equivalent transcript fixture sets discovered in different filesystem orders
- WHEN the runner renders its report
- THEN the report MUST be byte-stable or canonical-value stable.
