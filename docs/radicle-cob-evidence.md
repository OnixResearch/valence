# Radicle COB evidence sidecars

Valence records selected durable evidence links in the custom Radicle COB type `org.onixresearch.valence.evidence`. The COB is optional collaboration state. Valence Evidence IR artifacts remain the identity/role authority, and Cairn repository files remain the lifecycle authority.

## Architecture and boundaries

- `valence-core::cob_evidence` is pure. It validates typed targets, BLAKE3 identities, Git/Radicle OID roles, attachment payloads, actor-policy inputs, deterministic concurrency, binding receipts, and saved snapshots over already-loaded values.
- `valence-radicle` is the shell. It opens one explicitly selected local Git repository, uses `radicle-cob = 0.20.0`, verifies entry signatures, measures artifact bytes and Git embeds, signs through `ssh-agent`, and updates local COB refs.
- The existing `valence` CLI does not depend on Radicle or Git storage libraries.
- Cairn gates consume saved JSON/Evidence IR artifacts. They must not inspect live refs or contact a Radicle node.

A Radicle signature authenticates the entry origin. It does not prove the evidence, authorize a waiver, resolve a finding, satisfy a requirement, or establish release eligibility. Authorization is a separate, BLAKE3-bound input that maps an exact signer key to a reviewed profile role.

## Profile

`contracts/radicle-cob-evidence/profile.ncl` owns the reviewed type name, schemas, relationship vocabulary, actor operations, embed bound, confidentiality classes, and required non-claims. `generated/profile.json` is the checked runtime export.

Refresh only after reviewing the Nickel source:

```bash
nix develop -c nickel export --format json \
  contracts/radicle-cob-evidence/profile.ncl \
  > contracts/radicle-cob-evidence/generated/profile.json
nix build .#checks.x86_64-linux.radicle-cob-evidence-profile --no-link
```

The named `MaxEmbeddedArtifactBytes` policy is 64 KiB. Larger artifacts use reference-only mode.

## Prepare authorization

Load the actor's Ed25519 key in `ssh-agent`, then generate a reviewed runtime authorization input:

```bash
valence-radicle authorization \
  --profile contracts/radicle-cob-evidence/generated/profile.json \
  --actor z6Mk... \
  --role maintainer \
  --output-root evidence/radicle \
  --output authorization.json
```

`attacher` may initialize and attach. `maintainer` may additionally supersede and revoke. A deployment can author a richer Nickel policy and export the same closed JSON shape; a signer must have exactly one known role.

## Attach evidence

A mutation request is closed JSON. The operator should author equivalent configuration in Nickel and export it when requests are retained as policy/configuration.

```json
{
  "schema": "valence.radicle-cob-evidence.mutation-request.v1",
  "target": {
    "repository_rid": "rad:z3...",
    "type_name": "xyz.radicle.issue",
    "object_id": "0123456789abcdef0123456789abcdef01234567"
  },
  "actor": "z6Mk...",
  "sidecar_object_id": null,
  "action": {
    "attach": {
      "relationship": "evidence_for",
      "artifact_locator": "lint/durable-finding.json",
      "payload_mode": "reference_only",
      "confidentiality": "public"
    }
  }
}
```

Preview without mutation:

```bash
valence-radicle mutate \
  --repository /path/to/local/repository \
  --profile contracts/radicle-cob-evidence/generated/profile.json \
  --authorization evidence/radicle/authorization.json \
  --artifact-root evidence \
  --request evidence/radicle/attach-request.json \
  --output-root evidence/radicle \
  --output attach-plan.json
```

After reviewing the plan, repeat with `--execute`. Execution requires the request actor key in the running SSH agent. It creates or updates one local sidecar and emits a post-write binding receipt only after reloading the signed entry and exact bytes.

When no sidecar exists, execution creates a root and attachment in one signed entry. When one physical sidecar exists, it updates that sidecar. If offline peers created multiple physical roots for the target, mutation fails until `sidecar_object_id` selects one explicitly; snapshots still reduce every valid root together.

