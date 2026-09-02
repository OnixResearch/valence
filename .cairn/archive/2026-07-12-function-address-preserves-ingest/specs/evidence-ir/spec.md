# Evidence IR Specification

## Purpose

Allow Valence to ingest canonical Kamacite Preserves function-address receipts directly.

## Requirements

### Requirement: Valence validates Kamacite Preserves receipt identity
r[valence.evidence_ir.function_address_preserves_ingest.core] Valence MUST validate canonical Kamacite Preserves function-address receipt identity, schema version, digest domains, spans, dependency identity kind, roles, and non-claims before producing Evidence IR rows.

#### Scenario: Valid Preserves receipt becomes recorded-only Evidence IR
r[valence.evidence_ir.function_address_preserves_ingest.positive]
- GIVEN Kamacite provides a schema-valid canonical Preserves function-address receipt
- WHEN Valence validates the receipt
- THEN Valence MUST emit recorded-only function-address Evidence IR and graph rows bound to the Kamacite Preserves receipt hash.

### Requirement: CLI shell owns Preserves file reads
r[valence.evidence_ir.function_address_preserves_ingest.cli] Valence CLI MUST own file reads and Preserves parsing, while `valence-core` receives in-memory receipt values and canonical bytes.

#### Scenario: Core purity is preserved
r[valence.evidence_ir.function_address_preserves_ingest.validation]
- GIVEN Valence ingests a function-address Preserves receipt
- WHEN core validation runs
- THEN `valence-core` MUST NOT read files, inspect environment state, spawn processes, or print output.

### Requirement: JSON projection ingest is bound and non-authoritative
r[valence.evidence_ir.function_address_preserves_ingest.json_projection] Valence MAY ingest JSON compatibility projections only when they are bound to the canonical Kamacite Preserves receipt hash.

#### Scenario: Projection drift is rejected
r[valence.evidence_ir.function_address_preserves_ingest.negative]
- GIVEN a JSON projection differs from the canonical Preserves receipt, omits the receipt hash, substitutes a digest domain, promotes verification role, or weakens non-claims
- WHEN Valence validation runs
- THEN validation MUST fail with deterministic diagnostics.
