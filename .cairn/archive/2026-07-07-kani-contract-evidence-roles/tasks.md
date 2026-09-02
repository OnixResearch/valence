## Phase 1: Kani contract facets

- [x] [serial] r[valence.kani_contract_roles.contract_facets] Add typed Kani facets for preconditions, postconditions, loop invariants, loop decreases, quantifiers, and contract harnesses.
- [x] [serial] r[valence.kani_contract_roles.scope_identity] Bind contract facets to source, crate target, monomorphization or target scope, solver/toolchain identity, and receipt status.
- [x] [serial] r[valence.kani_contract_roles.claim_classification] Classify passing contracts, bounded-only harnesses, and partial-correctness loop contracts without overclaiming.

## Phase 2: Validation and fixtures

- [x] [parallel] r[valence.kani_contract_roles.positive_fixtures] Add positive fixtures for passing function contract, passing loop contract with decreases, and bounded-only evidence.
- [x] [parallel] r[valence.kani_contract_roles.negative_fixtures] Add negative fixtures for failed contract, missing solver, missing target scope, missing receipt, and all-monomorphization overclaim.
- [x] [serial] r[valence.kani_contract_roles.non_claims] Enforce non-claims for verifier soundness, unnamed targets, whole-program correctness, and unmodeled effects.

## Phase 3: Documentation and validation

- [x] [serial] r[valence.kani_contract_roles.contract_facets] Update Evidence IR docs and CLI validation examples.
- [x] [serial] r[valence.kani_contract_roles.claim_classification] Run fixture tests, CLI check tests, stack-provenance compatibility checks, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Passing function contract is scoped property evidence | Positive fixture test |
| Loop decreases upgrades termination status | Positive fixture test |
| Bounded-only harness remains bounded | Positive fixture test |
| Failed contract is rejected for property role | Negative fixture test |
| Missing target scope fails closed | Negative fixture test |
| All-monomorphization overclaim fails closed | Negative fixture test |
| Non-claims are preserved | CLI validation fixture |
