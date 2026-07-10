# Evidence IR Specification

## Purpose

Extends Valence Evidence IR with a function-address evidence profile for Kamacite portable receipts and Octet producer facts while preserving Valence's identity/linkage/non-claim boundary.

## Requirements

### Requirement: Function-address profile contract

r[valence.evidence_ir.function_address_profile.contract] Valence MUST define a function-address Evidence IR profile that validates Kamacite function-address receipt identity, Octet producer identity, source-envelope identities, function-address identities, optional dependency-closure identities, source spans, artifact roles, verification roles, and explicit non-claims.

#### Scenario: Valid profile preserves producer boundaries
r[valence.evidence_ir.function_address_profile.contract.boundaries]
- GIVEN a function-address evidence record names Octet producer facts and a Kamacite function-address receipt
- WHEN Valence validates the record
- THEN validation MUST preserve Octet as source-shape producer, Kamacite as portable receipt owner, and Valence as identity/linkage/non-claim verifier.

#### Scenario: Profile keeps identity domains distinct
r[valence.evidence_ir.function_address_profile.contract.identity_domains]
- GIVEN a record contains source-envelope identity, function address, dependency-closure identity, Kamacite receipt identity, Valence projection identity, sidecar digest, and release-binary identity
- WHEN Valence validates the record
- THEN each BLAKE3 identity MUST remain in its declared domain and MUST NOT be substituted for another domain.

### Requirement: Profile validation fails closed

r[valence.evidence_ir.function_address_profile.validation] Valence MUST fail closed when function-address profile records contain malformed BLAKE3 digests, stale identity links, unsupported profile versions, unknown roles, missing required source-span linkage, missing non-claims, weakened non-claims, or overclaim text.

#### Scenario: Stale receipt identity fails
r[valence.evidence_ir.function_address_profile.validation.stale_receipt]
- GIVEN a Valence profile record declares a Kamacite receipt identity that does not match the supplied receipt metadata
- WHEN Valence validates the record
- THEN validation MUST fail with a stale-receipt-identity diagnostic.

#### Scenario: Wrong domain substitution fails
r[valence.evidence_ir.function_address_profile.validation.domain_substitution]
- GIVEN a function address is supplied where a source-envelope identity or release-binary identity is required
- WHEN Valence validates the record
- THEN validation MUST fail with an identity-domain-substitution diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.function_address_profile.validation.overclaim]
- GIVEN a function-address profile record claims behavioral correctness, Rust semantic correctness, compiler correctness, verifier soundness, whole-program safety, or release eligibility
- WHEN Valence validates the record
- THEN validation MUST fail with an overclaim diagnostic.

### Requirement: Positive function-address fixtures

r[valence.evidence_ir.function_address_profile.fixtures.positive] Valence MUST include positive fixtures for Octet-only producer records, Octet-plus-Kamacite linked receipts, dependency-edge records, source-span preservation, and optional Mantle release sidecar linkage.

#### Scenario: Octet and Kamacite linked receipt passes
r[valence.evidence_ir.function_address_profile.fixtures.positive.kamacite_link]
- GIVEN a fixture contains Octet producer evidence linked to a Kamacite function-address receipt with preserved spans, dependencies, roles, and non-claims
- WHEN Valence validates the linked evidence
- THEN validation MUST pass and report the linkage without treating the receipt as runtime correctness proof.

#### Scenario: Optional Mantle linkage is preserved
r[valence.evidence_ir.function_address_profile.fixtures.positive.mantle_link]
- GIVEN a function-address profile record includes optional Mantle release sidecar metadata
- WHEN Valence validates the record
- THEN the linkage MUST be preserved as bundle-local artifact context and MUST NOT become release-readiness proof.

### Requirement: Negative function-address fixtures

r[valence.evidence_ir.function_address_profile.fixtures.negative] Valence MUST include negative fixtures for malformed digests, stale identities, wrong identity domains, unsupported versions, unknown roles, missing non-claims, overclaims, and attempted formal-proof promotion.

#### Scenario: Invalid fixture suite fails closed
r[valence.evidence_ir.function_address_profile.fixtures.negative.suite]
- GIVEN invalid function-address profile fixtures cover the rejected classes
- WHEN the fixture suite runs
- THEN each fixture MUST fail with deterministic diagnostics naming the invalid field, role, identity domain, or non-claim boundary.

### Requirement: Verification-role preservation

r[valence.evidence_ir.function_address_profile.roles] Function-address profile validation MUST preserve verification-role boundaries. Function-address receipts MUST remain recorded-only or boundary evidence unless linked to a separate formal proof receipt with its own verifier identity and tier.

#### Scenario: Function address is not formal proof
r[valence.evidence_ir.function_address_profile.roles.no_promotion]
- GIVEN a function-address receipt is valid and no separate proof receipt is supplied
- WHEN Valence validates the profile
- THEN the receipt MUST NOT be counted as formal proof or behavioral correctness evidence.

#### Scenario: Separate proof link remains separate
r[valence.evidence_ir.function_address_profile.roles.proof_link]
- GIVEN a function-address record links to a separate Trellis, Verus, Kani, or other formal proof receipt
- WHEN Valence validates the profile
- THEN the proof receipt MUST retain its own verifier identity and tier without promoting the function-address receipt itself.

### Requirement: Graph-report integration

r[valence.evidence_ir.function_address_profile.graph] Valence SHOULD include function-address nodes and typed edges in stack graph reports when supplied, including edges to Octet producer evidence, Kamacite receipts, source spans, dependency identities, Mantle sidecars, and Cairn gate inputs.

#### Scenario: Graph report preserves typed edges
r[valence.evidence_ir.function_address_profile.graph.typed_edges]
- GIVEN a valid function-address profile record with producer, receipt, source, dependency, and sidecar edges
- WHEN Valence emits a graph report
- THEN the report SHOULD preserve the typed edges and identity domains without claiming release eligibility.

### Requirement: Documentation and final validation

r[valence.evidence_ir.function_address_profile.docs] Valence documentation MUST describe function-address profile ownership boundaries and non-claims. r[valence.evidence_ir.function_address_profile.final_validation] The change MUST include positive and negative fixtures, focused core tests, and lifecycle validation before archive.

#### Scenario: Boundary is visible
r[valence.evidence_ir.function_address_profile.docs.boundary]
- GIVEN an operator reads Valence function-address profile documentation
- WHEN the supported claim is described
- THEN the documentation MUST state that Valence validates identity, linkage, roles, digest domains, spans, and non-claims only; Octet owns Rust source analysis, Kamacite owns portable receipts, Mantle owns bundle-local binding, and Cairn owns lifecycle policy.

#### Scenario: Final validation covers fixtures
r[valence.evidence_ir.function_address_profile.final_validation.fixtures]
- GIVEN the function-address profile implementation is complete
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, formatting and lifecycle validation MUST pass, and saved evidence MUST identify the checked profile identities.
