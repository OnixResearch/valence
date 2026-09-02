## Design

The stack role registry core is a pure `valence-core` surface. It validates in-memory role rows, schema rows, compatibility aliases, required-role sets, and non-claim boundaries. It does not read policy files, inspect repositories, execute release gates, or decide runtime authority.

### Decisions

1. **Canonical role IDs live in Valence.** Basalt, UCAN, Trellis, Octet, Valence, Cairn, Mantle, and external/manual-review evidence roles are represented as typed values with stable public string encodings.

2. **Schemas and roles are paired.** A schema row names the roles it may appear under. Validation fails closed when a role accepts a stale or unrelated schema by string coincidence.

3. **Aliases are explicit compatibility rows.** Legacy public strings and downstream spellings may parse only through a compatibility row that records owner, status, replacement, and non-claim boundary.

4. **Verification role vocabulary stays separate.** Evidence role, stack component role, and verification role are related but not interchangeable types.

5. **No authority promotion.** Passing role-registry validation proves role/schema/vocabulary compatibility only. It does not prove evidence truth, verifier soundness, source-code correctness, release eligibility, lifecycle readiness, or runtime authority.

### Validation shape

Positive tests cover canonical roles, schema/role pairs, verification-role rows, and approved aliases. Negative tests cover unknown roles, duplicate required roles, stale aliases, schema/role mismatch, missing verification role, missing non-claims, and overclaims that turn role compatibility into authority or release approval.
