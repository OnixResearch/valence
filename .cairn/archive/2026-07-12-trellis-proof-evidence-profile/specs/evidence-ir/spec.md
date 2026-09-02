# Evidence IR Specification

## Purpose

Validate Trellis proof evidence IR as a Valence evidence-chain profile.

## Requirements

### Requirement: Trellis proof evidence is an explicit profile
r[valence.evidence_ir.trellis_proof_evidence_profile.registry] Valence MUST register Trellis proof evidence as an explicit evidence-chain profile before accepting Trellis proof envelopes.

#### Scenario: Trellis envelope uses profile validator
r[valence.evidence_ir.trellis_proof_evidence_profile.positive]
- GIVEN Kamacite provides a canonical Trellis proof evidence envelope
- WHEN Valence validates the envelope
- THEN Valence MUST route proof-specific fields through the Trellis proof evidence profile validator.

### Requirement: Trellis proof links are validated
r[valence.evidence_ir.trellis_proof_evidence_profile.validation] Valence MUST validate source envelope links, proof IR links, verifier receipt links, assumptions, property IDs, requirement IDs, spans, digest domains, policy hashes, and non-claim boundaries for Trellis proof evidence.

#### Scenario: Stale proof linkage fails closed
r[valence.evidence_ir.trellis_proof_evidence_profile.negative]
- GIVEN Trellis proof evidence has stale source, missing assumptions, verifier mismatch, unsupported result status, malformed IDs, digest-domain substitution, or weakened non-claims
- WHEN Valence validates the profile
- THEN validation MUST fail with deterministic Trellis proof evidence diagnostics.

### Requirement: Proof role outcomes are distinct
r[valence.evidence_ir.trellis_proof_evidence_profile.roles] Valence MUST distinguish reference-only, formal-proof candidate, and accepted formal-proof outcomes for Trellis proof evidence.

#### Scenario: Candidate does not count without policy acceptance
- GIVEN Trellis proof evidence has a passed verifier receipt but lacks required policy or assumption compatibility
- WHEN Valence validates role counting
- THEN the evidence MUST NOT count as accepted formal-proof evidence.

### Requirement: Proof graph rows preserve traceability
r[valence.evidence_ir.trellis_proof_evidence_profile.graph] Valence SHOULD emit graph rows connecting Trellis proof receipts to requirements, source spans, proof obligations, verifier receipts, assumptions, and downstream artifacts.

#### Scenario: Graph rows preserve non-claims
r[valence.evidence_ir.trellis_proof_evidence_profile.final_validation]
- GIVEN Valence emits Trellis proof evidence graph rows
- WHEN downstream Mantle or Cairn consumes them
- THEN the rows MUST preserve proof-scope and non-claim boundaries without claiming release eligibility.
