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

### Requirement: Aeneas translation profile is typed

r[valence.provenance.aeneas_translation_chain.profile] Valence MUST define a versioned `aeneas-lean` formal-proof chain profile with closed typed nodes for checked Rust source, effective Charon extraction contract, Charon extraction, LLBC, Aeneas translation, `translation.json`, generated Lean, external models/assumptions/axioms/opaque definitions, maintained theorem source, theorem approval/adequacy/trust closure, Lean package/toolchain lock, Lean checker result, Octet receipt, proof card, and provenance projection.

#### Scenario: Complete profile input parses

- GIVEN a chain input contains every required typed node with supported schema, role, status, identity, and non-claim fields
- WHEN profile decoding runs
- THEN Valence MUST preserve the provider-specific node roles for linkage validation.

#### Scenario: Unknown or incomplete profile fails

- GIVEN a chain uses an unsupported provider profile, unknown stage role, missing required node, duplicate stable ID, or weakened closed field set
- WHEN profile decoding or validation runs
- THEN Valence MUST fail with deterministic field-scoped diagnostics.

### Requirement: Effective Charon contract is linked and admissible

r[valence.provenance.aeneas_translation_chain.charon_contract] Valence MUST require the frozen Octet input to bind reviewed effective Charon options, `has_errors = false`, `skip_borrowck = false`, exact target triple/pointer width/endianness, MIR/drop policy, selection/opacity/transformation settings, serialization format, and extraction-profile identity to the measured LLBC artifact.

#### Scenario: Reviewed extraction observation passes

- GIVEN the extraction contract is complete, non-errorful, borrow-check preserving, profile-matching, and linked to the measured LLBC
- WHEN pure chain validation runs
- THEN Valence MUST preserve it as boundary evidence and MAY continue linkage validation.

#### Scenario: Extraction observation is weakened or mismatched

- GIVEN the input reports extraction errors, skipped borrow checking, target drift, missing effective options, an unreviewed setting, or a different LLBC/profile link
- WHEN validation runs
- THEN Valence MUST fail without property-role output.

### Requirement: Translation linkage is exact

r[valence.provenance.aeneas_translation_chain.linkage] Valence MUST validate typed edges that bind the exact Rust source and reviewed extraction contract to LLBC, LLBC to `translation.json`, manifest declarations/files to generated Lean, generated Lean to models and maintained theorem source, theorem source to approval/adequacy/trust closure and the Lean checker result, and the checker result to the Octet proof receipt and proof card.

#### Scenario: Complete ordered chain passes

- GIVEN every required node is connected through supported role-compatible edges and all endpoints exist
- WHEN pure chain validation runs
- THEN Valence MUST accept the linkage independently of input ordering.

#### Scenario: Broken graph fails closed

- GIVEN the graph skips a required stage, swaps an input, duplicates an edge, contains a dangling endpoint, introduces a cycle, or uses a role-incompatible edge
- WHEN chain validation runs
- THEN validation MUST fail with deterministic node and edge diagnostics.

### Requirement: Translation manifest preserves bounded declaration linkage

r[valence.provenance.aeneas_translation_chain.translation_manifest] Valence MUST preserve measured `translation.json` declaration linkage for Rust names/spans, LLBC declarations, Lean names/files, `is_opaque`, `can_fail`, `can_diverge`, recursion, and loop-parent metadata while treating manifest `def_id` values as within-run join keys only.

#### Scenario: Frozen manifest linkage passes

- GIVEN each selected manifest row joins the frozen LLBC artifact to measured generated Lean files with valid typed edges
- WHEN manifest validation runs
- THEN Valence MUST preserve the mapping in deterministic order without promoting it to property evidence.

#### Scenario: Declaration ID is treated as durable identity

- GIVEN an input reuses a `def_id` across a different run/profile or substitutes it for a BLAKE3-bound source, model, theorem, or receipt stable ID
- WHEN validation runs
- THEN Valence MUST fail with a run-local-identity diagnostic.

### Requirement: Translation chain identity is canonical

