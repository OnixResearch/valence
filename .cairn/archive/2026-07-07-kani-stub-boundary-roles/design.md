# Design: Kani stub boundary roles

## Approach

Stub semantics are modeled as typed payload facets. The artifact's verification role is derived from the verified-contract linkage and from the declared stub kind.

## Stub kinds

Valence recognizes:

- verified contract stubs that point at a passing contract artifact;
- plain implementation stubs that replace a function with a handwritten model;
- environmental stubs that model clocks, randomness, syscalls, FFI, or other external effects.

## Promotion rules

Verified contract stubs may support property evidence for the caller only when the linked contract artifact is present, passing, scope-compatible, and current. Plain and environmental stubs are `Boundary` or `RecordedOnly` evidence and must carry assumptions.

## Fixture strategy

Positive fixtures cover a verified-stub link and an environmental-stub boundary artifact. Negative fixtures cover missing contract link, stale contract hash, signature mismatch, plain-stub promotion, and missing assumptions.

## What is explicitly not provided

- No proof that a plain stub is a faithful model.
- No promotion from boundary assumption to property evidence without a verified contract.
- No hidden trust in environmental models.
