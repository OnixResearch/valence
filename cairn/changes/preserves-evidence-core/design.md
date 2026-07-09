## Design

The Preserves evidence core is a pure `valence-core` module over already-loaded Preserves values, canonical bytes, envelope metadata, and hash fields. It does not evaluate Nickel, read files, run Preserves parsers with ambient state, or decide release/lifecycle policy.

### Decisions

1. **Canonical bytes are explicit input or explicit output.** Callers must choose a named Preserves canonicalization profile before computing `preserves_content_hash`.

2. **Envelope semantics are structural only.** The core validates schema, version, content hash, excluded self-hash fields, and required non-claims. It does not decide whether the envelope's domain claim is true.

3. **Self-hash profiles are named.** Profiles such as receipt hash, content hash, bundle hash, and sidecar hash exclusion must be declared. Unknown profiles fail closed.

4. **Adapters preserve public shapes.** Cairn, Mantle, Octet, and Valence may keep existing public field names while routing Preserves identity checks through the shared core.

5. **Claim boundary is mandatory.** Every validation report states that Preserves content identity is not receipt truth, release eligibility, lifecycle readiness, semantic equivalence, or verifier soundness.

### Validation shape

Positive tests cover known canonical Preserves values, matching content hashes, allowed envelope schemas, and self-hash exclusion profiles. Negative tests cover malformed Preserves text, stale hash, unknown schema, unknown exclusion profile, included self-reference, missing non-claims, and overclaim text.
