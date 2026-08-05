# Change: Restore workflow policy compatibility

## Why

The supported Cairn checkout now requires workflow-schema v1 policy content: typed artifact entries with `output_path`, `path_kind`, `structural_admission`, and `stage`; schema-level `bounds`, `states`, `spec_effect`, `task_link_class`, `receipt_fields`, and `required_non_claims`; a `workflow_profile_policy` registry that must name the `spec-driven`, `assurance`, and `no-spec-delta` profiles; and additional required top-level policy sections (`agent_guidance_policy`, `developer_loop_policy`, `document_collaboration_policy`, `generated_agent_workflow_policy`, `machine_contract_policy`, `lifecycle_store_policy`, `nominal_identity_policy`, plus the catalog sections `agent_completion_policy`, `cross_repo_dependency_policy`, and `global_registry_policy`).

Valence's checked-in Nickel contract and generated policy predate this schema. Every lifecycle command (`validate`, `gate`, `sync`, `archive`) aborts while parsing policy with `cairn.workflow.policy.shape at schemas: workflow schema shape is invalid: missing field output_path`; no lifecycle content is validated.

As with the earlier function-address drift, this is more than generated-file staleness: the Nickel source contract also lacks the new typed fields, so regenerating without updating the source of truth cannot produce a policy the supported Cairn accepts.

## What Changes

- Port the workflow-schema v1 profile catalog (`spec-driven`, `assurance`, `no-spec-delta`) and the workflow profile registry into the contracted Nickel policy.
- Add every newly required top-level policy section with Valence-reviewed values, including a Valence developer-loop profile and the shared nominal-identity and lifecycle-store contract imports.
- Extend `cairn-policy/contracts.ncl` so the typed contract covers the full supported field set; keep Valence's exact-value function-address tightening.
- Keep Valence-specific policy sections (gates, traceability, probe, receipts, determinism, authentication profiles) byte-identical to their reviewed values.
- Recompute policy field-set compatibility metadata; the supported field set now matches the full Cairn material list.
- Regenerate `cairn-policy/generated/cairn-policy.json` exclusively through Cairn's Nickel exporter.
- Add negative Nickel fixtures for a missing workflow profile registry and a workflow artifact missing `output_path`.

## Non-Goals

- Changing Valence lifecycle stages, gate stages, task markers, or receipt contracts.
- Adopting the assurance or no-spec-delta workflows for existing changes; declaring the profiles only restores parser compatibility.
- Claiming evidence truth, verifier soundness, or release eligibility from policy parse success.

## Impact

- **Lifecycle**: default `cairn validate`, proposal/design/tasks gates, sync, and archive become runnable with the supported Cairn checkout.
- **Configuration**: Nickel remains the source of truth; generated JSON is updated only through the repo-documented export path.
- **Affected files**: `cairn-policy/default.ncl`, `cairn-policy/contracts.ncl`, `cairn-policy/nominal-evidence-identities.ncl`, `cairn-policy/lifecycle-store-contracts.ncl`, `cairn-policy/generated/cairn-policy.json`, `cairn-policy/fixtures/`, `docs/cairn-policy-compatibility.md`.
- **Testing**: positive and negative Nickel fixtures, policy export freshness, supported-Cairn parse, default lifecycle validation, and the repo compatibility tool.
