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

## Compatibility checks

r[verify valence.evidence_ir.cairn_policy_compatibility.final_validation]

`CAIRN_SOURCE=../cairn ./tools/check-cairn-policy-compatibility.rs` runs the positive Nickel fixture, all `invalid-*.ncl` negative fixtures through Cairn's exporter, byte-for-byte generated-policy freshness, supported-parser loading, and default-path lifecycle validation. Direct drift in generated JSON fails the export check.

Passing these checks proves only that Valence's Nickel configuration has the supported field/value shape, the checked-in JSON is fresh, and the pinned Cairn parser can evaluate the lifecycle tree. It does not prove evidence truth, Rust semantic correctness, verifier soundness, requirement satisfaction, or release eligibility.
