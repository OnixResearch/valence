## Context

The canary exercised landed commit `6fb2473503ec8b72f72f2857cffee08e65b6bec6` with artifact-auth commit `799459346d5416fbd7b9f55840a7371441b55afa`. It generated an ephemeral non-production Ed25519 memory signer, attached the observed event through the signed Git COB path, reloaded cryptographically checked physical history, captured and attached RecordedOnly Evidence IR, materialized it, reopened Git and authorization state in a fresh process, then removed the actor binding and observed fail-closed denial.

## Decisions

### Decision: Archive a product-owned, self-contained public subset

**Choice:** Store the exact Valence harness and public run artifacts inside this Cairn package with a typed manifest and BLAKE3 inventory.

**Rationale:** A product-owned archive preserves the Evidence IR, COB identities, and authorization transition needed to review Valence's claim boundary without duplicating Mantle or Molten payloads. Exact revision and cross-consumer review links remain in the manifest; those links are not a joint signature or cross-consumer attestation.

### Decision: Keep private signer and mutable repository state outside the archive

**Choice:** Exclude the memory signer's private key, mutable Git repository, and capability-root state. Preserve only public signatures, refs, Evidence IR, profiles, authorization observations, summaries, and expected-denial logs.

**Rationale:** The later admission review needs public observations of signed COB linkage and authorization behavior, not credential custody. A secret scan and bounded hash inventory guard the archive boundary.

### Decision: Preserve RecordedOnly and protocol non-authority

**Choice:** Keep `legacy_authoritative = true`, `standalone_authority_admitted = false`, `rollback_available = true`, and `VerificationRole::RecordedOnly` as explicit boundaries.

**Rationale:** A local signed COB and authorization-removal test do not establish live Radicle membership, global revocation freshness, production signer custody, protocol authority, lifecycle handoff, or release interpretation.

## Risks / Trade-offs

- The archive proves only the captured local run and exact revisions; it does not recreate deleted private state.
- BLAKE3 inventory integrity does not prove semantic correctness, production rollout, or authority.
- An ephemeral memory signer exercises signature and COB mechanisms but does not represent production identity custody.
