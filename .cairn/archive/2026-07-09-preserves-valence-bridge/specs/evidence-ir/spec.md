# Evidence IR Specification

## Purpose

Adds a Preserves-to-Valence bridge for Molten evidence envelopes.

## Requirements

### Requirement: Preserves bridge contract
r[valence.evidence_ir.preserves_bridge.contract] Valence MUST support an adapter contract that maps canonical Preserves evidence envelopes to Valence Evidence IR artifact identity fields.

#### Scenario: Canonical Preserves envelope maps to Valence artifact
r[valence.evidence_ir.preserves_bridge.fixtures.positive]
- GIVEN a Preserves envelope has a supported schema label, canonical byte digest, artifact role, verification role, and required non-claims
- WHEN bridge validation runs
- THEN validation MUST pass and preserve the canonical Preserves digest.

#### Scenario: Invalid Preserves envelope fails
r[valence.evidence_ir.preserves_bridge.fixtures.negative]
- GIVEN a Preserves envelope has stale digest, unsupported role, malformed schema label, or missing non-claims
- WHEN bridge validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Bridge validation boundary
r[valence.evidence_ir.preserves_bridge.validation] The bridge MUST validate identity, schema, role, and non-claim mapping only.

#### Scenario: Boundary is visible
r[valence.evidence_ir.preserves_bridge.docs]
- GIVEN a Preserves bridge report passes
- WHEN the supported claim is rendered
- THEN it MUST state that Valence does not prove Molten runtime authority, transport correctness, or Preserves application semantics.

### Requirement: Final validation
r[valence.evidence_ir.preserves_bridge.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers bridge health
r[valence.evidence_ir.preserves_bridge.final_validation.fixtures]
- GIVEN valid and invalid bridge fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
