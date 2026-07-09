## Why

Stack evidence roles are currently repeated across Valence role/schema work, Molten stack evidence members, Mantle release evidence, Cairn release/lifecycle receipts, and Octet provenance exports. Each local copy is small, but role drift changes whether a stack artifact is treated as Basalt, UCAN, Trellis, Octet, Valence, Cairn, Mantle, or external/manual-review evidence.

Valence is the stack evidence identity layer, so it should own a reusable pure role-registry core that downstream repos can consume without delegating their runtime authority, release policy, or lifecycle policy to Valence.

## What Changes

- Define a Valence-owned stack role registry core for role IDs, schema IDs, verification role vocabulary, compatibility classes, and role/schema matching diagnostics.
- Add compatibility adapters for existing public strings used by Molten, Mantle, Cairn, Octet, and Valence sidecars.
- Add positive fixtures for known stack roles and accepted compatibility aliases.
- Add negative fixtures for unknown roles, stale aliases, schema/role mismatches, duplicate required roles, missing verification roles, and authority/release overclaims.
- Preserve downstream ownership boundaries: role compatibility proves identity/vocabulary alignment only.

## Impact

- **Valence** becomes the shared source for stack role vocabulary and compatibility predicates.
- **Molten/Mantle/Cairn/Octet** can replace local role string tables gradually through adapters.
- **Existing active Valence role work** remains compatible: this package focuses the reusable pure-core extraction boundary rather than replacing repo-local product changes.
