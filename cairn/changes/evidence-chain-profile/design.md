## Design

`valence-core` adds a pure evidence-chain profile that accepts already-loaded Kamacite envelope data and canonical bytes. The profile validates generic chain invariants, then delegates evidence-kind-specific payload checks to registered profile validators. Function-address uses the existing function-address profile as the first registered validator.

### Decisions

- **Generic chain first.** Valence validates envelope identity, typed parent links, profile roots, policy hashes, roles, projections, and non-claims before profile payload details.
- **Profile validators stay explicit.** Function-address, proof, lint, dependency, build, and attestation profiles must declare their own accepted payload domains before use.
- **Graph rows are reusable.** Valence emits chain graph nodes and edges for producer, canonical receipt, validation, release binding, and readiness links.
- **Role counting remains strict.** Generic evidence-chain validation does not promote recorded-only evidence to property or formal-proof roles.
- **CLI owns parsing.** Preserves and JSON file reads stay in `valence-cli`; `valence-core` validates in-memory values.

### Validation shape

Positive fixtures cover a function-address Kamacite envelope becoming Valence evidence-chain rows with graph roots. Negative fixtures cover wrong canonical envelope hash, unknown evidence kind, unsupported profile validator, cross-domain parent link, stale policy hash, projection drift, role promotion, and weakened non-claims.
