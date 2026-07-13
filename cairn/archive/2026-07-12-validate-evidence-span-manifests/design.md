## Context

`EvidenceSpan` documents one-based line and column coordinates and an exclusive end byte, but it is currently a passive DTO. `check_evidence_artifact` does not inspect `source_span`, and `explain_span` uses `Vec::find`. Because source-span display fields are deliberately excluded from artifact identity, hash validation cannot detect malformed or ambiguous span metadata.

## Decisions

### Validate span structure in a pure core

**Choice:** Introduce a deterministic validator over one `EvidenceSpan`. It requires non-empty, unpadded `source_id` and `source_path`; rejects NUL/control characters in identifiers and paths; requires nonzero one-based line and column values; requires `start_byte <= end_byte`; and requires the ending line/column position not to precede the start position. A present label must be non-empty and unpadded.

Zero-width spans may be accepted when both byte and line/column endpoints are equal. The validator does not infer byte offsets from Unicode columns or read source files.

### Validate the whole manifest before lookup

**Choice:** Parse and validate every span, then enforce one manifest row per `source_id`. Any invalid row or duplicate ID makes the manifest invalid, even when the requested ID would otherwise match a valid row. This prevents input order from choosing authority.

### Return a typed explanation outcome

**Choice:** Replace `Option<EvidenceSpan>` at the public validation boundary with a report or enum representing `Found`, `NotFound`, and `InvalidManifest`. Invalid outcomes include sorted, deduplicated field-scoped issue codes. The CLI keeps I/O errors separate, prints `not_found` only for a valid manifest lacking the ID, and prints manifest diagnostics for malformed or ambiguous input.

### Apply the same validator to artifacts

**Choice:** `check_evidence_artifact` validates a present `source_span` after strict envelope decoding. Invalid display metadata fails the artifact even though supported span fields remain excluded from canonical hash material. Builders return a construction error or validation report rather than manufacturing an artifact with a malformed span.

### Preserve the bounded claim

Span validation does not open `source_path`, compare source bytes, validate UTF-8 column semantics, prove compiler provenance, or establish semantic correctness. A future measured-source adapter may add those checks with separate evidence.

## Risks / Trade-offs

- Producers emitting zero line/column sentinels must migrate to absent spans or valid coordinates.
- Rejecting exact duplicate IDs is stricter than first-match behavior, but deterministic one-to-one explanation requires that strictness.
- The API change requires CLI and downstream callers to handle invalid-manifest diagnostics explicitly.
