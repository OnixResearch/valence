# Cairn policy compatibility

Valence keeps its lifecycle policy in `cairn-policy/default.ncl`, checked by the typed contracts in `cairn-policy/contracts.ncl`. The generated `cairn-policy/generated/cairn-policy.json` file is an export artifact and must not be edited directly.

The supported parser is the canonical local Cairn checkout selected by `CAIRN_SOURCE` (default `../cairn`). Refresh the generated policy with:

```bash
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- policy export \
  --output cairn-policy/generated/cairn-policy.json
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- policy export --check
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- validate --root .
```

## Reviewed function-address gate

r[impl valence.evidence_ir.cairn_policy_compatibility.function_address_gate]

Valence selects `optional` mode. An absent function-address receipt is therefore not required for lifecycle validation, while any supplied receipt remains subject to Cairn's function-address checks. The gate accepts only claim scope `function_address.v1.lifecycle_policy_conformance` and the current canonical Valence function-address profile identity:

```text
e5f4f35b3b339fdefcf7a195910c4eb04388dc66de8452891bbceabb60b557a3
```

That identity is the BLAKE3 digest produced by `valence_core::function_address_profile::function_address_policy_identity`. The policy also requires the reviewed sidecar/profile/Kamacite roles and schemas. Placeholder, stale, malformed, empty, or weakened fixture values fail the Nickel contract.

The workspace contract matrix gate is carried explicitly in disabled mode so the complete supported policy field set remains reviewable.

## Artifact substance and authenticated evidence

The gate policy carries Cairn's fail-closed proposal, design, requirement, task,
and dependency-marker substance thresholds. The authenticated-evidence policy
uses the non-promoting `diagnostic` profile, accepts the reviewed corrected
Preserves profile, and binds the archived Valence integrity receipt plus the
stack-provenance and function-address authentication profiles by BLAKE3. Empty
trusted-producer configuration grants no signature authority. These inputs make
the policy schema complete; they do not promote Valence to an authenticated
Onix release profile.

Nickel sources own the two authentication profiles. The JSON files under
`cairn-policy/generated/` are freshness-checked exports and must not be edited
directly. The archived integrity receipt is a byte-for-byte consumer copy of
the Cairn acceptance receipt for Valence revision
`1f4f6fe903a34c22d2cda53cd534ce58bbf84147`.

## Workflow schema catalog

r[impl valence.evidence_ir.cairn_policy_compatibility.workflow_schema]

The supported Cairn parser requires workflow-schema v1 policy content: typed
artifact entries with output paths, path kinds, structural admission classes,
and lifecycle stages, plus schema-level bounds, deterministic state order,
spec-effect and task-link classes, receipt fields, and required non-claims.
Valence declares the full supported profile catalog (`spec-driven`,
`assurance`, and `no-spec-delta`) and the workflow profile registry in
contracted Nickel. The default profile remains `spec-driven`; declaring the
assurance and no-spec-delta profiles restores parser compatibility without
changing how existing Valence changes flow.

The registry forbids ambient profile selection sources (`user-global`,
`environment`, `agent-prompt`, `remote-package`). Negative fixtures prove that
a policy missing the workflow profile registry or an artifact missing its
output path fails Nickel contract evaluation before generated JSON is
accepted.

Profile declaration proves parser compatibility only. It does not prove
evidence truth, workflow artifact quality, or release eligibility.

## Compatibility checks

r[verify valence.evidence_ir.cairn_policy_compatibility.final_validation]

`CAIRN_SOURCE=../cairn ./tools/check-cairn-policy-compatibility.rs` runs the positive Nickel fixture, all `invalid-*.ncl` negative fixtures through Cairn's exporter, byte-for-byte generated-policy freshness, supported-parser loading, and default-path lifecycle validation. Direct drift in generated JSON fails the export check.

Passing these checks proves only that Valence's Nickel configuration has the supported field/value shape, the checked-in JSON is fresh, and the pinned Cairn parser can evaluate the lifecycle tree. It does not prove evidence truth, Rust semantic correctness, verifier soundness, requirement satisfaction, or release eligibility.
