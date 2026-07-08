# Design: Executable transcript fixture runner

## Approach

The runner is a thin imperative shell over the existing pure transcript evaluator. The shell discovers checked-in fixture files, decodes them into typed transcript values, invokes the pure runner, and renders stable report output. The core remains deterministic over in-memory values.

## Fixture layout

Use stable fixture roots such as `testdata/transcripts/pass/` and `testdata/transcripts/fail/`. Each fixture records schema identity, scenario identity, inputs, verifier action, expected report fields, expected issue codes, expected hashes, and non-claims.

## Refresh behavior

Refreshing expected outputs is an explicit command or documented maintenance step. Normal tests compare expectations only; they must not rewrite fixture files.

## Failure model

Malformed fixtures, stale hashes, unexpected issue codes, missing non-claims, or unsupported actions fail closed with deterministic diagnostics.
