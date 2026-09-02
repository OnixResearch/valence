## Why

Verification role strings such as property, recorded-only, boundary, manual-review, and formal-proof tiers appear in Valence, Cairn, Octet, and Mantle. Duplicating them as literals risks role-counting drift and accidental tier promotion.

## What Changes

- Add a shared verification role vocabulary contract.
- Generate or validate role strings, tier mappings, and non-claim requirements.
- Add negative fixtures for unknown roles, role promotion, and weakened boundaries.
