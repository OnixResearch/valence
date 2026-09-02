# Design

## Completion contract

The slice is complete when deterministic positive and negative fixtures prove closed source-record validation, native issue and patch plan construction, attribution preservation without impersonation, idempotent mapping behavior, local built-in COB create/update round trips, exact written-entry verification, and CI admission only for one exact successful revision with threshold-matching delegate approvals. Saved migration and admission receipts must verify without network access. Full workspace, Nickel, Cairn, and flake checks must pass.

False completion includes a receipt-only importer that writes no native COB, a custom COB presented as a native issue/patch, shell interpolation of unvalidated metadata, source-actor or source-time impersonation, converting imported reviews into approvals, accepting a successful result without its exact event and signed bot observation, counting duplicate/stale/wrong-revision approvals, changing a canonical ref from Valence, widening live CI, or claiming protocol-level mandatory CI.

## Portfolio search

| Family | Mechanism | Decision |
|---|---|---|
| `native-builtins` | Deterministic pure plans plus built-in `xyz.radicle.issue` / `xyz.radicle.patch` CRU actions in a process-isolated local adapter | Selected; produces ordinary Radicle collaboration state while preventing upstream `serde_json/preserve_order` from changing Valence canonical JSON |
| `custom-import-cob` | Store source-host records only in a Valence custom COB | Rejected as false completion because ordinary issue/patch tools would not expose migrated collaboration state |
| `cli-replay` | Invoke `rad issue` and `rad patch` repeatedly and parse human output | Rejected for the core path because output parsing and partial replay make target identity, rollback, and exact-entry verification fragile |
| `protocol-hook` | Claim a Radicle protocol rule prevents merges without CI | Rejected because current Radicle collaboration and Git protocols do not provide that enforcement surface |
| `external-cas-guard` | Produce a typed admission receipt consumed by an onix-core compare-and-swap shell | Selected for downstream enforcement; preserves Valence's non-mutation boundary and makes bypass limits explicit |

Search budget: five mechanism families, one focused upstream source inspection, one advisory review attempt, and one adversarial audit round. Upstream Radicle 1.9.1 built-in issue and patch evaluators are the compatibility reference. The advisory model suggested shell/Python mechanisms that violate repository language and boundary rules; those suggestions are rejected.

## Architecture

### Pure migration core

`valence-core::radicle_forge_ops` receives already-decoded normalized records. It performs no file, Git, Radicle, clock, network, environment, or process access. The core:

1. validates a closed schema and configured bounds;
2. canonicalizes source records in stable source-key order;
3. computes a BLAKE3 source-record identity and batch identity;
4. maps issue state and labels without broadening authority;
5. requires pull-request base and head OIDs plus target branch;
6. renders source attribution in a fixed, escaped form;
7. emits a deterministic ordered operation plan;
8. compares existing source-target mappings to make exact reruns no-op and conflicting reruns fail closed; and
9. verifies post-write binding receipts against the plan and observed target entries.

Source actor/login and timestamp text are recorded facts supplied by an exporter. They are not Radicle actor identities or COB timestamps. Null/deleted source actors use an explicit tombstone form rather than an empty identity.

### Native COB adapter

`valence-radicle` parses the generated runtime profile and normalized batch and prepares a BLAKE3-bound plan. `valence-radicle-native` revalidates that plan, opens one explicit local repository, and is preview-first. Execute requires an SSH-agent signer matching the planned importer key. The native executable intentionally has no `valence-core` dependency: upstream Radicle enables `serde_json/preserve_order`, so JSON is the explicit process boundary that preserves Valence's default canonical sorted-map backend.

Issue plans create a built-in issue root, append attributed source comments in source order, apply mapped labels, and close or solve only after content writes succeed. Patch plans require base/head objects to exist locally, create a built-in patch against the reviewed target branch, and append source discussion/review metadata as attributed comments. Imported review decisions never call Radicle review/accept APIs. Merged and closed pull requests are recorded as archived historical patches only when the built-in format can represent the state without inventing authority; unsupported states fail with a stable diagnostic.

Before success, the native shell reloads the target through the built-in evaluator, checks source-key markers, title, description, labels, state, base/head/revision, comment count and entry identities, and confirms that ref mutations are confined to the selected importer's built-in COB namespace. It emits normalized observations only. `valence-radicle forge-bind` independently revalidates the plan and observations under the canonical Valence JSON backend, then emits a binding receipt containing the plan BLAKE3, source identities, target type/object, entry OIDs, signer, and required non-claims.

### CI admission evidence

The pure admission function receives:

- exact policy identity, RID, bot identity, delegates, threshold, required check name, and signed-reference feature;
- current canonical predecessor and candidate descendant observation;
- exact patch ID, revision ID, base/head and target branch;
- the typed CI event and result already validated by their producer schema;
- a signed bot comment observation whose author, revision, job ID, object OID, disposition, and artifact BLAKE3 match the result; and
- unique delegate review observations accepting that exact revision.

Admission succeeds only when the result disposition is succeeded with exit zero, every identity matches, the candidate is the exact patch head and a descendant of the supplied canonical predecessor, the bot is not a delegate, and the unique exact-revision approval count meets the threshold. Rejection is deterministic and returns no mutation plan. The admission receipt is BLAKE3-bound and explicitly suitable only as input to an external guarded compare-and-swap.

Valence does not read the clock. Freshness means exact current patch/revision/status/ref observations supplied by the shell in one materialization, not elapsed-time validity.

## Adversarial audit

The implementation must reject unknown fields, duplicate source keys, duplicate comments, unsorted or repeated event IDs, malformed OIDs/digests/RIDs, control characters, bound overflows, parent traversal in source locators, unsupported states, absent patch objects, base/head inversion, mismatched existing mappings, partial prior imports, signer mismatch, wrong built-in type, extra ref mutation, malformed or invalid COB histories, stale CI policy, non-patch CI results, failed/timeout/flood results, unsigned or wrong-bot status, status text/result drift, bot-as-delegate, duplicate approvals, wrong-revision approvals, below-threshold approvals, non-descendant candidates, stale canonical predecessors, and any receipt with weakened non-claims.

## Claim boundary

This establishes deterministic normalized metadata plans, observed native COB writes, source-to-target bindings, and bounded CI-admission evidence. It does not establish source-host export completeness, source actor authenticity, original signature or timestamp preservation, issue/patch semantic equivalence, review equivalence, merge eligibility, protocol-enforced mandatory CI, bypass-proof delegate behavior, canonical-ref mutation, repository correctness, CI correctness, host sandboxing, release readiness, or whole-stack GitHub independence.
