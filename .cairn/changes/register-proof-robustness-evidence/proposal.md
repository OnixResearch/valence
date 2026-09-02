# Proposal: Register proof robustness evidence

## Why

Octet plans a proof-robustness receipt that links controlled solver-seed runs, resource observations, assumption audits, and partial proof-use facts.

Valence must preserve those identities and roles without treating robustness as a proof result. Without an accepted mapping, consumers can promote stability observations or collapse assumptions into ordinary proof facts.

## What Changes

- Register the Octet proof-robustness receipt as an admitted producer schema.
- Define layer identities for profile, proof parent, child runs, resource observations, audit facts, proof-use facts, classification, and terminal snapshot.
- Map observations and classifications to `RecordedOnly`.
- Map assumptions, trust boundaries, unknown coverage, and non-claims to `Boundary`.
- Keep any `Property` role attached only to the separately validated parent proof artifact.
- Add positive and negative fixtures for complete linkage, stale parents, role promotion, missing boundaries, unsupported coverage promotion, and overclaims.

## Impact

- **Specs**: new `proof-robustness-evidence` capability.
- **Files**: producer schema registry, Evidence IR mapping, pure validation, fixtures, and documentation.
- **Compatibility**: existing proof, Aeneas, Kani, model-checking, and stack-provenance roles remain unchanged.

## Dependencies

Octet `add-proof-robustness-evidence` must publish a stable producer schema and fixture before Valence implementation can finish.

Valence can write the role and linkage contract first. It must remain blocked on exact producer identity until that input exists.

## Non-goals

- Do not recompute solver metrics or Octet classifications.
- Do not run Verus, Dafny, or another verifier.
- Do not define proof, lifecycle, or release policy.
- Do not accept live queued or running events as canonical proof evidence.
- Do not promote stable robustness observations to `Property`.

## Completion Evidence

The change is complete when a complete Octet fixture maps deterministically and every stale, role-confused, incomplete, or overclaiming fixture fails closed.
