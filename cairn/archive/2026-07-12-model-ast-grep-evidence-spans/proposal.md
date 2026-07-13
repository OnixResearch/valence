## Why

Valence owns canonical Evidence IR identity, spans, roles, and non-claim boundaries. ast-grep emits structured matches with file ranges, rule identifiers, severities, replacements, and metavariables. Valence should provide a bounded adapter profile that can wrap ast-grep findings as recorded-only or boundary evidence while preserving explicit non-claims.

## What Changes

- Define an ast-grep finding Evidence IR profile for source-shape matches and optional rewrite candidates.
- Validate rule bundle identity, tool identity, source span shape, finding identity, verification role, and non-claim labels.
- Reject artifacts that promote ast-grep matches to semantic correctness, proof discharge, or release eligibility.
- Add positive and negative fixtures for valid findings, missing spans, stale bundle identity, unsupported roles, and overclaims.

## Impact

- **Surfaces**: Evidence IR adapters, source spans, verification roles, non-claim validation, stack provenance links.
- **Non-claims**: Valence proves only artifact shape, identity linkage, span validity, role conformance, and non-claim preservation.
- **Validation**: adapter unit tests, malformed fixture tests, Cairn gates, cargo tests, and focused Valence checks.
