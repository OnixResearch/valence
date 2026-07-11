## Why

Valence's archived deterministic-core pilot admits two shared-identity production source files under Octet's direct source/HIR and function-address policy. That evidence explicitly does not cover transitive callees or dependency effects. Octet now provides a separate conservative selected-source closure rail, so Valence should dogfood it without widening the selected source universe or weakening the existing profile.

## What Changes

- Add a typed Nickel workspace effect-closure policy assigning the two selected files the `core` role and binding every non-local call summary to its exact function-address-derived call-site ID, provider version/source/features/target identity, effect set, and evidence reference.
- Retain an initial denied closure bundle before call summaries are added, demonstrating that unknown and unadmitted edges fail closed.
- Retain a final admitted and independently verified closure bundle plus repeat-replay evidence.
- Extend the operator guide with the new command, policy regeneration, review workflow, and explicit source-token/dependency-summary non-claims.

## Impact

- **Policy**: New `policy/octet-effect-closure.ncl`, local copied contract, and generated root `octet-effect-closure.json` runtime input.
- **Evidence**: Initial denied and final admitted bundles under `evidence/octet/effect-closure/`.
- **Source**: The selected production implementation remains unchanged unless a directly inferred forbidden effect requires an explicit safe refactor; no broad suppression is permitted.
- **Validation**: Existing workspace tests/Clippy/Nix checks plus Octet closure replay, verification, stale/tamper negatives, and compatible Cairn gates.

The resulting claim remains bounded to two selected production source files, conservative source-token inference, exact reviewed call summaries, and the recorded toolchain/target/Cargo graph. It does not verify dependency implementations or establish MIR closure, runtime reachability, semantic determinism, whole-program correctness, or release eligibility.
