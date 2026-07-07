## ADDED Requirements

### Requirement: Stack provenance adapter sidecar
r[valence.stack_provenance_adapter] Valence MUST provide a stack provenance adapter sidecar that records a validated Valence provenance chain plus the source artifact identities used to assemble that chain.

#### Scenario: Adapter sidecar carries chain and source identities
r[valence.stack_provenance_adapter.sidecar]
- GIVEN normalized Octet, Trellis, Cairn, and Mantle evidence inputs
- WHEN Valence builds a stack provenance sidecar
- THEN the sidecar MUST include a `valence.provenance-chain.v1` chain, source artifact identities, claim scope, diagnostics, and explicit non-claims.

#### Scenario: Adapter sidecar is deterministic
r[valence.stack_provenance_adapter.deterministic]
- GIVEN equivalent normalized evidence inputs in any supported input order
- WHEN Valence builds a stack provenance sidecar
- THEN the sidecar SHOULD serialize deterministically with stable ordering for links, artifact identities, and diagnostics.

### Requirement: Mantle external evidence verification
r[valence.stack_provenance_adapter.mantle_external_evidence] Valence MUST verify stack provenance sidecars included in Mantle release bundles through Mantle's generic external-evidence role and digest metadata, without requiring Mantle to interpret Valence semantics.

#### Scenario: Bundled sidecar digest matches
r[valence.stack_provenance_adapter.mantle_external_evidence.digest]
- GIVEN a Mantle release bundle contains an external evidence entry with role `stack-provenance-trace`
- WHEN Valence verifies the stack provenance sidecar
- THEN Valence MUST confirm the bundled sidecar bytes match the BLAKE3 digest recorded by Mantle.

#### Scenario: Missing Mantle role fails
r[valence.stack_provenance_adapter.mantle_external_evidence.missing_role]
- GIVEN a Mantle release bundle has no external evidence entry for the requested stack provenance role
- WHEN Valence verifies stack provenance for that release
- THEN verification MUST fail with a diagnostic naming the missing role.

### Requirement: Stack traceability validation
r[valence.stack_provenance_adapter.validation] Valence stack provenance verification MUST fail closed when chain identity, artifact identity, binary binding, parent linkage, or evidence-tier boundaries are missing or inconsistent.

#### Scenario: Binary hash mismatch fails
r[valence.stack_provenance_adapter.validation.binary_mismatch]
- GIVEN a provenance link names a binary hash not present in the Mantle release binary artifacts or explicitly scoped release artifacts
- WHEN Valence verifies the stack provenance sidecar
- THEN verification MUST fail with a diagnostic naming the binary hash mismatch.

#### Scenario: Parent artifact missing fails
r[valence.stack_provenance_adapter.validation.missing_parent]
- GIVEN a provenance link or proof-card-derived record names a parent artifact identity that is absent from the adapter inputs
- WHEN Valence verifies the stack provenance sidecar
- THEN verification MUST fail with a diagnostic naming the missing parent artifact.

#### Scenario: Manual-review evidence is not promoted
r[valence.stack_provenance_adapter.validation.no_tier_promotion]
- GIVEN imported Trellis or Octet evidence is marked `manual_review_required`
- WHEN Valence verifies a sidecar that claims the evidence as `formal_proof`
- THEN verification MUST fail with a diagnostic naming the evidence-tier promotion.

#### Scenario: Boundary weakening fails
r[valence.stack_provenance_adapter.validation.boundary]
- GIVEN a stack provenance sidecar weakens the required identity-and-linkage-only non-claim boundary
- WHEN Valence verifies the sidecar
- THEN verification MUST fail with a diagnostic naming the weakened boundary.

### Requirement: Stack provenance adapter CLI
r[valence.stack_provenance_adapter.cli] Valence SHOULD provide CLI commands for building stack provenance sidecars from local artifacts and verifying sidecars bundled in Mantle release evidence by external-evidence role.

#### Scenario: Build command writes sidecar
r[valence.stack_provenance_adapter.cli.build]
- GIVEN local Octet, Trellis, Cairn, and Mantle evidence inputs
- WHEN the operator runs the stack provenance build command with an output path
- THEN Valence SHOULD write the canonical stack provenance sidecar or fail with deterministic diagnostics.

#### Scenario: Verify command checks Mantle-bundled sidecar
r[valence.stack_provenance_adapter.cli.verify]
- GIVEN a Mantle release bundle contains a stack provenance external evidence sidecar
- WHEN the operator runs the stack provenance verify command for that role
- THEN Valence SHOULD verify the sidecar digest, schema, boundary, and stack traceability links.

### Requirement: Adapter boundary documentation
r[valence.stack_provenance_adapter.boundary] Valence MUST document that stack provenance proves identity and linkage only and that Mantle treats the sidecar as opaque external evidence.

#### Scenario: Boundary is visible to release operators
r[valence.stack_provenance_adapter.boundary.operator]
- GIVEN a release operator inspects stack provenance adapter documentation or verification output
- WHEN the supported claim is described
- THEN the output MUST state that the adapter does not prove behavioral correctness, semantic equivalence, whole-program safety, verifier soundness, or release eligibility by itself.
