# Evidence IR Specification

## Purpose

Restores compatibility between Valence's Nickel-authored lifecycle policy and the supported Cairn policy parser, including function-address evidence gates.

## Requirements

### Requirement: Nickel contracts cover the supported Cairn policy field set
r[valence.evidence_ir.cairn_policy_compatibility.nickel_contract] Valence MUST keep its Nickel policy contract compatible with every required field in the supported Cairn policy schema before exporting generated JSON.

#### Scenario: Current policy contract exports
r[valence.evidence_ir.cairn_policy_compatibility.fixtures]
- GIVEN the reviewed Valence Nickel policy includes the complete supported field set
- WHEN Cairn exports and validates the policy
- THEN export and policy parsing MUST succeed without missing-field diagnostics.

#### Scenario: Missing newly required field fails in Nickel
- GIVEN a policy fixture omits a required supported Cairn field
- WHEN Nickel contract evaluation runs
- THEN evaluation MUST fail before stale generated JSON is accepted.

### Requirement: Function-address lifecycle policy is explicit
r[valence.evidence_ir.cairn_policy_compatibility.function_address_gate] Valence MUST declare a reviewed function-address gate mode, allowed claim scopes, accepted canonical Valence policy identities, required roles, required schemas, and bounded non-claim text in Nickel.

#### Scenario: Reviewed profile identity is accepted
- GIVEN the current canonical Valence function-address profile identity and required recorded-only role/schema set
- WHEN policy validation runs
- THEN the policy MUST accept the reviewed identity without promoting function-address evidence to property proof or release eligibility.

#### Scenario: Placeholder or malformed policy identity fails
- GIVEN the accepted identity list contains a fixture placeholder, malformed BLAKE3 value, or stale unreviewed profile identity
- WHEN policy validation runs
- THEN validation MUST fail with a deterministic function-address policy diagnostic.

### Requirement: Generated Cairn policy comes only from Nickel
r[valence.evidence_ir.cairn_policy_compatibility.generated_policy] Valence MUST regenerate `cairn-policy/generated/cairn-policy.json` from the contracted Nickel source and update policy field-set compatibility metadata in the same reviewed change.

#### Scenario: Direct generated-file drift is rejected
r[valence.evidence_ir.cairn_policy_compatibility.freshness]
- GIVEN generated JSON differs from a fresh export of the Nickel source
- WHEN policy freshness validation runs
- THEN validation MUST fail and identify the generated policy as stale.

### Requirement: Default lifecycle validation parses repository policy
r[valence.evidence_ir.cairn_policy_compatibility.validation] The supported Cairn binary MUST parse Valence's default generated policy and run lifecycle validation without requiring an external substitute policy.

#### Scenario: Documented validation command reaches lifecycle checks
- GIVEN the generated policy is fresh
- WHEN the repo-documented default `cairn validate --root .` command runs
- THEN Cairn MUST scan specs and changes and MUST NOT abort on policy parsing.

### Requirement: Policy compatibility boundary is documented
r[valence.evidence_ir.cairn_policy_compatibility.docs] Valence documentation MUST identify the supported Cairn policy contract, Nickel refresh workflow, selected function-address mode, and the boundary between lifecycle policy compatibility and product evidence truth.

#### Scenario: Parse success is not promoted to evidence truth
- GIVEN the policy parses and lifecycle validation runs
- WHEN compatibility evidence is reported
- THEN Valence MUST state that the result proves configuration schema/value compatibility only, not correctness, verifier soundness, or release eligibility.

### Requirement: Policy compatibility remediation is verified
r[valence.evidence_ir.cairn_policy_compatibility.final_validation] The change MUST include positive and negative Nickel fixtures, generated-policy freshness evidence, supported-Cairn parse evidence, and default lifecycle validation evidence.

#### Scenario: Reproduced parser failure is closed
- GIVEN the repository uses its default policy path with the supported Cairn checkout
- WHEN focused policy checks and repository validation run
- THEN the policy MUST parse, malformed function-address fixtures MUST fail, and active lifecycle changes MUST be evaluated.
