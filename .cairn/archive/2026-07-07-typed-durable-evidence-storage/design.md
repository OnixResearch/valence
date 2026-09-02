## Context

Evidence artifacts and reports may be stored for a long time. If the current code changes, Valence still needs to know what type/schema/codec produced a persisted value and whether a migration is required.

## Decisions

### 1. Durable envelope is typed

Persisted evidence values should carry a durable envelope containing schema version, value type identity, codec identity, content hash, and optional migration history.

### 2. Decode validates before construction

Valence should validate the durable envelope before decoding into current in-memory DTOs. Unknown or mismatched type/schema identities fail closed unless a migration adapter is present.

### 3. Migrations are explicit evidence

A migration from an older schema to a newer DTO should record source identity, target identity, migration tool identity, and diagnostics.

### 4. Storage claim is bounded

Typed durable storage preserves value identity and decode compatibility. It does not prove the persisted evidence is true or that requirements are satisfied.

## Risks / Trade-offs

- Extra envelopes add verbosity. Keep the envelope compact and deterministic.
- Migration adapters can overclaim. Require non-claims and negative fixtures for bad migrations.
