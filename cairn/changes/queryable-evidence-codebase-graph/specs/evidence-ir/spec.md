# Evidence IR Specification

## Purpose

Extends `evidence-ir` with a deterministic, queryable evidence-codebase graph.

## Requirements

### Requirement: Queryable evidence-codebase graph
r[valence.evidence_ir.queryable_graph] Valence MUST provide a typed evidence graph that indexes evidence identities and linkage relationships across definitions, artifacts, requirements, verifiers, receipts, binaries, sidecars, and aliases.

#### Scenario: Graph contains typed nodes and edges
r[valence.evidence_ir.queryable_graph.nodes]
- GIVEN evidence artifacts, provenance chains, stack sidecars, receipts, binaries, and alias bindings
- WHEN Valence builds the evidence graph
- THEN the graph MUST contain typed nodes and typed edges for their identity and linkage relationships.

#### Scenario: Graph export is deterministic
r[valence.evidence_ir.queryable_graph.export]
- GIVEN equivalent graph inputs in different orders
- WHEN Valence exports the graph
- THEN the exported graph MUST be byte-stable or canonical-value stable.

#### Scenario: Core queries are pure
r[valence.evidence_ir.queryable_graph.queries]
- GIVEN a graph value in memory
- WHEN Valence queries dependency closure, reverse dependencies, requirement coverage, binary traceability, verifier evidence, or alias resolution
- THEN the query MUST return deterministic results without filesystem or process I/O.

#### Scenario: Equivalent inputs have stable graph identity
r[valence.evidence_ir.queryable_graph.determinism]
- GIVEN positive fixtures containing the same nodes and edges in different input order
- WHEN the graph determinism gate runs
- THEN Valence MUST compute the same graph identity and query results.

#### Scenario: Broken graph edges fail closed
r[valence.evidence_ir.queryable_graph.fail_closed]
- GIVEN duplicate node identities, dangling edges, or ambiguous aliases
- WHEN Valence validates the graph
- THEN validation MUST fail closed with deterministic diagnostics.

#### Scenario: Graph boundary is documented
r[valence.evidence_ir.queryable_graph.docs]
- GIVEN documentation or query output describes the evidence graph
- WHEN the supported claim is stated
- THEN the output MUST state that the graph indexes evidence identity and linkage only, not behavioral correctness, verifier soundness, release eligibility, or requirement satisfaction.