r[valence.provenance.aeneas_translation_chain.identity] Valence MUST compute a BLAKE3 identity over canonical profile, effective extraction contract, typed node, typed edge, translation-manifest artifact, generated-file inventory, approved theorem statement, requirement, claim class, adequacy/trust-closure decision, receipt, proof-card, role-summary, and non-claim material while keeping display paths, labels, and run-local declaration numbering outside durable canonical identity.

#### Scenario: Equivalent input order has stable identity

- GIVEN equivalent nodes and edges arrive in different orders or display paths change without content identity changes
- WHEN canonical chain identity is computed
- THEN the chain identity MUST remain equal.

#### Scenario: Proof-relevant material changes identity

- GIVEN a source, extraction contract, LLBC, translation manifest, generated Lean, model/trust input, theorem, approval/adequacy decision, package-lock, checker, receipt, proof-card, role, status, or required non-claim changes
- WHEN chain identity is recomputed
- THEN the identity MUST change or validation MUST fail closed.

### Requirement: Digest roles remain distinct

r[valence.provenance.aeneas_translation_chain.digest_roles] Valence MUST preserve distinct digest domains for source, extraction profile, LLBC, translation manifest, generated model, external model/trust input, theorem, adequacy decision, package lock, checker receipt, Octet receipt, sidecar, and complete chain identities.

#### Scenario: Same bytes do not erase artifact role

- GIVEN two artifacts have equal BLAKE3 bytes but different declared digest roles
- WHEN the chain is validated
- THEN Valence MUST preserve both roles and MUST NOT substitute one artifact role for the other.

#### Scenario: Cross-domain digest substitution fails

- GIVEN syntactically valid BLAKE3 text is supplied under the wrong stage domain
- WHEN validation runs
- THEN validation MUST fail with a digest-role diagnostic.

### Requirement: Only the verified Octet theorem receipt is property evidence

r[valence.provenance.aeneas_translation_chain.role_policy] Valence MUST classify extraction, translation, manifest, generated-definition, model, assumption/axiom/opaque-definition, adequacy, trust-closure, package-lock, and raw Lean-result nodes as `recorded_only` or `boundary`, and MUST accept `property` only for a matching Octet-owned theorem receipt whose artifact verification passed and whose complete chain validates.

#### Scenario: Verified Octet theorem receipt contributes property evidence

- GIVEN the final Octet receipt is passed, artifact-verified, linked to the matching theorem and proof card, and every prior stage validates
- WHEN role counting runs
- THEN the Octet receipt MAY contribute property evidence while intermediate nodes remain non-property.

#### Scenario: Direct Lean result requests promotion

- GIVEN a generated Lean file or raw Lean checker result declares `property` without a matching verified Octet receipt
- WHEN role validation runs
- THEN Valence MUST reject the promotion and MUST NOT increase the verified count.

### Requirement: Theorem identity agrees across the chain

r[valence.provenance.aeneas_translation_chain.theorem_link] Valence MUST require the named theorem/property ID, approved statement BLAKE3, requirement ID, and claim class to agree across theorem source, approval/adequacy record, Lean checker result, Octet receipt, proof card, and provenance projection.

#### Scenario: Matching theorem linkage passes

- GIVEN every theorem-bearing node names the same scoped property and links to matching artifact identities
- WHEN theorem linkage validation runs
- THEN Valence MUST accept the theorem link.

#### Scenario: Stale or renamed theorem fails

- GIVEN any theorem-bearing node names a different, missing, stale, or ambiguous property ID, statement hash, requirement ID, or claim class
- WHEN validation runs
- THEN validation MUST fail and MUST NOT count property evidence.

### Requirement: Adequacy and trust-closure evidence remains linked, not re-decided

r[valence.provenance.aeneas_translation_chain.adequacy_link] Valence MUST validate identity, status, role, and linkage for Octet's theorem-adequacy policy/decision and declared trust closure, but MUST NOT independently claim theorem truth, requirement sufficiency, or product-property adequacy.

#### Scenario: Matching adequacy decision and closure pass

