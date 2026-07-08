## Why

Unison's typed durable storage idea ties persisted values to stable content-addressed definitions. Valence persists evidence JSON and metadata, but durable storage should carry explicit type/schema identity so old evidence can be decoded and audited safely after code evolves.

This change proposes typed durable evidence storage metadata for persisted evidence values.

## What Changes

- Add typed durable storage metadata: value type identity, schema identity/hash, codec identity, content hash, and migration history.
- Validate persisted evidence before decoding into current DTOs.
- Add migration fixtures for older schemas and fail-closed tests for mismatched type/schema identity.
- Preserve bounded claims: typed storage proves decoding identity/compatibility, not correctness.

## Impact

- **Files**: likely Evidence IR DTOs, storage/codec helpers, fixtures, docs.
- **Compatibility**: additive envelope for new persisted evidence; old JSON can be imported through explicit legacy adapters.
- **Validation**: positive/negative decode and migration tests.
