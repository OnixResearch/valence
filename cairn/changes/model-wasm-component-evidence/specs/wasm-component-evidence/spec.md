# Wasm Component Evidence Specification

## Purpose

Represent WebAssembly component package, interface, build, composition, virtualization, transform, precompile, and execution identities as bounded Evidence IR linkage without transferring producer semantics or claims.

## Requirements

### Requirement: Wasm component evidence uses typed profiles

r[valence.wasm_component.profile] Valence MUST define typed Evidence IR roles and schemas for WIT source/interface, package reference, portable component, composition plan/output, virtualization plan/output, transform, target-specific precompiled artifact, producer receipt, and component configuration cohort.

#### Scenario: Known role is validated
- GIVEN a producer submits a component artifact under a supported schema and role
- WHEN Valence validates the profile
- THEN it MUST apply role-specific required fields, parent links, verification roles, and non-claims.

#### Scenario: Portable and native roles are confused
- GIVEN target-specific precompiled bytes are labeled as a portable component or portable bytes are labeled as native precompiled code
- WHEN profile validation runs
- THEN Valence MUST reject the artifact.

### Requirement: Exact artifact identity remains BLAKE3

r[valence.wasm_component.byte_identity] Valence MUST use BLAKE3 over exact artifact bytes or canonical Valence DTO bytes for every Valence-owned component evidence identity and MUST preserve digest-role separation across WIT, portable component, plan, derived component, precompiled native artifact, and receipt roles.

#### Scenario: Derived output is linked
- GIVEN a composition, virtualization, transform, or precompile stage produces exact output bytes
- WHEN Valence builds its node
- THEN the node MUST carry the output's exact BLAKE3 and an explicit parent edge to the exact input artifact.

#### Scenario: Digest from another role is substituted
- GIVEN a WIT, interface, package, receipt, or source-component digest is supplied as a derived output's exact-byte digest
- WHEN validation runs
- THEN Valence MUST reject the role substitution unless an explicit same-bytes relation is valid for those exact measured bytes.

### Requirement: WIT interface identity is shape-only

r[valence.wasm_component.interface_identity] Valence MUST represent normalized WIT package/world/import/export identity separately from component byte identity and MUST NOT infer same implementation, behavior, or semantic equivalence when components share that interface identity.

#### Scenario: Two components implement one world
- GIVEN producer evidence links two distinct component BLAKE3 identities to one normalized world
- WHEN the graph is queried
- THEN both MAY have `implements-world` edges while remaining distinct implementation artifacts.

#### Scenario: Interface match requests equivalence
- GIVEN an artifact or edge claims two components are behaviorally equivalent because their normalized WIT identities match
- WHEN validation runs
- THEN Valence MUST reject the overclaim.

### Requirement: External OCI digests cannot become canonical identity

r[valence.wasm_component.external_digest] Valence MAY record OCI/wkg SHA-256 as typed external interoperability metadata only when it names algorithm, registry, package, version, and an exact BLAKE3-identified byte artifact; it MUST NOT satisfy `stable_id`, `canonical_hash`, or Valence artifact-ref digest roles.

#### Scenario: Package carries both digest roles
- GIVEN package bytes have a verified OCI SHA-256 and measured BLAKE3
- WHEN package evidence is built
- THEN Valence MUST retain the SHA-256 as external metadata linked to the BLAKE3 artifact node.

#### Scenario: SHA-256 is placed in canonical hash
- GIVEN a producer places an OCI digest in a Valence canonical identity field
- WHEN validation runs
- THEN Valence MUST fail closed.

### Requirement: Tool and configuration identity is linked

r[valence.wasm_component.configuration] Component stage evidence MUST bind the applicable tool cohort, Wasm feature profile, WIT package versions, composition/virtualization/transform/runtime configuration, target, CPU features, and producer role without treating those declarations as evidence truth.

