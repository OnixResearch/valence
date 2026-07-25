# Design: Valence artifact-auth Radicle cutover

## Identity contract

The cutover accepts one public URL of the form `https://git.onix.computer/<RID>.git` and exact revision `799459346d5416fbd7b9f55840a7371441b55afa`. Cargo manifest entries, Cargo lock sources, the Nix flake input, and the Nix lock node must agree. The locked Nix `narHash` must remain content-identical to the GitHub-backed source.

## Functional boundary

No Rust implementation changes are required. Existing pure mapping, canonical statement, Ed25519, currentness, threshold, and authority-boundary tests are the behavioral oracle. Source parsing and admission checks remain deterministic; network fetching and lock regeneration are build-tool effects.

## Fail-closed checks

Validation rejects revision drift, RID/URL disagreement, GitHub fallback, absent package entries, changed Nix content identity, changed crate graph, or missing positive and negative test evidence.

## Rollback

Rollback is an explicit source-identity change back to the same reviewed Git object and requires regenerated owning-tool locks. There is no automatic runtime fallback because automatic fallback would hide forge outage and weaken evidence.

## Evidence

A typed Nickel source exports JSON and a BLAKE3 sidecar. It binds publication receipt identity, public URL, RID, revision, source archive identity, Cargo/Nix agreement, baseline/post-cutover results, and bounded non-claims.
