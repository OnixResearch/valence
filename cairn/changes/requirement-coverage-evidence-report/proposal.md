# Change: Requirement coverage evidence report

## Why

Accepted `r[...]` requirements should have clear test evidence. Valence can improve its harness by emitting a small deterministic report that maps accepted requirements to implementation and verification markers.

## What changes

- Add or document `r[impl ...]` and `r[verify ...]` marker expectations for harness-owned code/tests.
- Add a deterministic requirement coverage report for Evidence IR and stack-provenance surfaces.
- Fail or warn deterministically when accepted requirements lack verification evidence, according to policy.

## Out of scope

- No claim that marker coverage proves requirement satisfaction.
- No replacement for Cairn validation or Tracey-style deeper coverage tools.
