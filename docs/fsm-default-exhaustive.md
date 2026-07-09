# FSM default-exhaustive evidence profile

Valence validates the `fsm-default-exhaustive` profile as identity, linkage, role, digest-shape, coverage-record-shape, and non-claim evidence only.

## Ownership boundary

- Octet owns Rust source-shape analysis, lint/provenance receipts, and any claim that Rust code avoided wildcard/default FSM transitions.
- Kamacite owns portable FSM spec and receipt vocabulary: closed state/event universes, transition rows, denial rows, coverage receipts, and source-neutral non-claims.
- Valence owns the evidence record boundary: producer roles, identity domains, BLAKE3 digest syntax, source span linkage, artifact identities, optional proof-link separation, and overclaim rejection.

## Non-claims

A valid Valence FSM profile record does not prove Rust semantics, FSM semantics, behavioral correctness, protocol completeness, liveness, verifier soundness, exhaustive Rust coverage, release eligibility, lifecycle readiness, or whole-program safety.

Formal proof links remain separate evidence records. Linking a Trellis, Verus, or Kani proof receipt does not promote the FSM default-exhaustive receipt tier.
