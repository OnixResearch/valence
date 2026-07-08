# Design: Kani contract evidence roles

## Approach

Valence keeps the existing high-level verification-role boundary and adds Kani-specific payload facets. A Kani artifact may be `Property` only when the receipt says the exact scoped contract passed. Otherwise it remains `RecordedOnly` or `Boundary` according to the receipt and non-claim boundary.

## Contract facets

The typed Kani payload records:

- `requires` preconditions;
- `ensures` postconditions and `old(...)` history use;
- loop invariants;
- loop decreases and termination status;
- quantifier use and solver requirements;
- contract harness identity;
- exact function, crate target, and monomorphization or target-scope identity.

## Claim classification

A passing contract harness can support a scoped property claim. A bounded harness without passing unwind coverage remains bounded model-check evidence. A loop invariant without a decreases clause supports partial-correctness evidence only.

## Fixture strategy

Positive fixtures cover passing function contract, passing loop invariant with decreases, and bounded-only harness classification. Negative fixtures cover missing contract receipt, failed contract, missing solver identity, missing target scope, and attempted all-monomorphization overclaim.

## What is explicitly not provided

- No new Valence claim that Kani is sound.
- No proof for monomorphizations or targets not named in the receipt.
- No whole-program or release-eligibility claim.
