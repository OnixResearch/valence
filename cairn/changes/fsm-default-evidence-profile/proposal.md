## Why

Valence already accepts bounded Octet state-machine exhaustiveness receipts as evidence identity/linkage inputs. The proposed Octet and Kamacite work makes exhaustive-by-default FSMs a cross-repo profile: Octet owns Rust source-shape enforcement, Kamacite owns portable FSM spec/receipt vocabulary, and Valence must make the resulting evidence auditable without broadening the claim.

The current receipt boundary should be extended with an explicit default-exhaustive profile so consumers can distinguish ordinary state-machine source-shape evidence from evidence that also binds closed state/event universes, explicit invalid-transition denials, lint profile identity, optional Kamacite spec identity, and non-claim preservation.

## What Changes

- Define a Valence FSM default-exhaustive evidence profile over parsed in-memory receipt values.
- Bind Octet lint/provenance receipt identity, optional Kamacite FSM spec/receipt identity, closed state/event universe identities, transition coverage identity, denial coverage identity, source spans, roles, and non-claims.
- Preserve optional formal proof links as separate evidence records without promoting the default-exhaustive receipt tier.
- Add positive and negative fixtures for linked Octet/Kamacite evidence, stale identities, missing non-claims, missing denial coverage, and overclaims.
- Document the boundary: Valence verifies identity, linkage, role, digest, coverage-record shape, and non-claim policy only.

## Impact

Stack consumers get a stable place to ask “which FSM exhaustiveness evidence did this artifact carry?” without making Valence a Rust analyzer, FSM semantics engine, or release-readiness gate. Octet and Kamacite can evolve their producers independently while Valence preserves evidence identity and non-claim boundaries.
