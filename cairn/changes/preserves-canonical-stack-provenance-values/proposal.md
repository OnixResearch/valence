## Why

The built-binary thread now has Rust byte measurement and Nickel metadata contracts, but the metadata identity surface still relies on JSON rendering details for fixture drift and report comparison. Preserves can strengthen this layer by giving stack-provenance metadata a canonical semantic value encoding with stable BLAKE3 content hashes and deterministic corpus/diff checks.

This should not replace raw binary hashing or Mantle's sidecar byte digest. Preserves should harden the metadata identity layer: Octet export metadata, Valence sidecars, Mantle manifest views, built-binary thread input envelopes, and verification reports.

## What Changes

- Add canonical Preserves value projections for stack-provenance metadata surfaces.
- Compute stable `preserves_content_hash` values with BLAKE3 over canonical Preserves bytes.
- Add a Preserves corpus for representative positive and negative stack-provenance metadata fixtures.
- Add a deterministic gate that detects corpus drift and report-shape drift.
- Add report comparison support so automation can diff semantic verification reports without depending on JSON field order.
- Document the boundary: Preserves proves metadata value identity/canonicalization only; Rust still measures built binary bytes, and Mantle still records raw sidecar byte digests.

## Impact

- **Files**: stack-provenance core helpers, fixture/corpus directories, optional CLI/test harness gate, README, and Cairn specs.
- **Compatibility**: Existing JSON CLI output and Mantle sidecar digests remain valid. Preserves hashes are additive metadata identity evidence.
- **Testing**: Preserves corpus freshness checks, positive/negative fixture checks, Rust workspace tests, `cargo fmt --check`, and Cairn validation/gates.
