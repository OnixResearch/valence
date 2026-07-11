## Why

Valence's checked-in Nickel policy contract and generated Cairn policy predate Cairn's required `function_address_gate` field. Running the repo-documented `cairn validate --root .` with the current sibling Cairn binary aborts while parsing policy with `policy missing field function_address_gate`; no lifecycle content is validated.

This is more than generated-file staleness. The Nickel source contract also lacks the new typed field, so regenerating without first updating the source of truth cannot produce a policy accepted by current Cairn. The repository needs an explicit policy-schema compatibility refresh and a rail that catches future required-field drift.

## What Changes

- Update the Nickel policy contracts with the current typed function-address gate and any other newly required policy fields.
- Declare a Valence-specific function-address policy using reviewed claim scopes, roles, schemas, non-claim boundary, and accepted current Valence policy identities.
- Recompute policy field-set compatibility metadata and regenerate the checked-in JSON from Nickel.
- Add positive compatibility checks with the supported Cairn binary and negative fixtures for missing, malformed, placeholder, or stale function-address policy values.
- Add a freshness/parse check so default Cairn validation cannot silently drift behind the Nickel source again.

## Impact

- **Lifecycle**: default `cairn validate`, proposal/design/tasks gates, and policy inspection become runnable with the supported Cairn checkout.
- **Configuration**: Nickel remains the source of truth; generated JSON is updated only through the repo-documented export path.
- **Function-address evidence**: policy acceptance is explicit and bound to reviewed Valence profile identities rather than placeholders.
- **Claims**: policy compatibility proves only schema/field/value conformance and lifecycle configuration readability, not evidence truth or release eligibility.
