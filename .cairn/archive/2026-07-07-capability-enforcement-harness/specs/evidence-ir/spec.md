# Evidence IR Specification

## Purpose

Extends `evidence-ir` with a capability enforcement harness that distinguishes receipt labels from actual sandbox enforcement.

## Requirements

### Requirement: Capability enforcement harness
r[valence.evidence_ir.capability_enforcement_harness] Valence MUST test verifier capability label policy separately from opt-in sandbox enforcement behavior.

#### Scenario: Label policy tests stay pure
r[valence.evidence_ir.capability_enforcement_harness.policy]
- GIVEN verifier capability labels and a capability policy
- WHEN the fast capability test lane runs
- THEN it MUST evaluate pure in-memory policy decisions without claiming sandbox enforcement.

#### Scenario: Enforcement availability is explicit
r[valence.evidence_ir.capability_enforcement_harness.availability]
- GIVEN sandbox or effect-denial support is unavailable on the host
- WHEN the enforcement lane reports results
- THEN it MUST report unavailable or skipped enforcement and MUST NOT report denied effects as successfully enforced.

#### Scenario: Denied effects are probed when enforcement is active
r[valence.evidence_ir.capability_enforcement_harness.denied_effects]
- GIVEN an active enforcement backend and a policy denying filesystem, process, network, clock, remote-builder, or secret effects
- WHEN denial probes run
- THEN denied effects MUST be blocked with deterministic enforcement diagnostics.

#### Scenario: Missing sandbox is not false success
r[valence.evidence_ir.capability_enforcement_harness.no_false_success]
- GIVEN enforcement probes are requested but no enforcement backend is active
- WHEN the harness completes
- THEN the result MUST be unavailable or skipped rather than passed enforcement.

#### Scenario: Capability claims remain bounded
r[valence.evidence_ir.capability_enforcement_harness.non_claims]
- GIVEN capability reports describe labels or enforcement probes
- WHEN the supported claim is stated
- THEN the report MUST state that labels are receipt metadata unless a sandboxed runner enforces them, and that neither labels nor probes prove verifier soundness or requirement satisfaction.
