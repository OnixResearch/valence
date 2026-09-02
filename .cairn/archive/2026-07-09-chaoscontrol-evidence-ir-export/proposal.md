## Why

ChaosControl has rich dogfood, replay, readiness, and determinism evidence, but much of it remains repo-local JSON/Nickel contract material. Valence should provide an adapter contract so ChaosControl can export portable Evidence IR artifacts without Valence reimplementing VMM or replay semantics.

## What Changes

- Define ChaosControl-to-Valence Evidence IR export rows for replay verdicts, dogfood receipts, assertion summaries, and readiness reports.
- Preserve ChaosControl ownership of VMM/replay semantics.
- Add non-claims for universal determinism, hosted-product parity, and replay completeness.

## Impact

- ChaosControl evidence can flow into Valence, Cairn, and Mantle as portable identity/linkage artifacts.
- Valence remains the evidence identity layer, not a ChaosControl semantic verifier.
