# Tasks

## 1. Baseline and contract port

- [x] [serial] 1.1 Reproduce the parser failure with the supported Cairn checkout (`cairn.workflow.policy.shape at schemas: missing field output_path`). r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
- [x] [serial] 1.2 Port the workflow-schema v1 profile catalog and workflow profile registry into contracted Nickel. r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema]
- [x] [serial] 1.3 Add every newly required top-level policy section with Valence-reviewed values, including the nominal-identity and lifecycle-store contract imports. r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema]
- [x] [serial] 1.4 Extend `cairn-policy/contracts.ncl` to cover the full supported field set while keeping Valence's exact-value function-address tightening. r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema]

## 2. Regeneration and fixtures

- [x] [serial] 2.1 Verify the field-set digest scheme against the previous digest, then confirm the full supported material list reproduces the supported field-set digest. r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema]
- [x] [serial] 2.2 Regenerate `cairn-policy/generated/cairn-policy.json` exclusively through Cairn's Nickel exporter and confirm byte-for-byte freshness against a raw Nickel export. r[valence.evidence_ir.cairn_policy_compatibility.workflow_schema]
- [x] [serial] 2.3 Add negative Nickel fixtures for a missing workflow profile registry and a workflow artifact missing `output_path`. r[valence.evidence_ir.cairn_policy_compatibility.workflow_profile_fixtures]

## 3. Validate

- [x] [serial] 3.1 Run the positive fixture, all negative fixtures, and policy export freshness through `cairn policy export --check`. r[valence.evidence_ir.cairn_policy_compatibility.workflow_profile_fixtures]
- [x] [serial] 3.2 Run default `cairn validate --root .` with the supported Cairn checkout and confirm lifecycle content is scanned. r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
- [x] [serial] 3.3 Update `docs/cairn-policy-compatibility.md` with the workflow-schema v1 contract boundary. r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
- [x] [serial] 3.4 Run `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and the core purity smoke. r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
- [x] [serial] 3.5 Run the repo compatibility tool (`tools/check-cairn-policy-compatibility.rs`) and all active change gates. r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
- [x] [serial] 3.6 Repair the accepted `evidence-ir` requirement block that carried two bare requirement identities, preserving every identifier, normative sentence, and scenario. r[valence.evidence_ir.cairn_policy_compatibility.workflow_validation]
