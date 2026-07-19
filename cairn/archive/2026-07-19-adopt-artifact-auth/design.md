## Context

The standalone source is `ssh://git@github.com/OnixResearch/artifact-auth.git` at revision `799459346d5416fbd7b9f55840a7371441b55afa`. Its reviewed Valence profile freezes source baseline `8eccac35fcd8aae8ef64b3d1021e7f1a53656180`, shared fields, compatibility extensions, and retained authority. Current Valence code may have advanced, so the profile is review input and adoption starts with a current baseline.

## Goals and non-goals

Goals are exact-source admission, a pure one-way adapter over already protocol-verified observations, positive and negative compatibility evidence, and reversible cutover. This change does not reconstruct Radicle signatures, authorize actors, define Evidence IR identity/roles, reduce COB state, measure or mutate Git, invoke SSH agents, decide lifecycle handoff, or interpret release eligibility.

## Decisions

### Exact source and profile identity precede implementation

Cargo and Nix SHALL resolve one immutable revision. The consumer SHALL bind the typed Valence profile and checked JSON from that revision and reject floating refs, sibling paths, duplicate packages, source mismatch, or incompatible licensing.

### Radicle verification remains upstream of the adapter

A pure adapter SHALL consume only already protocol-verified actor key, entry subject, parent OIDs, related target, and profile context. Radicle public-key/OID and Evidence IR identity metadata remain explicit compatibility extensions. A standalone verifier must not claim it reconstructed or superseded the Radicle protocol.

### Cutover is evidence-gated and reversible

Legacy and standalone projections SHALL run over identical verified observations. Equal or intentionally mapped differences may be admitted only without claim widening. Unknown/lossy fields, unrelated failures, authorization promotion, or weaker non-claims block cutover. Existing Valence behavior remains authoritative until explicit admission and through a bounded rollback release.

## Risks

- Standalone Ed25519 verification could be incorrectly substituted for Radicle protocol verification.
- Key authenticity could be promoted into actor authorization.
- Evidence IR identity or role semantics could be collapsed into authentication identity.
- The frozen baseline could be stale relative to current code.

These are addressed with a one-way post-verification adapter, product-owned authorization/identity, a fresh baseline, mapping-loss tests, and rollback.
