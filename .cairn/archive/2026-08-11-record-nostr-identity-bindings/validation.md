# Validation

Date: 2026-08-11

## Producer identity

- UCAN source revision: `277ab768eb532ac0c715a3bbff2a7dc37a702f11`.
- UCAN credential revision: `c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026`.
- Basalt source revision: `bb8fb5f4a763f532df96b6e65407163ac668834e`.
- Basalt schema BLAKE3: `45824d99ae765e885f085ef2c8c8294343415af8e5489c35509c3e255476e258`.
- Basalt implementation BLAKE3: `ced0ffcfe006a48ffd8f2b856ad0da9641a03a24e6ebda3877163c398252897c`.
- Basalt policy BLAKE3: `9f5df806b5fac55a35a63f58c86e4218de1414141c20c999cbd140ea3c0556a2`.

## Checks

- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace`: passed, including 275 `valence-core` tests.
- Focused Nostr core tests: passed, 8 positive and negative cases.
- Capability-rooted CLI tests: passed, 3 cases.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Nickel profile typecheck, generated JSON freshness, and negative fixtures: passed through `nix flake check`.
- `nix flake check path:$PWD -L`: passed.
- Native Cairn validation: passed with 7 requirements and 13 scenarios in the delta.

## Boundaries

All emitted Nostr identity evidence is `RecordedOnly` and has `property_count: 0`.

The checks do not authenticate producers, establish authority, or prove relay, DNS, signer, clock, replay-store, deployment, or release facts.