#### Scenario: Precompile configuration is complete
- GIVEN a precompiled artifact node has source, Wasmtime cohort, full configuration, target, CPU-feature, and WIT-profile links
- WHEN linkage validation runs
- THEN Valence MAY accept the bounded identity graph.

#### Scenario: Target-specific artifact lacks target identity
- GIVEN a `.cwasm` node omits target, CPU features, runtime configuration, or source component parent
- WHEN validation runs
- THEN Valence MUST reject it as incomplete.

### Requirement: Component stage graphs use typed edges

r[valence.wasm_component.graph] Valence MUST validate typed edges for package resolution, world implementation, composition, virtualization, validation, transformation, configuration, precompilation, and execution, including allowed source/target roles, required parents, and cycle constraints.

#### Scenario: Complete pipeline graph is accepted
- GIVEN a graph links exact package/WIT inputs through build, composition, virtualization, validation, optional transform/precompile, and execution receipts with allowed edges
- WHEN graph validation runs
- THEN Valence MUST preserve the complete linkage and deterministic summary.

#### Scenario: Stage edge skips required parent
- GIVEN an execution or precompile node lacks the exact source component/configuration parent required by its edge type
- WHEN validation runs
- THEN Valence MUST fail closed with a deterministic missing-parent diagnostic.

### Requirement: Verification roles remain bounded

r[valence.wasm_component.roles] Static findings, build records, package metadata, composition/virtualization reports, benchmark results, and execution records MUST default to `RecordedOnly`; admission/gate decisions MAY use `Boundary`; no artifact MAY use `Property` solely because its producer reports success.

#### Scenario: Successful execution requests property role
- GIVEN a runtime receipt records successful component execution but no separately checked property evidence
- WHEN role validation runs
- THEN Valence MUST reject `Property` and retain only the supported bounded role.

### Requirement: Component evidence preserves explicit non-claims

r[valence.wasm_component.nonclaims] Component evidence MUST preserve non-claims for behavioral correctness, semantic equivalence, package trust, sandbox effectiveness, authority, compiler/runtime correctness, cross-target portability of native artifacts, requirement satisfaction, and release eligibility.

#### Scenario: Non-claim boundary is weakened
- GIVEN a derived artifact omits or weakens a required non-claim inherited from its parent profile
- WHEN Valence validates the graph
- THEN validation MUST fail closed.

### Requirement: Wasm component evidence has a functional core

r[valence.wasm_component.functional_core] DTO validation, canonicalization, BLAKE3 material construction, external-digest role checks, interface normalization, edge validation, graph queries, verification-role admission, and non-claim enforcement MUST be pure deterministic logic.

#### Scenario: Identical evidence facts produce identical graph
- GIVEN identical component DTOs, byte measurements, profiles, and edges
- WHEN the core validates and canonicalizes them
- THEN it MUST produce the same identities, diagnostics, and graph without filesystem, environment, process, clock, network, runtime, or output effects.

### Requirement: Positive and negative fixtures cover component evidence

r[valence.wasm_component.fixtures] Valence MUST include positive pipeline graphs and negative digest-role, interface-equivalence, missing-parent, wrong-edge, cycle, property-role, cross-target, external-digest, and non-claim fixtures.

#### Scenario: Fixture suite runs
- GIVEN component evidence fixtures are evaluated
- WHEN validation completes
- THEN valid bounded graphs MUST pass and every malformed, stale, role-confused, or overclaiming fixture MUST fail deterministically.

### Requirement: Component evidence validation is reviewable

r[valence.wasm_component.validation] Changes to the component profile MUST run focused pure-core, CLI, contract, graph, role, and non-claim tests plus positive/negative fixtures, core purity smoke, repository quality checks, and Cairn lifecycle gates.

#### Scenario: Profile change is reviewed
- GIVEN schema, role, identity, edge, contract, fixture, or report behavior changes
- WHEN validation evidence is assembled
- THEN it MUST identify the exact profile version and include both positive and negative results without invoking component execution as a Valence authority.
