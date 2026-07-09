## Design

The primitive layer is a pure `valence-core` design. It accepts in-memory DTOs, identifiers, maps, and already-parsed attestation metadata. It does not read files, inspect environment, call clocks, execute tools, or decide release eligibility.

### Decisions

1. **References are intake evidence, not implementation authority.** Oxide repositories are used to shape requirements and fixtures. Any crate dependency, copied concept, or reimplementation must pass license, MSRV, no-std, and non-claim review before code changes land.

2. **BLAKE3 remains the canonical stack identity.** Typed UUID wrappers are useful for external/user-facing IDs, but Valence-owned canonical artifact identity stays BLAKE3 unless an interop surface already requires another format.

3. **Derived-key indexes must fail closed.** The `iddqd` pattern is useful when a collection's key is borrowed or derived from its value. Valence adapters must reject duplicate derived keys, mismatched borrowed/owned query keys, and role-mismatched lookups deterministically.

4. **Semantic diffs are review evidence only.** `daft`-style structural diffs can make Evidence IR migration and stack-provenance changes reviewable. A diff report must not claim semantic equivalence, behavior preservation, or proof validity.

5. **Attestation vocabulary is normalized, not trusted.** CoRIM, VM attestation, DICE, and certificate-chain terminology can inform Valence metadata names. Valence must still require explicit issuer, subject, scope, digest, and non-claim boundaries before accepting any attestation-shaped artifact.

6. **Testing is fixture-first.** Positive fixtures cover valid typed IDs, duplicate-free derived indexes, stable semantic diffs, and bounded attestation metadata. Negative fixtures cover malformed IDs, duplicate keys, stale diffs, missing issuers, weakened non-claims, and overclaim text.

### Validation shape

The smallest implementation slice should add pure unit/property tests in `valence-core`, a fixture matrix under `fixtures/`, and Cairn validation/gate receipts for this change. Shell adapters may only read files or render reports after pure validation returns typed outcomes.
