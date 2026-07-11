# Strict Rust Admission Specification

## Purpose

Defines the `strict-rust-admission` capability.

## Requirements

### Requirement: Canonical Octet consumer scope

r[valence.strict_rust_admission.consumer_scope] Valence MUST provide canonical consumer configuration and a non-publishable focused Cargo target that compiles the exact production shared-identity source for Octet deterministic-core admission.

#### Scenario: Production core scope is explicit

r[valence.strict_rust_admission.consumer_scope.production]

- GIVEN an operator runs Octet from the Valence workspace root
- WHEN workspace and lint configuration are resolved
- THEN Cargo scope MUST select the `valence-shared-identity-pilot` production library target.
- AND that target MUST compile the canonical `valence-core` shared-identity source rather than a copied implementation.
- AND `core_source_scopes` MUST name `crates/valence-core/src/shared_identity.rs` and `crates/valence-core/src/shared_identity/canonical_hash_profile.rs` as workspace-relative paths.

#### Scenario: Ordinary Rust remains compatible

r[valence.strict_rust_admission.consumer_scope.compatibility]

- GIVEN an operator uses ordinary Cargo without selecting an Octet profile
- WHEN Valence builds or tests
- THEN the Octet consumer configuration MUST NOT change Rust compilation semantics or require Octet at runtime.

### Requirement: Reviewed function-address baseline

r[valence.strict_rust_admission.baseline] Valence MUST keep a reviewed Octet function-address baseline for every supported function in the selected shared-identity source scope.

#### Scenario: Observe output becomes explicit baseline

r[valence.strict_rust_admission.baseline.bootstrap]

- GIVEN the selected source scope is checked in observe mode
- WHEN the resulting function-address report is reviewed
- THEN the report MUST be stored under `evidence/octet/deterministic-core/`.
- AND deny-mode admission MUST consume that exact checked-in baseline.

#### Scenario: Changed source cannot inherit stale admission

r[valence.strict_rust_admission.baseline.stale]

- GIVEN selected function content changes while the reviewed baseline remains unchanged
- WHEN deterministic-core admission runs
- THEN Octet MUST fail closed with function-address drift diagnostics.

### Requirement: Profile-bound admission evidence

r[valence.strict_rust_admission.profile_evidence] Valence MUST retain and verify an Octet artifact bundle produced by `deterministic-core` over the configured scope.

#### Scenario: Deterministic-core pilot passes

r[valence.strict_rust_admission.profile_evidence.pass]

- GIVEN the selected source matches the reviewed baseline and required direct checks pass
- WHEN Octet runs with `--profile deterministic-core`
- THEN the command MUST complete without error findings.
- AND status and summary evidence MUST identify `deterministic-core` and bind BLAKE3 profile/config identities.
- AND Octet artifact verification MUST report a valid bundle with no diagnostics.

#### Scenario: Repeat replay preserves selected identities

r[valence.strict_rust_admission.profile_evidence.repeat]

- GIVEN source, configuration, profile catalog, and toolchain remain unchanged
- WHEN the pilot is run a second time
- THEN profile identity, configuration identity, and the selected function-address record set MUST match the first admitted run.

### Requirement: Admission claims remain bounded

r[valence.strict_rust_admission.boundaries] Valence MUST describe the pilot as direct source-shape policy and source-address evidence only.

#### Scenario: Evidence does not overclaim

r[valence.strict_rust_admission.boundaries.non_claims]

- GIVEN an operator reads the pilot documentation, baseline, artifact bundle, or validation report
- WHEN the admitted scope is described
- THEN the text MUST distinguish the focused harness target, selected production source files, and the broader unadmitted `valence-core` crate.
- AND it MUST NOT claim transitive effect absence, dependency closure, compiler correctness, verifier soundness, whole-program correctness, semantic equivalence, or release eligibility.

### Requirement: Positive and negative pilot verification

r[valence.strict_rust_admission.verification] The pilot MUST include reviewable positive and negative validation evidence.

#### Scenario: Validation record is complete

r[valence.strict_rust_admission.verification.complete]

- GIVEN the pilot implementation is ready to archive
- WHEN validation evidence is reviewed
- THEN it MUST record baseline and post-change Valence tests, profile bootstrap, admitted replay, artifact verification, repeat identity comparison, stale-source rejection, core-purity smoke, formatting, Clippy, and supported Cairn gates.

