## Context

Valence now has three distinct verification layers for the built-binary thread:

1. Nickel validates static metadata shape and bounded-claim invariants.
2. Rust reads binary bytes, computes BLAKE3, and verifies cross-artifact identity links.
3. JSON remains the operator-facing exchange format for the CLI and existing automation.

What is still missing is a canonical semantic value identity for metadata. JSON can be stable enough for current tests, but it is not the best long-term substrate for corpus freshness, report diffs, or release receipt identity because harmless formatting or field-order changes can obscure semantic drift.

## Decisions

### 1. Preserves is additive metadata identity evidence

**Choice:** Add Preserves projections and `preserves_content_hash` values for stack-provenance metadata. Do not replace raw binary byte hashes, Mantle sidecar byte digests, or existing JSON CLI payloads.

**Rationale:** The built-binary claim depends on raw bytes. Preserves is valuable for metadata canonicalization and semantic drift detection, not for measuring executables or sidecar file bytes.

### 2. Rust owns the projection from DTOs to Preserves values

**Choice:** Implement pure Rust helpers that convert parsed stack-provenance DTOs into canonical Preserves values, then hash the canonical bytes with BLAKE3.

**Rationale:** The projection must be testable without filesystem I/O and must not depend on operator-facing JSON formatting. The CLI/test harness can call the pure helpers, while `valence-core` keeps the functional-core boundary.

### 3. Nickel and Preserves validate different things

**Choice:** Keep Nickel as the schema/static-invariant gate and Preserves as the canonical value/corpus/diff gate.

**Rationale:** Nickel answers "is this metadata well-formed and bounded?" Preserves answers "what exact canonical value did this metadata represent, and did that value drift?" Combining the two gives stronger evidence without making either layer responsible for byte measurement.

### 4. Corpus fixtures are explicit compatibility evidence

**Choice:** Add a checked-in Preserves corpus for representative stack-provenance values and a freshness gate that fails when canonical values or hashes drift without an intentional fixture update.

**Rationale:** The corpus makes metadata identity changes reviewable and gives downstream release automation a stable regression surface.

### 5. Verification reports get semantic diffs

**Choice:** Add Preserves-backed report hashes/diffs for built-binary thread reports.

**Rationale:** Release automation should be able to compare reports by semantic value, not JSON formatting. Report diffs must remain bounded to metadata identity and diagnostics; they must not imply correctness or release eligibility.

## Risks / Trade-offs

- Introducing Preserves adds a second metadata representation. Keep it generated from existing DTOs and avoid hand-maintained duplicate business logic.
- Preserves hashes may be mistaken for raw byte hashes. Use explicit names such as `preserves_content_hash` and document that binary identity remains BLAKE3 over raw binary bytes.
- Corpus freshness can create maintenance work. Keep fixtures small, representative, and intentional.
