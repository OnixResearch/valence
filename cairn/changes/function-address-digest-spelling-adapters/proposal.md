## Why

The smoke path crossed digest spellings: Octet and Valence often use `b3:<hex>`, while Kamacite identities store domain-tagged bare hex. Adapters need to normalize these spellings without weakening digest domains or letting arbitrary strings pass.

## What Changes

- Add function-address digest spelling adapters for `b3:<hex>` and bare BLAKE3 hex at explicit boundaries.
- Preserve typed digest domains after normalization.
- Add positive and negative fixtures for accepted spellings, wrong domains, malformed hex, and accidental SHA-like substitutions.

## Impact

Valence becomes the stable stack identity layer for function-address evidence without requiring each repo to reimplement digest spelling normalization.