- GIVEN the Octet receipt, theorem node, adequacy decision, policy identity, and complete declared trust closure agree
- WHEN chain validation runs
- THEN Valence MUST preserve those artifacts as boundary evidence supporting the final receipt link.

#### Scenario: Adequacy or trust closure drifts

- GIVEN a vacuous statement is relabeled, an assumption/model/axiom/opaque dependency is missing, or the policy/decision identity differs from the Octet receipt
- WHEN chain validation runs
- THEN validation MUST fail without independently replacing Octet's adequacy decision.

### Requirement: Aeneas chain validation remains pure and read-only

r[valence.provenance.aeneas_translation_chain.cli] Valence MUST implement translation-chain decisions as pure functions over parsed values and MAY expose a read-only CLI shell that confines local file loading, byte measurement, and report rendering.

#### Scenario: CLI validates local frozen input

- GIVEN an operator supplies a chain input and confined artifact root
- WHEN the CLI validates the chain
- THEN it MUST measure declared local bytes, invoke the pure core, and render a deterministic report without mutating artifacts.

#### Scenario: Validation attempts tool execution

- GIVEN chain validation is requested
- WHEN the core or CLI evaluates the input
- THEN it MUST NOT execute Charon, Aeneas, Lean, Octet, Cairn, network fetches, or live sibling discovery.

### Requirement: Frozen producer fixtures are explicit

r[valence.provenance.aeneas_translation_chain.fixture] Valence MUST test the profile with a checked-in Octet adapter fixture that records its producer revision, schema, and exact BLAKE3 identity, plus positive and negative local variants.

#### Scenario: Frozen verified fixture passes

- GIVEN the checked-in fixture matches its recorded producer metadata and local artifacts
- WHEN fixture validation runs
- THEN the positive chain MUST pass without a live Octet checkout.

#### Scenario: Tampered fixture fails for the expected reason

- GIVEN a local negative variant changes one declared stage, edge, role, theorem, receipt, or digest domain
- WHEN fixture validation runs
- THEN it MUST fail with the declared deterministic issue code.

### Requirement: Generic downstream handoff is preserved

r[valence.provenance.aeneas_translation_chain.handoff] Valence MUST emit a canonical chain identity and property/recorded-only/boundary summary suitable for existing generic Cairn evidence-chain readiness and Mantle external-evidence boundaries without granting those layers proof execution authority.

#### Scenario: Valid chain emits bounded summary

- GIVEN translation-chain validation succeeds
- WHEN the handoff report is rendered
- THEN it MUST include the chain identity, role counts, Octet receipt identity, theorem ID, and non-claims in deterministic order.

#### Scenario: Invalid chain cannot become readiness input

- GIVEN translation-chain validation fails
- WHEN handoff rendering is requested
- THEN Valence MUST report the blockers and MUST NOT emit a passing property summary.

### Requirement: Translation-chain claims remain bounded

r[valence.provenance.aeneas_translation_chain.non_claims] Valence reports and documentation MUST state that passing validation proves canonical identity, exact linkage, role policy, and required non-claim conformance only.

#### Scenario: Passing report is inspected

- GIVEN an Aeneas translation-chain report is valid
- WHEN its supported claim is rendered
- THEN it MUST disclaim Rust, Charon, Aeneas, Lean, translation, external-model, theorem, adequacy-policy, requirement-selection, and verifier soundness; semantic equivalence; requirement satisfaction; whole-program correctness; lifecycle readiness; and release eligibility.

### Requirement: Translation-chain validation covers positive and negative paths

r[valence.provenance.aeneas_translation_chain.final_validation] The change MUST include a positive frozen chain plus negative cases for effective extraction settings, errorful LLBC, translation-manifest linkage, run-local `def_id` misuse, every required stage/edge/digest role, theorem and adequacy links, trust closure, role promotion, receipt link, non-claim boundary, and unsupported profile before archive.

#### Scenario: Focused validation matrix passes

- GIVEN implementation and fixtures are ready for review
- WHEN focused core, CLI, canonical-identity, workspace, Clippy, and Cairn checks run
- THEN the valid chain MUST pass and every declared malformed or overclaiming chain MUST fail for its expected reason.
