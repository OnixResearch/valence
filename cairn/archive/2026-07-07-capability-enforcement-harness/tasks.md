## Phase 1: Harness split

- [x] [serial] r[valence.evidence_ir.capability_enforcement_harness] Separate pure label-policy tests from sandbox enforcement tests.
- [x] [serial] r[valence.evidence_ir.capability_enforcement_harness.availability] Report enforcement backend availability explicitly.
- [x] [parallel] r[valence.evidence_ir.capability_enforcement_harness.policy] Add positive and negative label-policy tests for pure, local, unknown, and denied capabilities.

## Phase 2: Enforcement probes

- [x] [parallel] r[valence.evidence_ir.capability_enforcement_harness.denied_effects] Add opt-in denied-effect probes for filesystem, process, network, clock, remote-builder, and secret effects.
- [x] [parallel] r[valence.evidence_ir.capability_enforcement_harness.no_false_success] Add tests that missing sandbox support is unavailable/skipped, not successful enforcement.
- [x] [parallel] r[valence.evidence_ir.capability_enforcement_harness.non_claims] Document that labels are metadata unless an enforcement backend actually blocks effects.

## Phase 3: Evidence

- [x] [serial] r[valence.evidence_ir.capability_enforcement_harness] Run capability policy tests, opt-in enforcement probes when available, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
