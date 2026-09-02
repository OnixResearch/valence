## Context

Valence supplies canonical evidence spans, roles, and non-claim boundaries that other stack repos consume. The stack's model-checking surface and its consumers (Octet parity rail, Kamacite interchange) will produce and consume model-checked artifacts. Valence must give those artifacts a canonical, bounded evidence shape before consumers invent incompatible projections.

The existing conventions apply unchanged: one identity per layer, explicit non-claims, and fail-closed validation in pure code with Nickel owning policy shape.

## Decisions

### Decision: Model-checked evidence gets explicit roles

**Choice:** Add a bounded role vocabulary for spec, invariant, checker, bounds, verdict, and witness-trace artifacts, and record each as its own evidence span with independent identity.

**Rationale:** Valence's honesty boundary is layer separation; a model-checked claim has many layers and each must stay separate.

### Decision: The sidecar is BLAKE3-bound and validatable

**Choice:** Define a versioned sidecar schema that binds the model hash, invariant hashes, checker identity and version, declared bounds, verdict class, witness-trace hash where present, and non-claims, with validation that fails closed on any mismatch.

**Rationale:** Consumers need one canonical, checkable shape with content identity.

### Decision: Verdict classes are closed and exact

**Choice:** Allow only `verified-within-bounds`, `violation-with-witness`, and `unsupported-checker` verdict classes, and require witness-trace identity for the violation class.

**Rationale:** An open-ended verdict vocabulary erases the difference between "checked and clean within bounds" and "could not check".

### Decision: The non-claim stays attached

**Choice:** Every model-checked evidence sidecar MUST carry the non-claim that its verdict does not prove implementation correctness or general temporal satisfiability.

**Rationale:** The claim boundary is the reason the stack can trust the artifact without over-reading it.

### Decision: Policy shape stays in Nickel, validation in Rust

**Choice:** Author the sidecar and role schema as Nickel contracts; implement validation in the pure Rust core.

**Rationale:** This matches the shared conventions for reviewable defaults and checked enforcement.

## Functional core / imperative shell split

- **Pure core**: role and sidecar validation, identity linkage checks, verdict-class admission, non-claim enforcement, and projection DTO construction.
- **Imperative shell**: read artifact bytes, compute BLAKE3 identities, and persist or export validated evidence records.

## Risks / Trade-offs

- Engine and trace formats change between versions. The checker identity and version fields are mandatory admission inputs, not optional metadata.
- Witness traces can be large. Bind only the trace hash in the sidecar and keep the trace artifact out of the core record.
- Role vocabulary growth invites overclaim. Keep the vocabulary closed and extend it through explicit compatibility changes.

## Non-Goals

- No new Evidence IR format; existing Valence spans and sidecar conventions stay authoritative.
- No claim of engine correctness, model validity, or implementation correctness.
- No storage or transport authority.
- No migration of existing proof evidence roles.
