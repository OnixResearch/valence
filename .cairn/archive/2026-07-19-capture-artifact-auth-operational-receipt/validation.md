# Validation: Valence artifact-auth operational receipt

Validated on 2026-07-19 from implementation commits `a18856d` and `e72a957`.

## Focused evidence

- `cargo test -p valence-radicle operational_receipt` passed capability-root write, authorized signed Git COB attachment, materialization, reopen, independent replay, and negative sequence/authorization/operation/payload drift cases.
- The positive test reloads its `ObservedCobEvent` from actual signed COB history; it does not reconstruct protocol verification or sequence from caller text.
- `cargo fmt -p valence-radicle` passed.
- `cargo clippy -p valence-radicle --all-targets -- -D warnings` passed.
- No `valence-core` source changed, preserving the existing pure-core boundary; capability-root and Git effects remain in `valence-radicle`.
- Cairn validate plus proposal, design, and tasks gates passed.

## Full evidence

- `cargo fmt --check`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `nix flake check -L --option builders ''` passed after the persisted-event reload change.
- Cairn sync dry-run passed with receipt `d552da02ec878e50e16efac9e21a34f77fef4f4b0bde6768274c666c0647439b`.
- Cairn sync execution passed with receipt `21ed761b6c6afab5c88b4c4e4c022e0b51a565422b55f7e26f4694650191fb6c`; all four accepted requirement IDs are present in `cairn/specs/artifact-auth-operational-receipt/spec.md`.

## Bounded result and blocker

The exercised shell persists real Evidence IR bytes under a capability root and a reference-only identity in a separately signed Git COB entry, then materializes and independently replays them. Currentness is bounded to the validated authorization profile and exact actor/operation. Valence still has no live Radicle membership/global revocation source, the external bytes remain operator-managed, and no cross-consumer admission review exists. The artifact therefore remains `RecordedOnly`; protocol authority is unchanged and rollback remains available.

The advisory VibeThinker audit was unavailable with `fetch failed`; deterministic source, test, Cairn, Git, Evidence IR, and Nix evidence remains authoritative.
