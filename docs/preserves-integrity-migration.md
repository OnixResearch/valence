# Preserves integrity profile migration

Valence versions Preserves evidence canonicalization explicitly and never reinterprets historical digests under a newer encoding profile:

| Profile | Format | Active writes | Reproduction / migration |
| --- | --- | --- | --- |
| `preserves-text-v1` | Legacy line-filtering text hash | Rejected (`legacy_profile_rejected`) | Explicit migration source only |
| `preserves-packed-structural-v2` | Historical private count-prefixed S-expression list encoding (NOT upstream Preserves binary) | Rejected (`historical_private_profile_rejected`) | Explicit historical verification (`validate_historical_preserves_v2_evidence`) and migration source only |
| `preserves-canonical-binary-v3` | Bounded upstream Preserves 0.996.3 canonical binary encoding with BLAKE3 identity | Active | Active canonical target |

## Active upstream-compatible profile (`preserves-canonical-binary-v3`)

The active profile encodes admitted Preserves values per <https://preserves.dev/preserves-binary.html> and <https://preserves.dev/canonical-binary.html> (`0.996.3`) and hashes the resulting canonical binary bytes with BLAKE3:

- **Admitted values:** booleans (`0x80` / `0x81`), arbitrary-precision signed integers (`0xB0`, minimal base-128 varint length, minimal big-endian two's-complement bytes; `0` encodes as `0xB0 0x00`), UTF-8 strings (`0xB1`), byte strings (`0xB2`), UTF-8 symbols (`0xB3`), records (`0xB4` ... `0x84`), sequences (`0xB5` ... `0x84`), sets (`0xB6` ... `0x84`), and dictionaries (`0xB7` ... `0x84`).
- **Canonical collection ordering:** set members and dictionary entries are ordered strictly ascending by their canonical encoded binary bytes (lexicographic byte comparison).
- **Explicit rejections:** annotations (`@`, `;`, `0x85`), embedded values (`#:`, `0x86`), IEEE 754 floats (`0x87` and float literals), duplicate set members or dictionary keys, non-minimal varints or integer encodings, non-canonical binary set/dictionary ordering, trailing bytes or tokens, invalid UTF-8, and breaches of configured `PreservesLimits` (`max_input_bytes`, `max_output_bytes`, `max_depth`, `max_collection_items`, `max_atom_bytes`, `max_values`) all fail closed.
- **Structural self-hash exclusion:** `receipt-self-hash-canonical-v3` removes only top-level symbol keys `receipt_hash` and `preserves_content_hash` from a top-level dictionary or single-dictionary record envelope (`content-hash-self-canonical-v3` removes only `preserves_content_hash`; `none-canonical-v3` removes nothing). Payload strings, payload string keys, and symbol names that merely contain `receipt_hash` or `preserves_content_hash` remain hash-bearing. Duplicate, nested, or misplaced structural self-hash fields and malformed envelopes fail closed.

## Historical private-format retention (`preserves-packed-structural-v2`)

The prior `preserves-packed-structural-v2` profile used private tags (`0x80` list, `0xB0` symbol, `0xB1` string) and 8-byte big-endian counts rather than upstream Preserves binary. It is retained explicitly under `historical_private_packed_v2_bytes` and `validate_historical_preserves_v2_evidence` so existing archived receipts remain reproducible under their original profile (including the minimal `(x)` 19-byte regression). Active `validate_preserves_evidence` rejects `preserves-packed-structural-v2` so new writes cannot silently advertise the private encoding as upstream canonical Preserves.

## Opaque-byte carrier vs canonical Preserves validation

`validate_preserves_bridge` performs an opaque-byte carrier check (BLAKE3 digest binding, schema, role, and non-claim conformance over raw `canonical_bytes`) and does not parse Preserves syntax. `validate_preserves_bridge_canonical` additionally decodes and strictly verifies that `canonical_bytes` is a single canonical upstream Preserves 0.996.3 value with no trailing bytes. Interoperability with Kamacite's `preserves-rs 5.0.0-rc.7` canonical binary output is pinned in `fixtures/preserves-canonical-v3/pinned-interop-v1.json`.

## Migration ordering (`preserves-integrity-migration-v2`)

1. Retain the exact original historical envelope bytes (`preserves-text-v1` or `preserves-packed-structural-v2`).
2. Validate the declared source identity against those original bytes under the declared source profile and source exclusion rule.
3. Supply the target upstream Preserves value and select an explicit v3 exclusion profile (`receipt-self-hash-canonical-v3`, `content-hash-self-canonical-v3`, or `none-canonical-v3`).
4. Emit a `preserves-integrity-migration-v2` receipt (`migrate_preserves_profile`) binding `original_bytes_hash`, `source_profile`, `source_exclusion_profile`, `source_identity`, `target_profile`, `target_exclusion_profile`, `target_identity`, and the required non-claim boundary.
5. Configure each downstream consumer to accept `preserves-canonical-binary-v3` or validate the explicit migration receipt before publishing migrated evidence.

Valence never rewrites a historical receipt in place, never reinterprets a v1 or v2 digest as a v3 canonical Preserves digest, and fails closed when original bytes are missing or profiles are unsupported.

## Typed claim boundary

Claim authority is carried by typed claim class, verification role, required boundary identifiers, and prohibited capability values. Human-readable text remains display metadata: it cannot grant authority, promote a role, or replace the required `valence.identity-linkage-only.v1` boundary. Contradictory metadata that asserts correctness, equivalence, authority, verifier soundness, or release eligibility fails validation.

Passing these checks proves only canonical content identity, explicit migration linkage, and typed claim-policy conformance within the bounded profile. It does not prove receipt truth, behavioral correctness, semantic equivalence, verifier soundness, execution authority, or release eligibility.
