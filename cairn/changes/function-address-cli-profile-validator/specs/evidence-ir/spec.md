# Evidence IR Specification

## Purpose

Add a Valence CLI surface for the function-address Evidence IR profile.

## Requirements

### Requirement: CLI validates function-address Evidence IR
r[valence.evidence_ir.function_address_cli.command] Valence MUST provide a CLI command that validates function-address Evidence IR inputs through the pure `function_address_profile` core.

#### Scenario: Valid Kamacite-backed evidence passes
r[valence.evidence_ir.function_address_cli.positive]
- GIVEN function-address evidence records, source spans, roles, digest domains, and Kamacite receipt identity all match
- WHEN the Valence CLI validates the input
- THEN it MUST emit a passing recorded-only Valence function-address receipt.

### Requirement: CLI emits deterministic receipt and graph outputs
r[valence.evidence_ir.function_address_cli.receipt] Valence MUST emit deterministic function-address receipt/sidecar output suitable for Mantle binding and Cairn readiness input.

#### Scenario: Graph output is stable
r[valence.evidence_ir.function_address_cli.graph]
- GIVEN the same typed function-address input is validated twice
- WHEN graph-summary output is requested
- THEN Valence MUST emit stable graph rows and BLAKE3 graph summary identity.

### Requirement: Recorded-only boundary is enforced
r[valence.evidence_ir.function_address_cli.negative] Valence MUST reject function-address evidence that uses the wrong digest domain, lacks spans, has stale Kamacite identity, declares unknown roles, contains overclaims, or promotes recorded-only evidence to property/formal-proof status.

#### Scenario: CLI preserves core boundary
r[valence.evidence_ir.function_address_cli.validation]
- GIVEN Valence CLI validation succeeds
- WHEN an operator reads the receipt
- THEN the receipt MUST state that function-address evidence records identity/linkage only and does not prove Rust semantic correctness or release eligibility.
