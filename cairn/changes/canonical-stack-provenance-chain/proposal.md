# Change: Canonical stack provenance chain

## Why

Valence is the stack identity layer. The current adapter already links Octet
exports, Valence sidecars, Mantle metadata, and measured binary bytes, but the
next hardening step is to make the whole chain explicitly content-addressed,
typed, replayable, and non-overclaiming before Mantle and Cairn consume it as a
release gate input.

## What Changes

- Define canonical stack provenance artifact IDs for requirements, spans,
  receipts, proof artifacts, sidecars, release bundles, and lifecycle gates using
  BLAKE3 over deterministic canonical content.
- Add a typed edge vocabulary that records why artifacts are linked, including
  local proof, source-shape check, provenance export, release bundle inclusion,
  gate satisfaction, and manual-review input.
- Enforce non-claim boundaries as machine-checked policy so Trellis, Octet,
  Mantle, and Cairn artifacts cannot imply unsupported correctness or release
  claims.
- Emit a provenance graph report in machine JSON and operator Markdown with
  artifacts, edges, trust boundaries, manual nodes, and failed or incomplete
  links.
- Add positive and negative fixtures for valid chains, missing parents, digest
  mismatches, stale policy, unsupported edge kinds, tier promotion, and weakened
  non-claims.

## Impact

- **Files**: stack-provenance adapter core, Nickel contracts, Preserves corpus,
  fixtures, CLI report rendering, and tests.
- **Consumers**: Mantle and Cairn can require Valence-verified sidecars without
  implementing stack semantics themselves.
- **Non-claims**: Valence verifies identity, linkage, role, and non-claim policy;
  it does not prove behavioral correctness, verifier soundness, release
  eligibility, or requirement satisfaction.
