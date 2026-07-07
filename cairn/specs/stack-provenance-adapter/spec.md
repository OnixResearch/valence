# Stack Provenance Adapter Specification

## Purpose

Defines the `stack-provenance-adapter` capability.

## Requirements

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

### Requirement: Built binary provenance thread
r[valence.stack_provenance_adapter.built_binary_thread] Valence MUST provide a verification path that accepts a built binary, an Octet Valence provenance export, a Valence stack sidecar, and Mantle release evidence metadata, then reports success only when the measured binary bytes are linked through the sidecar and release metadata.

#### Scenario: Measured binary digest is bound to the sidecar
r[valence.stack_provenance_adapter.built_binary_thread.binary_digest]
- GIVEN a local built binary and a candidate Valence stack provenance sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST compute the BLAKE3 digest of the binary bytes and require it to match the sidecar `binary_hash` and every provenance link `binary_hash`.

#### Scenario: Mantle release binary identity is required
r[valence.stack_provenance_adapter.built_binary_thread.mantle_binary_identity]
- GIVEN a Mantle release manifest or release evidence manifest view and a measured binary digest
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST require a Mantle release binary identity or explicitly scoped release artifact identity that matches the measured digest.

#### Scenario: Octet export remains verified evidence facts
r[valence.stack_provenance_adapter.built_binary_thread.octet_export]
- GIVEN an Octet Valence provenance export used to build or verify the sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST require the export to have valid verification status, no diagnostics, matching parent artifact identities, and evidence tiers that are not promoted beyond the Octet export.

#### Scenario: Full thread reaches the built binary
r[valence.stack_provenance_adapter.built_binary_thread.e2e]
- GIVEN a built binary, a verified Octet provenance export, a Valence sidecar, and Mantle external evidence metadata for that sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN the report MUST show source/provenance/artifact/binary linkage through the measured binary digest without claiming behavioral correctness.

#### Scenario: Stale binary bytes fail closed
r[valence.stack_provenance_adapter.built_binary_thread.stale_binary]
- GIVEN a valid sidecar and Mantle manifest for one binary digest
- WHEN the operator verifies different binary bytes against that sidecar and manifest
- THEN verification MUST fail with a diagnostic naming the stale or mismatched binary digest.

#### Scenario: Missing Mantle binary identity fails closed
r[valence.stack_provenance_adapter.built_binary_thread.missing_mantle_binary]
- GIVEN a sidecar whose digest matches Mantle external evidence metadata but no Mantle binary identity matches the measured binary
- WHEN Valence verifies the built-binary provenance thread
- THEN verification MUST fail with a diagnostic naming the missing Mantle binary identity.

#### Scenario: Sidecar digest remains mandatory
r[valence.stack_provenance_adapter.built_binary_thread.sidecar_digest]
- GIVEN a built binary and sidecar whose binary hash matches the measured bytes
- WHEN the sidecar bytes do not match the BLAKE3 digest recorded by Mantle external evidence metadata
- THEN verification MUST fail with a diagnostic naming the sidecar digest mismatch.

#### Scenario: Built-binary thread does not broaden the claim
r[valence.stack_provenance_adapter.built_binary_thread.non_claims]
- GIVEN a successful built-binary thread verification report
- WHEN an operator reads the supported claim
- THEN the report MUST state that the check proves measured-byte identity and linkage only and does not prove behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, verifier soundness, or whole-program safety.

### Requirement: Built binary provenance thread CLI
r[valence.stack_provenance_adapter.built_binary_thread.cli] Valence SHOULD expose a CLI command for verifying a built binary against the Octet export, Valence sidecar, and Mantle release metadata.

#### Scenario: CLI verifies a built binary thread
r[valence.stack_provenance_adapter.built_binary_thread.cli.verify]
- GIVEN paths to a built binary, Octet Valence provenance export, Valence stack sidecar, and Mantle release metadata
- WHEN the operator runs `valence stack-provenance verify-built-binary`
- THEN the command SHOULD run the pure verifier and exit successfully only when all binary, sidecar, export, and release metadata bindings match.

#### Scenario: CLI emits deterministic diagnostics
r[valence.stack_provenance_adapter.built_binary_thread.cli.diagnostics]
- GIVEN stale binary bytes, missing Mantle binary identity, sidecar digest mismatch, Octet export mismatch, or evidence-tier promotion
- WHEN the operator runs `valence stack-provenance verify-built-binary`
- THEN the command SHOULD emit stable diagnostics suitable for release automation and human review.
