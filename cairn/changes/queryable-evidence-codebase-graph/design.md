## Context

Valence already has `evidence_codebase_index`, provenance chains, and stack-provenance reports. A Unison-inspired codebase database suggests one deterministic graph model over those surfaces.

## Decisions

### 1. Graph is a typed value first

The graph should be built by pure functions from in-memory evidence inputs. CLI import/export and filesystem discovery remain imperative shell work.

### 2. Nodes and edges are explicit

Nodes should include evidence definitions, artifacts, requirements, verifier receipts, binaries, sidecars, and aliases. Edges should include depends-on, verifies, covers, aliases, produced-by, and bundled-in relationships.

### 3. Deterministic export is required

Graph export must sort nodes and edges by stable identity and use canonical serialization so diff and receipt evidence stays stable.

### 4. Query helpers are pure

Queries should run over the graph value without I/O. CLI/UI code can render query results later.

## Risks / Trade-offs

- The graph can become too broad. Start with identity/linkage queries that current modules can populate.
- Duplicate surfaces can drift. Tests should compare graph output against existing provenance/stack-provenance fixtures.
