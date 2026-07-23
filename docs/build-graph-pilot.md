# build-graph advisory navigation pilot

## Decision

Valence does not integrate `build-graph` at reviewed upstream revision
`68a12dcac9b2475e27b6062f974b5f37321d3c4f`.

The pilot found useful bounded navigation and byte-identical same-host outputs,
but it also reproduced silent identity collisions and incomplete coverage of a
bin-only workspace package. Those failures make generated architecture text or
graphs unsuitable as reviewed Valence inputs. No Cargo dependency, Nix input,
CI workflow, generated `ARCHITECTURE.md`, or Evidence IR role was added.

This decision is scoped to the reviewed revision. It is not a claim about later
`build-graph` revisions.

## Scope and exact inputs

- Valence revision: `4f27958469e5fe0689d603ef4122d25d357a28f0`.
- `build-graph` revision: `68a12dcac9b2475e27b6062f974b5f37321d3c4f`.
- `build-graph` manifest version: `0.2.1`.
- Rustdoc JSON toolchain: `nightly-2026-02-27`.
- Layer 3 backend: the `rust-analyzer` component from that toolchain.
- Output mode: uncompressed JSON beneath ignored `target/build-graph-pilot/`.

The pilot did not evaluate the optional rustc-driver backend, the GitHub Action,
cross-machine determinism, cross-toolchain determinism, incremental-update
correctness after source mutation, or generated-document review workflows.

## Positive observations

Layer 2 produced 5,725 nodes and 7,440 edges. It located
`evidence_ir::build_evidence_artifact` at
`crates/valence-core/src/evidence_ir.rs:336`, and bounded `find` and `refs`
queries returned useful type, field, method, parameter, and return relations.

Layer 3 added 1,548 `calls`, 7,466 `uses`, 937 `member_calls`, and 5,074
`member_uses` edges, producing 22,465 total edges. The backend emitted duplicate
SCIP-symbol diagnostics but completed successfully.

Two independent same-host Layer 2 runs produced identical BLAKE3 identities:

- `graph.json`: `1efd8f6d1d24321313ce6bd2770e599cbf72a4fe8d8eca624eb117e5d2852823`
- `GRAPH_REPORT.md`: `b8d97bd4d34e781b3df04e955f56014c0f5f63be859402275d53cc9f6488f443`
- `graph.html`: `eedb8b20a53014b2239e1709e7b3644f1fa8adc471d967147ad0fd950f218173`

Two independent same-host Layer 3 runs also matched:

- `graph.json`: `5e5137e70350adc95e43b42133f6833538e74541a107064d57818728bafb9e0c`
- `GRAPH_REPORT.md`: `b9fc5b1f5b1957c5aae4d8501bb49753f18273a0b12ada4e05bf73966480d8eb`
- `graph.html`: `ed748a096006ce61ca054effd0b6d68c3fd6c746fd08c557c8f15cbad14fb016`

These matches prove only byte identity for the recorded repeated runs on this
host and toolchain.

## Blocking negative observations

### Distinct Rust identities merge silently

`build-graph` lowercases ASCII alphanumeric characters and maps every other
character to `_` when constructing crate, file, and item IDs. Its graph stores
nodes in a map keyed by those IDs and enriches an existing node on collision
instead of rejecting conflicting identities.

A compiled negative fixture contained both:

- source files `src/a-b.rs` and `src/a_b.rs`;
- same-kind functions `Foo` and `foo`.

Both file paths normalized to
`file__build_graph_collision_fixture__src_a_b_rs`. The graph retained only the
`src/a-b.rs` node. Both functions normalized to
`item__build_graph_collision_fixture__foo__function`. The graph retained only
`Foo`; exact searches for either `Foo` or `foo` returned that same node.

The fixture contained three logical source files and six logical rich items.
The extractor reported three Layer 1 files but only five Layer 2 items, then
wrote eight total nodes. Two valid source identities were therefore lost
without a failed command or collision diagnostic.

### The bin-only shell is absent from the rich graph

The workspace contains four packages, including the bin-only `valence-cli`.
The graph contains its metadata crate node and dependency edge, but the rich
layer documented only three packages. Its report attributes all 38 represented
source files to `valence-core`, `valence-radicle`, and the shared-identity pilot.

A source-location query for `crates/valence-cli/src/main.rs` returned zero
matches in both Layer 2 and Layer 3 outputs. The reference layer connects
existing rich-layer nodes and therefore did not restore the missing shell
symbols.

This omission prevents the graph from representing Valence's functional-core /
imperative-shell architecture as a complete workspace map.

## Re-entry gates

A later revision may be reconsidered only when all of these gates pass:

1. IDs use collision-resistant, deterministic identity rather than lossy
   normalization, and conflicting raw identities fail closed.
2. Positive and negative fixtures cover case differences, punctuation
   differences, Unicode, delimiter handling, and incremental replacement.
3. Bin-only packages receive file and symbol coverage, including bounded
   source-location and caller queries.
4. Layer 2 and each supported Layer 3 backend produce byte-identical repeated
   outputs under exact toolchain and backend pins.
5. The crate version, release tag, and any action revision form one reviewed,
   immutable source identity.
6. Generated outputs remain advisory navigation artifacts. They do not become
   canonical Evidence IR, Mantle semantic/provenance graphs, execution-graph
   semantics, proof evidence, or release-readiness evidence.

## Reproduction outline

Install only the reviewed revision and its locked dependency set:

```sh
cargo install \
  --git https://github.com/d-tietjen/build-graph \
  --rev 68a12dcac9b2475e27b6062f974b5f37321d3c4f \
  --locked \
  --root target/build-graph-pilot/tool

rustup toolchain install nightly-2026-02-27 --profile minimal
rustup component add rust-analyzer --toolchain nightly-2026-02-27
```

Generate a Layer 2 run:

```sh
target/build-graph-pilot/tool/bin/cargo-build-graph build \
  --manifest-path Cargo.toml \
  --out target/build-graph-pilot/run-a \
  --rich \
  --nightly nightly-2026-02-27 \
  --no-compress
```

Add `--references` and use a separate output directory for the Layer 3 run.
Compare repeated outputs with BLAKE3. Do not use an ambient branch, moving tag,
or auto-commit workflow for this pilot.

## Claim boundary

This pilot records observations about one external tool revision over one
Valence revision. It does not prove extractor completeness, Rust semantic
correctness, stable identity, cross-host reproducibility, caller precision,
Evidence IR validity, or release eligibility.
