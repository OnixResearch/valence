# Evidence IR Specification

## Purpose

Validate generic Kamacite evidence-chain envelopes in Valence Evidence IR.

## Requirements

### Requirement: Valence validates generic evidence-chain envelopes
r[valence.evidence_ir.evidence_chain_profile.core] Valence MUST provide a pure Evidence IR profile that validates canonical Kamacite evidence-chain envelope identity, evidence kind, typed parent links, profile roots, policy/profile hashes, compatibility projections, verification roles, and non-claim boundaries.

#### Scenario: Function-address envelope validates through generic chain
r[valence.evidence_ir.evidence_chain_profile.positive]
- GIVEN Kamacite provides a canonical function-address evidence-chain envelope
- WHEN Valence validates the envelope
- THEN Valence MUST produce recorded-only Evidence IR rows bound to the canonical envelope identity.

### Requirement: Profile-specific validators are explicit
r[valence.evidence_ir.evidence_chain_profile.profile_registry] Valence MUST reject evidence-chain envelopes whose evidence kind lacks a registered profile validator for the current profile version.

#### Scenario: Unknown evidence kind is rejected
r[valence.evidence_ir.evidence_chain_profile.negative]
- GIVEN an evidence-chain envelope declares an unknown evidence kind or unsupported profile version
- WHEN Valence validation runs
- THEN validation MUST fail with a deterministic unsupported-profile diagnostic.

### Requirement: Generic graph rows preserve chain linkage
r[valence.evidence_ir.evidence_chain_profile.graph] Valence SHOULD emit reusable graph rows and deterministic graph roots for producer, canonical receipt, validation, release binding, and readiness links.

#### Scenario: Graph root commits to typed links
- GIVEN an evidence-chain envelope validates
- WHEN Valence emits graph rows
- THEN the graph root MUST commit to typed parent links without allowing cross-domain digest substitution.

### Requirement: CLI shell owns parsing and file effects
r[valence.evidence_ir.evidence_chain_profile.cli_shell] Valence CLI MUST own Preserves/JSON parsing, file reads, stdout/stderr, and exit codes for evidence-chain validation.

#### Scenario: Core purity is preserved
r[valence.evidence_ir.evidence_chain_profile.final_validation]
- GIVEN evidence-chain validation runs through the CLI
- WHEN `valence-core` validates the typed input
- THEN core validation MUST NOT read files, inspect environment state, spawn processes, access clocks, or print output.

### Requirement: Evidence-chain validation keeps role boundaries
r[valence.evidence_ir.evidence_chain_profile.validation_rules] Valence MUST reject stale policy hashes, wrong canonical envelope hashes, projection drift, role promotion, weakened non-claims, and cross-domain parent-link substitution.

#### Scenario: Recorded-only evidence cannot become proof
- GIVEN an evidence-chain envelope contains recorded-only producer evidence
- WHEN Valence validation runs
- THEN validation MUST NOT count the evidence as property or formal-proof evidence unless a profile-specific validator explicitly permits that role.
