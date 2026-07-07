## Phase 1: Cache key model

- [ ] [serial] r[valence.evidence_ir.dependency_closure_cache] Define a typed verifier cache key with canonical input, verifier, policy, dependency-closure, capability-set, and schema identities.
- [ ] [serial] r[valence.evidence_ir.dependency_closure_cache.eligibility] Define pure/replay-safe eligibility rules.

## Phase 2: Decisions and tests

- [ ] [serial] r[valence.evidence_ir.dependency_closure_cache.decisions] Add pure cache-decision helpers that return hit, miss, or blocked diagnostics.
- [ ] [parallel] r[valence.evidence_ir.dependency_closure_cache.hit] Add positive cache-hit tests for identical keys.
- [ ] [parallel] r[valence.evidence_ir.dependency_closure_cache.miss] Add negative tests for changed inputs, verifier identity, policy hash, dependency closure, and capability set.
- [ ] [parallel] r[valence.evidence_ir.dependency_closure_cache.boundary] Add tests that effectful/unknown capability sets are blocked unless policy marks them replay-safe.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.dependency_closure_cache.docs] Document cache claims and non-claims.
- [ ] [serial] r[valence.evidence_ir.dependency_closure_cache] Run cache tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
