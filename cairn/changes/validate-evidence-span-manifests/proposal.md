## Why

`EvidenceSpan` is the stable source-to-receipt linkage record, but neither artifact checking nor `explain_span` validates its shape. The explainer parses a vector and returns the first matching `source_id`, so duplicate IDs are silently order-dependent. It also accepts impossible ranges and zero-valued one-based coordinates. A manifest with duplicate IDs, `start_byte` greater than `end_byte`, and zero line/column values currently exits successfully and prints the invalid first span.

This weakens the identity/linkage layer itself: callers cannot distinguish an absent ID from a malformed or ambiguous manifest, and invalid display metadata can be attached to an otherwise valid artifact because source spans are excluded from the artifact hash.

## What Changes

- Define a pure `EvidenceSpan` structural validator with deterministic field-scoped diagnostics.
- Validate optional artifact spans and complete span manifests before accepting or resolving them.
- Reject duplicate `source_id` entries instead of selecting the first match.
- Replace the `Option`-only explainer result with typed found, not-found, and invalid-manifest outcomes.
- Add positive and negative fixtures for coordinate boundaries, blank identifiers/paths/labels, duplicate IDs, malformed JSON, and artifact-attached invalid spans.
- Keep source-byte correspondence explicitly out of scope unless source bytes are separately supplied and measured.

## Impact

- **Core API**: span explanation becomes diagnostic-bearing rather than conflating invalid input with absence.
- **CLI behavior**: `valence explain` reports malformed and ambiguous manifests distinctly from `not_found`.
- **Evidence checking**: artifacts with structurally invalid optional spans fail even though valid span display metadata remains excluded from canonical artifact identity.
- **Claims**: passing structural validation proves only internally consistent span metadata and unique manifest lookup, not that the span matches source bytes or compiler semantics.
