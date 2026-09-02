# Change: Kani non-claim boundary

## Why

Kani evidence is powerful but bounded by tool semantics: plain bounded harnesses need unwind coverage, recursive termination is not automatically proved, FFI and plain stubs are assumptions, and Kani does not model every aliasing or concurrency behavior. Valence should make those boundaries machine-checkable.

## What Changes

- Add required Kani non-claims for whole-program correctness, verifier soundness, aliasing models, concurrency/async, FFI/stubs, recursive termination, and bounded-only checks.
- Bind non-claims to Kani evidence roles so weakening or omitting them fails closed.
- Add fixtures for valid scoped claims and invalid overclaims.
- Surface the boundary in CLI diagnostics and stack-provenance reports.

## Impact

- **Files**: Evidence IR validators, non-claim policy fixtures, stack-provenance report tests, and docs.
- **Consumers**: Cairn and Mantle receive Kani evidence with explicit claim limits.
- **Non-claims**: This change records limits; it does not reduce the value of scoped Kani properties that pass validation.
