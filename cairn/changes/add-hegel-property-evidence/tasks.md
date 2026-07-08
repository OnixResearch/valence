## Phase 1: Hegel receipt contract

- [ ] [serial] r[valence.evidence_ir.hegel_property_receipts] Add Rust-owned Hegel property receipt DTOs with BLAKE3 identity, linked requirement IDs, Hegel runner identity, settings, generator identity, status, issue codes, counterexample identity, and non-claims.
- [ ] [serial] r[valence.evidence_ir.hegel_property_receipts.fail_closed] Add positive and negative receipt validation fixtures for complete receipts, missing identity, malformed hashes, missing non-claims, stale generator identities, and overclaim attempts.
- [ ] [serial] r[valence.evidence_ir.hegel_property_receipts.non_claims] Ensure receipt reports and docs state bounded generated testing only.

## Phase 2: Pure Hegel property suites

- [ ] [parallel] r[valence.evidence_ir.hegel_property_suites.identity] Add Hegel properties for EvidenceArtifact/function-object canonical identity stability under display metadata churn and equivalent input ordering.
- [ ] [parallel] r[valence.evidence_ir.hegel_property_suites.graph] Add Hegel properties for graph ordering determinism, alias resolution, and cache-key equality over generated positive cases.
- [ ] [parallel] r[valence.evidence_ir.hegel_property_suites.fail_closed] Add Hegel-generated negative cases for duplicate identities, dangling graph edges, stale aliases, missing boundaries, and cache-key mismatches.
- [ ] [serial] r[valence.evidence_ir.hegel_property_suites.pure_core] Keep all generated-input evaluation in pure core functions with Hegel as a test-only dependency.

## Phase 3: Lanes, docs, and validation

- [ ] [serial] r[valence.evidence_ir.hegel_suite_lanes.fast] Add named fast-lane budgets and deterministic/replayable settings for ordinary local validation.
- [ ] [serial] r[valence.evidence_ir.hegel_suite_lanes.deep] Add an opt-in deep-lane posture for larger budgets or stateful models without making them a hidden release prerequisite.
- [ ] [serial] r[valence.evidence_ir.hegel_suite_lanes.docs] Update README/testing-harness docs with Hegel commands, replay guidance, non-claims, and sandbox limitations.
- [ ] [serial] r[valence.evidence_ir.hegel_property_receipts] Run focused Hegel tests, `cargo test --workspace`, `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and Cairn validation/gates.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Complete Hegel receipt is accepted | Positive receipt fixture/unit test |
| Missing identity, hash, or non-claim fails closed | Negative receipt fixtures/unit tests |
| Overclaim wording is rejected | Negative receipt fixture/unit test |
| Canonical identity survives display/order churn | Hegel property test plus concrete regression fixture |
| Duplicate, dangling, stale, and cache-mismatch inputs fail closed | Hegel property tests plus malformed corpus cases |
| Fast/deep lane behavior is visible and bounded | README/docs check and suite-lane test |
