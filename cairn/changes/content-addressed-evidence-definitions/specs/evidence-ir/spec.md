# Evidence IR Specification

## Purpose

Extends `evidence-ir` with content-addressed evidence-definition identities inspired by Unison's hash-identified definitions.

## Requirements

### Requirement: Content-addressed evidence definitions
r[valence.evidence_ir.content_addressed_definitions] Valence MUST identify evidence-relevant definitions by a BLAKE3 hash over a typed canonical value, not by a human-readable name or path.

#### Scenario: Canonical content defines identity
r[valence.evidence_ir.content_addressed_definitions.canonical_identity]
- GIVEN an evidence-relevant definition projected into its typed canonical value
- WHEN Valence computes its evidence-definition identity
- THEN the identity MUST be the BLAKE3 hash of that canonical value.

#### Scenario: Dependency identities are part of canonical content
r[valence.evidence_ir.content_addressed_definitions.dependencies]
- GIVEN an evidence-relevant definition depends on other evidence definitions, artifacts, verifier inputs, or requirements with known content identities
- WHEN Valence builds the canonical value
- THEN dependency references SHOULD use content identities rather than display names.

#### Scenario: Names and paths are metadata
r[valence.evidence_ir.content_addressed_definitions.metadata_boundary]
- GIVEN the display name, source path, or alias for an evidence definition changes while canonical content and dependency identities are unchanged
- WHEN Valence recomputes the evidence-definition identity
- THEN the identity MUST remain stable.

#### Scenario: Rename and path churn fixtures stay stable
r[valence.evidence_ir.content_addressed_definitions.rename_stability]
- GIVEN positive fixtures that vary display names and source paths only
- WHEN the identity gate runs
- THEN Valence MUST report the same evidence-definition identity for each fixture.

#### Scenario: Dependency changes alter identity
r[valence.evidence_ir.content_addressed_definitions.dependency_change]
- GIVEN a fixture where a dependency content identity changes
- WHEN Valence recomputes the parent evidence-definition identity
- THEN the parent identity MUST change or the gate MUST fail closed.

#### Scenario: Identity claim remains bounded
r[valence.evidence_ir.content_addressed_definitions.non_claims]
- GIVEN documentation or receipts mention content-addressed evidence-definition identity
- WHEN the supported claim is described
- THEN the output MUST state that content identity proves canonical content and dependency identity only, not behavioral correctness, semantic equivalence, verifier soundness, release eligibility, or requirement satisfaction.
