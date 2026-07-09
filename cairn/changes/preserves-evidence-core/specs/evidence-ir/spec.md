# Evidence IR Specification

## Purpose

Adds a reusable Preserves evidence identity core to Valence.

## Requirements

### Requirement: Preserves evidence core contract
r[valence.evidence_ir.preserves_evidence_core.contract] Valence MUST define pure Preserves evidence contracts for canonicalization profiles, envelope schema/version metadata, `preserves_content_hash`, self-hash exclusion profiles, and required non-claims.

#### Scenario: Valid Preserves evidence validates
r[valence.evidence_ir.preserves_evidence_core.fixtures.positive]
- GIVEN a Preserves evidence envelope uses a supported schema, supported canonicalization profile, matching content hash, declared self-hash exclusion profile, and required non-claim boundary
- WHEN Preserves evidence validation runs
- THEN validation MUST accept the envelope and report the normalized content identity.

#### Scenario: Invalid Preserves evidence fails closed
r[valence.evidence_ir.preserves_evidence_core.fixtures.negative]
- GIVEN a Preserves evidence envelope has malformed Preserves text, a stale content hash, an unknown schema, a wrong exclusion profile, missing non-claims, or overclaim text
- WHEN Preserves evidence validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid field.

### Requirement: Preserves self-hash exclusion profiles
r[valence.evidence_ir.preserves_evidence_core.self_hash_exclusion] Valence MUST require named self-hash exclusion profiles for receipt-shaped Preserves evidence.

#### Scenario: Self-reference is excluded only by profile
r[valence.evidence_ir.preserves_evidence_core.self_hash_exclusion.profile]
- GIVEN a receipt-shaped Preserves value includes fields that name its own content hash or receipt hash
- WHEN the declared exclusion profile builds hash material
- THEN only fields declared by that profile MUST be excluded and ordinary content fields MUST remain included.

### Requirement: Preserves evidence validation is pure
r[valence.evidence_ir.preserves_evidence_core.validation] Valence MUST implement Preserves evidence identity validation as pure deterministic logic over in-memory Preserves values, envelope metadata, canonical bytes, and hash profiles.

#### Scenario: Shell owns Preserves loading
r[valence.evidence_ir.preserves_evidence_core.validation.shell]
- GIVEN a CLI or downstream crate loads Preserves text, generated contracts, or evidence files from disk
- WHEN Preserves evidence validation is invoked
- THEN file reads, Nickel evaluation, stdout/stderr, process execution, and receipt writing MUST remain outside the Preserves evidence core.

### Requirement: Compatibility adapters preserve public fields
r[valence.evidence_ir.preserves_evidence_core.adapters] Valence SHOULD expose compatibility adapters for existing Cairn, Mantle, Octet, and Valence Preserves evidence shapes.

#### Scenario: Adapter keeps existing field names
r[valence.evidence_ir.preserves_evidence_core.adapters.shape]
- GIVEN a downstream Preserves evidence record uses current public field names
- WHEN an adapter validates that record
- THEN the adapter SHOULD preserve the public DTO shape while routing identity checks through the shared core.

### Requirement: Preserves evidence core remains bounded
r[valence.evidence_ir.preserves_evidence_core.docs] Passing Preserves evidence validation MUST NOT prove receipt truth, release eligibility, lifecycle readiness, semantic equivalence, behavioral correctness, verifier soundness, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.preserves_evidence_core.final_validation]
- GIVEN a Preserves evidence validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only canonical Preserves content identity and bounded envelope compatibility.
