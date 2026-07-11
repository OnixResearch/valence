# Evidence IR Specification

## Purpose

Ensures every accepted Evidence IR envelope control field is recognized and every open payload value is bound to feature-independent canonical identity.

## Requirements

### Requirement: Evidence IR envelope decoding is strict
r[valence.evidence_ir.complete_envelope.strict_decode] Valence MUST decode the closed Evidence IR envelope and closed nested control records without silently discarding unknown, duplicate, misspelled, or misplaced fields.

#### Scenario: Current closed envelope passes
r[valence.evidence_ir.complete_envelope.fixtures.positive]
- GIVEN a current Evidence IR artifact contains exactly the supported outer fields and a valid closed source-span record
- WHEN strict decoding runs
- THEN Valence MUST return the typed artifact for ordinary validation.

#### Scenario: Kind-specific payload remains open and hash-bound
- GIVEN a producer adds a new field beneath the kind-specific `payload` object
- WHEN the artifact is decoded and its canonical identity is computed
- THEN Valence MAY accept the payload field and MUST include its complete value in canonical hash material.

### Requirement: Unrecognized control fields fail closed
r[valence.evidence_ir.complete_envelope.unknown_fields] Valence MUST reject unrecognized or duplicate fields in the outer artifact envelope and other schema-closed control records before hash comparison or claim-policy evaluation.

#### Scenario: Unknown top-level claim cannot reuse an old hash
r[valence.evidence_ir.complete_envelope.fixtures.negative]
- GIVEN a valid artifact is modified by adding an unknown top-level claim-bearing field while retaining its prior canonical hash
- WHEN `check_evidence_artifact` runs
- THEN validation MUST fail with an unknown-artifact-field diagnostic and MUST NOT report the envelope as valid.

#### Scenario: Unknown display metadata is not silently dropped
- GIVEN a source-span object includes an unsupported control field
- WHEN strict nested decoding runs
- THEN validation MUST reject that field even though supported source-span display fields remain excluded from artifact identity.

#### Scenario: Duplicate control field is rejected
- GIVEN serialized input repeats a closed envelope field with conflicting values
- WHEN decoding runs
- THEN Valence MUST fail with a duplicate-field or malformed-envelope diagnostic and MUST NOT select one value.

### Requirement: Canonical artifact bytes are feature-independent
r[valence.evidence_ir.complete_envelope.canonical_hash] Valence MUST derive artifact and stack-owned JSON identity bytes through explicit recursive object-key ordering that does not depend on optional `serde_json` map-order features.

#### Scenario: Object insertion order does not alter identity
- GIVEN equivalent payload objects with different insertion orders
- WHEN canonical bytes and BLAKE3 identity are computed
- THEN the canonical bytes and identity MUST match.

#### Scenario: Downstream feature unification does not alter identity
r[valence.evidence_ir.complete_envelope.feature_matrix]
- GIVEN a downstream consumer enables `serde_json/preserve_order`
- WHEN it computes the same Valence artifact and policy identities as the ordinary workspace build
- THEN every golden canonical byte sequence and BLAKE3 identity MUST remain unchanged.

### Requirement: Strict decoding emits bounded diagnostics
r[valence.evidence_ir.complete_envelope.diagnostics] Valence MUST distinguish malformed JSON, unknown fields, duplicate fields, typed field mismatches, and canonical hash mismatches with deterministic field-scoped issue codes.

#### Scenario: Diagnostic does not echo untrusted claim text
- GIVEN an unknown field contains attacker-controlled text
- WHEN the checker rejects it
- THEN the report MUST identify the field path and issue class without reproducing the field value as trusted output.

### Requirement: Existing valid identities remain compatible
r[valence.evidence_ir.complete_envelope.compatibility] Valence MUST preserve current accepted known-field artifact identities unless a named versioned hash profile and explicit migration are introduced.

#### Scenario: Current artifact goldens retain their hashes
- GIVEN representative accepted Evidence IR artifacts produced before this remediation
- WHEN strict decoding and explicit canonicalization run
- THEN their canonical BLAKE3 identities MUST equal the checked-in compatibility goldens.

### Requirement: Envelope and claim boundaries are documented
r[valence.evidence_ir.complete_envelope.docs] Valence documentation MUST distinguish closed envelope control fields, open hash-bound payload values, and typed claim-policy authority.

#### Scenario: Unknown prose cannot become authority
- GIVEN an operator reads the Evidence IR acceptance contract
- WHEN an unrecognized outer field is encountered
- THEN the documentation MUST state that the field is rejected rather than ignored or interpreted as claim authority.

### Requirement: Complete-envelope remediation is verified
r[valence.evidence_ir.complete_envelope.final_validation] The change MUST include positive compatibility evidence and negative exploit regressions for strict decoding and feature-independent identity.

#### Scenario: Exploit regression fails closed
- GIVEN the previously accepted unknown top-level overclaim fixture
- WHEN focused tests and repository validation run
- THEN the fixture MUST fail while current known-field artifacts and open hash-bound payload extensions pass.
