## Context

Unison identifies definitions by hashing a normalized syntax tree whose dependencies are also represented by hashes. Names are metadata pointers. Valence can adopt the same architectural separation for evidence identities: content hashes are identity; names and paths are presentation.

## Decisions

### 1. Use BLAKE3 over typed canonical forms

Valence SHOULD use BLAKE3 for new evidence-definition identities. The hash input must be a typed canonical value, not pretty JSON or human display text.

### 2. Dependency references use identities

When a canonical evidence definition depends on another function, artifact, requirement, or verifier input, the canonical payload should record that dependency's content identity where available. Human aliases may be recorded separately.

### 3. Additive rollout

Existing stable IDs and JSON fields remain compatible. New content-addressed identity fields should be additive until downstream Octet, Mantle, or Cairn consumers explicitly migrate.

### 4. Non-claim boundary remains bounded

Content identity proves exact canonical content and dependency closure identity only. It does not prove behavioral correctness, semantic equivalence, verifier soundness, release eligibility, or requirement satisfaction.

## Risks / Trade-offs

- Over-normalization could hide meaningful changes. Canonicalization rules must be explicit and fixture-backed.
- Under-normalization could make harmless display changes churn hashes. Rename/path fixtures must catch this.
- Some legacy fields may remain name-based for compatibility; typed wrappers should mark those as aliases, not identities.
