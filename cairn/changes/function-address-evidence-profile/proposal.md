## Why

Octet and Kamacite can produce function-address receipts, but stack consumers still need a Valence-owned Evidence IR boundary before release bundles or lifecycle gates rely on them. Without a Valence profile, downstream tools could confuse local function identity, dependency-closure identity, Kamacite receipt identity, and release artifact identity.

Valence should validate the function-address evidence profile as identity, linkage, role, digest-shape, source-span, and non-claim evidence only. It should not parse Rust, re-run Octet, interpret Kamacite semantics, or decide release readiness.

## What Changes

- Add a Valence Evidence IR profile for Kamacite function-address receipts and Octet producer metadata.
- Validate source-envelope, function-address, dependency-closure, receipt, projection, sidecar, and release-binary identity domains as distinct BLAKE3-backed roles.
- Produce graph-report/linkage rows that connect function-address receipts to Octet producer evidence, Kamacite receipt identity, source spans, and optional Mantle release sidecar metadata.
- Reject missing non-claims, overclaims, stale/malformed identities, unknown roles, identity-domain substitution, and unsupported profile versions.
- Document the boundary between Octet source analysis, Kamacite portable receipts, Valence linkage semantics, Mantle bundle binding, and Cairn readiness gates.

## Impact

- **Files**: Valence core adapter/profile validation, fixture rows, CLI/graph-report surfaces if needed, documentation, and Cairn lifecycle artifacts.
- **Testing**: positive Octet/Kamacite function-address fixtures, negative stale/domain/overclaim fixtures, graph report checks, core tests, CLI checks if exposed, Nickel/Preserves contract checks when applicable, and Cairn validation/gates before archive.
