# Validation: Valence artifact-auth non-production canary evidence

Validated on 2026-07-20 against Valence `6fb2473503ec8b72f72f2857cffee08e65b6bec6` and artifact-auth `799459346d5416fbd7b9f55840a7371441b55afa`.

## Baseline

- Cairn validation passed before this evidence-only change.

## Evidence checks

- The self-contained public subset contains 15 regular files: the exact harness, RecordedOnly Evidence IR and lint receipt, profile/context/authorization observations, capture/replay/removal summaries, fresh-process denial log, typed manifest, bounded hash script, README, and inventory.
- `nix shell nixpkgs#nickel -c nickel typecheck .../manifest.ncl` passed.
- BLAKE3 inventory regeneration reproduced byte-for-byte; the negative symlink fixture failed with `symlink is forbidden`.
- JSON/log secret scanning found no private-key marker, serialized secret-key field, or long name-prefixed private-key value.
- Evidence IR retains `standalone_authority_admitted = false` and canonical `verification_role = recorded_only`; the fresh-process log contains the expected actor-authorization denial.
- The signer source remains explicitly ephemeral and non-production; mutable Git/capability state and private signer material are absent.

## Lifecycle checks

- Cairn validate plus proposal, design, and tasks gates passed.
- Sync dry-run passed with receipt `13a18f4cdfb02dae844ab7cd24ec57971676896f0885a46194701a71a483c932`.
- Sync execution passed with receipt `5ee91bc1863f9665e1da894833cb3d9a690854540fcd2682cba027933a6f1638`.
- Accepted requirements `valence.artifact_auth_operational_receipt.canary_archive` and `valence.artifact_auth_operational_receipt.canary_authority` are present.

## Adversarial audit

The advisory audit challenged self-containment and cross-consumer identity binding. Cross-consumer revisions are now explicitly review linkage rather than a joint signature or attestation. Its suggestion to clear `legacy_authoritative` was rejected because retaining protocol/legacy authority is the required fail-safe while standalone authority remains unadmitted. Deterministic checks remain authoritative.

## Bounded result

The exact non-production signed COB, materialization, replay, and authorization-removal evidence is durably reviewable without private signer or mutable repository state. This archive remains RecordedOnly and does not establish live Radicle membership, global revocation freshness, production signer custody, protocol or COB authority, lifecycle handoff, release interpretation, production rollout, or standalone authority. Protocol authority and rollback remain active.