### Requirement: Nickel-authored closure policy

r[valence.strict_rust_admission.effect_closure_policy] Valence MUST keep a typed Nickel-authored workspace effect-closure policy and checked-in generated runtime JSON for the existing shared-identity pilot.

#### Scenario: Selected paths have explicit core roles

r[valence.strict_rust_admission.effect_closure_policy.roles]

- GIVEN the closure policy is exported
- WHEN module-role assignments are reviewed
- THEN the two existing shared-identity production source paths MUST each appear exactly once with role `core`.
- AND no broader source path or wildcard call matcher MAY be introduced.

#### Scenario: Exact call summaries are identity bound

r[valence.strict_rust_admission.effect_closure_policy.calls]

- GIVEN a selected non-macro call is not resolved as one selected local function, including apparent `core` or `alloc` constructors whose names could be shadowed
- WHEN Valence admits that call
- THEN its policy entry MUST bind the exact call-site ID, review label, callee label, provider kind/name/version/source/target/features, effect set, and evidence BLAKE3.
- AND it MUST NOT declare `unknown`, erase a direct forbidden effect, or claim dependency implementation verification.

### Requirement: Initial fail-closed closure evidence

r[valence.strict_rust_admission.effect_closure_initial] Valence MUST retain an initial closure bundle produced before reviewed call summaries are added.

#### Scenario: Unadmitted calls deny closure

r[valence.strict_rust_admission.effect_closure_initial.denied]

- GIVEN the existing direct deterministic-core check passes but the closure policy contains no call summaries
- WHEN `cargo octet admit --profile deterministic-core --closure -- --target x86_64-unknown-linux-gnu` runs
- THEN the command MUST deny closure for unknown or unadmitted edges.
- AND the retained receipt MUST expose stable call-site identities suitable for review.

### Requirement: Final bounded closure admission

r[valence.strict_rust_admission.effect_closure_final] Valence MUST retain an admitted effect-closure bundle for the existing two-file shared-identity scope and independently verify its stored artifact integrity.

#### Scenario: Exact policy admits selected source closure

r[valence.strict_rust_admission.effect_closure_final.admitted]

- GIVEN the direct check passes under an explicit `x86_64-unknown-linux-gnu` target, addresses match, module roles are valid, exact summaries match the resolved providers, and propagated effects satisfy `deterministic-core`
- WHEN closure admission runs
- THEN status MUST be `admitted` with no closure diagnostics.
- AND `cargo octet admit verify --artifact-dir PATH` MUST report the final bundle valid.

#### Scenario: Unchanged replay preserves identity

r[valence.strict_rust_admission.effect_closure_final.repeat]

- GIVEN source, baseline, policy, lockfile, features, target, toolchain, and profile catalog remain unchanged
- WHEN closure admission is replayed
- THEN receipt identity and canonical nodes, calls, providers, effects, and status MUST match the retained final bundle.

#### Scenario: Stale or tampered evidence is rejected

r[valence.strict_rust_admission.effect_closure_final.negative]

- GIVEN a call summary, provider identity, parent artifact, receipt, status, summary, or provenance artifact is stale or modified
- WHEN admission or stored-bundle verification runs
- THEN Valence MUST receive a nonzero result with a deterministic stale, mismatch, or integrity diagnostic.

### Requirement: Closure claim remains bounded

r[valence.strict_rust_admission.effect_closure_boundaries] Valence MUST describe closure admission as conservative selected source-token propagation under exact reviewed call summaries.

#### Scenario: Documentation preserves non-claims

r[valence.strict_rust_admission.effect_closure_boundaries.non_claims]

- GIVEN an operator reads the policy, guide, receipt, status, summary, provenance, or lifecycle evidence
- WHEN the admitted result is described
- THEN it MUST distinguish the two selected files, broader unadmitted `valence-core`, and unverified dependency implementations.
- AND it MUST NOT claim complete name resolution, macro or trait-dispatch completeness, MIR closure, runtime reachability or ordering, panic freedom, semantic determinism, compiler correctness, whole-program correctness, verifier soundness, signer authenticity, authorization, coherent trust-root replacement resistance, or release eligibility.
