# Evidence IR Specification

## Purpose

Adds a reusable artifact reference core for stack evidence identities.

## Requirements

### Requirement: Artifact reference core contract
r[valence.evidence_ir.artifact_ref_core.contract] Valence MUST define typed artifact reference contracts for artifact roles, digest spelling profiles, path metadata confinement, hash/size metadata, and bounded non-claim text.

#### Scenario: Valid artifact ref validates
r[valence.evidence_ir.artifact_ref_core.fixtures.positive]
- GIVEN an artifact reference uses a supported role, digest spelling profile, confined metadata path, matching hash/size metadata, and required non-claim boundary
- WHEN artifact ref validation runs
- THEN validation MUST accept the ref and preserve its role-specific type.

#### Scenario: Invalid artifact ref fails closed
r[valence.evidence_ir.artifact_ref_core.fixtures.negative]
- GIVEN an artifact reference has malformed digest text, unsupported prefix, role mismatch, path escape, stale hash/size metadata, missing non-claims, or overclaim text
- WHEN artifact ref validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid ref field.

### Requirement: Artifact roles remain distinct
r[valence.evidence_ir.artifact_ref_core.roles] Artifact ref validation MUST preserve role separation between binary artifacts, sidecars, receipts, source artifacts, release bundle members, and compatibility refs.

#### Scenario: Same digest cannot erase role
r[valence.evidence_ir.artifact_ref_core.roles.same_digest]
- GIVEN two artifact refs have the same digest bytes but different declared roles
- WHEN they are validated or compared
- THEN validation MUST preserve the role distinction and MUST NOT silently treat one role as the other.

### Requirement: Artifact ref validation is pure
r[valence.evidence_ir.artifact_ref_core.validation] Valence MUST implement artifact ref validation as pure deterministic logic over in-memory refs, roles, path metadata, and declared hash/size metadata.

#### Scenario: Shell owns artifact measurement
r[valence.evidence_ir.artifact_ref_core.validation.shell]
- GIVEN a CLI or downstream crate reads artifact bytes, resolves paths, or measures byte digests
- WHEN artifact ref validation is invoked
- THEN filesystem access, byte measurement, path resolution, stdout/stderr, and receipt writing MUST remain outside the artifact ref core.

### Requirement: Compatibility profiles preserve existing spellings
r[valence.evidence_ir.artifact_ref_core.adapters] Valence SHOULD provide compatibility profiles for existing public digest spellings and field names.

#### Scenario: Public field is adapter-backed
r[valence.evidence_ir.artifact_ref_core.adapters.shape]
- GIVEN an existing stack evidence surface exposes a raw hex, `b3:`, or `blake3:` artifact ref field
- WHEN an adapter validates the surface
- THEN the adapter SHOULD preserve that public field while requiring an explicit compatibility profile.

### Requirement: Artifact ref core remains bounded
r[valence.evidence_ir.artifact_ref_core.docs] Passing artifact ref validation MUST NOT prove artifact truth, behavioral correctness, semantic equivalence, verifier soundness, release eligibility, lifecycle readiness, runtime authority, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.artifact_ref_core.final_validation]
- GIVEN an artifact ref validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only artifact ref syntax, role separation, metadata confinement, and declared hash/size consistency.
