## Phase 1: Consumer boundary

- [ ] [serial] Add canonical workspace and lint configuration for production `valence-core` checks with `shared_identity.rs` as the reviewed deterministic core scope. r[valence.strict_rust_admission.consumer_scope] r[valence.strict_rust_admission.consumer_scope.production] r[valence.strict_rust_admission.consumer_scope.compatibility]
- [ ] [serial] Document the crate-level Cargo scope, file-level function-address scope, bootstrap workflow, and explicit non-claims. r[valence.strict_rust_admission.boundaries] r[valence.strict_rust_admission.boundaries.non_claims]

## Phase 2: Baseline and admission

- [ ] [serial] Generate, review, and check in the shared-identity observe-mode function-address baseline. r[valence.strict_rust_admission.baseline] r[valence.strict_rust_admission.baseline.bootstrap]
- [ ] [serial] Resolve direct deterministic-core findings without weakening profile lint floors or adding unjustified suppressions. r[valence.strict_rust_admission.profile_evidence] r[valence.strict_rust_admission.profile_evidence.pass]
- [ ] [serial] Retain and verify the deterministic-core artifact bundle with visible BLAKE3 profile/config identities. r[valence.strict_rust_admission.profile_evidence] r[valence.strict_rust_admission.profile_evidence.pass]

## Phase 3: Adversarial replay and lifecycle

- [ ] [serial] Repeat the admitted run and compare profile/config identities plus selected function-address records. r[valence.strict_rust_admission.profile_evidence.repeat]
- [ ] [serial] Demonstrate that changed selected source fails against the reviewed baseline. r[valence.strict_rust_admission.baseline.stale]
- [ ] [serial] Run Valence tests, formatting, focused Clippy, core-purity smoke, Octet artifact verification, and supported Cairn gates; record exact outcomes before sync and archive. r[valence.strict_rust_admission.verification] r[valence.strict_rust_admission.verification.complete]
