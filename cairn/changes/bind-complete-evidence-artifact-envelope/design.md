## Context

The current checker parses JSON straight into `EvidenceArtifact`. Serde ignores unknown struct fields unless strict decoding is requested. Hash recomputation then serializes only the recognized struct, and overclaim detection scans only `payload` and `non_claims`. Unknown top-level data is therefore neither rejected nor hash-bound.

Separately, artifact hashing and several policy identities depend on a `serde_json::Value` serialization path whose map ordering can vary when dependency features are unified by a downstream consumer. Valence owns canonical identity and cannot delegate that invariant to an optional dependency feature.

## Decisions

### Inventory closed fields before accepting the envelope

**Choice:** Parse the input into an in-memory JSON value, require an object, inventory the exact allowed top-level field set, and then perform strict typed decoding. Closed nested control records, including `EvidenceSpan`, also reject unknown fields. Duplicate JSON keys must fail during decoding rather than selecting one value.

The kind-specific `payload` remains intentionally open. Every payload key and value is included in canonical hash material, so new producer-specific payload fields do not require widening the closed outer envelope.

### Return field-aware deterministic diagnostics

**Choice:** Distinguish malformed JSON, unknown artifact field, duplicate artifact field, unknown closed-nested field, and typed field mismatch. Diagnostics identify the rejected JSON path without echoing untrusted values. The pure decoder returns a typed result/report; the CLI only renders it.

### Canonicalize independently of `serde_json` feature flags

**Choice:** Use one explicit canonical JSON byte encoder that recursively sorts object keys and serializes scalars deterministically. Artifact hashing, shared JSON identity, and function-address policy identity use that encoder instead of relying on `serde_json::Map` implementation order.

A dedicated consumer fixture enables `serde_json/preserve_order` and must reproduce the same golden bytes and BLAKE3 identities as the ordinary workspace build.

### Preserve current valid identities

**Choice:** Check in representative golden artifacts and assert that strict decoding plus explicit canonicalization reproduces their existing hashes. If implementation discovers a current hash that cannot be preserved, the change must stop and introduce a named versioned hash profile and migration; it must not silently rewrite identities.

### Complement typed claim policy

The active `harden-preserves-integrity-boundaries` package replaces prose authority with typed claims. This change is orthogonal: it ensures no unrecognized outer field can bypass that policy or escape the hash. It does not duplicate Preserves parsing or claim-class design.

## Risks / Trade-offs

- Rejecting unknown outer fields narrows forward compatibility, but silently discarding evidence control fields is unsafe. New fields require a schema version or explicit compatibility adapter.
- Strict nested decoding must distinguish closed control records from intentionally open payloads.
- Feature-matrix tests add a small verification cost but protect canonical identity at library-consumer boundaries.
