# Design: Proof robustness evidence registration

## Context

Valence owns canonical evidence identity, linkage, roles, and non-claim boundaries. Octet owns proof execution, audit collection, resource comparison, and robustness classification.

## Decisions

### Decision: Register one producer-owned schema

**Choice:** Admit the exact Octet `proof-robustness` schema through the stack schema registry.

**Rationale:** Valence must not invent a second wire contract for the same producer artifact.

### Decision: Keep each layer identity separate

**Choice:** Preserve profile, proof parent, child run, observation, audit, proof-use, classification, snapshot, and receipt identities as separate linked layers.

**Rationale:** One aggregate hash would hide substitution between proof results, assumptions, and observations.

### Decision: Robustness stays recorded evidence

**Choice:** Seed runs, metrics, proof-use facts, classifications, and terminal snapshots use `RecordedOnly`.

**Rationale:** Stable search behavior is not theorem evidence.

### Decision: Trust facts stay boundaries

**Choice:** Assumptions, trusted boundaries, unknown coverage, unsupported provider facts, and required non-claims use `Boundary`.

**Rationale:** These facts constrain the claim instead of supporting it.

### Decision: Parent proof role remains independent

**Choice:** A matching parent proof artifact can retain `Property` only through its existing validation path.

**Rationale:** A robustness sidecar cannot create or strengthen a proof role.

### Decision: Accept only terminal snapshots

**Choice:** Valence accepts a current terminal snapshot linked to a complete receipt. It rejects live queued, running, stale, or cross-generation state as proof evidence.

**Rationale:** Live operator state belongs to Octet and presentation consumers.

## Functional Core and Imperative Shell

`valence-core` validates already-loaded schema identity, layer links, role mapping, freshness, bounds, and non-claims.

The CLI shell reads capability-relative artifacts, recomputes bytes and BLAKE3 identities, and passes typed inputs to the core.

## Risks and Controls

- Producer schema drift can break intake. Exact schema identity and registry freshness make drift explicit.
- Role confusion can promote observations. Closed mapping rejects `Property` on robustness layers.
- Large seed matrices can expand artifacts. The accepted producer profile and Valence intake both enforce bounds.

## Validation

Positive fixtures cover complete stable and unstable classifications with correct role mapping.

Negative fixtures cover stale parents, swapped child receipts, wrong roles, missing boundaries, live-state promotion, unknown schemas, excessive collections, and overclaims.
