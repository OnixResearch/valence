# Provenance Specification

## Purpose

Defines the `provenance` capability.

## Requirements

### Requirement: Provenance link connects source to binary
r[valence.provenance] The system MUST provide a `ProvenanceLink` type connecting `source_id`, `function_object_id`, `requirement_ids`, `artifact_hash`, and `binary_hash` so the chain threads source to binary.

#### Scenario: Link carries all five identifiers
r[valence.provenance.link_fields]
- GIVEN a constructed ProvenanceLink
- WHEN its fields are inspected
- THEN it carries source_id, function_object_id, requirement_ids, artifact_hash, and binary_hash

### Requirement: Provenance chain resolves in three directions
r[valence.provenance.resolution] The system MUST provide `resolve` (source ID to link), `resolve_requirement` (requirement ID to function objects), and `trace_binary` (binary hash to source functions).

#### Scenario: Resolve source ID
r[valence.provenance.resolution.source]
- GIVEN a chain with a link carrying source_id "eq:bs_001"
- WHEN `resolve("eq:bs_001")` is called
- THEN the matching link is returned

#### Scenario: Resolve requirement to function objects
r[valence.provenance.resolution.requirement]
- GIVEN a chain with a link carrying requirement_id "r[cairn.pricing]"
- WHEN `resolve_requirement("r[cairn.pricing]")` is called
- THEN the matching link(s) are returned

#### Scenario: Trace binary to source functions
r[valence.provenance.resolution.binary]
- GIVEN a chain with links carrying binary_hash "b3:binary_001"
- WHEN `trace_binary("b3:binary_001")` is called
- THEN all matching links are returned

### Requirement: Provenance chain checker fails closed
r[valence.provenance.checker] The system MUST provide `check_provenance_chain` that fails closed on schema mismatch, weakened boundary, empty source_id, empty function_object_id, and duplicate source_ids.

#### Scenario: Check passes valid chain
r[valence.provenance.checker.valid]
- GIVEN a valid ProvenanceChain
- WHEN checked
- THEN the report is valid with zero issues

#### Scenario: Check fails closed for duplicate source_id
r[valence.provenance.checker.duplicate]
- GIVEN a chain with two links sharing the same source_id
- WHEN checked
- THEN the report is invalid with `duplicate_source_id`

### Requirement: Provenance non-claim boundary
r[valence.provenance.non_claim] The system MUST carry a non-claim boundary stating that provenance proves identity and linkage only, not behavioral correctness or that the binary satisfies the requirement.

#### Scenario: Boundary present in chain
r[valence.provenance.non_claim.present]
- GIVEN a ProvenanceChain
- WHEN inspected
- THEN it carries a boundary field stating "proves identity and linkage only"


### Requirement: Canonical Valence core manifest
r[valence.provenance.canonical_core_manifest] Valence MUST publish a canonical manifest for Valence-owned `valence-core` modules, schema families, fixture roots, compatibility dispositions, and required non-claim boundaries.

#### Scenario: Canonical rows pass
r[valence.provenance.canonical_core_manifest.positive]
- GIVEN a manifest row names a Valence-owned module or schema family with a supported compatibility disposition and required non-claim text
- WHEN the manifest validator evaluates the row
- THEN the row MUST pass and preserve the canonical Valence path and schema identity.

#### Scenario: Unsupported rows fail closed
r[valence.provenance.canonical_core_manifest.negative]
- GIVEN a manifest row omits the canonical Valence path, weakens required non-claims, or uses an unsupported compatibility disposition
- WHEN the manifest validator evaluates the row
- THEN validation MUST fail with deterministic diagnostics naming the invalid field.

### Requirement: Canonical core drift gate
r[valence.provenance.canonical_core_drift] Valence MUST provide a deterministic drift gate that compares declared legacy or consumer-visible `valence-core` rows against the canonical Valence-owned module/schema identity.

#### Scenario: Matching legacy copy is accepted
r[valence.provenance.canonical_core_drift.match]
- GIVEN a legacy Octet-hosted row declares byte or schema identity that matches the canonical Valence manifest
- WHEN the drift gate runs
- THEN it MUST report the row as compatible without changing the evidence tier.

#### Scenario: Stale legacy copy is rejected
r[valence.provenance.canonical_core_drift.stale]
- GIVEN a legacy Octet-hosted row diverges from canonical Valence identity without a bounded migration exception
- WHEN the drift gate runs
- THEN it MUST fail closed with a stale-copy diagnostic.

### Requirement: Dependency direction remains explicit
r[valence.provenance.canonical_core_dependency_direction] Valence documentation and manifests MUST state that Valence owns Evidence IR semantics while Octet owns Octet-specific scan, lint, proof-rail, and publication orchestration.

#### Scenario: Consumer guidance points to Valence
r[valence.provenance.canonical_core_dependency_direction.consumer]
- GIVEN a downstream consumer wants `valence-core` semantics
- WHEN they read Valence or Octet integration guidance
- THEN the guidance MUST point at standalone Valence as the canonical semantic dependency.

#### Scenario: Cutover keeps non-claims visible
r[valence.provenance.canonical_core_dependency_direction.non_claims]
- GIVEN a cutover report or compatibility receipt is emitted
- WHEN a reviewer inspects the supported claim
- THEN it MUST state that the cutover proves ownership and identity compatibility only, not behavioral correctness, semantic equivalence, release eligibility, verifier soundness, or requirement satisfaction.

### Requirement: Validation evidence
r[valence.provenance.canonical_core_validation] The cutover change MUST include focused positive and negative tests plus Cairn validation evidence before archive.

#### Scenario: Focused validation covers pass and fail paths
r[valence.provenance.canonical_core_validation.focused]
- GIVEN canonical, legacy-compatible, stale, and weakened-boundary fixtures
- WHEN focused validation runs
- THEN pass fixtures MUST pass, fail fixtures MUST fail closed, and the receipt MUST identify the manifest and input hashes.
