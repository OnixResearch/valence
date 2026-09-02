## Why

Valence owns the canonical Evidence IR, evidence roles, and stack-provenance identity for the workspace. Model-checked artifacts that come from the stack's bounded model-checking surface have no explicit role vocabulary, sidecar shape, or verdict classes today. Consumers either drop these artifacts, relabel them as ordinary proof evidence, or invent ad-hoc projections.

A model-checked verdict is a distinct bounded claim: it certifies identity linkage and a bounded verifier outcome, not implementation correctness. Valence must record that claim exactly or record nothing.

## What Changes

- Add a bounded role vocabulary and evidence spans for model-checked artifacts: spec, invariant, checker, bounds, verdict, and witness trace.
- Define a BLAKE3-bound evidence sidecar for a model-checked artifact that keeps each layer identity separate.
- Define verdict classes (verified-within-bounds, violation-with-witness, unsupported-checker) and bind witness-trace identity where present.
- Record the non-claim that a verdict does not prove implementation correctness or general temporal satisfiability.
- Add positive and negative fixtures for valid sidecars, stale checker identity, tampered traces, wrong roles, and overclaims.

## Impact

- **Files**: Valence evidence roles, sidecar schema and validation, verdict vocabulary, fixtures, and docs.
- **Testing**: Positive sidecar admission and role validation plus negative stale, tampered, wrong-role, and overclaim cases.
- **Boundary**: Valence proves identity linkage, roles, and bounded verdict conformance for model-checked artifacts. It does not prove model validity, engine correctness, or implementation correctness.
