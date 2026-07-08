## Why

Schema strings, evidence roles, claim scopes, and required non-claim text are copied across Valence, Mantle, Cairn, Octet, and Molten. A stack-wide registry prevents accidental drift without creating a mega core crate.

## What Changes

- Add a Valence-owned schema/role registry contract for evidence-facing stack constants.
- Validate consumer constants and docs against registry rows.
- Include negative fixtures for unknown roles, stale schemas, and overbroad non-claims.
