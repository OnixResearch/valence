## Design

The existing `valence-core::function_address_profile` remains the pure validator. `valence-cli` adds an imperative shell that reads pre-existing Octet/Kamacite evidence projections, builds typed profile input, invokes the core, and writes receipt/graph JSON.

### Decisions

1. **Valence does not parse Rust.** CLI inputs are already-produced evidence records and receipts.
2. **Core is unchanged authority.** Role, digest-domain, non-claim, and graph-linkage checks stay in `valence-core`.
3. **Receipt and graph outputs are deterministic.** Output JSON is stable enough for Mantle binding and Cairn readiness inputs.
4. **Recorded-only role is enforced.** CLI validation cannot promote function-address evidence to property or formal-proof status.

### Validation shape

Positive fixtures cover a Kamacite projection with matching receipt identity and function-address records. Negative fixtures cover wrong digest domain, missing span, stale Kamacite identity, unknown role, forbidden semantic claim, and verification-role promotion.
