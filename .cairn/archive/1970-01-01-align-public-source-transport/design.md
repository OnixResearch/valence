# Design: Anonymous public source transport

## Context

Valence pins one reviewed unified Artifact revision for its two authentication dependencies. Cargo, `flake.nix`, `flake.lock`, and `nix/artifact-source-pin.nix` all describe that identity. Only the transport scheme required credentials; the repository and revision are already public.

## Decisions

### Use HTTPS at the existing revision

Change only the scheme in `Cargo.toml`, `flake.nix`, and the source-pin expectation. The revision stays `5f3962d7471d1547e044f57daeda72f56621fd52`, so source bytes, the NAR hash, the package set, and the consumer graph do not change. `nix/artifact-source-pin.nix` keeps validating the exact HTTPS URL in both the Cargo declarations and the Nix lock so a future credential-bearing URL fails admission rather than silently resolving.

### Regenerate every lock with its owning tool

Regenerate the two `Cargo.lock` source entries with `cargo update -p artifact-auth-core -p artifact-auth-ed25519` and relock only the `artifact` input with `nix flake update artifact`. Confirm by diff that no unrelated entry moves. Do not hand-edit a lock or change the NAR hash.

### Keep historical evidence historical

`evidence/source/artifact-workspace-migration-v1.{ncl,json}` and `lib/artifact-source-migration-receipt.ncl` record the transport used at migration time and are BLAKE3-bound. Rewriting them would falsify history and break the checked receipt. The same applies to the archived Aeneas provenance fixture. Report these as residual historical references rather than editing them.

### Prefer portable documentation over machine-specific paths

Agent notes and reproduction instructions use a `${CAIRN_SOURCE}` environment variable pointing at a Cairn checkout; no home directory is hard-coded. Rendered documentation strings use role descriptions instead of a personal checkout path. The license policy enumerates every workspace member so the boundary check fails closed.

## Rejected Alternatives

- Keep SSH and add a deploy key or CI secret. That requires private credentials for a public repository and still blocks anonymous reviewers.
- Switch to a newer Artifact revision. That would change reviewed source bytes and expand the migration scope beyond transport.
- Use a sibling `../artifact` path. Workspace-local paths break clean clones and are already rejected by source admission.
- Add a fallback transport. Silent fallback would defeat the fail-closed identity check.

## Validation

Run `cargo fetch --locked` in a fresh clone with an empty `CARGO_HOME` and SSH disabled; run anonymous `git ls-remote` for every flake input URL; run `nix flake check path:$PWD -L`; run `cargo fmt --check`, `cargo build --workspace`, `cargo test --workspace`, and strict Clippy in the devshell; run the core-purity smoke; run the license-boundary tool; and run Cairn validation plus proposal, design, and tasks gates. A Gitleaks full-history scan is triaged; every hit must be a test identifier or digest, never a credential.

## Risks and Claim Boundaries

The switch changes only how an already-public object is fetched. It does not prove artifact-source correctness, whole-repository behavior correctness, forge availability, authorization, or release readiness. A passing anonymous fetch proves only that the declared public sources are reachable at their pinned revisions. Residual references in immutable receipts and archived evidence remain recorded as historical facts.

## Rollback

Restore the prior SSH declarations and regenerate both locks with their owning tools as one reviewed change. No automatic fallback is added.
