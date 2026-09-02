## Phase 1: Nickel policy source

- [x] [serial] Add current typed function-address and other required policy-field contracts to the Nickel source of truth. r[valence.evidence_ir.cairn_policy_compatibility.nickel_contract]
- [x] [serial] Define reviewed Valence function-address mode, claim scopes, accepted profile identities, roles, schemas, and bounded non-claim text without placeholders. r[valence.evidence_ir.cairn_policy_compatibility.function_address_gate]
- [x] [parallel] Add positive Nickel fixtures for the reviewed policy and negative fixtures for missing fields, invalid modes, malformed or placeholder hashes, empty roles/schemas, and weakened boundaries. r[valence.evidence_ir.cairn_policy_compatibility.fixtures]

## Phase 2: Generated compatibility artifact

- [x] [serial] Recompute policy field-set compatibility metadata and regenerate checked-in policy JSON exclusively through Cairn's Nickel exporter. r[valence.evidence_ir.cairn_policy_compatibility.generated_policy]
- [x] [serial] Add a freshness check comparing exported Nickel policy with the generated JSON. r[valence.evidence_ir.cairn_policy_compatibility.freshness]
- [x] [serial] Add a supported-Cairn parse and default-path lifecycle validation check. r[valence.evidence_ir.cairn_policy_compatibility.validation]

## Phase 3: Documentation and evidence

- [x] [serial] Document the supported Cairn policy compatibility boundary, refresh command, reviewed function-address mode, and bounded lifecycle claim. r[valence.evidence_ir.cairn_policy_compatibility.docs]
- [x] [serial] Run Nickel positive/negative fixtures, policy export freshness, default `cairn validate`, all active change gates, and repository checks. r[valence.evidence_ir.cairn_policy_compatibility.final_validation]
