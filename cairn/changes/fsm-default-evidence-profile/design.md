## Design

Valence extends the existing state-machine evidence boundary with a named `fsm-default-exhaustive` profile. The profile is a validation and linkage shape for already-produced evidence. Valence does not inspect Rust HIR, re-run Octet, interpret Kamacite FSM semantics, or decide runtime behavior.

## Receipt profile

A default-exhaustive FSM evidence record carries:

- profile version and receipt kind;
- producer role and checker identity;
- Octet FSM lint/provenance receipt identity;
- optional Kamacite FSM spec identity and Kamacite receipt identity;
- transition-core source span and stable source identifier;
- closed state universe identity and event universe identity;
- state/event variant inventory identity;
- transition coverage summary identity;
- explicit denial coverage identity for invalid state/event pairs;
- status, diagnostics, evidence tier, artifact BLAKE3 identity, parent export identity, and required non-claims.

The coverage and denial identities bind producer-supplied canonical bytes. Valence validates shape, digest spelling, required links, role vocabulary, and non-claims. It does not reconstruct the matrix or prove the transition relation complete.

## Linkage rules

When Octet and Kamacite evidence are both present, Valence validates that the record preserves distinct identity domains: Octet owns source-shape/lint evidence; Kamacite owns portable spec/receipt vocabulary; Valence owns linkage and non-claim enforcement. Missing optional Kamacite evidence does not invalidate an Octet-only receipt unless the profile declares that Kamacite spec identity is required.

Formal proofs for invariants, reachability, or impossible states remain separate artifacts with their own verifier identity and tier. Linking such proofs to an FSM default-exhaustive receipt does not promote the source-shape receipt to formal proof evidence.

## Validation shape

Positive fixtures cover Octet-only receipts and Octet-plus-Kamacite linked receipts. Negative fixtures cover stale universe identities, malformed coverage digests, missing denial coverage, missing non-claims, identity-domain substitution, unknown roles, unsupported profile versions, and text that claims correctness, liveness, release eligibility, protocol completeness, or verifier soundness.

The implementation remains a pure core over parsed values. CLI/file loading, JSON rendering, artifact bundle discovery, and stdout/stderr stay in `valence-cli` or external shells.
