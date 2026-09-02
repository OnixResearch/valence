# Valence unified Artifact source migration

Observed: 2026-07-26

## Source identities

Historical source:

- Repository: `https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git`
- Revision: `799459346d5416fbd7b9f55840a7371441b55afa`
- Historical receipt BLAKE3: `ef8d13a0cb8d560a99b79f91f9471350db774e8c3bf753d52fed7382f9bd85f7`

Active unified source:

- Repository: `ssh://git@github.com/OnixResearch/onix-artifact.git`
- Revision: `e41340bec587b6d049b5cc518ec7db925dde84be`
- Nix NAR hash: `sha256-2cM912L2YXVnVX9LquwhAPKyjPP/z/oFQRe7Qq9bHHE=`
- Source archive BLAKE3: `8fc65f3d7968d76af419f08ac972955514c8256f8aa5ed90815d8bdcced28113`
- Valence migration receipt BLAKE3: `0b3b742587f06ce6ae131b41fa954e9285d10f5d556800cbe262517fc44f9a6a`

Remote main resolved to the full unified revision during the observation window. This proves object availability at that time only.

## Authentication source parity

Recursive comparison found no differences in either authentication crate directory.

- `artifact-auth-core/src/lib.rs`: `839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21`
- `artifact-auth-ed25519/src/lib.rs`: `7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633`

The migration changes source repository and revision only. It does not change Valence Rust files, package versions, features, adapters, or authority boundaries.

## Package boundaries

The unified source workspace and release profile contain exactly:

1. `artifact-auth-core`
2. `artifact-auth-ed25519`
3. `artifact-transfer-core`

Valence's Cargo lock resolves exactly the two authentication packages from this source. It resolves no transfer or binding package.

## Positive and negative validation

Before migration:

- three focused `valence-core` artifact-auth tests passed;
- four focused `valence-radicle` artifact-auth tests passed;
- the historical Radicle cutover receipt check passed.

After migration:

- the pure source-admission core accepted aligned Cargo, Nix, source-profile, source-workspace, license, and package identities;
- source entry-point BLAKE3 checks passed and a mutated entry point was rejected;
- focused core and shell positive and negative behavior passed;
- no-std, workspace, formatting, and strict Clippy checks passed;
- the historical receipt remained valid as historical evidence;
- native Cairn validation and change gates passed;
- traceability reported the same pre-existing 83 of 108 accepted requirement IDs at parent revision `4daa5030f2b0ace54c4c08b3a30ebd8cd0c57621` and after this migration;
- the host-system Nix flake check passed.

Negative source-admission and receipt tests reject predecessor Cargo or Nix sources, floating or local dependencies, revision/NAR drift, duplicate packages, consumer graph widening, incomplete source membership, incompatible licensing, source-byte drift, incomplete validation, automatic rollback, absolute evidence paths, and authority overclaims.

## Authority and rollback

Valence retains Radicle protocol verification, actor authorization, COB reduction, Git measurement/storage, Evidence IR identity and roles, lifecycle handoff, diagnostics, and release interpretation.

The historical Radicle revision remains an explicit rollback coordinate. It is not an active Cargo or Nix source. Rollback requires source review plus Cargo and Nix lock regeneration.

The unchanged repository-wide traceability gap is a separate blocker and is not accepted as full coverage by this migration.

This migration does not prove source correctness, whole-Valence behavior correctness, forge availability, automatic fallback, whole-stack GitHub independence, authorization, or release readiness.
