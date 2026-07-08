# Evidence IR Specification

## Purpose

Adds a shared verification role vocabulary for stack evidence consumers.

## Requirements

### Requirement: Verification role vocabulary contract
r[valence.evidence_ir.verification_role_vocabulary.contract] Valence MUST define the canonical vocabulary for Evidence IR verification roles, tier mappings, counting behavior, and non-claim boundaries.

#### Scenario: Known roles pass
r[valence.evidence_ir.verification_role_vocabulary.fixtures.positive]
- GIVEN a role row declares a known role, allowed tier mapping, counting behavior, and non-claims
- WHEN vocabulary validation runs
- THEN validation MUST pass and preserve the role identity.

#### Scenario: Unknown role fails
r[valence.evidence_ir.verification_role_vocabulary.fixtures.negative]
- GIVEN a consumer declares an unknown role, role-counting drift, tier promotion, or missing non-claims
- WHEN vocabulary validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Vocabulary constants
r[valence.evidence_ir.verification_role_vocabulary.constants] Valence SHOULD provide generated constants or drift validation so consumers do not copy role literals unchecked.

#### Scenario: Consumer constants match
r[valence.evidence_ir.verification_role_vocabulary.constants.match]
- GIVEN a consumer validates local constants against the vocabulary digest
- WHEN constants match
- THEN validation SHOULD pass and record the vocabulary digest.

### Requirement: Role docs
r[valence.evidence_ir.verification_role_vocabulary.docs] Documentation MUST state which roles count as verified property evidence and which remain recorded-only, boundary, or manual-review context.

#### Scenario: Counting boundary is visible
r[valence.evidence_ir.verification_role_vocabulary.docs.counting]
- GIVEN a report summarizes verification roles
- WHEN the supported claim is rendered
- THEN it MUST state that only configured property roles count as verified property evidence.

### Requirement: Final validation
r[valence.evidence_ir.verification_role_vocabulary.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers role health
r[valence.evidence_ir.verification_role_vocabulary.final_validation.fixtures]
- GIVEN valid and invalid role fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.
