## Phase 1: Required non-claims

- [x] [serial] r[valence.kani_nonclaims.required_boundaries] Add required Kani non-claim labels for whole-program, verifier-soundness, aliasing, concurrency/async, FFI/stub, recursive-termination, and bounded-only boundaries.
- [x] [serial] r[valence.kani_nonclaims.role_binding] Bind required non-claims to Kani evidence roles and facets.
- [x] [serial] r[valence.kani_nonclaims.weakened_boundary_rejection] Reject missing, weakened, or contradictory Kani non-claim boundaries.

## Phase 2: Fixtures and reports

- [x] [parallel] r[valence.kani_nonclaims.positive_fixtures] Add positive fixtures for bounded harness, contract harness, and verified-stub evidence with required non-claims.
- [x] [parallel] r[valence.kani_nonclaims.negative_fixtures] Add negative fixtures for whole-program, verifier-soundness, aliasing, concurrency, FFI, recursive-termination, and bounded-unbounded overclaims.
- [x] [serial] r[valence.kani_nonclaims.report_visibility] Surface Kani non-claim boundaries in CLI and stack-provenance reports.

## Phase 3: Documentation and validation

- [x] [serial] r[valence.kani_nonclaims.required_boundaries] Update docs for Kani evidence boundaries.
- [x] [serial] r[valence.kani_nonclaims.weakened_boundary_rejection] Run fixture tests, CLI report tests, stack-provenance compatibility checks, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Valid Kani non-claims pass | Positive fixture test |
| Whole-program overclaim fails | Negative fixture test |
| Verifier-soundness overclaim fails | Negative fixture test |
| Aliasing overclaim fails | Negative fixture test |
| Concurrency/async overclaim fails | Negative fixture test |
| FFI or plain-stub overclaim fails | Negative fixture test |
| Bounded-to-unbounded promotion fails | Negative fixture test |
| Reports show boundaries | CLI report fixture |
