## Design

`valence-core` receives already-loaded Preserves receipt bytes or typed Preserves values from the CLI shell. The core validates canonical receipt identity, schema/version, digest domain roles, span boundaries, dependency identity kind, and required non-claims, then builds the existing function-address Evidence IR report and graph rows.

### Decisions

1. **Core stays pure.** File reads and Preserves parser invocation live in `valence-cli`; `valence-core` validates in-memory receipt values and canonical bytes.
2. **Preserves receipt hash is authoritative.** Valence uses the Kamacite canonical Preserves receipt hash as the external receipt identity.
3. **Projection ingest is compatibility-only.** JSON projections must declare the canonical Preserves receipt hash and fail closed if drift is detected.
4. **Valence owns role/domain checks.** Preserves schema conformance does not replace Valence digest-domain, role, graph, or non-claim validation.
5. **Recorded-only boundary remains strict.** Preserves canonical identity does not promote evidence to property or formal-proof roles.

### Validation shape

Positive fixtures cover a valid Kamacite Preserves receipt becoming Valence recorded-only evidence and graph rows. Negative fixtures cover wrong canonical hash, schema mismatch, JSON projection drift, digest-domain substitution, missing spans, dependency closure overclaim, verification-role promotion, and weakened non-claims.
