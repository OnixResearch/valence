# valence-radicle

`valence-radicle` is the AGPL imperative adapter for signed local Radicle Collaborative Objects.

It records Valence Evidence IR links and uses the reviewed `radicle-cob` release from the workspace.

It writes only `refs/rad/<actor>/cobs/org.onixresearch.valence.evidence/<object-id>`.

The pure `valence-core::cob_evidence` module controls identity, authorization input, reduction, roles, non-claims, bindings, and snapshots.

The executable is preview-first:

```text
valence-radicle mutate ...                  # plan without writes
valence-radicle mutate ... --execute        # sign through SSH agent and update one local COB ref
valence-radicle snapshot ...                # saved, recorded-only handoff
valence-radicle verify-binding ...
valence-radicle verify-snapshot ...
```

See [`../../docs/radicle-cob-evidence.md`](../../docs/radicle-cob-evidence.md) for request shapes, operator flow, claim boundaries, privacy/retention limits, backup expectations, and Cairn handoff.

This crate does not:

- run a Radicle node
- fetch peers
- seed repositories
- manage private keys
- mutate Cairn lifecycle files
- prove evidence correctness or release readiness
