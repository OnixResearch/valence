## Context

`evidence_codebase_index` builds a deterministic index from in-memory evidence artifacts. Sources are normalized by locator and content hash, then provenance records compare referenced artifact locators against the supplied source bytes. A plain ordered map is deterministic, but duplicate normalized locators still need explicit policy instead of accidental overwrite semantics.

## Design

### Source identity index

Build an internal source index from normalized sources using `IdOrdMap<NormalizedSource>` keyed by `NormalizedSource.locator`. The value already carries the locator and BLAKE3 digest, so iddqd removes the need to maintain a separate map key/value pair.

Use ordered maps rather than hash maps because source artifacts are externally supplied and Valence already prefers deterministic ordering for reports and hashes.

### Duplicate policy

Strict mode should treat duplicate normalized locators as errors when they would make artifact identity ambiguous. Advisory mode may continue to collapse exact duplicate `(locator, blake3)` pairs, but conflicting duplicate locators must be reported as caveats rather than silently replacing a source hash.

### Provenance validation

Provenance record ingestion should query the source identity index by normalized locator. Matching BLAKE3 values pass, mismatches produce the existing mismatch diagnostic, and missing locators continue to produce referenced-artifact-missing caveats. The public `EvidenceCodebaseIndex` record and issue shapes remain unchanged.

### Fixtures

Positive fixtures cover unique sources, exact duplicate advisory sources, provenance records referencing supplied artifacts, and stable index hashes. Negative fixtures cover conflicting duplicate locators, stale provenance hashes, missing referenced artifacts, malformed JSON/JSONL, and overbroad correctness claims.

## Non-goals

- Do not change Evidence IR public JSON schemas or index query output.
- Do not claim evidence truth, requirement satisfaction, verifier soundness, semantic equivalence, release eligibility, or whole-program safety.
- Do not add filesystem, process, clock, or environment access to `valence-core`.
