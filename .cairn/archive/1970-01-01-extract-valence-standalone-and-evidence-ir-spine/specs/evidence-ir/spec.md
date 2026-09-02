## ADDED Requirements

### Requirement: Evidence IR v1 canonical form
r[valence.evidence_ir.canonical_form] The system MUST provide a canonical, serializable `EvidenceArtifact` type carrying `schema`, `artifact_kind`, `stable_id`, `canonical_hash`, `source_span`, `verification_role`, `payload`, and `non_claims` fields.

#### Scenario: Canonical form round-trips through serialization
r[valence.evidence_ir.canonical_form.round_trip]
- GIVEN a built EvidenceArtifact
- WHEN it is serialized to JSON and deserialized
- THEN all fields are preserved

#### Scenario: Canonical hash is stable for same core inputs
r[valence.evidence_ir.canonical_form.hash_stable]
- GIVEN two artifacts built from identical core inputs
- WHEN their canonical hashes are compared
- THEN they are equal

#### Scenario: Canonical hash excludes display fields
r[valence.evidence_ir.canonical_form.hash_excludes_display]
- GIVEN an artifact with a source span
- WHEN the source span path is changed
- THEN the canonical hash is unchanged

### Requirement: Evidence span provenance threading
r[valence.evidence_ir.span_threading] The system MUST provide an `EvidenceSpan` type with a `source_id` field that is the stable identifier threading source to receipt to binary.

#### Scenario: Explain resolves a known source ID
r[valence.evidence_ir.span_threading.explain_known]
- GIVEN a spans manifest containing an ID
- WHEN `explain_span` is called with that ID
- THEN the corresponding span is returned

#### Scenario: Explain returns none for unknown ID
r[valence.evidence_ir.span_threading.explain_unknown]
- GIVEN a spans manifest
- WHEN `explain_span` is called with an absent ID
- THEN None is returned

### Requirement: Verification role classification
r[valence.evidence_ir.verification_role] The system MUST classify every artifact with a `VerificationRole` of `Property`, `RecordedOnly`, or `Boundary`.

#### Scenario: Roles serialize to snake_case
r[valence.evidence_ir.verification_role.serialize]
- GIVEN each VerificationRole variant
- WHEN serialized to JSON
- THEN it appears as `"property"`, `"recorded_only"`, or `"boundary"`

### Requirement: Non-claim boundary enforcement
r[valence.evidence_ir.non_claim_boundary] The system MUST reject artifacts with missing or weakened non-claim boundaries, and MUST reject overclaim markers in payload or non_claims.

#### Scenario: Checker fails closed for missing non_claims
r[valence.evidence_ir.non_claim_boundary.missing]
- GIVEN an artifact with empty non_claims
- WHEN checked
- THEN the report is invalid with `non_claims_missing`

#### Scenario: Checker fails closed for weakened non_claims
r[valence.evidence_ir.non_claim_boundary.weakened]
- GIVEN an artifact whose non_claims omit the required boundary text
- WHEN checked
- THEN the report is invalid with `non_claims_weakened`

#### Scenario: Checker rejects overclaim in payload
r[valence.evidence_ir.non_claim_boundary.overclaim_payload]
- GIVEN an artifact whose payload contains "proves semantic equivalence"
- WHEN checked
- THEN the report is invalid with `overclaim_detected`

#### Scenario: Checker rejects overclaim in non_claims
r[valence.evidence_ir.non_claim_boundary.overclaim_non_claims]
- GIVEN an artifact whose non_claims contains "proves whole-program correctness"
- WHEN checked
- THEN the report is invalid with `overclaim_detected`

### Requirement: CLI shell for Evidence IR
r[valence.evidence_ir.cli] The system MUST expose `valence check <FILE>`, `valence explain --id <ID> --manifest <FILE>`, and `valence version` commands with stable exit codes 0, 1, and 2.

#### Scenario: Check accepts valid artifact
r[valence.evidence_ir.cli.check_valid]
- GIVEN a library-emitted valid artifact written to a file
- WHEN `valence check <FILE>` is run
- THEN it exits 0 and reports `valid: true`

#### Scenario: Check rejects overclaim artifact
r[valence.evidence_ir.cli.check_overclaim]
- GIVEN an artifact with an overclaim written to a file
- WHEN `valence check <FILE>` is run
- THEN it exits 1 and reports `overclaim_detected`

#### Scenario: Explain resolves known ID
r[valence.evidence_ir.cli.explain_known]
- GIVEN a spans manifest file containing an ID
- WHEN `valence explain --id <ID> --manifest <FILE>` is run
- THEN it exits 0 and prints the span details

#### Scenario: Explain reports not_found for unknown ID
r[valence.evidence_ir.cli.explain_unknown]
- GIVEN a spans manifest file
- WHEN `valence explain --id <unknown> --manifest <FILE>` is run
- THEN it exits 1 and prints `not_found`

### Requirement: Stable v1 contract preservation
r[valence.evidence_ir.v1_contract_preservation] The system MUST preserve the byte-identical stable v1 contract strings from octet's valence-core for `function_object`, `evidence_codebase_index`, and `evidence_index` modules.

#### Scenario: Function Object v1 schema string preserved
r[valence.evidence_ir.v1_contract_preservation.function_object_schema]
- GIVEN the function_object module
- WHEN `FUNCTION_OBJECT_SCHEMA` is read
- THEN it equals `octet.function-object.v1`

#### Scenario: Evidence codebase index schema preserved
r[valence.evidence_ir.v1_contract_preservation.index_schema]
- GIVEN the evidence_codebase_index module
- WHEN `INDEX_SCHEMA_VERSION` is read
- THEN it equals `octet-evidence-codebase-index/v1`
