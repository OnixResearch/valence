# Evidence IR Specification

## Purpose

Extends `evidence-ir` with executable receipt transcripts for checked examples and diagnostics.

## Requirements

### Requirement: Executable receipt transcripts
r[valence.evidence_ir.executable_transcripts] Valence MUST support checked transcript fixtures that bind verifier inputs to expected reports, diagnostics, and content hashes.

#### Scenario: Transcript fixture has stable shape
r[valence.evidence_ir.executable_transcripts.schema]
- GIVEN a transcript fixture
- WHEN Valence parses it
- THEN it MUST include schema version, scenario identity, inputs, verifier action, expected report fields, expected issue codes, expected hashes, and non-claims.

#### Scenario: Transcript runner evaluates pure scenarios
r[valence.evidence_ir.executable_transcripts.runner]
- GIVEN a transcript scenario that can run through pure verifier functions
- WHEN the transcript gate runs
- THEN Valence MUST evaluate it without filesystem or process I/O.

#### Scenario: Positive transcripts pass
r[valence.evidence_ir.executable_transcripts.positive]
- GIVEN a valid Evidence IR or stack-provenance transcript fixture
- WHEN the transcript gate runs
- THEN expected reports, issue codes, and hashes MUST match.

#### Scenario: Negative transcripts fail as expected
r[valence.evidence_ir.executable_transcripts.negative]
- GIVEN malformed input, overclaim, stale hash, or denied capability transcript fixtures
- WHEN the transcript gate runs
- THEN each fixture MUST produce its expected deterministic failure diagnostics.

#### Scenario: Transcript refresh is explicit
r[valence.evidence_ir.executable_transcripts.refresh]
- GIVEN verifier output or hash expectations intentionally change
- WHEN maintainers refresh transcript fixtures
- THEN the refresh MUST produce reviewable fixture diffs and MUST NOT update expectations implicitly during normal validation.

#### Scenario: Transcript boundary is documented
r[valence.evidence_ir.executable_transcripts.docs]
- GIVEN documentation or receipt output describes executable transcripts
- WHEN the supported claim is stated
- THEN the output MUST state that transcripts prove fixture behavior for checked inputs only, not general correctness, verifier soundness, or requirement satisfaction.
