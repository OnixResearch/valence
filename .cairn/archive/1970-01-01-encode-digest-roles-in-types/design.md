# Design: Encode digest roles in Rust types

## Context

Valence owns canonical identity and role semantics for stack evidence. The current `SharedDigest` checks role agreement at runtime. A caller can still pass a receipt digest to a binary-digest API because both values have the same Rust type.

The stack-provenance adapter demonstrates that small wrappers prevent this category error. The shared core needs one reusable pattern rather than repeated local wrappers.

## Decisions

### Decision: Use a generic marker-role digest

**Choice:** Add `Digest<Role>` with private normalized BLAKE3 storage and `PhantomData<Role>`. Define zero-sized marker types and public role aliases.

Add a borrowed `DigestRef<'a, Role>` where adapters need borrowed input.

**Rationale:** Distinct generic instantiations are not interchangeable. One parser and one invariant implementation can serve every role.

### Decision: Keep spelling metadata outside role identity

**Choice:** The typed digest owns the normalized BLAKE3 value. Accepted input spelling and original text remain adapter metadata where receipts need them.

**Rationale:** Prefix and case handling are compatibility concerns. They must not erase the role or change normalized content identity.

### Decision: Separate wire and admitted models

**Choice:** Keep a closed wire DTO with its serialized role tag. Convert it through `TryFrom` into a closed `AnyDigest` enum whose variants contain role-specific typed digests.

Core functions with a known role accept the exact alias. Heterogeneous reports accept `AnyDigest` and must pattern-match the role.

**Rationale:** Serialized records need a role tag. Core APIs need compile-time separation.

### Decision: Reuse the type across evidence surfaces

**Choice:** Migrate shared identity, artifact references, Evidence IR helper APIs, stack provenance, function-address policy identities, and graph summary identities in bounded steps.

Keep each public migration behind a compatibility adapter. Do not change all string fields in one patch.

**Rationale:** Valence is a shared dependency. Small migrations reduce compatibility risk.

### Decision: Preserve canonical bytes

**Choice:** Add golden tests for current JSON, canonical material, and BLAKE3 identities before changing core APIs. The newtypes must preserve those bytes unless a separate versioned schema change is approved.

**Rationale:** A zero-cost Rust boundary must not silently change evidence identity.

### Decision: Test category errors at compile time

**Choice:** Add compile-fail fixtures for receipt-to-binary, policy-to-artifact, and graph-to-sidecar substitution. Add compile-pass fixtures for same-role borrowing and owned conversion.

**Rationale:** Runtime tests cannot prove that a wrong-role call fails to compile.

## Functional core and shell boundary

Parsing, normalization, role conversion, comparison, and canonical projection remain pure over in-memory values.

CLI file reads, byte measurement, root confinement, output writes, and diagnostics remain in `valence-cli`.

## Test design

Positive tests cover every role alias, supported spelling, owned and borrowed conversion, heterogeneous enum round trips, unchanged canonical bytes, and existing stack fixtures.

Negative tests cover malformed BLAKE3 text, wrong prefixes, unsupported roles, cross-role wire conversion, duplicate role tags, stale canonical identities, and compile-time role substitution.

## Risks and trade-offs

- Generic aliases can make diagnostics verbose. Public aliases and focused error types keep messages readable.
- Heterogeneous collections still require runtime matching. The closed enum makes that matching explicit.
- Consumer Rust APIs can change. Compatibility constructors and a migration note limit breakage.
- Marker types do not prove evidence truth. Existing claim boundaries remain mandatory.

## Claim boundary

Typed digests prevent selected cross-role Rust calls and centralize local digest invariants. They do not prove measured bytes, evidence truth, producer authority, or release eligibility.
