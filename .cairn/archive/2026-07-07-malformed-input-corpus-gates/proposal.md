# Change: Malformed input corpus gates

## Why

Fail-closed behavior is a Valence boundary guarantee. The harness should collect malformed inputs into a durable corpus so regressions in validation, parsing, and diagnostic stability are easy to detect.

## What changes

- Add a malformed-input corpus for Evidence IR, evidence model, and stack-provenance surfaces.
- Add a corpus runner that expects deterministic failures and sorted issue codes.
- Cover malformed digests, duplicate graph nodes, dangling edges, unknown capabilities, stale bindings, weakened non-claims, and durable-envelope mismatches.

## Out of scope

- No fuzzing-only, non-reproducible failures in the required gate.
- No broad semantic correctness claims.
