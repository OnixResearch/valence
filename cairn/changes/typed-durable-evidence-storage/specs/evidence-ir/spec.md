# Evidence IR Specification

## Purpose

Extends `evidence-ir` with typed durable storage envelopes for persisted evidence values.

## Requirements

### Requirement: Typed durable evidence storage
r[valence.evidence_ir.typed_durable_storage] Valence MUST persist durable evidence values with typed storage metadata that identifies schema, value type, codec, content hash, and migration history where applicable.

#### Scenario: Durable envelope carries type and schema identity
r[valence.evidence_ir.typed_durable_storage.envelope]
- GIVEN Valence persists an evidence value through the durable storage path
- WHEN the value is encoded
- THEN the envelope MUST include schema version, value type identity, codec identity, content hash, and non-claims.

#### Scenario: Decode validates before constructing DTOs
r[valence.evidence_ir.typed_durable_storage.decode]
- GIVEN a persisted evidence value is loaded
- WHEN Valence decodes it
- THEN Valence MUST validate schema identity, value type identity, codec identity, and content hash before constructing the current DTO.

#### Scenario: Legacy values require explicit migrations
r[valence.evidence_ir.typed_durable_storage.migration]
- GIVEN a persisted value uses an older supported schema
- WHEN Valence migrates it to a current DTO
- THEN the migration SHOULD record source identity, target identity, migration tool identity, diagnostics, and non-claims.

#### Scenario: Valid envelopes and migrations pass
r[valence.evidence_ir.typed_durable_storage.tests]
- GIVEN positive durable storage fixtures
- WHEN the durable storage gate runs
- THEN valid current envelopes and supported legacy migrations MUST pass deterministically.

#### Scenario: Mismatches fail closed
r[valence.evidence_ir.typed_durable_storage.fail_closed]
- GIVEN schema, type, codec, content hash, or migration metadata is missing or mismatched
- WHEN Valence decodes the persisted value
- THEN decoding MUST fail closed with deterministic diagnostics.

#### Scenario: Durable storage boundary is documented
r[valence.evidence_ir.typed_durable_storage.docs]
- GIVEN documentation or receipts describe typed durable evidence storage
- WHEN the supported claim is stated
- THEN the output MUST state that typed storage proves decode identity and compatibility only, not evidence truth, correctness, verifier soundness, or requirement satisfaction.
