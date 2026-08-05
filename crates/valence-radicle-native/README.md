# valence-radicle-native

`valence-radicle-native` is the process-isolated AGPL shell for built-in Radicle issue and patch mutation.

It deliberately does **not** depend on `valence-core`. Upstream Radicle enables `serde_json/preserve_order`; loading that feature into the Valence core process changes the canonical map backend and can invalidate evidence identities. The reviewed migration plan JSON is therefore the explicit process boundary.

```text
valence-radicle-native import \
  --repository /var/lib/radicle/storage/<RID> \
  --plan migration-plan.json \
  --output-root evidence/radicle \
  --output native-observations.json             # validation preview, no writes

valence-radicle-native import ... --execute      # SSH-agent signer + built-in COB writes
```

The executable revalidates the plan BLAKE3, signer DID, repository RID, Git objects, operation shape, built-in evaluator state, attribution markers, lifecycle, labels, patch range, and changed-ref confinement. Successful execution writes only normalized observation JSON. `valence-radicle forge-bind` must separately validate those observations and create the Valence binding receipt.

See [`../../docs/radicle-forge-operations.md`](../../docs/radicle-forge-operations.md).
