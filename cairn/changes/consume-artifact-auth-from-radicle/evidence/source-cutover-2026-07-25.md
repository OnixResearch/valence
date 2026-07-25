# Valence artifact-auth Radicle cutover — 2026-07-25

Valence now resolves `artifact-auth-core` and `artifact-auth-ed25519` from `https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git` at unchanged revision `799459346d5416fbd7b9f55840a7371441b55afa`. The source archive BLAKE3 remains `246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7`.

The accepted artifact-auth publication receipt at revision `b4343c1a1217de67726b259b5909657d720f09fd` has BLAKE3 `ac9bc126ea66c4ea9debcc320f88df667b8c5559a06eea97437f4e195d815fc2`.

## Lock agreement

Owning tools regenerated `Cargo.lock` and `flake.lock`. The source URL is the only Cargo package-graph change for both artifact-auth packages. The Nix input retained revision, revision count, and `narHash` `sha256-nEgz2FtVuDesX95yyxidp0vhjxL4INB6Ve8rkpLyJk0=`; only locked and original URLs changed.

| Surface | BLAKE3 |
|---|---|
| `Cargo.toml` | `45ff39f643cec55c40991acf67657cfc3835b60014201c804ab62c8391542fd0` |
| `Cargo.lock` | `855618624f1fadb4c5eca0e8c948a68aac8c56c2e1aaab265cb3c277c08e297a` |
| `flake.nix` | `d4a38c66262fb8e0e5c598fe45158f6c39fcee446abd51f3dd307c03049e757f` |
| `flake.lock` | `02fea0fd364148495be2b39d59522431c378a0acb6303d9c223f9900175ea27a` |

## Behavioral evidence

Baseline workspace tests passed at `70b36b91ec596915ae1a29aa3bbb53f0cba1ae48`. After cutover, workspace tests, focused positive and negative artifact-auth mapping tests, formatting, and workspace/all-target Clippy passed with no Rust implementation changes. The frozen source independently passed workspace tests, both no-std `wasm32-unknown-unknown` package checks, formatting, and workspace/all-target Clippy.

Typed mutation tests reject GitHub URLs, unknown RIDs, revision changes, source archive changes, publication-link changes, package omissions, crate-graph changes, Nix `narHash` changes, missing negative behavior, automatic fallback, and weakened non-claims.

## Claim boundary

No executable artifact-auth GitHub fallback remains in Cargo or Nix manifests or locks. An outage fails dependency resolution visibly. Rollback is explicit and requires source review plus owning-tool lock regeneration. This transport receipt does not prove artifact-auth source correctness, whole-Valence behavior correctness, forge availability, whole-stack GitHub independence, or release readiness.
