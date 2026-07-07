# Stack Provenance Adapter Specification

## Purpose

Extends the `stack-provenance-adapter` capability with Nickel contracts for static built-binary thread metadata hardening.

## Requirements

### Requirement: Nickel contract metadata hardening
r[valence.stack_provenance_adapter.nickel_contracts] Valence MUST provide Nickel contracts that validate static metadata shapes for the built-binary provenance thread before Rust performs measured-byte verification.

#### Scenario: Contracts cover stack provenance metadata inputs
r[valence.stack_provenance_adapter.nickel_contracts.metadata_inputs]
- GIVEN an Octet Valence provenance export, a Valence stack sidecar, and a Mantle release manifest view
- WHEN the metadata is evaluated against the Nickel contracts
- THEN the contracts MUST validate required object shapes, fixed schema strings, required role names, claim scope, BLAKE3 digest syntax, and required non-claims.

#### Scenario: Static invariants are enforced
r[valence.stack_provenance_adapter.nickel_contracts.static_invariants]
- GIVEN metadata with a wrong schema string, malformed BLAKE3 digest, missing `stack-provenance-trace` role, missing identity-and-linkage non-claim, or manual-review evidence promoted to `formal_proof`
- WHEN the metadata is evaluated against the Nickel contracts
- THEN contract evaluation MUST fail before Rust built-binary verification is treated as eligible to run.

#### Scenario: Built-binary thread input envelope is contracted
r[valence.stack_provenance_adapter.nickel_contracts.thread_input]
- GIVEN a built-binary thread input envelope containing Octet export metadata, sidecar metadata, Mantle release metadata, and binary artifact locator metadata
- WHEN the envelope is evaluated against the Nickel contract
- THEN the contract MUST validate static metadata and MUST NOT claim to measure local binary bytes.

#### Scenario: Contracts include positive and negative fixtures
r[valence.stack_provenance_adapter.nickel_contracts.fixtures]
- GIVEN the checked-in Nickel contracts and fixture data
- WHEN fixture validation runs
- THEN valid fixtures MUST pass and invalid fixtures for schema, digest, role, non-claim, tier-promotion, parent-linkage, and overclaim failures MUST fail deterministically.

#### Scenario: Contract gate is deterministic
r[valence.stack_provenance_adapter.nickel_contracts.gate]
- GIVEN a repository checkout with Nickel tooling available
- WHEN the contract validation command or test target runs
- THEN it SHOULD evaluate all contract fixtures without network access and report stable diagnostics.

#### Scenario: Runtime boundary stays in Rust
r[valence.stack_provenance_adapter.nickel_contracts.rust_boundary]
- GIVEN metadata has passed Nickel contract validation
- WHEN Valence verifies the built-binary provenance thread
- THEN Rust MUST remain responsible for reading binary bytes, computing BLAKE3, and comparing measured identity across the sidecar, provenance links, Octet export, and Mantle release metadata.

#### Scenario: CLI preflight is bounded
r[valence.stack_provenance_adapter.nickel_contracts.cli]
- GIVEN an optional CLI or test-harness preflight for contract validation
- WHEN the preflight succeeds
- THEN the result MAY permit Rust verification to proceed but MUST NOT be reported as measured-byte identity proof by itself.

#### Scenario: Contract boundary is documented
r[valence.stack_provenance_adapter.nickel_contracts.docs]
- GIVEN an operator reads the built-binary thread documentation or contract validation output
- WHEN the supported claim is described
- THEN the output MUST state that Nickel proves metadata shape/static invariants only and does not prove measured-byte identity, behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, or whole-program safety.
