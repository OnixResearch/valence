# Design: Unified Artifact source migration

## Context

Valence consumes `artifact-auth-core` and `artifact-auth-ed25519` through exact Cargo and Nix pins. The unified Artifact workspace contains byte-identical authentication crates plus `artifact-transfer-core` at reviewed revision `e41340bec587b6d049b5cc518ec7db925dde84be`. The predecessor Radicle source remains valid historical evidence, but it no longer represents current package ownership.

## Goals

- Align Cargo and Nix with one immutable unified source.
- Validate the complete source workspace separately from Valence's narrower consumer graph.
- Preserve positive and negative authentication behavior.
- Preserve historical source and receipt identities for bounded rollback.
- Keep all Valence authority boundaries unchanged.

## Non-goals

- Change authentication APIs, features, package versions, or Rust implementation.
- Admit transfer or binding packages into Valence's dependency graph.
- Grant Artifact any Radicle protocol, actor authorization, Evidence IR, lifecycle, or release authority.
- Prove source correctness, forge availability, or whole-stack GitHub independence.

## Decisions

### Use one pure source-admission core

A Nix function receives parsed manifests, locks, source metadata, and resolved Nix identity. It returns deterministic issue codes. The flake shell reads files, passes observations to the core, checks source entry-point BLAKE3 identities, and fails on any issue.

### Validate source breadth and consumer breadth separately

The source workspace and release profile must contain exactly the two authentication packages and `artifact-transfer-core`. Valence's lock must contain exactly the two authentication packages from that source. This prevents silent graph widening while admitting the real unified workspace.

### Retain the Radicle receipt as historical evidence

The old typed receipt remains immutable evidence for the prior transport cutover. A new typed receipt describes the current unified-source migration. The active source-pin check no longer treats the historical receipt as current state.

### Preserve stable requirement IDs

The existing transport and evidence requirement IDs remain stable while their text changes to the current owner. Archived deltas preserve the former Radicle-specific requirement text.

## Risks and mitigations

- **Source graph widening:** reject lock entries beyond the two authentication packages.
- **Package byte drift:** bind both authentication entry points by BLAKE3 and test a mutation.
- **License drift:** require `MIT OR Apache-2.0` in the workspace and release profile.
- **Mixed source state:** reject predecessor, floating, local-path, branch, tag, revision, or NAR mismatches.
- **Historical evidence overclaim:** label the old Radicle check as historical and do not compare it with current locks.

## Rollback

Rollback is not automatic. Restore the predecessor repository and revision in Cargo and Nix, regenerate both locks with their owning tools, rerun source admission plus focused positive and negative behavior tests, and record a new receiver-local decision.
