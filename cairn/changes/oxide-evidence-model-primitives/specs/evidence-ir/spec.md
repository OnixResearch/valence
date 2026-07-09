# Evidence IR Specification

## Purpose

Adds an Oxide-reference intake path for typed evidence identifiers, derived-key evidence collections, semantic evidence diffs, and bounded attestation vocabulary.

## Requirements

### Requirement: Oxide primitive reference inventory
r[valence.evidence_ir.oxide_primitives.reference_inventory] Valence MUST maintain a reference-only inventory for each Oxide primitive or attestation repository used to shape Evidence IR primitives.

#### Scenario: Reference intake records boundaries
r[valence.evidence_ir.oxide_primitives.reference_inventory.boundaries]
- GIVEN an Oxide repository is evaluated for Valence adoption
- WHEN the intake record is written
- THEN it MUST name the source repository, license posture, MSRV/no-std posture when relevant, intended adaptation, and explicit non-adoption boundaries.

#### Scenario: Reference intake rejects hidden authority
r[valence.evidence_ir.oxide_primitives.reference_inventory.reject_hidden_authority]
- GIVEN an external repository has a useful implementation pattern
- WHEN Valence records it as a reference
- THEN the record MUST NOT treat that repository as proof of Evidence IR correctness, release readiness, or attestation truth.

### Requirement: Typed evidence identifiers
r[valence.evidence_ir.oxide_primitives.typed_ids] Valence MUST distinguish stack-owned BLAKE3 evidence identities from UUID-compatible external identifiers through typed wrappers or equivalent type-level roles.

#### Scenario: BLAKE3 identity remains canonical
r[valence.evidence_ir.oxide_primitives.typed_ids.blake3_canonical]
- GIVEN a Valence artifact has both a stack-owned canonical identity and an external UUID-compatible identifier
- WHEN identity validation runs
- THEN the canonical evidence identity MUST remain the BLAKE3-bound value and the UUID-compatible identifier MUST remain external metadata.

#### Scenario: Role mismatch fails closed
r[valence.evidence_ir.oxide_primitives.typed_ids.role_mismatch]
- GIVEN an identifier for one evidence role is supplied where another role is required
- WHEN typed validation compares the identifiers
- THEN validation MUST fail closed with deterministic diagnostics.

### Requirement: Derived-key evidence collections
r[valence.evidence_ir.oxide_primitives.identity_maps] Valence SHOULD support in-memory evidence collections whose lookup keys are derived from values while rejecting duplicate or inconsistent keys.

#### Scenario: Duplicate derived key is rejected
r[valence.evidence_ir.oxide_primitives.identity_maps.duplicate_rejection]
- GIVEN two evidence records derive the same canonical key
- WHEN the collection is constructed
- THEN construction MUST reject the duplicate instead of silently overwriting either record.

#### Scenario: Borrowed and owned lookup forms agree
r[valence.evidence_ir.oxide_primitives.identity_maps.lookup_consistency]
- GIVEN a collection supports borrowed and owned lookup forms for the same key profile
- WHEN both lookup forms target the same evidence record
- THEN both lookups SHOULD resolve to the same record or both fail with the same deterministic issue code.

### Requirement: Semantic evidence diffs
r[valence.evidence_ir.oxide_primitives.semantic_diffs] Valence SHOULD provide semantic diff reports for reviewable Evidence IR migrations without claiming semantic equivalence.

#### Scenario: Structural diff is review evidence
r[valence.evidence_ir.oxide_primitives.semantic_diffs.review_only]
- GIVEN two Evidence IR values differ structurally
- WHEN Valence emits a semantic diff report
- THEN the report SHOULD identify added, removed, and changed typed fields and MUST state that it is review evidence only.

#### Scenario: Stale baseline is rejected
r[valence.evidence_ir.oxide_primitives.semantic_diffs.stale_baseline]
- GIVEN a stored diff baseline no longer matches the current before-value identity
- WHEN a diff refresh is requested
- THEN Valence MUST reject the refresh until the caller supplies the current baseline identity.

### Requirement: Attestation vocabulary mapping
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary] Valence MAY map attestation vocabulary from CoRIM, VM attestation, DICE, or certificate-chain references into bounded Evidence IR metadata.

#### Scenario: Attestation metadata has explicit scope
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary.scope]
- GIVEN an attestation-shaped artifact is imported
- WHEN Valence normalizes its metadata
- THEN the normalized record MUST include issuer, subject, scope, digest binding, and non-claim boundary fields.

#### Scenario: Missing boundary fails closed
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary.missing_boundary]
- GIVEN imported attestation metadata omits scope or non-claim fields
- WHEN Valence validates the record
- THEN validation MUST fail closed and MUST NOT infer trust from repository origin.

### Requirement: Fixture-backed adoption
r[valence.evidence_ir.oxide_primitives.validation] Valence MUST require positive and negative fixture coverage before adopting any Oxide-inspired primitive into maintained core behavior.

#### Scenario: Positive fixtures cover accepted behavior
r[valence.evidence_ir.oxide_primitives.validation.positive]
- GIVEN typed IDs, derived-key collections, semantic diffs, and attestation metadata are accepted
- WHEN the fixture matrix runs
- THEN valid examples MUST pass with stable BLAKE3-bound identities and deterministic reports.

#### Scenario: Negative fixtures cover failure behavior
r[valence.evidence_ir.oxide_primitives.validation.negative]
- GIVEN malformed IDs, duplicate keys, stale diffs, missing issuers, weakened non-claims, or overclaim text are supplied
- WHEN the fixture matrix runs
- THEN invalid examples MUST fail with deterministic diagnostics and no partial acceptance.
