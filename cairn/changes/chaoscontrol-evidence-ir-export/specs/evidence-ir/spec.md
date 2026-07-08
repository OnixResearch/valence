# Evidence IR Specification

## Purpose

Adds ChaosControl evidence export adapters for Valence Evidence IR.

## Requirements

### Requirement: ChaosControl Evidence IR export contract
r[valence.evidence_ir.chaoscontrol_export.contract] Valence MUST support an adapter contract for selected ChaosControl evidence families without taking ownership of VMM, replay, or determinism semantics.

#### Scenario: Supported ChaosControl export passes
r[valence.evidence_ir.chaoscontrol_export.fixtures.positive]
- GIVEN an export row maps a ChaosControl replay verdict, dogfood receipt, assertion summary, or readiness report to a Valence Evidence IR artifact with schema, stable ID, canonical hash, verification role, payload digest, and required non-claims
- WHEN adapter validation runs
- THEN the row MUST pass and preserve the ChaosControl artifact identity.

#### Scenario: Invalid ChaosControl export fails
r[valence.evidence_ir.chaoscontrol_export.fixtures.negative]
- GIVEN an export row has malformed hash, unsupported schema, missing non-claims, or attempts to promote recorded evidence to property evidence
- WHEN adapter validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Adapter validation boundary
r[valence.evidence_ir.chaoscontrol_export.validation] Valence MUST validate identity, role, schema, and non-claim boundaries for ChaosControl exports without claiming replay or VMM correctness.

#### Scenario: Boundary is visible
r[valence.evidence_ir.chaoscontrol_export.docs]
- GIVEN a ChaosControl export validates
- WHEN a report renders the supported claim
- THEN it MUST state that Valence proves evidence identity/linkage only and does not prove universal determinism, hosted-product parity, replay completeness, or VMM correctness.

### Requirement: Final validation
r[valence.evidence_ir.chaoscontrol_export.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers export health
r[valence.evidence_ir.chaoscontrol_export.final_validation.fixtures]
- GIVEN valid and invalid ChaosControl export fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
