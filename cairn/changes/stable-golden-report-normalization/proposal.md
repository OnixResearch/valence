# Change: Stable golden report normalization

## Why

Golden tests are useful only when diffs are semantic and stable. Valence should normalize report output before comparison so field order, fixture discovery order, and incidental stdout formatting do not create noisy failures.

## What changes

- Add canonical JSON report normalization helpers for harness outputs.
- Prefer semantic golden comparisons over raw stdout snapshots.
- Keep raw stdout snapshots limited to CLI boundary behavior.

## Out of scope

- No automatic golden updates during normal tests.
- No replacement of CLI smoke tests that intentionally assert user-facing text.
