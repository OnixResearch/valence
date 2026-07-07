## Context

The built-binary verifier has two different responsibilities:

1. static metadata validation: JSON shape, schema constants, digest syntax, bounded evidence tiers, and required non-claims;
2. dynamic identity validation: read binary bytes, compute BLAKE3, and compare measured digests across sidecar, provenance links, Octet export, and Mantle release metadata.

Rust currently handles both. Nickel contracts are a better fit for the static metadata surface because they are typed, reviewable, and can validate fixtures and automation inputs before Rust computes identities.

## Decisions

### 1. Nickel contracts are preflight/static checks, not byte-measurement authority

**Choice:** Add Nickel contracts that validate metadata shapes and static invariants. Keep binary byte reads, BLAKE3 computation, and cross-artifact digest comparison in Rust.

**Rationale:** Nickel contracts cannot measure local binary bytes in the same deterministic way as the Rust verifier. Keeping byte identity in Rust preserves the current functional-core model and avoids turning Nickel into an execution dependency for the core verifier.

### 2. Contracts live in `contracts/` and fixture data lives under tests

**Choice:** Place reusable contracts under `contracts/stack-provenance/` and positive/negative examples under a test fixture tree.

**Rationale:** Contracts are product artifacts that should be reviewable outside Rust. Fixtures keep positive and negative contract behavior explicit and make contract drift visible in tests.

### 3. Generated JSON schemas may be checked in, but runtime Rust stays independent

**Choice:** If Rust tests need a stable representation, export contract-backed JSON schema/metadata during development and check it in. Do not make `valence-core` execute Nickel at runtime.

**Rationale:** `valence-core` should remain a pure Rust library with no Nickel interpreter dependency. The CLI may offer an explicit preflight validation path later, but the core verifier must remain deterministic over in-memory inputs.

### 4. Contract failures must mirror Rust failure classes

**Choice:** Negative fixtures should cover the same release-risk classes: wrong schema version, invalid BLAKE3 shape, missing sidecar role, missing required non-claim, `manual_review_required` promoted to `formal_proof`, stale parent identity, and overclaim wording.

**Rationale:** Nickel should harden the metadata path without creating a separate taxonomy that release automation cannot reconcile with Rust diagnostics.

## Risks / Trade-offs

- Nickel contract syntax and exported JSON shape can drift. Keep contract fixtures small and include both positive and negative tests.
- Contract validation does not replace Rust verification. Documentation and reports must continue to say that only Rust verifies measured binary bytes.
- Introducing contracts may add a devshell/tooling requirement. Gate it with explicit validation commands rather than runtime coupling.
