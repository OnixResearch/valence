## Why

Unison watch expressions make examples executable and keep docs close to checked behavior. Valence already has corpus fixtures for Preserves; broader receipt transcripts would make examples, CLI diagnostics, expected hashes, and failure behavior executable review artifacts.

This change proposes checked-in executable receipt transcripts for evidence workflows.

## What Changes

- Define a transcript fixture format for commands or pure verifier scenarios, expected outputs, expected issue codes, and expected hashes.
- Add a transcript runner/gate with deterministic positive and negative cases.
- Support intentional refresh workflow for expected hash/output changes.
- Document transcripts as executable evidence examples, not proof of correctness.

## Impact

- **Files**: transcript fixtures under contracts/tests, core transcript DTOs, CLI/test harness as needed, docs.
- **Compatibility**: additive validation surface.
- **Validation**: transcript runner tests and workspace tests.
