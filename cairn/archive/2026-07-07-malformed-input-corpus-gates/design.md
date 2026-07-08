# Design: Malformed input corpus gates

## Approach

Use checked-in corpus fixtures grouped by surface and issue class. Each fixture records expected issue codes and a brief reason. The runner loads fixtures in sorted order, invokes pure validation cores where possible, and compares deterministic diagnostics.

## Corpus classes

- BLAKE3 and content identity syntax errors.
- Duplicate or dangling evidence graph structure.
- Ambiguous aliases and stale name bindings.
- Unknown or denied verifier capabilities.
- Missing or weakened non-claim boundaries.
- Durable storage schema, type, codec, content hash, and migration mismatches.
- Stack-provenance stale parent, digest, tier, and role errors.

## Failure model

A fixture passes only when the expected failure is observed. Unexpected success is a harness failure.
