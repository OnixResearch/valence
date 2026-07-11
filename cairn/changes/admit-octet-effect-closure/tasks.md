## Phase 1: Policy and initial denial

- [ ] [serial] Add the copied typed Nickel contract, workspace role/call policy, generated runtime JSON, and deterministic freshness check for the existing two selected source files. r[valence.strict_rust_admission.effect_closure_policy] r[valence.strict_rust_admission.effect_closure_policy.roles]
- [ ] [serial] Run and retain the no-summary closure probe; confirm direct check success followed by fail-closed unknown/unadmitted call diagnostics with stable call-site IDs. r[valence.strict_rust_admission.effect_closure_initial] r[valence.strict_rust_admission.effect_closure_initial.denied]
- [ ] [serial] Review every denied call site and add exact provider/effect/evidence summaries without broad matchers or source suppressions. r[valence.strict_rust_admission.effect_closure_policy.calls]

## Phase 2: Final evidence and documentation

- [ ] [serial] Run final closure admission, retain the admitted bundle, and independently verify it. r[valence.strict_rust_admission.effect_closure_final] r[valence.strict_rust_admission.effect_closure_final.admitted]
- [ ] [serial] Replay unchanged inputs and prove matching receipt/status artifacts; add stale-policy or artifact-tamper negative evidence. r[valence.strict_rust_admission.effect_closure_final.repeat] r[valence.strict_rust_admission.effect_closure_final.negative]
- [ ] [serial] Extend the operator guide with policy export, review workflow, replay, verification, selected scope, and explicit dependency/source-token non-claims. r[valence.strict_rust_admission.effect_closure_boundaries] r[valence.strict_rust_admission.effect_closure_boundaries.non_claims]

## Phase 3: Validation and lifecycle

- [ ] [serial] Run formatting, workspace tests, all-target Clippy, Nix checks, policy freshness, Octet admission/verification, compatible Cairn validation/gates, and record exact outcomes before sync and archive. r[valence.strict_rust_admission.effect_closure_final] r[valence.strict_rust_admission.effect_closure_boundaries]
