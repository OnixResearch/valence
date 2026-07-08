# Evidence IR Specification

## Purpose

Extends `evidence-ir` with dependency-closure-aware verifier result caching.

## Requirements

### Requirement: Dependency-closure verifier cache
r[valence.evidence_ir.dependency_closure_cache] Valence MUST define verifier cache keys that include canonical input identity, verifier identity, policy identity, dependency closure identity, capability set identity, and cache schema version.

#### Scenario: Cache key includes all identity inputs
r[valence.evidence_ir.dependency_closure_cache.key]
- GIVEN a verifier result is considered for reuse
- WHEN Valence computes the cache key
- THEN the key MUST include canonical input hash, verifier identity, policy hash, dependency closure hash, capability set hash, and cache schema version.

#### Scenario: Cache eligibility is bounded by capabilities
r[valence.evidence_ir.dependency_closure_cache.eligibility]
- GIVEN a verifier capability set is pure or explicitly replay-safe
- WHEN Valence evaluates cache eligibility
- THEN the result MAY be eligible only if all cache-key inputs match.

#### Scenario: Cache decisions are deterministic
r[valence.evidence_ir.dependency_closure_cache.decisions]
- GIVEN a cache lookup and current verifier context
- WHEN Valence compares cache keys
- THEN it MUST return a deterministic hit, miss, or blocked decision with issue codes.

#### Scenario: Matching keys hit
r[valence.evidence_ir.dependency_closure_cache.hit]
- GIVEN a cached result and current verifier context with identical cache-key fields
- WHEN Valence performs a cache lookup
- THEN it SHOULD report a cache hit and reuse the result only if policy allows reuse.

#### Scenario: Changed key fields miss
r[valence.evidence_ir.dependency_closure_cache.miss]
- GIVEN any cache-key field changes
- WHEN Valence performs a cache lookup
- THEN it MUST report a cache miss rather than reuse the stale result.

#### Scenario: Effectful cache reuse is blocked by default
r[valence.evidence_ir.dependency_closure_cache.boundary]
- GIVEN a verifier used filesystem, clock, process, network, remote-builder, secret, or unknown capabilities
- WHEN no policy marks that capability set replay-safe
- THEN cache reuse MUST be blocked.

#### Scenario: Cache boundary is documented
r[valence.evidence_ir.dependency_closure_cache.docs]
- GIVEN documentation or receipts describe verifier caching
- WHEN the supported claim is stated
- THEN the output MUST state that cache hits prove matching cache-key identities only, not verifier soundness, correctness, or requirement satisfaction.
