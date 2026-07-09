## Why

Cairn, Mantle, Valence, Basalt, Aspen/Molten, and Octet all validate stack-owned evidence identities with local helper code: BLAKE3 hex parsing, optional digest prefixes, canonical JSON or Preserves hashing, self-hash field exclusion, and non-claim boundary checks. Those helpers are small, but drift in this layer changes what the stack treats as the same artifact.

Valence is already the stack evidence identity layer. A narrow shared evidence-identity core gives downstream repos one pure Rust source for digest syntax, canonical identity helpers, role-specific digest wrappers, and bounded non-claim predicates without making Valence responsible for each repo's domain semantics.

## What Changes

- Define a Valence-owned evidence identity core for BLAKE3 digest strings, canonical JSON/Preserves content hashes, self-referential hash-field exclusion, and non-claim boundary predicates.
- Add role-specific digest wrappers for binary bytes, sidecar bytes, artifact identities, Preserves metadata, receipts, and compatibility references.
- Provide adapter profiles for existing stack spellings such as raw 64-character hex, `b3:`, and `blake3:` where compatibility requires them.
- Add positive and negative fixture matrices covering Valence, Cairn, Mantle, Basalt, Octet, and Molten/Aspen examples.
- Preserve repo ownership boundaries: consumers may use shared identity helpers, but each repo still owns its evidence semantics and release/lifecycle decisions.

## Impact

- **Valence** becomes the source of truth for shared evidence-identity primitives.
- **Cairn/Mantle/Basalt/Aspen/Octet** may replace local digest and non-claim helpers gradually through adapter profiles.
- **Testing** must cover valid identities, malformed digests, prefix mismatches, self-hash exclusion, deterministic canonical bytes, and overclaim rejection.
