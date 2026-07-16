# valence-radicle

`valence-radicle` is the AGPL imperative adapter for recording Valence Evidence IR links in signed local Radicle Collaborative Objects. It uses the exact reviewed `radicle-cob` release declared by the workspace, writes only `refs/rad/<actor>/cobs/org.onixresearch.valence.evidence/<object-id>`, and delegates identity, authorization-input, reduction, role, non-claim, binding, and snapshot decisions to the pure `valence-core::cob_evidence` module.

The executable is preview-first:

```text
valence-radicle mutate ...                  # plan; no writes
valence-radicle mutate ... --execute        # sign through SSH agent and update one local COB ref
valence-radicle snapshot ...                # saved, recorded-only handoff
valence-radicle verify-binding ...
valence-radicle verify-snapshot ...
```

See [`../../docs/radicle-cob-evidence.md`](../../docs/radicle-cob-evidence.md) for request shapes, operator flow, claim boundaries, privacy/retention limits, backup expectations, and Cairn handoff.

This crate does not run a Radicle node, fetch peers, seed repositories, manage private keys, mutate Cairn lifecycle files, or make evidence-correctness or release-readiness claims.
