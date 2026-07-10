## Design

Valence registers `trellis.proof-evidence` as an evidence-chain profile. The pure core validates canonical Kamacite envelope identity, source envelope links, proof IR links, verifier receipt links, assumption roots, property/requirement IDs, spans, role metadata, and non-claims. Accepted formal-proof role requires the profile input to meet stricter verifier-result and policy criteria; otherwise valid imports remain recorded-only.

### Decisions

- **Profile-specific validation is explicit.** Generic evidence-chain checks run first; Trellis proof fields run through a dedicated validator.
- **Three role states are separate.** Reference-only, formal-proof candidate, and accepted formal-proof roles are distinct validation outcomes.
- **Assumptions are gate inputs.** Missing, stale, or policy-forbidden assumptions prevent accepted formal-proof status.
- **Graph rows preserve traceability.** Valence emits edges from proof receipts to requirements, source spans, verifier receipts, assumptions, and downstream release bindings.
- **No verifier soundness claim.** Valence validates receipt identity and role policy, not Verus soundness or Trellis theorem truth beyond recorded verifier status.

### Validation shape

Positive fixtures cover recorded-only reference evidence, accepted local Trellis formal-proof evidence, and graph row emission. Negative fixtures cover stale source, missing assumptions, verifier mismatch, unsupported result status, malformed property IDs, role promotion without policy, digest-domain substitution, and weakened non-claims.
