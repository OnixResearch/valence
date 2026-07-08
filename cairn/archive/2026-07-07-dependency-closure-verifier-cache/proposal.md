## Why

Unison avoids rebuilding and retesting pure definitions when their dependency closure is unchanged. Valence can adopt the same principle for verifier results: a result can be reused only when the canonical inputs, verifier identity, policy identity, dependency closure, and capability set match.

This would make repeated stack/evidence checks faster while keeping replay evidence bounded and auditable.

## What Changes

- Define a verifier cache key based on BLAKE3 hashes of canonical inputs, verifier binary/definition identity, policy identity, dependency closure identity, and capability set.
- Add pure functions for cache eligibility and hit/miss diagnostics.
- Permit reuse only for pure or explicitly replay-safe capability sets.
- Add tests for cache hits, misses, stale dependency closures, and denied capability sets.

## Impact

- **Files**: likely Evidence IR report/cache module, docs, fixtures.
- **Compatibility**: additive; no existing verifier must use the cache initially.
- **Validation**: cache-key determinism tests and workspace tests.
