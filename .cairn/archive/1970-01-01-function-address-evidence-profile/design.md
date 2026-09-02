## Context

Valence already models content-addressed evidence definitions, Function Object v1 compatibility, stack provenance sidecars, and FSM default-exhaustive evidence profiles. Function-address receipts are the next adapter profile: they bind a producer's function inventory to source spans and artifact identities without making Valence a Rust analyzer or Kamacite runtime.

The profile should preserve the same non-claim discipline as existing evidence adapters. Passing validation means the supplied function-address evidence is well-formed, linked, role-separated, and non-overclaiming. It does not mean the underlying program is correct.

## Decisions

### 1. Validate profile records over parsed values

**Choice:** Implement a pure Valence core profile over already-loaded function-address records and artifact refs. CLI/file loading, byte measurement, graph report rendering, and sidecar writing remain in `valence-cli` or upstream shells.

**Rationale:** Valence core purity is mandatory, and the profile should be easy to test with positive and negative in-memory fixtures.

### 2. Preserve identity domains explicitly

**Choice:** The profile distinguishes source-envelope identity, local function address, dependency-closure identity, Kamacite receipt identity, Valence projection identity, sidecar digest, Octet producer receipt identity, and release-binary identity.

**Rationale:** The evidence is only useful if downstream gates cannot substitute one BLAKE3 digest role for another.

### 3. Treat function addresses as recorded/boundary evidence unless paired with stronger proof

**Choice:** Function-address receipts default to recorded-only or boundary verification roles. They may be linked to separate formal proof receipts, but the function-address profile itself does not become formal proof evidence.

**Rationale:** Content identity is not behavioral correctness. This mirrors the formal-proof and FSM profile boundary already in Valence.

### 4. Support graph-report linkage without owning release policy

**Choice:** Valence may include function-address nodes and edges in stack graph reports, including edges to Mantle release sidecars. Cairn decides lifecycle/release readiness, and Mantle validates bundle-local artifacts.

**Rationale:** Valence owns evidence linkage semantics, not release gates or artifact packaging.

### 5. Reject overclaims and weakened boundaries

**Choice:** The profile fails closed on text or roles that claim Rust semantic correctness, compiler correctness, whole-program safety, verifier soundness, or release eligibility from function-address evidence alone.

**Rationale:** The function-address profile must be safe to bundle into release evidence without upgrading its claim.

## Risks / Trade-offs

- A profile that is too generic can blur digest roles. Typed identity-domain validation should be explicit even when all identities are BLAKE3-shaped.
- A profile that is too Rust-specific can undermine Kamacite portability. Keep Octet metadata as producer facts and keep receipt shape source-neutral.
- Graph reports may make evidence look stronger than it is. Verification-role summaries and non-claims must be visible.
