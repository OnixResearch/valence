# Evidence IR Specification

## Purpose

Extends `evidence-ir` with typed verifier capability labels inspired by Unison abilities.

## Requirements

### Requirement: Verifier capability labels
r[valence.evidence_ir.verifier_capabilities] Valence MUST model verifier effects with typed capability labels so receipts distinguish pure checks from checks that depend on external effects.

#### Scenario: Capability sets serialize deterministically
r[valence.evidence_ir.verifier_capabilities.serialization]
- GIVEN a verifier capability set
- WHEN Valence serializes it into a report or receipt
- THEN the capabilities MUST be typed, sorted deterministically, and round-trip through parsing.

#### Scenario: Receipts carry capabilities
r[valence.evidence_ir.verifier_capabilities.receipts]
- GIVEN a verifier report or receipt is emitted
- WHEN the verifier used known capabilities
- THEN the report SHOULD include its capability set as additive evidence metadata.

#### Scenario: Pure checks are identified
r[valence.evidence_ir.verifier_capabilities.pure]
- GIVEN a verifier capability set contains only `Pure`
- WHEN Valence evaluates replay or cache eligibility
- THEN the result MAY be treated as pure-cache eligible subject to matching input and verifier identities.

#### Scenario: Denied capabilities fail policy
r[valence.evidence_ir.verifier_capabilities.policy]
- GIVEN policy denies network, secret, remote-builder, process, clock, or filesystem capabilities for a gate
- WHEN a verifier report carries a denied capability
- THEN Valence MUST fail closed with a deterministic capability-policy diagnostic.

#### Scenario: Capability tests cover positive and negative cases
r[valence.evidence_ir.verifier_capabilities.tests]
- GIVEN capability fixtures and policies
- WHEN the capability gate runs
- THEN valid pure/local capabilities MUST pass and unknown or denied capabilities MUST fail deterministically.

#### Scenario: Capability boundary is documented
r[valence.evidence_ir.verifier_capabilities.docs]
- GIVEN documentation or receipts describe verifier capabilities
- WHEN the supported claim is stated
- THEN the output MUST state that labels are receipt evidence unless a sandboxed runner enforces them, and that labels do not prove verifier soundness or requirement satisfaction.
