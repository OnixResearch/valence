## Why

The stack is adding WebAssembly component build, validation, composition, virtualization, precompilation, and runtime receipts across Mantle, Octet, Kamacite, and Aspen. Valence has generic artifact refs and stack-provenance edges, but no profile that keeps portable component bytes, WIT interface identity, OCI package digests, composition graphs, WASI virtualization, target-specific `.cwasm`, and execution receipts in distinct evidence roles.

Without a shared profile, consumers can confuse matching WIT worlds with matching implementations, treat OCI SHA-256 as Valence canonical identity, or promote static/build/runtime evidence beyond its owner-defined claim.

## What Changes

- Define a WebAssembly component Evidence IR profile and typed artifact/edge vocabulary.
- Keep exact portable component, WIT source, composition/virtualization plan, precompiled artifact, and receipt identities BLAKE3-based and role-separated.
- Record OCI/wkg SHA-256 only as external interoperability metadata linked to exact BLAKE3-identified bytes.
- Model normalized WIT package/world/import/export identity independently from component byte identity and reject semantic-equivalence claims.
- Bind component tooling cohort, runtime configuration, target, CPU features, policy/resource refs, and producer evidence without interpreting producer behavior.
- Add graph edges for package resolution, world implementation, composition, virtualization, validation, transformation, precompilation, and execution.
- Enforce recorded-only/boundary roles and explicit non-claims with positive and negative fixtures.

## Impact

- **Surfaces**: `valence-core` Evidence IR adapters and graph vocabulary, canonical DTOs, Nickel contracts, fixtures, CLI adapter validation, and stack-provenance reports.
- **Owners**: Mantle owns builds, Octet owns static artifact checks, Kamacite owns adapter/source/value receipts, Aspen owns runtime execution, and Cairn owns lifecycle policy. Valence owns only identity, linkage, roles, and non-claim conformance.
- **Hashing**: all Valence-owned canonical identity remains BLAKE3. External SHA-256 is never accepted in a Valence artifact identity role.
- **Claims**: the profile does not prove component behavior, WIT semantic equivalence, sandbox effectiveness, package trust, authority, or release eligibility.
