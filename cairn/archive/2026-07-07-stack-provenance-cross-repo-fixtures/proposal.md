# Change: Stack provenance cross-repo fixtures

## Why

Valence’s stack-provenance verifier protects a cross-repo thread: Octet export, Valence sidecar, Mantle manifest, and measured binary hash. The harness should include tiny frozen fixtures that exercise this thread quickly and deterministically.

## What changes

- Add minimal checked-in fixtures for Octet export, Valence sidecar, Mantle manifest, and binary hash thread inputs.
- Add positive and negative fixture tests for identity/linkage checks.
- Keep larger sibling-repo integration as an optional deep/nightly suite.

## Out of scope

- No behavioral correctness, release eligibility, semantic-equivalence, or whole-program safety claims.
- No dependency on live sibling repositories in the fast fixture suite.
