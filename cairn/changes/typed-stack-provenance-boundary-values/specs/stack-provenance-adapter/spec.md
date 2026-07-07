# Stack Provenance Adapter Specification

## Purpose

Extends the `stack-provenance-adapter` capability with typed internal boundary values for string-compatible metadata fields.

## Requirements

### Requirement: Typed stack-provenance boundary values
r[valence.stack_provenance_adapter.typed_boundary_values] Valence MUST parse high-risk stack-provenance metadata strings into typed internal boundary values before using them for verification decisions.

#### Scenario: Public DTO shape remains compatible
r[valence.stack_provenance_adapter.typed_boundary_values.dto_parse]
- GIVEN an Octet Valence provenance export, Valence stack sidecar, Mantle release manifest view, or built-binary thread report using the existing JSON string fields
- WHEN Valence parses the metadata for verification
- THEN Valence MUST convert schema, role, claim-scope, status, digest, and evidence-tier fields into typed internal values without changing the public JSON field names or serialized string values.

#### Scenario: Digest roles are explicit
r[valence.stack_provenance_adapter.typed_boundary_values.digest_roles]
- GIVEN raw binary digests, sidecar byte digests, artifact digests, and Preserves metadata hashes are all BLAKE3-shaped strings
- WHEN Valence validates or compares them
- THEN the Rust core MUST use typed wrappers or explicit role names so cross-role comparisons are intentional and reviewable.

#### Scenario: Validation uses typed values
r[valence.stack_provenance_adapter.typed_boundary_values.validation]
- GIVEN stack-provenance metadata has been parsed into typed boundary values
- WHEN Valence validates schema, external evidence role, claim scope, verification status, digest syntax, and evidence-tier policy
- THEN validation MUST fail closed with deterministic diagnostics for invalid typed values before treating the built-binary thread as valid.

#### Scenario: Evidence tiers remain conservative
r[valence.stack_provenance_adapter.typed_boundary_values.evidence_tiers]
- GIVEN an unknown evidence tier, a manual-review record, or imported Trellis/manual-review evidence promoted to `formal_proof`
- WHEN Valence parses and validates the tier
- THEN the typed model MUST preserve unknown source strings but MUST NOT count them as formal proof or allow manual-review evidence promotion.

#### Scenario: Preserves projections use typed-normalized metadata
r[valence.stack_provenance_adapter.typed_boundary_values.preserves]
- GIVEN a metadata surface with typed boundary values
- WHEN Valence projects it into canonical Preserves metadata values
- THEN the projection SHOULD use typed-normalized values where available and MUST keep corpus hash changes explicit through the Preserves corpus gate.

#### Scenario: Layer boundary is preserved
r[valence.stack_provenance_adapter.typed_boundary_values.boundary]
- GIVEN stack-provenance metadata passes typed parsing
- WHEN Valence verifies the built-binary provenance thread
- THEN Nickel MUST remain the static metadata contract layer, Preserves MUST remain the canonical metadata value layer, and Rust MUST remain responsible for raw binary measurement and digest comparison.

#### Scenario: Typed parsing has positive and negative tests
r[valence.stack_provenance_adapter.typed_boundary_values.tests]
- GIVEN typed stack-provenance boundary parsers and fixtures
- WHEN focused tests run
- THEN valid metadata MUST parse successfully and malformed digests, wrong schema/version strings, invalid claim scopes, missing or wrong Mantle roles, invalid statuses, tier-promotion attempts, and unknown evidence tiers MUST be covered by deterministic negative tests.

#### Scenario: Typed boundary is documented
r[valence.stack_provenance_adapter.typed_boundary_values.docs]
- GIVEN an operator or downstream maintainer reads the stack-provenance documentation
- WHEN typed boundary values are described
- THEN the documentation MUST state that typed values harden internal validation only, do not change public JSON contracts, and do not broaden the measured-byte identity-and-linkage-only claim.
