## Context

A component pipeline can produce several byte artifacts with related but non-interchangeable meaning: WIT source/package encodings, portable component bytes, component-library package bytes, WAC plans and composed components, WASI-Virt plans and virtualized components, Wizer transforms, and Wasmtime target-specific precompiled native bytes. Producers also emit static validation, build, transform, and runtime receipts.

Valence must make those identities and edges queryable without re-running tools or deciding whether producer claims are true.

## Decisions

### 1. Add a bounded Wasm component profile

**Choice:** Define typed component evidence DTOs for WIT source/interface, package reference, portable component, composition plan/output, virtualization plan/output, transform, precompiled native artifact, Mantle materialization bundle, static/build/runtime receipt, and configuration cohort.

**Rationale:** Generic artifact strings do not prevent role confusion between portable inputs, derived outputs, native code, and evidence.

### 2. Exact byte identities are always BLAKE3

**Choice:** Every Valence-owned artifact identity is BLAKE3 over the exact bytes or canonical Valence DTO bytes for its declared role. The profile validates role-specific metadata and forbids one digest from satisfying incompatible roles without an explicit same-bytes edge.

**Rationale:** Exact bytes are the only durable basis for artifact linkage; names, versions, paths, and WIT worlds are metadata.

### 3. OCI SHA-256 is external interoperability metadata

**Choice:** Model a package's OCI/wkg digest as an opaque typed external digest with algorithm `sha256`, registry/package/version scope, and a required edge to the BLAKE3 exact-byte artifact. It cannot become `canonical_hash`, `stable_id`, or a Valence artifact ref digest.

**Rationale:** The protocol requires SHA-256, while Valence's identity contract remains BLAKE3.

### 4. Interface identity is separate from implementation identity

**Choice:** Compute a normalized WIT interface identity from declared package/world/import/export facts and source/package identity. Link a component through `implements-world` only after producer evidence says it conforms. Multiple components may share one interface identity without same-implementation or semantic-equivalence edges.

**Rationale:** Interface shape is useful for compatibility queries but cannot identify behavior.

### 5. Tool and configuration identity are first-class

**Choice:** Record the component cohort, Wasm feature profile, WIT package versions, WAC/WASI-Virt/Wizer/Wasmtime versions, runtime configuration, target, CPU features, and producer role as typed nodes or payload fields. Valence validates shape and linkage only.

**Rationale:** Derived component and precompiled bytes are meaningful only with their producing configuration.

### 6. The graph uses explicit stage edges

**Choice:** Add bounded edges such as `resolved-package-by`, `implements-world`, `composed-from`, `composed-by`, `virtualized-by`, `validated-by`, `transformed-from`, `precompiled-from`, `materialized-in`, `configured-by`, and `executed-as`. Edge validation enforces allowed source/target roles and prevents cycles or missing parents where the stage contract requires them.

**Rationale:** A flat list cannot explain which exact component was validated, transformed, precompiled, or executed.

### 7. Verification roles remain bounded

**Choice:** Static facts, build records, benchmark results, and execution records default to `RecordedOnly`; policy/admission decisions may use `Boundary`. No artifact receives `Property` solely because a producer reports success. Imported property evidence requires its own checked profile and property scope.

**Rationale:** Valence must not transfer producer success into proof.

### 8. Non-claims are profile requirements

**Choice:** Require explicit non-claims for behavioral correctness, semantic equivalence, package trust, sandbox effectiveness, authority, compiler/runtime correctness, cross-target portability of precompiled bytes, and release eligibility. Reject weakened or missing boundaries.

### 9. Mantle bundles are producer envelopes, not identity shortcuts

**Choice:** Model a Mantle materialization bundle as its own canonical DTO/byte artifact with `materialized-in` edges to every required exact child object and producer receipt. Validate that measured child BLAKE3 values, profile/cohort refs, artifact roles, and parent stages agree. A store path, package name, bundle signature, or bundle hash cannot substitute for a portable component, WIT, Octet report, Wizer output, or `.cwasm` identity.

**Rationale:** Consumers need one handoff envelope, but collapsing its contents into the envelope identity would recreate the role confusion Valence exists to prevent.

## Functional core / imperative shell split

- **Pure core**: DTO and Mantle-bundle validation, canonicalization, BLAKE3 identity material, external-digest role checks, interface normalization, edge validation, role admission, graph queries, non-claim enforcement, and deterministic diagnostics.
- **Imperative shell**: read producer artifacts, measure raw bytes, load contracts/fixtures, write Evidence IR/graph reports, and render explanations.

## Risks / Trade-offs

- Component tooling vocabulary will evolve. Keep schema versions explicit and add new typed roles rather than weakening old ones.
- Normalized WIT identity can be mistaken for compatibility or semantics. Names and edge labels must keep its shape-only claim visible.
- External digests increase schema surface. Restrict them to an explicit interoperability metadata role.
- Large composition graphs can be noisy. Preserve exact stage edges but allow deterministic summary projections.

## Non-Goals

- No Wasm parsing, WIT conformance checking, component execution, package fetching, composition, virtualization, precompilation, or Mantle bundle materialization in Valence.
- No decision that producer evidence is true, sufficient, current, or release-eligible.
- No SHA-256 canonical Valence identity.
- No semantic-equivalence edge from matching WIT, outputs, replay traces, or benchmark results.
