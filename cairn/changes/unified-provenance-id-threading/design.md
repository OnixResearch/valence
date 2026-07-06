## Design: unified provenance ID threading

### The chain model

```
source_id (EvidenceSpan)  →  function_object_id (FunctionObjectV1)
                           →  requirement_ids (cairn r[...])
                           →  artifact_hash (cairn/mantle receipt)
                           →  binary_hash (mantle release-evidence)
```

A `ProvenanceLink` connects all five. A `ProvenanceChain` is a collection of
links. The chain supports three resolution directions:

- `resolve(source_id)` — source → full link (forward trace)
- `resolve_requirement(requirement_id)` — requirement → function objects (coverage)
- `trace_binary(binary_hash)` — binary → source functions (reverse trace)

Valence preserves source-span linkage through its evidence chain and threads
IDs through cairn receipts to mantle release-evidence.

### What lands now vs Phase 3

- **Now**: the `ProvenanceLink`/`ProvenanceChain` data model, checker, and
  resolution functions. Tools can construct and validate chains.
- **Phase 3**: cairn's EARS property vocabulary gate will use
  `resolve_requirement` to check that every requirement has at least one
  function object discharging it (the "response resolves to a predicate" check).
- **Phase 2 mantle**: `release-evidence-provenance-id-coverage` will record
  `binary_hash` → `source_id` coverage in release-evidence bundles.

### Non-claims

Provenance threading proves identity and linkage only. It does not prove:
- behavioral correctness
- semantic equivalence
- that the binary satisfies the requirement
- that the function object correctly implements the requirement

It proves the binary's behavior is *traceable* to a source function and a
requirement, not that it is *correct*.
