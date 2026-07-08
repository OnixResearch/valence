# Design: Requirement coverage evidence report

## Approach

The report scans accepted requirement IDs and checked-in source/test markers, then renders deterministic coverage rows. The pure core computes coverage from in-memory requirement and marker sets; the shell owns file discovery and report writing.

## Marker model

Implementation markers use `r[impl ...]`; verification markers use `r[verify ...]`. Missing verification evidence is reported separately from missing implementation markers.

## Policy posture

The initial report may be advisory for existing surfaces, but it must be deterministic and CI-readable. Future policy can promote selected missing evidence to failures.

## Boundary

Coverage markers show traceability between requirements and tests; they do not prove semantic correctness or requirement satisfaction.