Supersede and revoke requests replace the `attach` action with:

```json
{"supersede":{"attachment_id":"b3:...","replacement_attachment_id":"b3:..."}}
```

or:

```json
{"revoke":{"attachment_id":"b3:...","reason":"producer withdrew this evidence"}}
```

These events change accepted state. They do not delete historical Git objects.

## Materialize a saved snapshot

Store the target reference as JSON and run:

```bash
valence-radicle snapshot \
  --repository /path/to/local/repository \
  --profile contracts/radicle-cob-evidence/generated/profile.json \
  --authorization evidence/radicle/authorization.json \
  --artifact-root evidence \
  --target evidence/radicle/target.json \
  --output-root evidence/radicle \
  --output snapshot.json
```

The result binds target tips, all physical sidecar IDs/tips, accepted attachments, rejected events, explicit conflicts, profile and authorization identities, materialized-state hash, receipt hash, and required non-claims. The nested Evidence IR artifact and snapshot are `recorded_only` even when a referenced artifact has the `property` role.

A future Cairn policy may ingest the saved snapshot and binding receipts as read-only inputs. It must verify them offline, apply its own freshness and authorization policy, and retain Cairn files/receipts as source of truth. Snapshot presence alone cannot waive a lint, complete a task, or pass a gate.

## Lint guidance

Do not create one COB per raw finding. Keep full scanner output in producer receipts, triage first, and attach only durable findings, reviewed dispositions, or evidence artifacts whose identity should survive discussion and replication. `context_for`, `evidence_for`, `evidence_against`, and `disposition_for` remain linkage vocabulary, not proof strength.

## Concurrency

Attachments form an idempotent set keyed by BLAKE3 attachment identity. Input and output order cannot change the snapshot. Causally later supersede/revoke events may change accepted status. Incompatible concurrent maximal dispositions become explicit conflicts; timestamps, traversal order, and hash-order tie breaking never choose a winner.

## Repository and jj safety

The adapter does not checkout, stage, commit tracked source, update `HEAD`, create tags, update `refs/heads`, update remotes, or access the network. Explicit mutation changes one `refs/rad/<actor>/cobs/...` ref plus the Git objects needed by that ref. Tests compare every pre-existing ref, `HEAD`, and tracked bytes before and after execution. This special-ref confinement leaves a colocated jj working-copy commit unchanged.

## Privacy and retention

- Embed only public, non-secret review evidence allowed by the profile.
- `restricted` evidence is reference-only and still requires an allowed confidentiality profile.
- Secret, regulated, credential-bearing, and deletion-dependent evidence is rejected.
- Radicle selective replication is not at-rest encryption.
- Supersession/revocation cannot promise deletion from peer object databases, backups, reflogs, or prior clones.

## Backup and restore

A branch-only clone is not a sidecar backup. Backups must retain the repository object database and `refs/rad/*/cobs/org.onixresearch.valence.evidence/*` refs, plus referenced external evidence and authorization/profile inputs. After restore, run snapshot materialization and offline verification and compare target, sidecar-tip, materialized-state, profile, authorization, and receipt identities to the retained snapshot. Availability, replication, and disaster recovery remain operator responsibilities.

## Dependency upgrades and project split

`radicle-cob` is exact-pinned because its pre-1.0 storage and evaluator APIs are compatibility-sensitive. Upgrade only with a reviewed dependency diff, local create/list/update round trip, signature negatives, multi-root convergence, embed/reference tests, and repository-ref confinement checks.

Keep the adapter in this workspace while it versions with Valence schemas. Split it into an independently operated project only if it gains a daemon, node/seeding lifecycle, peer discovery, key custody, independent releases, or support for multiple Valence compatibility generations.

## Non-goals

This integration does not run or configure a Radicle node, discover peers, fetch or seed repositories, manage keys, provide forge UI, replace issue/patch reducers, mutate Cairn lifecycle state, prove evidence truth, prove target correctness, or establish lifecycle/release readiness.
