## Why

Valence owns Evidence IR identity and roles, Radicle protocol verification, actor authorization, COB reduction, Git object measurement/storage, SSH-agent signing, lifecycle handoff, and release interpretation. The independent `artifact-auth` repository now publishes a reviewed Valence mapping profile at immutable revision `799459346d5416fbd7b9f55840a7371441b55afa`, but that profile cannot silently redefine Radicle or Evidence IR semantics.

A consumer-owned change is required to baseline current Valence protocol/evidence behavior, review the frozen mapping, select one exact source, and dual-run a bounded adapter before any standalone path is admitted. Existing Valence verification remains authoritative throughout the handoff.

## What Changes

- Review `config/consumers/valence.ncl` and `fixtures/consumers/valence.json` from the immutable standalone revision against current Radicle/Evidence IR behavior.
- Pin Cargo and Nix to one reviewed standalone source only after local source, licensing, and isolation checks pass.
- Add a pure one-way adapter from already protocol-verified actor key, entry subject, parent OIDs, related target, and profile context into standalone observations.
- Dual-run positive and tamper cases with explicit drift classes and bounded rollback.
- Keep protocol bytes/verification, actor authorization, Evidence IR identities/roles, COB/Git/SSH effects, lifecycle handoff, and release interpretation in Valence.

## Impact

- **Planned surfaces**: Valence core compatibility adapters, CLI shell composition, exact Cargo/Nix pins, mapping fixtures, migration documentation, and bounded evidence.
- **No producer-side migration**: this lifecycle package changes no Valence implementation or dependency by itself.
- **Claims**: standalone authentication does not prove Evidence IR truth, actor authorization, COB correctness, Git integrity, lifecycle readiness, or release eligibility.
