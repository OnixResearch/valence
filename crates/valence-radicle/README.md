# valence-radicle

`valence-radicle` is the AGPL imperative adapter for recording Valence Evidence IR links and bounded forge metadata in signed local Radicle Collaborative Objects. It uses exact reviewed Radicle collaboration dependencies, confines evidence sidecars to `org.onixresearch.valence.evidence`, confines forge imports to the importer's built-in `xyz.radicle.issue` or `xyz.radicle.patch` namespace, and delegates identity, planning, admission, non-claim, binding, and snapshot decisions to pure `valence-core` modules.

The executable is preview-first:

```text
valence-radicle mutate ...                  # plan without writes
valence-radicle mutate ... --execute        # sign through SSH agent and update one local COB ref
valence-radicle snapshot ...                # saved, recorded-only handoff
valence-radicle verify-binding ...
valence-radicle verify-snapshot ...
valence-radicle forge-plan ...              # deterministic plan; no writes
valence-radicle forge-bind ...
valence-radicle verify-forge-binding ...
valence-radicle ci-admission ...            # external guarded-CAS input only
valence-radicle verify-ci-admission ...
```

See [`../../docs/radicle-cob-evidence.md`](../../docs/radicle-cob-evidence.md) for sidecar operations and [`../../docs/radicle-forge-operations.md`](../../docs/radicle-forge-operations.md) for normalized metadata, native import, CI-admission, and external canonical-guard boundaries.

Native built-in issue and patch mutation is process-isolated in [`../valence-radicle-native`](../valence-radicle-native/README.md), because upstream Radicle's `serde_json/preserve_order` feature must not change Valence canonical JSON behavior.

This crate does not run a Radicle node, fetch peers, seed repositories, collect source-host APIs, manage private keys, mutate canonical refs or Cairn lifecycle files, verify the truth of caller-supplied signature/currentness observations, or make evidence-correctness, protocol-enforcement, merge-eligibility, or release-readiness claims.
