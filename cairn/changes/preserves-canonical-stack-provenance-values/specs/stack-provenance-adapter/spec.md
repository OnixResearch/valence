# Stack Provenance Adapter Specification

## Purpose

Extends the `stack-provenance-adapter` capability with Preserves canonical value identity for stack-provenance metadata.

## Requirements

### Requirement: Preserves canonical stack-provenance values
r[valence.stack_provenance_adapter.preserves_values] Valence MUST provide canonical Preserves value projections for stack-provenance metadata surfaces used by built-binary thread verification.

#### Scenario: Stack provenance metadata has canonical values
r[valence.stack_provenance_adapter.preserves_values.metadata_surfaces]
- GIVEN an Octet Valence provenance export, a Valence stack sidecar, a Mantle release manifest view, a built-binary thread input envelope, or a built-binary thread verification report
- WHEN Valence projects the metadata to Preserves
- THEN the projection MUST produce a deterministic canonical Preserves value for the metadata surface.

#### Scenario: Preserves content hashes are named distinctly
r[valence.stack_provenance_adapter.preserves_values.hashes]
- GIVEN a canonical Preserves value for stack-provenance metadata
- WHEN Valence computes metadata identity evidence
- THEN Valence MUST compute a BLAKE3 hash over canonical Preserves bytes and expose it as `preserves_content_hash` or an equally explicit metadata-value hash name that cannot be confused with raw binary or sidecar byte digests.

#### Scenario: Preserves does not replace raw byte identity
r[valence.stack_provenance_adapter.preserves_values.boundary]
- GIVEN stack-provenance metadata has a valid Preserves content hash
- WHEN Valence verifies the built-binary provenance thread
- THEN Rust MUST still measure raw binary bytes with BLAKE3 and Mantle external evidence MUST still bind the raw sidecar byte digest.

#### Scenario: Preserves corpus covers valid metadata
r[valence.stack_provenance_adapter.preserves_values.corpus]
- GIVEN checked-in positive corpus fixtures for Octet export metadata, sidecar metadata, Mantle release metadata, built-binary thread input metadata, and verification report metadata
- WHEN the Preserves corpus gate runs
- THEN every positive fixture MUST project to the expected canonical Preserves value and `preserves_content_hash`.

#### Scenario: Preserves corpus catches metadata drift
r[valence.stack_provenance_adapter.preserves_values.gate]
- GIVEN stale parent linkage, evidence-tier promotion, missing required non-claims, overclaim wording, or changed report diagnostics
- WHEN the Preserves corpus gate evaluates negative or drift fixtures
- THEN the gate MUST fail deterministically unless the expected corpus update is made intentionally.

#### Scenario: Verification reports can be compared semantically
r[valence.stack_provenance_adapter.preserves_values.report_diff]
- GIVEN two built-binary thread verification reports rendered through different JSON formatting or field ordering
- WHEN Valence compares their Preserves projections
- THEN equivalent report values SHOULD compare equal and changed diagnostics or identity fields SHOULD produce a deterministic Preserves diff or hash mismatch.

#### Scenario: Receipts can include Preserves metadata hashes
r[valence.stack_provenance_adapter.preserves_values.receipts]
- GIVEN release or test automation records stack-provenance metadata identity evidence
- WHEN the receipt includes Preserves evidence
- THEN the receipt SHOULD include `preserves_content_hash` values for metadata surfaces without replacing raw binary BLAKE3 digests, Mantle sidecar byte digests, or the bounded identity-and-linkage claim.

#### Scenario: Preserves boundary is documented
r[valence.stack_provenance_adapter.preserves_values.docs]
- GIVEN an operator reads stack-provenance documentation or Preserves corpus output
- WHEN the supported claim is described
- THEN the output MUST state that Preserves proves canonical metadata value identity only and does not prove measured-byte identity, behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, verifier soundness, or whole-program safety.
