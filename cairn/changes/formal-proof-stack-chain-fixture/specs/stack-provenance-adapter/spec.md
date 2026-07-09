# Stack Provenance Adapter Specification Delta

## ADDED Requirements

### Requirement: Formal proof stack-chain fixture contract
r[valence.stack_provenance.formal_proof_chain_fixture.contract] Valence MUST define a frozen formal-proof stack-chain fixture contract that links Octet formal-proof receipts, proof cards, provenance exports, Valence sidecars, graph reports, and Cairn readiness input summaries with typed identities, verification roles, and non-claims.

#### Scenario: Fixture contract names every hop
r[valence.stack_provenance.formal_proof_chain_fixture.contract.hops]
- GIVEN a formal-proof stack-chain fixture is declared
- WHEN Valence validates the fixture contract
- THEN the fixture MUST name the Octet receipt, proof card, provenance record, Valence sidecar, graph report, Cairn readiness input summary, and required non-claim boundary.

### Requirement: Positive formal-proof chain fixture
r[valence.stack_provenance.formal_proof_chain_fixture.positive] Valence MUST accept a fixture where a scoped Octet-owned `formal_proof` receipt has matching proof-card, provenance, sidecar, graph, and Cairn readiness-input identities.

#### Scenario: Scoped property survives the chain
r[valence.stack_provenance.formal_proof_chain_fixture.positive.property]
- GIVEN a fixture contains one scoped Octet-owned passed formal-proof receipt with matching proof card, provenance record, sidecar, graph node, and Cairn readiness input summary
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST pass, preserve `verification_role = property` for that record, and report that only that property record counts as verified.

### Requirement: Negative formal-proof chain fixtures
r[valence.stack_provenance.formal_proof_chain_fixture.negative] Valence MUST reject fixtures with stale proof links, manual-review promotion, digest drift, unsupported edges, missing non-claims, or invalid verified counts.

#### Scenario: Manual review cannot become property
r[valence.stack_provenance.formal_proof_chain_fixture.negative.manual_review]
- GIVEN an imported Trellis or manual-review record is marked as `property` without an Octet-owned rerun receipt
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST fail with a deterministic tier-promotion diagnostic.

#### Scenario: Recorded-only evidence cannot be counted as verified
r[valence.stack_provenance.formal_proof_chain_fixture.negative.role_count]
- GIVEN a Cairn readiness input summary counts `recorded_only` or `boundary` evidence as verified
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST fail with a deterministic verification-role-count diagnostic.

### Requirement: Stack-chain validation core
r[valence.stack_provenance.formal_proof_chain_fixture.validation] Valence MUST validate formal-proof stack-chain fixtures through a pure deterministic core over parsed in-memory fixture values.

#### Scenario: Core has no shell effects
r[valence.stack_provenance.formal_proof_chain_fixture.validation.core]
- GIVEN parsed fixture values and expected BLAKE3 identities
- WHEN the formal-proof stack-chain validator runs
- THEN it MUST return pass/fail diagnostics without reading files, invoking processes, inspecting environment, or contacting sibling repositories.

### Requirement: Stack-chain ownership boundary
r[valence.stack_provenance.formal_proof_chain_fixture.docs] Valence documentation MUST state that the fixture proves stack identity and role-preserving linkage only.

#### Scenario: Fixture non-claims are visible
r[valence.stack_provenance.formal_proof_chain_fixture.docs.non_claims]
- GIVEN a formal-proof stack-chain fixture report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that the fixture does not prove whole-program correctness, verifier soundness, semantic equivalence, release eligibility, live sibling freshness, or requirement satisfaction outside the declared fixture scope.

### Requirement: Formal-proof chain validation evidence
r[valence.stack_provenance.formal_proof_chain_fixture.final_validation] The change MUST include positive and negative fixtures plus focused Valence and Cairn validation evidence before archive.

#### Scenario: Focused validation covers stack-chain health
r[valence.stack_provenance.formal_proof_chain_fixture.final_validation.fixtures]
- GIVEN positive and negative formal-proof stack-chain fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved validation evidence MUST identify fixture and policy inputs.
