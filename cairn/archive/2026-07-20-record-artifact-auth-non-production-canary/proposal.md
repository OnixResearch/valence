## Why

Valence's first landed artifact-auth operational canary passed through real signed local Git COB attachment and materialization, but its public Evidence IR, authorization observations, and exact harness remain only in an unversioned workspace directory. A durable product-owned archive is required before later reviewers can evaluate rollout evidence without confusing a local non-production signer with Radicle protocol or production authority.

## What Changes

- Archive the Valence-specific public artifacts from `artifact-auth/run-001` with the exact Valence and artifact-auth revisions.
- Preserve the exact temporary harness, RecordedOnly Evidence IR, signed COB identities, materialization and fresh-process replay, actor-authorization removal, and fresh-process denial.
- Add a typed Nickel manifest and reproducible BLAKE3 inventory while excluding private signing material and mutable Git/capability state.
- Keep the canary explicitly non-production and non-authoritative; protocol behavior remains authoritative and rollback remains available.

## Impact

- **Files**: this Cairn change, its evidence bundle, and the accepted `artifact-auth-operational-receipt` specification.
- **Testing**: Nickel typecheck, deterministic BLAKE3 regeneration, negative symlink rejection, secret scan, expected-denial inspection, Cairn gates, sync, and archive validation.
