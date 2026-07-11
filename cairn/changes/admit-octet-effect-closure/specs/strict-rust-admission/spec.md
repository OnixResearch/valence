# Strict Rust Admission Specification

## Purpose

Extends Valence's bounded Octet pilot with conservative selected-source effect-closure evidence.

## Requirements

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

- GIVEN a selected call is not resolved as a local selected function or tool-owned intrinsic
- WHEN Valence admits that call
- THEN its policy entry MUST bind the exact call-site ID, review label, callee label, provider kind/name/version/source/target/features, effect set, and evidence BLAKE3.
- AND it MUST NOT declare `unknown`, erase a direct forbidden effect, or claim dependency implementation verification.

### Requirement: Initial fail-closed closure evidence

r[valence.strict_rust_admission.effect_closure_initial] Valence MUST retain an initial closure bundle produced before reviewed call summaries are added.

#### Scenario: Unadmitted calls deny closure

r[valence.strict_rust_admission.effect_closure_initial.denied]

- GIVEN the existing direct deterministic-core check passes but the closure policy contains no call summaries
- WHEN `cargo octet admit --profile deterministic-core --closure` runs
- THEN the command MUST deny closure for unknown or unadmitted edges.
- AND the retained receipt MUST expose stable call-site identities suitable for review.

### Requirement: Final bounded closure admission

r[valence.strict_rust_admission.effect_closure_final] Valence MUST retain an admitted effect-closure bundle for the existing two-file shared-identity scope and independently verify its stored artifact integrity.

#### Scenario: Exact policy admits selected source closure

r[valence.strict_rust_admission.effect_closure_final.admitted]

- GIVEN the direct check passes, addresses match, module roles are valid, exact summaries match the resolved providers, and propagated effects satisfy `deterministic-core`
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
- AND it MUST NOT claim complete name resolution, macro or trait-dispatch completeness, MIR closure, runtime reachability or ordering, panic freedom, semantic determinism, compiler correctness, whole-program correctness, verifier soundness, or release eligibility.
