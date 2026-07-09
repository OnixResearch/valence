## Why

Oxide has several small Rust crates and attestation-oriented repositories whose design pressure matches Valence's Evidence IR work: typed identities, indexed in-memory evidence collections, semantic diffs, and bounded attestation vocabulary. Valence should evaluate those patterns without importing downstream claims or letting external wire formats redefine stack-owned identity.

This change creates a Valence Cairn package for adapting the useful parts of `iddqd`, `newtype-uuid`, `daft`, `rats-corim`, `vm-attest`, `dice-util`, and `pki-playground` as reference inputs. The goal is a pure Evidence IR primitive layer that remains BLAKE3-centered, typed, fail-closed, and explicit about non-claims.

## What Changes

- Add a reference inventory for the Oxide primitives and attestation repositories, including license/MSRV/no-std and claim-boundary review notes.
- Define typed evidence identifiers that distinguish stack-owned BLAKE3 identities from UUID-compatible external identifiers.
- Define indexed evidence collection behavior inspired by value-derived keys and duplicate-key rejection.
- Define semantic evidence diff reports for review and migration sessions.
- Define attestation vocabulary mapping rules that preserve Valence non-claim boundaries.
- Require positive and negative fixture matrices before any dependency or copied design is adopted into `valence-core`.

## Impact

- Valence gains a structured intake path for Oxide design references while keeping `valence-core` pure.
- Downstream consumers get typed identity and diff primitives only after fixture-backed compatibility review.
- MPL/Apache/MIT reference repositories remain references unless a later implementation change explicitly chooses dependency or reimplementation boundaries.
