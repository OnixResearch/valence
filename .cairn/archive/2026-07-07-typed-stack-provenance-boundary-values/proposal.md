## Why

Stack-provenance verification currently accepts JSON DTOs with many string fields, then compares schema versions, digests, evidence tiers, statuses, roles, and claim scopes as raw strings. The public JSON strings are useful compatibility boundaries, but the core verifier and Preserves projection logic should not remain fully stringly typed.

Typed boundary values can reject malformed values once, prevent accidental comparisons between different hash/digest roles, and make validation errors more deterministic while preserving the existing JSON shape.

## What Changes

- Add typed internal boundary values for stack-provenance verification, starting with BLAKE3 digests, evidence tiers, verification status, claim scope, schema names, and external-evidence roles.
- Parse public DTO string fields into typed internal structs/enums before validation, Preserves projection, and report comparison.
- Keep JSON DTOs and CLI output backward compatible: external fields remain strings unless an explicit future compatibility break is planned.
- Add positive and negative tests for typed parsing, including malformed digests, unknown or promoted evidence tiers, invalid statuses, wrong claim scopes, and role/schema mismatches.
- Preserve Nickel and Preserves behavior: Nickel still validates metadata shape, Preserves still canonicalizes metadata values, and Rust still measures raw binary bytes.

## Impact

- **Files**: `crates/valence-core/src/stack_provenance_adapter.rs`, possibly a new submodule for typed boundary values, focused tests, README, and accepted Cairn specs.
- **Compatibility**: JSON shape remains unchanged; typed values are internal verifier/projection inputs.
- **Testing**: focused stack-provenance tests, Preserves corpus freshness, Nickel contract gate, full workspace tests, and Cairn validation/gates.
