# Change: Consume artifact-auth from the unified Artifact workspace

## Why

`OnixResearch/onix-artifact` now owns the unchanged `artifact-auth-core` and `artifact-auth-ed25519` packages. Valence still resolves the predecessor `artifact-auth` repository. This split leaves package ownership, source admission, and rollback evidence on different repository identities.

## What Changes

- Pin Cargo and Nix to unified Artifact revision `e41340bec587b6d049b5cc518ec7db925dde84be`.
- Admit the complete three-package source workspace while keeping Valence's resolved graph limited to the two authentication packages.
- Add positive and negative source-admission checks for repository, revision, NAR hash, package set, source bytes, license, and consumer-graph drift.
- Add typed migration evidence and retain the accepted predecessor Radicle receipt as historical evidence.
- Preserve Valence-owned protocol, identity, authorization, lifecycle, and release authority without changing Rust implementation behavior.

## Impact

- Affected spec: `artifact-auth-adoption`.
- Affected files: Cargo and Nix pins, source-admission checks, migration evidence, and source documentation.
- Rollback remains an explicit owning-tool regeneration to predecessor revision `799459346d5416fbd7b9f55840a7371441b55afa` after source identity review.
