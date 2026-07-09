# Evidence IR Specification

## Purpose

Adds a Valence-owned reusable stack role registry core.

## Requirements

### Requirement: Stack role registry core contract
r[valence.evidence_ir.stack_role_registry_core.contract] Valence MUST define pure stack role registry contracts for canonical stack roles, schema role bindings, verification role vocabulary, compatibility aliases, and bounded non-claim text.

#### Scenario: Canonical role row validates
r[valence.evidence_ir.stack_role_registry_core.fixtures.positive]
- GIVEN a registry row names a canonical stack role, supported schema, verification role, compatibility class, and non-claim boundary
- WHEN role registry validation runs
- THEN validation MUST accept the row and preserve the public role encoding.

#### Scenario: Invalid role row fails closed
r[valence.evidence_ir.stack_role_registry_core.fixtures.negative]
- GIVEN a registry row has an unknown role, duplicate required role, stale alias, schema/role mismatch, missing verification role, missing non-claim, or authority overclaim
- WHEN role registry validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid role or schema field.

### Requirement: Compatibility adapters preserve public role strings
r[valence.evidence_ir.stack_role_registry_core.adapters] Valence SHOULD provide adapters for existing stack role strings without silently broadening their meaning.

#### Scenario: Legacy role spelling maps through an alias row
r[valence.evidence_ir.stack_role_registry_core.adapters.alias]
- GIVEN a downstream evidence record uses an approved legacy role spelling
- WHEN the adapter parses the record
- THEN parsing MUST require a compatibility alias row that names the canonical replacement, owner, status, and non-claim boundary.

### Requirement: Role registry validation is pure core
r[valence.evidence_ir.stack_role_registry_core.validation] Stack role registry validation MUST be pure deterministic logic over in-memory registry rows and evidence rows.

#### Scenario: Shell owns policy loading
r[valence.evidence_ir.stack_role_registry_core.validation.shell_boundary]
- GIVEN a CLI or downstream crate loads role policy from files or generated artifacts
- WHEN registry validation is invoked
- THEN file reads, environment access, process execution, stdout/stderr, and receipt writing MUST remain outside the role registry core.

### Requirement: Role registry remains bounded
r[valence.evidence_ir.stack_role_registry_core.docs] Passing stack role registry validation MUST NOT prove evidence truth, verifier soundness, source-code correctness, release eligibility, lifecycle readiness, runtime authority, or whole-system safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.stack_role_registry_core.final_validation]
- GIVEN a role registry validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves role/schema/vocabulary compatibility only.
