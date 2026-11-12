# Change: Align Valence build sources with anonymous public transport

## Why

Valence's GitHub repository is public, but Cargo and the non-flake Nix input still fetch `artifact-auth-core` and `artifact-auth-ed25519` over `ssh://git@github.com/OnixResearch/onix-artifact.git`. That transport requires an SSH key: the `nix flake check` workflow on public `main` fails with `git@github.com: Permission denied (publickey)`. A public reviewer or CI runner with no credentials and no private network access cannot resolve the dependency graph or reproduce the checks.

The same release pass found three smaller public-readiness gaps: `license-policy.tsv` does not cover two workspace packages, the license text for `pilots/schema-identity` is absent, and active documentation and agent notes still contain personal absolute paths.

## What Changes

- Declare both `artifact-auth-core` and `artifact-auth-ed25519` with the public HTTPS repository URL at the same immutable revision `5f3962d7471d1547e044f57daeda72f56621fd52`.
- Point the `artifact` flake input and source-admission expectation at the same HTTPS URL and revision, regenerate `Cargo.lock` and `flake.lock` with Cargo and Nix, and keep the existing NAR hash and two-package consumer graph.
- Record the transport requirement in `artifact-auth-adoption`, and make the source-pin validator, current documentation, and the pinned revision statement agree.
- Complete `license-policy.tsv` coverage for `valence-radicle` and `pilots/schema-identity`, and add the missing AGPL text to the schema pilot.
- Replace personal absolute paths in active documentation and one rendered evidence-index page with portable placeholders; retain immutable evidence and hash-bound fixtures unchanged.
- Document a portable Cairn invocation that does not hard-code a home directory, and give the flake-check workflow an explicit least-privilege `contents: read` permission.
- Link the security policy from the README.

## Impact

Affected surfaces are dependency-transport metadata, source-admission expectations, license-policy coverage, documentation, agent notes, and CI permissions. Rust behavior, Evidence IR schemas, canonical identities, the reviewed source revision, and the consumer package graph are unchanged. Historical migration and cutover receipts remain historical evidence.

## Out of Scope

No dependency version changes, no Rust refactors or feature work, no license-term changes, no Radicle mutation or publication, no history rewrite, and no claim of release authority. Historical evidence under `.cairn/archive/`, `cairn/archive/`, `evidence/`, and the hash-bound Aeneas provenance fixture are not edited.
