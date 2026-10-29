# Stack provenance policy-check record admission

## ADDED Requirements

### Requirement: Policy-check provenance records retain their source tier
r[valence.stack_provenance_adapter.policy_check_records] Valence MUST admit the exact `policy_check` evidence tier on validated Octet provenance records, preserve that source string, and retain identity/linkage-only non-claims and recorded-only evidence semantics.

#### Scenario: Actual Octet policy-check export builds a sidecar
r[valence.stack_provenance_adapter.policy_check_records.admit]
- GIVEN a valid official Octet provenance export containing `policy_check` records and matching artifact identities
- WHEN Valence builds and validates a stack-provenance sidecar
- THEN admission MUST succeed without rewriting the export or promoting the policy-check evidence to formal proof.

#### Scenario: Unknown provenance tiers remain denied
r[valence.stack_provenance_adapter.policy_check_records.unknown]
- GIVEN a provenance record with an unrecognized evidence tier
- WHEN Valence validates the export
- THEN validation MUST fail with the existing unknown-tier diagnostic and MUST preserve the source string in recorded projections.

#### Scenario: Policy checks do not become proof-card tiers
r[valence.stack_provenance_adapter.policy_check_records.cards]
- GIVEN a proof card using `policy_check` instead of the existing formal-proof or manual-review tier
- WHEN Valence validates the export
- THEN the proof card MUST remain denied even though that tier is recognized for provenance records.

#### Scenario: Recognition does not increase verification authority
r[valence.stack_provenance_adapter.policy_check_records.nonclaims]
- GIVEN admitted policy-check records
- WHEN Valence projects their evidence or describes the resulting sidecar
- THEN the result MUST remain recorded-only identity/linkage evidence and MUST NOT establish runtime correctness, requirement satisfaction, authority or release eligibility.
