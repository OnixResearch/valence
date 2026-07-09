## Why

Octet can produce bounded evidence that a marked Rust state-machine transition core handled its declared state/event enum domain without fallback branches. Valence needs a canonical Evidence IR boundary for that receipt so stack consumers can link it to source spans and release artifacts without treating source-shape evidence as a semantic proof.

## What Changes

- Add a state-machine exhaustiveness evidence receipt contract for Octet-produced checks.
- Validate transition-core identity, state/event domain identity, coverage summary identity, status, diagnostics, and BLAKE3 artifact binding.
- Preserve optional Verus or Kani proof links as separate evidence records rather than promoting the Octet lint receipt.
- Reject missing non-claims, malformed domain identities, stale hashes, unknown evidence tiers, and correctness or release-readiness overclaims.

## Non-claims

This change does not make Valence a Rust analyzer, model checker, proof assistant, or release-readiness engine. A valid receipt means the Evidence IR accepted the bounded Octet check record and its links; it does not prove behavioral correctness, semantic equivalence, protocol completeness, liveness, verifier soundness, or exhaustive Rust coverage.
