## Why

Unison's core idea is that definitions are identified by content hash while names are separate metadata. Valence already computes canonical hashes for evidence artifacts, but evidence-relevant definitions and verifier inputs still need a clearer contract: identity must be content-addressed and stable across display names, paths, and aliases.

This change adopts the content-addressed-definition model for Valence evidence surfaces without adopting the Unison runtime or language.

## What Changes

- Define canonical evidence-definition identity as a BLAKE3 hash over a typed canonical form.
- Ensure dependency references inside that canonical form use content identities rather than display names.
- Treat paths, source IDs, aliases, and names as metadata unless an existing compatibility contract explicitly says otherwise.
- Add positive and negative tests for rename/path churn, dependency changes, and canonical hash stability.

## Impact

- **Files**: likely `crates/valence-core/src/evidence_ir.rs`, `crates/valence-core/src/function_object.rs`, tests, and documentation.
- **Compatibility**: existing JSON fields stay compatible; new identity fields are additive unless a later change explicitly migrates them.
- **Validation**: focused Evidence IR tests, provenance tests, stack-provenance tests if touched, and Cairn gates.
