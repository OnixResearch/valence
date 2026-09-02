# Evidence IR Specification

## Purpose

Makes EvidenceSpan metadata structurally valid and span-manifest lookup unambiguous before Valence reports provenance linkage.

## Requirements

### Requirement: Evidence spans have a validated structural contract
r[valence.evidence_ir.span_integrity.contract] Valence MUST provide pure deterministic validation for `EvidenceSpan` identifiers, paths, optional labels, byte ranges, and line/column ranges.

#### Scenario: Structurally valid span passes
r[valence.evidence_ir.span_integrity.fixtures.positive]
- GIVEN a span has non-empty identifiers and path, ordered byte endpoints, nonzero one-based line and column coordinates, and an ending position not before its start
- WHEN span validation runs
- THEN validation MUST pass without filesystem or source-byte access.

#### Scenario: Valid zero-width span is explicit
- GIVEN a point span has equal byte endpoints and equal line/column endpoints
- WHEN span validation runs
- THEN Valence MAY accept it as a zero-width span and MUST preserve that shape deterministically.

### Requirement: Span coordinates fail closed
r[valence.evidence_ir.span_integrity.coordinates] Valence MUST reject spans with blank or padded required text, prohibited control characters, reversed byte endpoints, zero one-based coordinates, reversed line/column positions, or blank present labels.

#### Scenario: Impossible coordinates are rejected
r[valence.evidence_ir.span_integrity.fixtures.negative]
- GIVEN a span declares `start_byte` after `end_byte` or uses zero for a one-based line or column
- WHEN validation runs
- THEN Valence MUST return deterministic field-scoped coordinate diagnostics.

### Requirement: Span manifests are unique before lookup
r[valence.evidence_ir.span_integrity.manifest_uniqueness] Valence MUST validate every manifest row and MUST reject duplicate `source_id` values before resolving any requested ID.

#### Scenario: Duplicate ID cannot select the first span
- GIVEN a manifest contains two rows with the same `source_id` and different source locations
- WHEN explanation is requested for that ID
- THEN Valence MUST report an invalid ambiguous manifest and MUST NOT return either row based on input order.

#### Scenario: Unrelated invalid row invalidates the manifest
- GIVEN a requested ID has a valid row but another manifest row is structurally invalid
- WHEN explanation runs
- THEN Valence MUST report the manifest invalid rather than treating unchecked rows as harmless.

### Requirement: Artifact-attached spans are validated
r[valence.evidence_ir.span_integrity.artifact_validation] Valence MUST apply the span structural validator to every present `EvidenceArtifact.source_span` during construction and checking.

#### Scenario: Valid hash cannot mask invalid display span
- GIVEN an artifact's recognized hash-bearing fields match its canonical hash but its optional source span is structurally invalid
- WHEN `check_evidence_artifact` runs
- THEN the artifact MUST fail with span diagnostics even though supported span display fields are excluded from canonical identity.

### Requirement: Explain outcomes distinguish absence from invalid input
r[valence.evidence_ir.span_integrity.explain_result] Valence MUST expose typed found, not-found, and invalid-manifest outcomes with sorted deterministic diagnostics.

#### Scenario: Unknown ID in a valid manifest is not found
- GIVEN a structurally valid unique-ID manifest does not contain the requested ID
- WHEN explanation runs
- THEN Valence MUST return the not-found outcome without malformed-manifest diagnostics.

#### Scenario: Malformed manifest is not reported as not found
- GIVEN the manifest is malformed JSON or contains an invalid span
- WHEN explanation runs
- THEN Valence MUST return the invalid-manifest outcome and MUST NOT print `not_found` as the sole result.

### Requirement: Span integrity claims remain bounded
r[valence.evidence_ir.span_integrity.docs] Valence documentation MUST state that structural span validation and unique lookup do not prove that offsets match source bytes, that columns match compiler conventions, or that linked code is correct.

#### Scenario: Source-byte validation remains separate
- GIVEN no measured source bytes are supplied
- WHEN a span passes structural validation
- THEN Valence MUST NOT claim source-content correspondence or compiler provenance.

### Requirement: Span-manifest remediation is verified
r[valence.evidence_ir.span_integrity.final_validation] The change MUST include positive and negative core, artifact, and CLI evidence for structural and manifest invariants.

#### Scenario: Reproduced invalid manifest fails
- GIVEN the duplicate-ID fixture with reversed bytes and zero coordinates
- WHEN focused tests and repository validation run
- THEN Valence MUST reject the manifest without returning or printing a selected span.
