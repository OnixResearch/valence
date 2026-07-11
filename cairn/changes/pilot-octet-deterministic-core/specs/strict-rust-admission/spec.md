# Strict Rust Admission Specification

## Purpose

Defines Valence's bounded downstream adoption of Octet strict-profile evidence for selected deterministic identity cores.

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
