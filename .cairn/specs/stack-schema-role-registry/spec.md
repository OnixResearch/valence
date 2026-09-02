# Stack Schema Role Registry Specification

## Purpose

Defines the `stack-schema-role-registry` capability.

## Requirements




### Requirement: Stack schema role registry contract
r[valence.stack_schema_role_registry.contract] Valence MUST define a registry contract for evidence-facing schema strings, role names, claim scopes, owners, allowed consumers, status, and non-claim boundaries.

#### Scenario: Known registry row passes
r[valence.stack_schema_role_registry.fixtures.positive]
- GIVEN a registry row declares a schema or role with owner, allowed consumers, status, and required non-claims
- WHEN registry validation runs
- THEN the row MUST pass and preserve the registry identity.

#### Scenario: Invalid registry row fails
r[valence.stack_schema_role_registry.fixtures.negative]
- GIVEN a row declares a stale schema, unknown role, unsupported consumer, or weakened non-claims
- WHEN registry validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Constant drift validation
r[valence.stack_schema_role_registry.constants] Valence SHOULD provide generated constants or drift validation for consumer repos.

#### Scenario: Consumer constants are checked
r[valence.stack_schema_role_registry.constants.checked]
- GIVEN a consumer submits local schema/role constants for validation
- WHEN they match the registry digest
- THEN validation SHOULD pass and record the digest.

### Requirement: Registry docs
r[valence.stack_schema_role_registry.docs] Documentation MUST state that registry validation prevents literal drift only and does not prove consumer semantic correctness.

#### Scenario: Non-claim is visible
r[valence.stack_schema_role_registry.docs.non_claims]
- GIVEN a registry row validates
- WHEN the supported claim is rendered
- THEN it MUST state that validation does not prove the producer or consumer implemented the schema semantics correctly.

### Requirement: Final validation
r[valence.stack_schema_role_registry.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers registry health
r[valence.stack_schema_role_registry.final_validation.fixtures]
- GIVEN valid and invalid registry fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
