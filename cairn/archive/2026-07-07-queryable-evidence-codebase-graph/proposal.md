## Why

Unison treats the codebase as a structured database with types, dependencies, hashes, docs, and search. Valence has pieces of this model across Evidence IR, provenance chains, stack sidecars, Preserves corpus fixtures, and receipts, but lacks one queryable evidence graph.

A queryable evidence-codebase graph would let tools answer identity, dependency, requirement, verifier, and release-linkage questions from one deterministic index.

## What Changes

- Define a typed evidence graph containing artifacts, definitions, requirements, verifiers, receipts, binaries, sidecars, and alias bindings.
- Add deterministic graph export/import and query helpers.
- Support queries for reverse dependencies, requirement coverage, binary traceability, verifier evidence, and alias resolution.
- Document bounded claims: the graph indexes evidence identity/linkage, not correctness.

## Impact

- **Files**: likely `evidence_codebase_index`, `provenance`, docs, fixtures, and CLI surfaces later.
- **Compatibility**: additive graph/index surface.
- **Validation**: graph determinism tests, query tests, and Cairn gates.
