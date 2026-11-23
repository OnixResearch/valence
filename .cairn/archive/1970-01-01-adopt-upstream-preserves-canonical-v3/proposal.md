# Change: Adopt upstream-compatible Preserves canonical binary v3 and versioned migration

## Why

Valence's `preserves-packed-structural-v2` profile emitted a private count-prefixed S-expression list encoding (tag `0x80` for list, `0xB0` for symbol, 8-byte big-endian counts) rather than upstream Preserves canonical binary syntax (`0.996.3`). An upstream Preserves reader cannot decode its output as a single value (for example, the 19-byte encoding of `(x)` starts with `0x80`, which upstream Preserves parses as `#f` followed by 18 trailing bytes), while valid upstream records such as `<x>` were rejected.

## What Changes

- Add a pure, bounded, zero-unsafe upstream Preserves `0.996.3` text parser, canonical binary encoder, and strict canonical binary decoder in `valence-core::preserves_canonical` covering booleans, arbitrary-precision signed integers, UTF-8 strings, byte strings, UTF-8 symbols, records, sequences, sets, and dictionaries.
- Enforce canonical encoded-byte ordering for sets and dictionaries and explicitly reject annotations, embedded values, IEEE 754 floats, duplicate set members/dictionary keys, non-minimal varints or integers, non-canonical binary ordering, trailing bytes/tokens, and resource limit breaches.
- Introduce active profile `preserves-canonical-binary-v3` with v3 structural self-hash exclusion profiles (`receipt-self-hash-canonical-v3`, `content-hash-self-canonical-v3`, `none-canonical-v3`) and BLAKE3 identities.
- Retain `preserves-packed-structural-v2` explicitly as historical private-format evidence (`historical_private_packed_v2_bytes` and `validate_historical_preserves_v2_evidence`), including the minimal `(x)` 19-byte regression, while rejecting v2 on the active `validate_preserves_evidence` write/validation path.
- Add versioned migration evidence (`preserves-integrity-migration-v2`) binding exact original bytes hash, source profile and exclusion rule, source identity, target profile and exclusion rule, target canonical Preserves identity, and required non-claims.
- Distinguish opaque-byte carrier validation (`validate_preserves_bridge`) from strict canonical Preserves binary validation (`validate_preserves_bridge_canonical`) and pin a Kamacite producer/Valence consumer interoperability fixture.

## Impact

Active Preserves evidence validation in `valence-core` uses upstream canonical binary bytes under `preserves-canonical-binary-v3`. Archived v2 receipts remain reproducible only through explicit historical validation and never get reinterpreted as upstream canonical Preserves hashes.

## Out of Scope

No in-place rewriting of historical receipts, no external Cargo crate additions, no I/O in `valence-core`, and no behavioral, authority, or release eligibility claims.
