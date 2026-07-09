## Why

Preserves-shaped evidence appears in Valence stack-provenance metadata, Cairn Preserves receipt ingestion, Mantle release evidence, and Octet evidence receipt export. These surfaces share canonical Preserves bytes, content hashes, schema/version envelopes, and self-hash exclusion rules, but each repo currently owns pieces of that validation locally.

A Valence-owned Preserves evidence core gives the stack one pure source for Preserves evidence identity while preserving each repo's domain-specific receipt meaning.

## What Changes

- Define a reusable Preserves evidence core for canonical Preserves bytes, `preserves_content_hash`, schema/version envelopes, and self-hash exclusion profiles.
- Add DTOs for Preserves evidence envelopes, content identity reports, and compatibility diagnostics.
- Add positive fixtures for Valence, Cairn, Mantle, and Octet Preserves evidence examples.
- Add negative fixtures for malformed Preserves text, stale content hashes, unknown schemas, wrong exclusion profiles, missing non-claims, and overclaims.
- Document that Preserves content identity does not prove receipt truth, release readiness, lifecycle readiness, or behavioral correctness.

## Impact

- **Valence** owns reusable Preserves identity and validation primitives.
- **Cairn/Mantle/Octet** can adopt the core through adapters without moving their receipt semantics.
- **Existing Valence Preserves bridge work** can depend on this reusable boundary rather than growing repo-specific helpers.
