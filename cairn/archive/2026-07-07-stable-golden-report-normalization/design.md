# Design: Stable golden report normalization

## Approach

Introduce one canonical report normalization path for test harness outputs. Reports are converted to typed values or sorted JSON values, then rendered with deterministic key and array ordering where ordering is not semantically meaningful.

## Golden comparison

Golden files store canonical report values. Tests compare normalized values first and render readable diffs when they disagree. Raw stdout snapshots are reserved for CLI contract tests.

## Refresh behavior

Refresh is explicit and reviewable. Normal validation never rewrites golden files.

## Failure model

A report that cannot be normalized fails closed with a deterministic normalization diagnostic.
