## Why

The trust stack has provenance IDs at each layer (cairn `r[...]` requirement IDs, Tracey markers, valence function-object IDs, mantle attestation sidecars) but no single span that threads through all of them. A stable span model provides the target: every value carries an ID that flows source→receipt→binary and is resolvable back. This change adds the `provenance` module that threads the chain so the stack can answer "show me where this number comes from" mechanically through an end-to-end audit chain.

## What Changes

- Add `crates/valence-core/src/provenance.rs`: the `ProvenanceLink` type connecting `source_id` ↔ `function_object_id` ↔ `requirement_ids` ↔ `artifact_hash` ↔ `binary_hash`. The `ProvenanceChain` type threading multiple links. `check_provenance_chain` validator (fails closed on schema/boundary/empty/duplicate). `resolve` (source ID → link), `resolve_requirement` (requirement ID → function objects), `trace_binary` (binary hash → source functions).
- Register the module in `lib.rs`.
- 15 positive + negative tests covering resolution, tracing, schema mismatch, boundary weakening, empty IDs, duplicate IDs, multi-link chains, partial links (no artifact/binary), and JSON round-trip.

## Impact

- **Files**: `crates/valence-core/src/provenance.rs` (new), `crates/valence-core/src/lib.rs`.
- **Testing**: `cargo test --workspace` (62 tests: 55 core + 2 CLI unit + 5 CLI integration), `cargo clippy` (clean), `cargo fmt --check` (clean).
- **Non-claims**: provenance threading proves identity and linkage only. It does not prove behavioral correctness, semantic equivalence, or that the binary satisfies the requirement. It proves the binary's behavior is traceable to a source function and a requirement, not that it is correct.
