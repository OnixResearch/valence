## Why

Artifact references appear across Basalt, Molten, Mantle, Cairn, Octet, and Valence as raw BLAKE3 hex strings, `b3:` refs, `blake3:` refs, path-bound artifacts, sidecars, receipts, and release bundle members. These references are identity plumbing, not domain semantics, and drift here creates avoidable compatibility failures.

Valence should own a reusable artifact reference core that can be shared directly or implemented as the artifact-ref facet of the shared evidence identity core.

## What Changes

- Define typed artifact reference DTOs for bytes, sidecars, receipts, release artifacts, source artifacts, and compatibility refs.
- Support declared digest spelling profiles while preserving existing public fields.
- Add path confinement and size/hash metadata validators for already-loaded artifact metadata.
- Add positive fixtures for current Valence, Cairn, Mantle, Basalt, Octet, and Molten/Aspen examples.
- Add negative fixtures for malformed refs, wrong digest spellings, role mixups, path escapes, stale size/hash metadata, and overclaims.

## Impact

- **Valence** becomes the owner of reusable artifact reference parsing and compatibility checks.
- **Consumers** can stop maintaining local digest/ref validators without moving their domain evidence logic.
- **Implementation** may be a standalone module or a public submodule of `shared-evidence-identity-core` depending on landing order.
