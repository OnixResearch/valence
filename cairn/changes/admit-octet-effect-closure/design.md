## Context

The archived `pilot-octet-deterministic-core` change established a focused `valence-shared-identity-pilot` package, two exact `core_source_scopes`, a reviewed 16-record function-address baseline, profile-bound direct-check artifacts, and stale-source rejection. Its documentation correctly excludes transitive effects and dependency closure.

Octet effect-closure v1 source-parses that same selected universe, anchors nodes and call-site IDs to the deny-mode function addresses, propagates a finite effect set over selected local calls, and requires exact reviewed summaries for every other call. It remains intentionally less expressive than Rust name resolution or MIR.

## Decisions

### 1. Preserve the existing selected source universe

**Choice:** Keep `crates/valence-core/src/shared_identity.rs` and `crates/valence-core/src/shared_identity/canonical_hash_profile.rs` as the only selected paths and continue checking the focused non-publishable pilot target.

**Rationale:** The new evidence should strengthen one known pilot rather than silently admit broader `valence-core` code.

### 2. Author policy in Nickel and commit generated JSON

**Choice:** Keep a copied typed contract and workspace policy under `policy/`, export deterministic runtime JSON to `octet-effect-closure.json`, and check freshness. Assign both selected paths explicitly to `core`.

**Rationale:** Human review belongs in typed policy; the Rust runner consumes deterministic JSON without requiring Nickel at runtime.

### 3. Use the denied receipt as the review queue

**Choice:** Run closure with no call summaries first and retain the denied artifact bundle. Extract each stable call-site ID, caller address, shape, and provider boundary from that report. Add only exact entries required by the selected source.

**Rationale:** This demonstrates fail-closed behavior and avoids speculative broad API allowlists.

### 4. Call summaries are identity-bound assertions, not waivers or proof

**Choice:** Each summary records the exact call-site ID, callee review label, provider kind/name/version/source/target/features, inferred effect set, and evidence BLAKE3 required by Octet. No summary may declare `unknown`, hide a direct forbidden effect, use a wildcard, or apply to an unused site.

**Rationale:** Review assertions become stale with source, provider, feature, target, or lockfile drift while retaining their bounded epistemic status.

### 5. Preserve ordinary Rust and direct-check behavior

**Choice:** The Nickel policy and generated JSON affect only explicit `cargo octet admit --profile deterministic-core --closure`. Ordinary Cargo and existing `cargo octet check` remain unchanged.

**Rationale:** Closure is an additional evidence rail, not a language fork or ambient compilation mode.

### 6. Retain and verify both outcomes

**Choice:** Store initial denied and final admitted bundles, run `cargo octet admit verify` on the final bundle, replay unchanged inputs for byte-identical receipt/status evidence, and run a tamper or stale-policy negative.

**Rationale:** Positive evidence alone would not demonstrate that the new boundary fails correctly.

## Risks / Trade-offs

- Exact call summaries are verbose and intentionally churn when a caller function address changes.
- The source-token resolver can reject valid Rust that a future HIR/MIR resolver could understand; Valence records those limits rather than adding broad fallbacks.
- A reviewed pure dependency call summary does not verify the dependency implementation. Exact Cargo/toolchain binding and non-claims prevent that assertion from being promoted.
- The canonical Valence Cairn policy compatibility blocker may require the compatible pinned Cairn CLI used by the prior pilot; that blocker remains separate and must be recorded rather than bypassed silently.
