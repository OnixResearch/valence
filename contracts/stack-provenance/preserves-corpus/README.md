# Stack provenance Preserves corpus

`corpus.json` is the checked-in freshness corpus for canonical stack-provenance metadata values.

It records:

- Positive Preserves projections for each stack-provenance metadata value.
- The BLAKE3 `preserves_content_hash` for each canonical Preserves text value.
- A drift case that produces a deterministic hash mismatch and changed-field list.

The corpus is checked by the `stack_provenance_preserves_*` Rust tests. Update this file only when the metadata value contract intentionally changes.

## Claim boundary

This corpus proves canonical metadata value identity only.

It does not measure executable bytes or replace Mantle sidecar byte digests. It does not prove behavior, semantic equivalence, requirements, release eligibility, verifier soundness, or whole-program safety.
