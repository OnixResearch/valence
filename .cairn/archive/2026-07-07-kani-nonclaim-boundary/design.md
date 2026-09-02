# Design: Kani non-claim boundary

## Approach

Valence treats Kani non-claims as required policy-owned text or typed labels for Kani artifacts. A valid artifact states both what was checked and what remains outside the claim.

## Required boundary labels

Kani artifacts require boundaries for:

- no whole-program correctness;
- no verifier-soundness proof;
- no Stacked Borrows or Tree Borrows aliasing model claim unless separately evidenced;
- no concurrency or async scheduling claim unless separately modeled;
- FFI and plain stubs remain assumptions;
- recursive termination is user responsibility unless separately evidenced;
- bounded harnesses remain bounded unless unwinding assertions or contracts justify more.

## Validation behavior

Missing or weakened non-claims fail closed. Extra precise non-claims are allowed when they do not contradict required boundaries.

## Fixture strategy

Positive fixtures cover bounded harness, contract harness, and verified-stub evidence with required non-claims. Negative fixtures cover whole-program overclaim, verifier-soundness overclaim, aliasing overclaim, concurrency overclaim, FFI overclaim, recursive-termination overclaim, and bounded-to-unbounded promotion.

## What is explicitly not provided

- No assertion that Kani evidence is weak; scoped passing properties remain useful.
- No enforcement of Kani execution; Valence checks submitted artifacts only.
