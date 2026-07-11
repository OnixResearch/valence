## Context

The current Valence `cairn-policy/contracts.ncl` ends its policy field set with `stack_provenance_gate` followed by runtime evidence fields. Current Cairn requires a typed `function_address_gate`; its canonical Nickel contract also includes a defaulted workspace contract matrix gate. Valence's generated JSON mirrors the older contract and fails before Cairn can scan specs or changes.

The policy file carries a field-set digest and a refresh command, so both source and generated compatibility metadata must move together.

## Decisions

### Keep Nickel authoritative

**Choice:** Port the required policy contracts into `cairn-policy/contracts.ncl`, define Valence-owned values in `cairn-policy/default.ncl`, recompute the field-set compatibility digest, and regenerate `cairn-policy/generated/cairn-policy.json` through Cairn's policy exporter. Direct hand-editing of generated JSON is prohibited.

### Review function-address policy values instead of copying placeholders

**Choice:** Select disabled, optional, or required mode deliberately for Valence; enumerate accepted claim scopes, required roles, required schemas, and the bounded lifecycle non-claim; and populate accepted Valence policy hashes from the current canonical function-address profile identity. Placeholder hashes from Cairn fixtures are not valid production policy.

The policy must keep recorded-only function-address evidence separate from property proof and release eligibility.

### Refresh the complete required field set

**Choice:** Compare Valence's contract with the supported Cairn policy contract and add every required field needed for parse compatibility, including defaulted fields when explicitly carrying them improves reviewability. The remediation is not limited to making the first parser error disappear.

### Add compatibility and freshness rails

**Choice:** A focused check exports Nickel, compares it byte-for-byte with checked-in generated JSON, asks the supported Cairn binary to parse the policy, and runs Valence lifecycle validation with the default policy path. Negative fixtures remove or corrupt function-address fields and must fail with deterministic policy diagnostics.

If the supported Cairn checkout changes its required policy field set, this check fails before normal lifecycle work relies on the stale policy.

### Keep product and lifecycle identities separate

The function-address gate references Valence profile identities but does not recompute product evidence inside Nickel or Cairn. Product canonicalization remains in `valence-core`; lifecycle policy records reviewed accepted identities.

## Risks / Trade-offs

- Tying compatibility tests to a supported Cairn revision requires maintainers to refresh policy intentionally when Cairn evolves.
- Choosing optional or required mode affects release-readiness behavior and must be reviewed, not inferred from parser defaults.
- A parseable policy can still encode poor governance; negative fixtures cover shape and bounded values, while policy review owns intent.
