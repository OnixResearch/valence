# Consume artifact-auth from Radicle-backed HTTPS

## Why

Valence already consumes the reviewed `artifact-auth` revision `799459346d5416fbd7b9f55840a7371441b55afa`, but Cargo and Nix fetch it from GitHub over SSH. The source has no registry release, so Git transport remains a bootstrap dependency.

## Outcome

Change only the source transport to the governed Radicle HTTPS adapter. Preserve the exact Git object, Nix content identity, package versions, feature graph, tests, and Valence authority boundaries. Remove every GitHub source fallback for this dependency and emit a typed cutover receipt.

## Scope

- Align `Cargo.toml`, `Cargo.lock`, `flake.nix`, and `flake.lock` on one Radicle HTTPS URL and exact revision.
- Preserve `artifact-auth-core` and `artifact-auth-ed25519` behavior and Valence’s existing Radicle/Evidence IR authority.
- Validate ordinary workspace, focused mapping, no-std, and negative source-admission behavior.
- Retain GitHub only as historical documentation, not an executable dependency fallback.

## Non-goals

This change does not migrate Valence itself to Radicle, move Valence to the unified `artifact` workspace, alter artifact-auth APIs, claim semantic equivalence, or grant source transport release authority.
