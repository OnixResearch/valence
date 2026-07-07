## Context

Valence's Preserves corpus already demonstrates stable metadata identity through fixtures. Executable transcripts generalize that approach to full evidence workflows: inputs, verifier action, expected report, and expected diagnostics.

## Decisions

### 1. Transcript core is pure

A transcript scenario should be represented as data and evaluated by pure helpers where possible. CLI command execution, filesystem setup, and binary invocation remain test-harness shell behavior.

### 2. Positive and negative cases are required

Every transcript suite must include passing scenarios and failing scenarios with expected issue codes.

### 3. Refresh is explicit

Expected outputs and hashes are updated only through an intentional refresh command/test mode that leaves reviewable diffs.

### 4. Transcript claims are bounded

A passing transcript proves the scenario matched expected outputs for checked inputs. It does not prove correctness outside the fixture.

## Risks / Trade-offs

- Transcript fixtures can become brittle. Keep them focused on stable issue codes and canonical values.
- Command transcripts can be environment-sensitive. Prefer pure verifier transcripts when possible.
