# Change: Executable transcript fixture runner

## Why

Valence now has typed transcript values, but the testing harness still needs a first-class fixture runner so transcript examples become durable regression evidence instead of ad hoc unit-test construction.

## What changes

- Add pass/fail transcript fixture directories for Evidence IR and stack-provenance scenarios.
- Add a deterministic runner that loads transcript fixtures, evaluates only pure verifier actions, and reports stable diagnostics.
- Add an explicit refresh workflow that produces reviewable expectation diffs and is never invoked by normal validation.

## Out of scope

- No implicit golden refresh during test runs.
- No filesystem, process, or network effects in the transcript evaluation core.
- No claim that passing transcripts prove general verifier soundness or requirement satisfaction.
