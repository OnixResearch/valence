## Why

Valence should validate stack evidence-chain identity and linkage generically instead of adding a custom path for every evidence kind. Function-address evidence can be the first concrete profile, but the shared checks for canonical receipt hash, parent links, policy hashes, roles, graph roots, projections, and non-claims should be reusable.

## What Changes

- Add a generic Evidence IR profile for Kamacite evidence-chain envelopes.
- Validate canonical Preserves envelope identity, typed parent links, evidence kind, profile roots, policy/profile hashes, verification roles, projections, and non-claim boundaries.
- Build reusable graph rows for chain links while preserving profile-specific payload checks.
- Use function-address as the first profile exercising the generic path.

## Impact

Valence becomes the shared evidence-chain validator for the stack while preserving profile-specific semantics and recorded-only boundaries.
