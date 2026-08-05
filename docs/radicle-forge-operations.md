# Radicle forge metadata and CI-admission evidence

r[verify valence.radicle_forge_ops.shell]
r[verify valence.radicle_forge_ops.boundaries]

Valence provides two bounded mechanisms:

1. deterministic normalized issue/pull-request metadata plans with preview-first native Radicle issue/patch execution; and
2. exact-revision CI-admission receipts over already verified status, approval, and canonical-predecessor observations.

Neither mechanism contacts a source forge, runs a Radicle node, mutates a canonical ref, or decides release readiness.

## Ownership boundary

- A source-specific exporter owns API pagination, authentication, completeness, rate limits, and conversion into the closed normalized batch schema.
- `valence-core::radicle_forge_ops` owns deterministic validation, BLAKE3 identities, operation ordering, attribution rendering, idempotency, binding receipts, and CI-admission decisions over already-loaded facts.
- `valence-radicle` owns canonical planning, binding, and receipt verification without loading upstream Radicle's JSON feature set.
- `valence-radicle-native` owns one selected local repository, SSH-agent signing, built-in issue/patch writes, post-write evaluation, and ref-confinement checks. The process boundary prevents upstream `serde_json/preserve_order` from changing Valence's canonical sorted-map backend.
- An onix-core guard must independently verify live bot/delegate signatures, current patch state, exact Git ancestry, and compare-and-swap preconditions before consuming a Valence admission receipt and updating a canonical ref.

A caller-provided `signature_verified: true` value is not proof by itself. It is a normalized observation boundary for the downstream verifier. Saved Valence validity does not replace live signature or currentness checks.

## Profile

Nickel owns the bounds and claim boundary:

```text
contracts/radicle-forge-operations/profile.ncl
contracts/radicle-forge-operations/generated/profile.json
```

The checked profile supports normalized `issue` and `pull_request` records, built-in `xyz.radicle.issue` and `xyz.radicle.patch` targets, attributed-comment-only imported reviews, bounded titles/text/labels/events, and three mandatory non-claims.

## Normalized source batch

`fixtures/radicle-forge-operations/source-batch.json` is the executable schema example. JSON decoding is closed: unknown fields fail. Each source record carries:

- a globally unique source key, source number, HTTPS URL, title, body, state, actor, and source timestamps;
- labels and stable event IDs;
- comments or reviews with explicit source actor/timestamp/URL; and
- for pull requests, exact base/head Git OIDs and `refs/heads/main` target.

Deleted source actors use `login: null` plus a non-empty source ID. Imported timestamps and actors remain text attribution. The actual COB entry is signed by the importer and receives native Radicle ordering/timestamp semantics.

Merged pull requests require a merged commit observation but are imported as archived historical patches. Source review decisions become comments containing `source-review-state`; they never become Radicle reviews or approvals.

## Preview and execute

Preview is deterministic and has no Git or COB effects:

```text
valence-radicle forge-plan \
  --profile contracts/radicle-forge-operations/generated/profile.json \
  --source normalized-source.json \
  --output-root evidence/radicle \
  --output migration-plan.json
```

Native execution requires an explicitly selected bare Radicle storage repository and the planned importer key in the running SSH agent:

```text
valence-radicle-native import \
  --repository /var/lib/radicle/storage/<RID> \
  --plan evidence/radicle/migration-plan.json \
  --output-root evidence/radicle \
  --output native-observations.json

# Repeat only after reviewing the preview:
valence-radicle-native import ... --execute

valence-radicle forge-bind \
  --profile contracts/radicle-forge-operations/generated/profile.json \
  --plan evidence/radicle/migration-plan.json \
  --observations evidence/radicle/native-observations.json \
  --output-root evidence/radicle \
  --output migration-binding.json
```

Native execution revalidates the plan BLAKE3 before effects, reloads every built-in object, verifies title, attribution markers, labels, lifecycle, patch range, comments, and source key, and rejects mutation outside the importer's built-in COB refs plus its signed-ref update. The separate binder revalidates the same plan under Valence's canonical JSON backend before issuing a receipt. A failed operation may leave signed partial Git history; it never emits completed observations or a binding. Repair requires reviewing the retained refs and supplying an exact existing mapping or a new bounded migration decision.

Exact reruns use `existing_mappings`. A mapping must bind the same source-record BLAKE3, built-in type, and target object. Drift or partial/conflicting mappings fail closed.

Offline receipt verification:

```text
valence-radicle verify-forge-binding \
  --profile contracts/radicle-forge-operations/generated/profile.json \
  --receipt evidence/radicle/migration-binding.json
```

## CI admission

`ci-admission` is a pure materialization step over observations already verified by an owning shell:

```text
valence-radicle ci-admission \
  --input verified-ci-admission-input.json \
  --output-root evidence/radicle \
  --output ci-admission-report.json
```

Admission requires agreement among:

- exact policy BLAKE3, RID, non-delegate bot, sorted delegate set, threshold, check name, and signed-reference feature `parent`;
- exact patch/revision/base/head/target and current canonical predecessor;
- a succeeded zero-exit typed CI event/result pair;
- a matching signed bot status on that revision;
- unique signed delegate accept observations for that exact revision meeting threshold; and
- a current candidate descendant observation.

Any mismatch returns `admitted: false` and no receipt. The admitted receipt's scope is `external-guarded-compare-and-swap-input`. It contains no ref mutation command and cannot prove that the supplied signature/currentness booleans were truthful.

Offline structural verification:

```text
jq '.receipt' ci-admission-report.json > ci-admission-receipt.json
valence-radicle verify-ci-admission --receipt ci-admission-receipt.json
```

## Non-claims

This capability does not prove source export completeness, source actor authenticity, preservation of source signatures or timestamps, issue/patch semantic equivalence, approval equivalence, CI correctness, host sandboxing, merge eligibility, protocol-enforced mandatory CI, bypass-proof delegate behavior, canonical-ref authority, canonical mutation, release readiness, or whole-stack GitHub independence.
