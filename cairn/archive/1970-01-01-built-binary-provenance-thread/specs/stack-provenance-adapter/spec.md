# Stack Provenance Adapter Specification

## Purpose

Extends the `stack-provenance-adapter` capability with an end-to-end built-binary thread verifier.

## Requirements

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
