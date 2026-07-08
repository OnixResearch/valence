## Context

Valence already computes stable hashes for evidence artifacts and stack-provenance metadata. A Unison-like cache should reuse verifier outputs only when all identity-relevant inputs match.

## Decisions

### 1. Cache keys are explicit typed values

A cache key includes canonical input hash, verifier identity, policy hash, dependency closure hash, capability set hash, and cache schema version. Use BLAKE3 for new hashes.

### 2. Pure/replay-safe only

Pure verifier results can be reused when the key matches. Effectful results require additional environmental provenance or are cache-ineligible by default.

### 3. Cache decisions are reports

Cache lookup should return deterministic hit/miss/blocked diagnostics suitable for receipts.

### 4. Cache is optional

The verifier remains correct without cache. Cache code must not weaken fail-closed validation.

## Risks / Trade-offs

- Incorrect cache keys can hide changes. Negative tests must cover every key component.
- Capability labels are receipt evidence unless enforced. Strict policies should default to no reuse for effectful verifiers.
