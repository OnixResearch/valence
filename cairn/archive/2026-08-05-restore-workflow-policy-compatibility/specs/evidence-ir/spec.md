# Evidence IR delta: restore workflow policy compatibility

## ADDED Requirements

### Requirement: Nickel policy declares the supported workflow schema catalog
r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema] Valence MUST declare the supported workflow schema profiles, the workflow profile registry, and every parser-required policy section in contracted Nickel, with fixed catalog identifiers and deterministic state order matching the supported Cairn contract.

#### Scenario: Supported profiles and registry parse
- GIVEN the generated policy is exported from the reviewed Nickel source
- WHEN the supported Cairn parser loads the policy
- THEN the spec-driven, assurance, and no-spec-delta profiles MUST be present, the registry MUST name an allowed default profile, and ambient profile selection sources MUST be forbidden.

#### Scenario: Valence-specific sections are preserved
- GIVEN the policy is rebased onto the supported contract
- WHEN the generated policy is inspected
- THEN Valence's reviewed gate, traceability, probe, receipt, determinism, and authentication sections MUST keep their reviewed values.

### Requirement: Workflow policy drift fails in Nickel fixtures
r[valence.evidence_ir.cairn_policy_compatibility.workflow_profile_fixtures] Valence MUST keep negative Nickel fixtures that reject a policy missing the workflow profile registry or a workflow artifact missing its output path before generated JSON is accepted.

#### Scenario: Missing workflow profile registry fails
- GIVEN a fixture removes the workflow profile registry from the reviewed policy
- WHEN Nickel contract evaluation runs
- THEN evaluation MUST fail.

#### Scenario: Artifact without output path fails
- GIVEN a fixture removes the output path from a workflow artifact
- WHEN Nickel contract evaluation runs
- THEN evaluation MUST fail.

### Requirement: Workflow compatibility remediation is verified
r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation] The change MUST include supported-Cairn parse evidence, generated-policy freshness evidence, fixture evidence, default lifecycle validation evidence, and documentation of the workflow-schema v1 boundary.

#### Scenario: Workflow parser failure is closed
- GIVEN the repository uses its default policy path with the supported Cairn checkout
- WHEN policy export and default lifecycle validation run
- THEN the policy MUST parse, the workflow fixtures MUST fail closed, and active lifecycle changes MUST be evaluated.

#### Scenario: Workflow boundary is documented
- GIVEN the policy declares the workflow-schema v1 catalog
- WHEN compatibility documentation is reviewed
- THEN the documentation MUST state that profile declaration proves parser compatibility only, not evidence truth or release eligibility.
