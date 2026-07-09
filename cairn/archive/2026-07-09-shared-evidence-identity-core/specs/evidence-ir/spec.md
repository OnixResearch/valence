# Evidence IR Specification

## Purpose

Adds a shared evidence-identity core for stack-owned digest, canonicalization, and non-claim boundary primitives.

## Requirements

### Requirement: Shared evidence identity contract
r[valence.evidence_ir.shared_identity.contract] Valence MUST provide pure evidence-identity contracts for BLAKE3 digest spellings, digest roles, canonical hash profiles, self-referential hash-field exclusion, and reusable non-claim boundary checks.

#### Scenario: Supported digest spellings normalize by profile
r[valence.evidence_ir.shared_identity.digest_profiles]
- GIVEN a raw BLAKE3 hex digest, a `b3:` digest, or a `blake3:` digest accepted by a compatibility profile
- WHEN Valence parses the value for a declared role
- THEN parsing MUST produce a normalized digest value or fail closed if the spelling is unsupported for that profile.

#### Scenario: Digest roles are not interchangeable
r[valence.evidence_ir.shared_identity.digest_roles]
- GIVEN a binary digest, sidecar digest, artifact digest, receipt digest, and Preserves metadata digest have the same underlying bytes
- WHEN typed validation compares evidence roles
- THEN validation MUST preserve the declared role and MUST NOT silently treat one role as another.

### Requirement: Canonical hash profiles
r[valence.evidence_ir.shared_identity.canonical_hashes] Valence MUST compute content identities through explicit canonical hash profiles rather than ambient display serialization.

#### Scenario: Canonical JSON hash is stable
r[valence.evidence_ir.shared_identity.fixtures.positive]
- GIVEN two equivalent typed JSON values with different map insertion order
- WHEN the shared identity core hashes them through the declared canonical JSON profile
- THEN the computed BLAKE3 identity MUST be stable for both values.

#### Scenario: Unsupported canonical profile fails closed
r[valence.evidence_ir.shared_identity.fixtures.negative]
- GIVEN a caller requests an unknown canonicalization profile or supplies malformed digest text
- WHEN the shared identity core validates the input
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Self-hash exclusion profiles
r[valence.evidence_ir.shared_identity.self_hash_exclusion] Valence MUST support named self-hash exclusion profiles for receipt-shaped values whose own digest fields are excluded from their hash material.

#### Scenario: Receipt hash excludes self-reference
r[valence.evidence_ir.shared_identity.self_hash_exclusion.receipt]
- GIVEN a receipt-shaped value has `receipt_hash` or `preserves_content_hash` fields
- WHEN the declared receipt exclusion profile computes hash material
- THEN those self-referential fields MUST be excluded and ordinary content fields MUST remain included.

#### Scenario: Unknown exclusion is rejected
r[valence.evidence_ir.shared_identity.self_hash_exclusion.unknown]
- GIVEN a caller requests an unknown exclusion profile or tries to exclude undeclared fields
- WHEN the shared identity core builds hash material
- THEN validation MUST fail closed with a deterministic issue code.

### Requirement: Compatibility adapters preserve existing contracts
r[valence.evidence_ir.shared_identity.compatibility_adapters] Valence SHOULD expose compatibility adapters for existing stack surfaces while preserving public field names, digest spellings, and bounded claim text.

#### Scenario: Downstream adapter preserves public shape
r[valence.evidence_ir.shared_identity.compatibility_adapters.shape]
- GIVEN an existing Cairn, Mantle, Basalt, Octet, or Molten/Aspen evidence surface uses a supported digest spelling and field name
- WHEN a compatibility adapter validates that surface
- THEN the adapter SHOULD preserve the public DTO shape while routing digest and non-claim validation through shared identity primitives.

### Requirement: Shared identity remains bounded
r[valence.evidence_ir.shared_identity.docs] Shared identity helpers MUST NOT claim evidence truth, behavioral correctness, semantic equivalence, verifier soundness, release eligibility, lifecycle readiness, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.shared_identity.final_validation]
- GIVEN a shared identity validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that shared identity proves only syntax, canonical content identity, role separation, and non-claim boundary conformance for the selected profile.
