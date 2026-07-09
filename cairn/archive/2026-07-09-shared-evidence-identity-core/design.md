## Design

The evidence identity core is a pure `valence-core` surface. It accepts in-memory strings, bytes, typed DTOs, and already-parsed metadata values. It does not read files, execute Nickel, inspect environment, spawn processes, or decide release eligibility.

### Decisions

1. **BLAKE3 is the canonical stack-owned identity algorithm.** The core provides a canonical normalized digest value plus compatibility parsers for raw hex, `b3:`, and `blake3:` spellings. Compatibility parsing must preserve the original public contract where a repo already exposes one of those spellings.

2. **Digest roles are distinct types.** Binary byte digests, sidecar byte digests, artifact digests, receipt digests, and Preserves metadata digests share parsing mechanics but are not interchangeable in public APIs.

3. **Canonical hashing is explicit.** JSON identity uses typed canonical DTO serialization or a named `serde_json::Value` profile. Preserves identity uses a named canonical Preserves bytes/text profile. Callers must choose the profile rather than relying on ambient `serde_json` formatting.

4. **Self-hash exclusion is a profile.** Receipt-shaped identities must state which fields are excluded, for example `receipt_hash`, `preserves_content_hash`, `object_hash`, `bundle_hash`, or role-specific equivalents. Missing or unknown exclusion profiles fail closed.

5. **Non-claim checks are reusable but bounded.** The core may detect required boundary text, missing non-claims, and known overclaim fragments. It does not determine whether a repo's evidence is true, sufficient, release-eligible, or behaviorally correct.

6. **Adapters migrate gradually.** Existing Cairn, Mantle, Basalt, Octet, and Molten/Aspen code may use adapters while preserving public JSON field names and error strings. Fixture compatibility is required before removing local helper code.

### Validation shape

The core is tested with positive fixtures for each accepted digest spelling, canonical JSON/Preserves value profile, self-hash exclusion profile, and required non-claim boundary. Negative fixtures cover wrong length, non-hex characters, prefix-role mismatch, uppercase policy where lowercase is required, unknown exclusion fields, hash mismatch, missing non-claims, and overclaim text.
