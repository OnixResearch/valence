# Design: Restore workflow policy compatibility

## Context

The supported Cairn policy parser validates workflow schemas against `cairn.workflow-schema.v1` and requires a profile registry plus eleven additional top-level sections. Valence's policy stopped at the older artifact shape (`id`, `generates`, `requires`) and parsed nothing past the `schemas` field.

## Decision

**Choice:** Rebase Valence's policy onto the supported Cairn policy contract, then re-apply Valence's reviewed overrides. Concretely:

1. Take the supported Cairn `default.ncl` as the structural base so every parser-required section, bound, non-claim, and fixed catalog value (`spec-driven`/`assurance`/`no-spec-delta` profiles, `cairn-local` developer-loop profile, exact `states` order) matches the validator by construction.
2. Re-apply Valence's reviewed sections over that base: `traceability_policy`, `stack_provenance_gate`, `function_address_gate`, `workspace_contract_matrix_gate`, `probe_policy`, `receipt_schemas`, `determinism_replay_policy`, and `determinism_surface_inventory` keep their reviewed Valence values; `name` stays `valence-default`.
3. Adapt only the operator-facing values of the developer-loop policy to Valence's checks (format, default validation, core-purity smoke); the fixed initial profile catalog keeps its required identifier.
4. Take the supported Cairn `contracts.ncl` as the contract base, restore Valence's exact-value function-address predicates, and import the two supporting contract files (`nominal-evidence-identities.ncl`, `lifecycle-store-contracts.ncl`).

Direct hand-editing of generated JSON remains prohibited. The generated file is produced only by `cairn policy export`, and the flake freshness rail compares it byte-for-byte against a raw Nickel export.

## Field-set digest

The compatibility digest is BLAKE3 over the sorted, newline-terminated list of supported policy field names present in the policy. Valence's policy previously carried five of the sixteen material fields; after this port it carries all sixteen, so the digest equals the supported field-set digest `9f44153637df26127588d81e70bb2095f07f64c4e7d5c009608f6c70b6ca961a`. The digest computation was verified against the previous value (`5b6ca8c0…`) before the port.

## Fixtures

Existing function-address fixtures are unchanged and keep their exact-value Nickel predicates. Two new negative fixtures follow the repository idiom of merging over `../default.ncl` and re-applying `Contracts.Policy`:

- `invalid-missing-workflow-profile-policy.ncl` removes `workflow_profile_policy` and fails in Nickel because the contract requires the field.
- `invalid-workflow-artifact-missing-output-path.ncl` removes `output_path` from the spec-driven proposal artifact and fails in Nickel because the artifact contract requires the field.

## Boundaries

- Policy compatibility proves schema/field/value conformance and lifecycle configuration readability only.
- Declaring the assurance and no-spec-delta profiles does not change how existing Valence changes flow; the default profile remains `spec-driven`.
- Parse success is not evidence truth, verifier soundness, or release eligibility.

## Accepted-spec conformance repair

The delta merger requires every accepted requirement block to carry exactly one bare `r[...]` identity before its first scenario. The accepted `evidence-ir` spec carried two bare identities in `Function-address profile policy identity is deterministic` (`.material` and `.digest`), which blocked every sync into that spec. The block is split into two sibling requirements — one for deterministic material, one for the derived digest — preserving all five requirement identifiers, all normative text, and all three scenarios. This is a structural conformance repair only; no requirement text changed.
