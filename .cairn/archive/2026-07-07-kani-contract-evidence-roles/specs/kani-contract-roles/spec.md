## ADDED Requirements

### Requirement: Kani contract facets
r[valence.kani_contract_roles.contract_facets] Valence MUST represent Kani preconditions, postconditions, loop invariants, loop decreases, quantifiers, and contract harnesses as explicit evidence facets rather than generic proof text.

#### Scenario: Contract facets are preserved
r[valence.kani_contract_roles.contract_facets.preserved]
- GIVEN a Kani receipt contains function and loop contract metadata
- WHEN Valence validates the Evidence IR artifact
- THEN the contract facets MUST be preserved in typed payload data.

#### Scenario: Missing facet metadata is rejected for contract claims
r[valence.kani_contract_roles.contract_facets.missing]
- GIVEN a Kani artifact claims contract-backed evidence but lacks contract facet metadata
- WHEN Valence validates the artifact
- THEN validation MUST fail with a deterministic missing-contract-facet diagnostic.

### Requirement: Kani contract scope identity
r[valence.kani_contract_roles.scope_identity] Valence MUST bind Kani contract evidence to exact source identity, crate target, monomorphization or target scope, solver/toolchain identity, and receipt status.

#### Scenario: Exact scope passes
r[valence.kani_contract_roles.scope_identity.exact]
- GIVEN a Kani contract artifact names the exact source hash and target scope it verified
- WHEN Valence checks the artifact
- THEN validation MUST preserve the scope identity and receipt status.

#### Scenario: Missing target scope fails closed
r[valence.kani_contract_roles.scope_identity.missing]
- GIVEN a Kani contract artifact omits crate target, monomorphization, or equivalent target scope
- WHEN Valence checks the artifact
- THEN validation MUST fail closed.

### Requirement: Kani contract claim classification
r[valence.kani_contract_roles.claim_classification] Valence MUST classify passing `proof_for_contract` evidence as scoped property evidence only for the declared contract and MUST classify bounded-only or partial-correctness results separately.

#### Scenario: Passing contract becomes scoped property evidence
r[valence.kani_contract_roles.claim_classification.contract]
- GIVEN a Kani `proof_for_contract` receipt passes for a declared function contract
- WHEN Valence classifies the artifact
- THEN Valence MAY assign a property role for that exact contract and source scope.

#### Scenario: Loop without decreases remains partial correctness
r[valence.kani_contract_roles.claim_classification.partial]
- GIVEN a Kani loop invariant passes but no loop decreases clause is verified
- WHEN Valence classifies the artifact
- THEN Valence MUST NOT describe the loop evidence as total correctness.

### Requirement: Kani contract positive fixtures
r[valence.kani_contract_roles.positive_fixtures] Valence MUST include positive fixtures for passing function contracts, loop contracts with decreases, and bounded-only harness classification.

#### Scenario: Passing fixtures validate
r[valence.kani_contract_roles.positive_fixtures.valid]
- GIVEN positive Kani contract and bounded harness fixtures
- WHEN the Valence fixture suite runs
- THEN each valid fixture MUST produce deterministic accepted validation output.

### Requirement: Kani contract negative fixtures
r[valence.kani_contract_roles.negative_fixtures] Valence MUST include negative fixtures for failed contracts, missing solver identity, missing target scope, missing receipts, and overbroad monomorphization claims.

#### Scenario: Overbroad monomorphization claim fails
r[valence.kani_contract_roles.negative_fixtures.monomorphization]
- GIVEN a Kani artifact claims all monomorphizations without receipt support
- WHEN Valence validates the artifact
- THEN validation MUST fail with a deterministic scope-overclaim diagnostic.

#### Scenario: Failed contract cannot be property evidence
r[valence.kani_contract_roles.negative_fixtures.failed]
- GIVEN a Kani contract receipt reports failure
- WHEN Valence validates a property role for it
- THEN validation MUST reject the property role.

### Requirement: Kani contract non-claims
r[valence.kani_contract_roles.non_claims] Valence MUST preserve non-claims that Kani contract evidence does not prove verifier soundness, unnamed targets, whole-program correctness, release eligibility, or unmodeled effects.

#### Scenario: Non-claims are required
r[valence.kani_contract_roles.non_claims.required]
- GIVEN a Kani contract artifact lacks required non-claims
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.
