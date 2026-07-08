## Why

Cairn validates content-addressed Rust evidence receipts for Function Object, name-pointer, export-bundle, downstream-pilot, and circuit-primitive surfaces. Valence owns the canonical Evidence IR and current Function Object v1 construction contract. The two surfaces overlap around object hashes, BLAKE3 identity, source-token caveats, and no-correctness boundaries, but they are not currently one reviewed compatibility layer.

This change defines a Valence compatibility adapter for Cairn's content-addressed Rust evidence receipts. The adapter lets Valence consume or compare those receipts without pretending Cairn lifecycle validation proves Rust behavior or that Valence owns Cairn release-readiness policy.

## What Changes

- Define a compatibility manifest for Cairn content-addressed Rust evidence families and their Valence Evidence IR artifact mappings.
- Add adapters for Function Object v1 receipts and bundle/name-pointer linkage where schemas and hash material are compatible.
- Add explicit migration exceptions for legacy or divergent Cairn receipt shapes.
- Add positive fixtures for compatible source-token objects and export bundle/name-pointer linkage.
- Add negative fixtures for stale object hashes, ambient entropy in hash material, schema drift, identity/correctness overclaims, and unsupported receipt families.

## Impact

- **Valence** can treat Cairn content-addressed evidence as bounded identity/linkage input.
- **Cairn** keeps lifecycle/gate policy ownership and receipt validation semantics.
- **Octet/Valence Function Object v1** gains a reviewed compatibility story for older Cairn receipt validators.
