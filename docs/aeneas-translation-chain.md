# Aeneas/Lean translation-chain evidence

Valence validates an `aeneas-lean/v1` provider profile as a closed extension of
its formal-proof chain authority. The profile binds checked Rust source,
reviewed Charon configuration and extraction, LLBC, Aeneas translation and
manifest output, generated Lean, declared model/trust inputs, maintained theorem
source, adequacy and trust-closure decisions, package lock, Lean checker result,
and the Octet receipt/proof-card/provenance projection.

Passing validation establishes canonical BLAKE3 identity, exact typed linkage,
role policy, and required non-claim conformance only. It does not establish Rust
correctness; Charon, Aeneas, Lean, translation, external-model, theorem,
adequacy-policy, requirement-selection, or verifier soundness; semantic
equivalence; requirement satisfaction; whole-program correctness; lifecycle
readiness; or release eligibility.

## Execution ownership

Octet owns Charon, Aeneas, and Lean execution and artifact verification. Valence
never launches those tools, discovers a live sibling repository, or replaces
Octet's theorem-adequacy decision. The CLI only opens an operator-declared
capability root, loads the checked-in input and referenced bytes, invokes the
pure in-memory validator, and renders its report:

```text
valence formal-proof aeneas-chain validate \
  --artifact-root fixtures/aeneas-translation-chain \
  --input chain.json
```

The shell measures every loaded artifact and parses JSON producer observations
before invoking the core. The core fails closed on stale measurements,
extraction drift, manifest drift, broken edges, claim disagreement, role
promotion, or weakened non-claims, and performs no filesystem, process,
environment, clock, or network I/O.

## Roles and downstream handoff

The initial frozen chain has one `property` node: the artifact-verified,
Octet-owned theorem receipt. Charon/Aeneas observations, LLBC, manifest, and
generated Lean remain `recorded_only`. Provider configuration, external-model
inventory, theorem source, adequacy/trust decisions, package lock, raw checker
result, proof card, and provenance remain `boundary` evidence.

A valid report emits the canonical chain identity, extraction-profile,
translation-manifest, adequacy-decision, and Octet-receipt identities; theorem
ID; role counts; and full non-claim boundary. Cairn may consume that report via
its generic evidence-chain readiness policy, and Mantle may carry it as opaque
external evidence. Neither layer gains proof-execution authority.

## Frozen producer fixture

The fixture under `fixtures/aeneas-translation-chain/` was imported from Octet
revision `ef18352bf579bb7c6eb39070d917e3d005385a61` after Octet artifact
verification passed. The source adapter schema is
`octet-aeneas-lean-valence-adapter/v1`, and its exact BLAKE3 is
`19a10c8d3e4ce29b4a058bd2202ff52fc42cee7508f04312840d0bfd398b00a8`.
The Nickel source `chain.ncl` owns the reviewed Valence input; `chain.json` is
its deterministic runtime export. `negative-cases.ncl` owns the typed tamper
matrix and exports `negative-cases.json`.

Do not edit imported artifacts casually. To refresh:

1. drain and archive the producing Octet change;
2. run Octet artifact verification and confirm empty diagnostics;
3. run `tools/import-aeneas-translation-fixture.rs <octet-root> <valence-root>`;
4. record the reviewed Octet revision and measured BLAKE3 values in `chain.ncl`;
5. export both Nickel fixtures and inspect their diffs;
6. run the positive CLI validation and complete negative matrix; and
7. retain the prior fixture when its producer schema remains supported.

A changed producer revision or digest is compatibility-review input, not an
implicit freshness upgrade.
