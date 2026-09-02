## Phase 1: Stub facets and promotion rules

- [x] [serial] r[valence.kani_stub_roles.stub_facets] Add typed facets for verified contract stubs, plain stubs, and environmental stubs.
- [x] [serial] r[valence.kani_stub_roles.verified_linkage] Require verified stubs to link to passing contract artifacts with matching original function and source scope.
- [x] [serial] r[valence.kani_stub_roles.boundary_classification] Classify plain and environmental stubs as boundary or recorded-only evidence with explicit assumptions.

## Phase 2: Validation and fixtures

- [x] [parallel] r[valence.kani_stub_roles.positive_fixtures] Add positive fixtures for verified-stub linkage and environmental-stub boundary evidence.
- [x] [parallel] r[valence.kani_stub_roles.negative_fixtures] Add negative fixtures for missing link, stale contract hash, signature mismatch, plain-stub promotion, and missing assumptions.
- [x] [serial] r[valence.kani_stub_roles.no_unverified_promotion] Enforce fail-closed promotion rules in artifact validation and stack-provenance compatibility paths.

## Phase 3: Documentation and validation

- [x] [serial] r[valence.kani_stub_roles.boundary_docs] Document stub role meanings and non-claims.
- [x] [serial] r[valence.kani_stub_roles.verified_linkage] Run fixture tests, CLI validation tests, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Verified stub links to passing contract | Positive fixture test |
| Environmental stub remains boundary evidence | Positive fixture test |
| Missing contract link fails closed | Negative fixture test |
| Stale contract hash fails closed | Negative fixture test |
| Plain-stub promotion fails closed | Negative fixture test |
| Missing environmental assumptions fail closed | Negative fixture test |
