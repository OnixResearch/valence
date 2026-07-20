# Valence artifact-auth non-production canary run 001

This product-owned archive records the bounded public evidence produced on 2026-07-20 from Valence `6fb2473503ec8b72f72f2857cffee08e65b6bec6` with artifact-auth `799459346d5416fbd7b9f55840a7371441b55afa`.

## Observed result

- An ephemeral non-production Ed25519 memory signer executed real signed local Git COB mutations.
- Valence reloaded the observed event from cryptographically checked physical history, captured RecordedOnly Evidence IR, attached and materialized it, then passed fresh-process replay with receipt `blake3:7672dfa1941ed027f774561d0a99b2ad256eea8882b744b374a0c33bbc10aaaf`.
- Replacing persisted authorization with an empty actor-binding set caused replay to fail closed.
- A later fresh process returned `artifact-auth operational currentness requires actor authorization`.

## Members

`harness.rs` is the exact temporary Rust example compiled against the landed revision. `artifacts/` contains the public Evidence IR and lint receipt. The other JSON files and log preserve profiles, authorization observations, signed COB identities, replay, and authorization-removal behavior. `manifest.ncl` declares the claim boundary; its cross-consumer revisions are review linkage, not a joint signature or attestation. `BLAKE3SUMS` binds every regular member except itself.

Private signing material, mutable Git repository state, and capability-root state are intentionally absent.

## Validation

```text
nix shell nixpkgs#nickel -c nickel typecheck manifest.ncl
./hash-evidence.rs . > regenerated && cmp BLAKE3SUMS regenerated
```

A negative fixture containing a symlink must be rejected by `hash-evidence.rs`. JSON and log members must contain no private-key marker or serialized secret-key field.

## Non-claims

This archive is non-production RecordedOnly evidence. It does not establish live Radicle membership, global revocation freshness, production signer custody, protocol or COB reduction authority, lifecycle handoff, release interpretation, production rollout, or standalone authority.
