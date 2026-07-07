## Context

Valence stack provenance now has three hardened layers:

1. Nickel contracts for static metadata shape and bounded-claim invariants.
2. Preserves projections and `preserves_content_hash` values for canonical metadata value identity.
3. Rust verification for raw binary byte measurement and cross-artifact digest/link checks.

The remaining weakness is that Rust core validation still works directly over many raw strings. That makes it easier to compare the wrong field class accidentally, repeat validation logic, or let malformed values progress farther than necessary.

## Decisions

### 1. Keep public DTOs string-compatible

**Choice:** Do not change the serialized JSON shapes for Octet exports, Valence sidecars, Mantle manifest views, or reports. Add typed internal boundary values that are parsed from those DTO strings.

**Rationale:** Octet, Mantle, and release automation already exchange these JSON strings. Internal typing should harden Valence without creating a cross-repo migration unless a future Cairn explicitly requests one.

### 2. Start with the highest-risk string classes

**Choice:** Introduce typed values for BLAKE3 digests, evidence tiers, verification status, claim scope, schema/version constants, and Mantle external-evidence roles before typing every identifier.

**Rationale:** These fields drive validity and fail-closed behavior. IDs such as source IDs and requirement IDs are lower-risk and can remain strings initially, with later newtypes if needed.

### 3. Parse once, then validate/projection use typed values

**Choice:** Add pure parsing/normalization functions that convert DTOs into typed internal views. Validation, Preserves projection, and report comparison should consume typed values where available.

**Rationale:** This keeps the functional core testable, reduces duplicated string checks, and makes parse errors deterministic.

### 4. Unknown compatibility values are explicit

**Choice:** Evidence tiers may support an `Unknown(String)` compatibility variant where Valence must preserve the source string but treat it conservatively. Verification statuses, claim scopes, schema versions, and roles should fail closed unless explicitly accepted.

**Rationale:** Evidence tier vocabularies may grow across repos, but statuses/scopes/schemas/roles define verifier semantics and should not silently broaden.

### 5. Preserves corpus drift is intentional

**Choice:** If typed normalization changes canonical metadata output, update the Preserves corpus intentionally and keep tests showing whether hashes stayed stable or changed for a documented reason.

**Rationale:** Typed internals should not silently churn release evidence. Corpus drift must remain reviewable.

## Risks / Trade-offs

- Adding typed views can duplicate DTO data. Keep the view small and pure; do not introduce runtime I/O or global state.
- Unknown evidence tiers can be mishandled. Unknown tiers must never be promoted to formal proof and should be reported or treated as manual-review-like unless explicitly recognized.
- Digest newtypes can be confused with role-specific identity. Prefer distinct wrappers or aliases where role matters, e.g. binary digest versus sidecar byte digest, even if they share BLAKE3 parsing.
