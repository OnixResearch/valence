# Nominal digest roles

Valence uses one checked BLAKE3 implementation with separate Rust role markers.
The role marker prevents selected digest categories from crossing typed core APIs.

## Owned and borrowed types

Use an owned digest when the core must retain the identity:

```rust
use valence_core::shared_identity::{BinaryDigest, STRICT_B3_PROFILE};

let binary = BinaryDigest::parse(
    "b3:000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    STRICT_B3_PROFILE,
    "binary_hash",
)?;
```

Use `DigestRef<Role>` or a borrowed role alias for allocation-free validation.
Both forms keep the digest field private. They expose the normalized hex through
`as_hex()` and do not implement unrestricted primitive conversion.

Valence provides aliases for these roles:

- binary bytes
- sidecar bytes
- artifact identity
- receipt identity
- policy identity
- graph identity
- source identity
- Preserves metadata
- compatibility references

A `ReceiptDigest` cannot be passed to an API that requires `BinaryDigest`.
Compile-fail documentation fixtures check each reviewed role distinction.

## Wire admission

Serialized heterogeneous evidence keeps its `role`, `original`,
`normalized_hex`, and `spelling` fields. `SharedDigestWire` is the closed wire
DTO. It rejects unknown fields, duplicate fields, and unknown roles.

Convert the DTO with `AnyDigest::try_from`. Each `AnyDigest` variant contains a
role-specific admitted digest. The adapter checks the original spelling and the
normalized value before the typed core receives the value.

Converting `AnyDigest` back to `SharedDigestWire` preserves the accepted public
JSON bytes. Existing producer fields can remain strings while their adapters
admit typed values before core use.

The CLI admits `--binary-hash` as `AdmittedBinaryDigest` before it builds a
stack-provenance sidecar. The sidecar projection retains the original accepted
spelling.

## Consumer migration

1. Keep wire and Serde DTO fields unchanged.
2. Parse each string at its existing adapter boundary.
3. Pass the exact role alias into pure core functions.
4. Project through the compatibility DTO when public JSON must remain stable.
5. Use `parse_compatible` only for a documented boundary that already accepts
   uppercase hexadecimal text.

Do not add `Deref<Target = str>` or unrestricted `From<String>` conversions.
Use explicit `as_hex()` and fallible parsing.

## Claim boundary

Typed digests establish local syntax, normalization, and selected Rust role
separation. They do not prove measured bytes, evidence truth, producer
authority, behavioral correctness, verifier soundness, or release eligibility.
