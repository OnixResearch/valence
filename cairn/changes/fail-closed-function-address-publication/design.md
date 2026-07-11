## Context

The shell currently calls `validate_function_address_evidence`, then independently calls `function_address_graph_rows(&input)` regardless of `report.valid`. It serializes and writes both report and graph before returning the validation exit status. The graph output is only a JSON array, so it does not carry the failed verdict, policy identity, validated artifact hash, or non-claim boundary visible in stdout.

Direct writes replace destinations one at a time. Confinement and validation do not make those replacements atomic as a pair.

## Decisions

### Plan publication only from validated core output

**Choice:** Introduce a pure function that validates the input and returns either a diagnostic failure report with no publishable bytes or a typed `FunctionAddressPublication` containing the artifact, canonical graph report, and publication identities. Raw input graph-row construction is private to the successful branch.

Invalid input may be rendered to stdout or stderr for operator diagnosis, but publishable receipt, graph, bundle, and graph-root fields are absent. Requested destination files are not created, truncated, or replaced.

### Version and bind the graph report

**Choice:** Replace the canonical bare row-array output with a versioned report containing the profile version, policy identity, validated Evidence IR artifact identity, sorted typed rows, summary counts, `valid: true`, required non-claims, and a BLAKE3 report identity computed over explicit self-hash-excluded canonical material.

Rows sort by the complete `(from, to, edge_kind)` key. Empty locators and exact duplicate dependency edges fail validation. Self-dependency remains representable because recursive functions may legitimately depend on themselves; this change does not infer call-graph semantics.

### Prefer one atomic publication artifact

**Choice:** Add a canonical bundle output that contains the receipt and graph report in one versioned artifact. The shell writes it to a temporary sibling through the declared output capability, flushes it, and atomically renames it over the destination. A failed validation or pre-commit I/O error leaves an existing destination unchanged.

The exact durability guarantee is documented per supported platform; atomic replacement does not imply crash-proof storage hardware.

### Bound legacy split-output behavior

**Choice:** Existing separate receipt and graph outputs may remain behind explicit compatibility flags. Each file is atomically replaced individually and carries the same publication identity, policy identity, artifact identity, and valid verdict. The shell validates and stages both destinations before either replacement. Because two renames cannot provide portable crash-atomicity across arbitrary directories, split output must not claim transactional pair publication and is not the preferred readiness input.

### Coordinate with explicit capability roots

Implementation should consume the authority model from `enforce-declared-cli-capability-roots`. Until that change lands, this package must not reintroduce parent-derived output authority while adding temporary or bundle paths.

## Risks / Trade-offs

- Consumers of the raw graph array need a compatibility adapter or migration to the versioned report.
- Atomic rename semantics vary by filesystem; tests cover supported local filesystems and documentation states the boundary.
- Suppressing invalid-output files removes a convenient persisted failure report; operators can redirect the diagnostic stream or request a separately named diagnostic report that is never shaped like valid evidence.
