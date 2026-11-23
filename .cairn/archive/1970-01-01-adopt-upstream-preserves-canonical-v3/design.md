# Design: Upstream Preserves canonical binary v3 profile

## Context

Normative Preserves `0.996.3` binary syntax (`https://preserves.dev/preserves-binary.html`) and canonical binary form (`https://preserves.dev/canonical-binary.html`) define tag bytes `0x80`/`0x81` for booleans, `0x84` as compound end tag, `0xB0`..`0xB7` for signed integers, strings, byte strings, symbols, records, sequences, sets, and dictionaries, minimal base-128 varint lengths, minimal big-endian two's-complement integer bytes, and lexicographic encoded-byte ordering for sets and dictionaries.

## Decisions

### Pure bounded codec in `valence-core::preserves_canonical`

Implement the bounded Preserves `0.996.3` codec directly in `valence-core` with zero `unsafe`, zero I/O, and zero new external Cargo dependencies. This preserves offline Nix/Cargo reproducibility and `license-policy.tsv` while guaranteeing deterministic `PreservesLimits` enforcement (`max_input_bytes`, `max_output_bytes`, `max_depth`, `max_collection_items`, `max_atom_bytes`, `max_values`).

### Explicit profile separation and historical retention

Name the new upstream-compatible profile `preserves-canonical-binary-v3` with exclusion profiles `receipt-self-hash-canonical-v3`, `content-hash-self-canonical-v3`, and `none-canonical-v3`. Retain `preserves-packed-structural-v2` only under explicit historical reproduction entry points (`historical_private_packed_v2_bytes`, `validate_historical_preserves_v2_evidence`). Reject `preserves-packed-structural-v2` and `preserves-text-v1` in active `validate_preserves_evidence`.

### Structural self-hash exclusion and migration binding

Under `receipt-self-hash-canonical-v3` and `content-hash-self-canonical-v3`, remove only exact top-level symbol keys (`receipt_hash`, `preserves_content_hash`) from top-level dictionaries or single-dictionary record envelopes. Keep payload strings, payload string keys, and symbol keys containing those names as substrings hash-bearing; fail closed on duplicate, nested, or misplaced structural self-hash fields. Versioned migration (`preserves-integrity-migration-v2`) binds exact original bytes, source profile/exclusion/identity, target profile/exclusion/identity, and non-claims, failing closed on missing originals or unsupported profiles.

### Opaque-byte carrier vs canonical binary bridge

Keep `validate_preserves_bridge` explicitly scoped to opaque-byte digest binding and add `validate_preserves_bridge_canonical` for strict single-value canonical binary decoding and validation. Pin a Kamacite `preserves-rs 5.0.0-rc.7` interoperability fixture in `fixtures/preserves-canonical-v3/pinned-interop-v1.json`.

## Rejected Alternatives

- Mutating `preserves-packed-structural-v2` in place: silently changes historical digests and violates immutability of archived receipts.
- Adding a new external `preserves` crate to `Cargo.toml`: expands dependency and license surface unnecessarily for the bounded pure subset.

## Validation

Run `cargo fmt --check`, `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, core-purity `rg` smoke, license-boundary check, and Cairn `validate` plus `gate proposal/design/tasks`.

## Risks and Claim Boundaries

Passing validation proves only canonical Preserves `0.996.3` content identity, explicit migration linkage, and typed claim-policy conformance within configured resource limits. It does not prove receipt truth, behavioral correctness, semantic equivalence, verifier soundness, execution authority, or release eligibility.

## Rollback

Revert the v3 module and spec additions as one commit; historical v2 receipts and fixtures are never modified in place.
