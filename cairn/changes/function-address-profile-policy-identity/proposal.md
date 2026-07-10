## Why

The first smoke fixture used a placeholder Valence policy hash (`aaaaaaaa...`) while exercising the function-address Evidence IR profile. A portable stack path needs a real, deterministic Valence policy/profile identity so Mantle and Cairn can bind the evidence to a reviewed Valence validation contract.

## What Changes

- Define the canonical Valence function-address profile policy material.
- Compute and expose a deterministic BLAKE3 policy/profile identity.
- Replace placeholder policy hashes in docs, fixtures, adapters, and smoke outputs.
- Add positive and negative fixtures for matching, stale, missing, and malformed policy identity.

## Impact

Function-address Valence receipts can be bound by Mantle and gated by Cairn without relying on placeholder policy identity values.
