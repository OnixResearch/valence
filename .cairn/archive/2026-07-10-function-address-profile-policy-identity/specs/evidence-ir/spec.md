# Evidence IR Specification

## Purpose

Replace placeholder function-address policy hashes with a deterministic Valence profile policy identity.

## Requirements

### Requirement: Function-address profile has a deterministic policy identity
r[valence.evidence_ir.function_address_policy_identity.material] Valence MUST define canonical function-address Evidence IR profile material and derive a deterministic BLAKE3 policy/profile identity from it.

#### Scenario: Current profile identity is emitted
r[valence.evidence_ir.function_address_policy_identity.positive]
- GIVEN function-address evidence validates against the current Valence profile
- WHEN Valence emits a receipt or projection
- THEN the output MUST include the current function-address profile policy identity.

### Requirement: Placeholder and stale policy hashes fail closed
r[valence.evidence_ir.function_address_policy_identity.negative] Valence MUST reject missing, placeholder, stale, malformed, or substituted function-address policy identities when validating profile-bound receipts or projections.

#### Scenario: Placeholder hash is rejected
r[valence.evidence_ir.function_address_policy_identity.replace_placeholder]
- GIVEN a function-address receipt contains the placeholder `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa` policy hash
- WHEN Valence validation runs
- THEN validation MUST fail with a deterministic stale-or-placeholder policy identity diagnostic.

### Requirement: Policy identity is exposed for downstream binding
r[valence.evidence_ir.function_address_policy_identity.digest] Valence MUST expose the function-address profile policy identity in outputs consumed by Mantle and Cairn.

#### Scenario: Stack smoke binds real policy identity
r[valence.evidence_ir.function_address_policy_identity.validation]
- GIVEN the stack smoke uses Valence function-address outputs
- WHEN Mantle binds and Cairn gates the evidence
- THEN the real Valence profile policy identity MUST be present and no placeholder hash may appear.
