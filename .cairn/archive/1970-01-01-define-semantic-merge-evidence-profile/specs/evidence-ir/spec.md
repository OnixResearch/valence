# Evidence IR Delta: Semantic Merge Evidence Profile

## ADDED Requirements

### Requirement: Four-version semantic merge evidence registers inside the generic evidence chain
r[valence.evidence_ir.semantic_merge_evidence_profile.registry] Valence MUST register `semantic-merge` (`valence.semantic-merge-evidence.v1`) inside `valence.evidence-chain.v1`, reusing canonical envelope identity, profile/policy roots, parent links, projection drift checks, generic non-claims, and graph roots while enforcing envelope-to-payload verification-role equality.

#### Scenario: Valid semantic-merge envelope passes generic chain validation
- GIVEN a canonical `valence.evidence-chain.v1` envelope with `evidence_kind = "semantic-merge"` and a valid `valence.semantic-merge-evidence.v1` payload
- WHEN `validate_evidence_chain` runs
- THEN validation MUST pass and emit deterministic graph rows and a BLAKE3 graph root.

### Requirement: Four subject roles and provider-native provenance domains remain strictly separated
r[valence.evidence_ir.semantic_merge_evidence_profile.subjects_and_domains] Valence MUST require exact `base`, `variant_a`, `variant_b`, and `candidate` role bindings with non-interchangeable digest domains for `source-tree`, `semantic-subject`, `correspondence-evidence`, and provider-native provenance (including `pijul-channel-state` distinct from `pijul-change-hash`), while permitting identical digests in distinct roles via role-tagged binding identities.

#### Scenario: Identical version digests across distinct roles produce distinct role bindings
- GIVEN a valid four-version semantic-merge payload where `variant_a` and `base` share the same source-tree and semantic-subject digests
- WHEN validation computes role-tagged bindings
- THEN validation MUST succeed and emit distinct role-binding digests for `base` and `variant_a`.

### Requirement: Six outcomes and distinct artifact roles prevent recorded-to-property promotion
r[valence.evidence_ir.semantic_merge_evidence_profile.outcomes_and_roles] Valence MUST represent `verified`, `validated_counterexample`, `unknown`, `unsupported`, `timeout_exhausted`, and `execution_error` explicitly, allow well-formed nonverified envelopes with non-property roles, keep extraction transcripts, test results, solver outputs, independently checked proofs, and assumptions in separate roles, and deny promoting recorded-only evidence or nonverified outcomes to `VerificationRole::Property`.

#### Scenario: Nonverified outcomes round-trip validly without promoting to property role
- GIVEN well-formed envelopes for each nonverified outcome carrying `VerificationRole::RecordedOnly`
- WHEN envelopes round-trip through JSON and projection validation
- THEN structural validation MUST succeed while preserving exact outcome, role, and scope, and `VerificationRole::Property` MUST be rejected for nonverified outcomes or non-proof artifacts.

### Requirement: Consumer-supplied applicability rejects stale, swapped, or unsatisfied expectations
r[valence.evidence_ir.semantic_merge_evidence_profile.applicability] Valence MUST validate applicability against consumer-supplied expected subjects, property, profile, model, dependencies, assumptions, checker toolchain, policy identity, and optional proof digest, and MUST ensure nonverified outcomes or non-property roles never satisfy a required verified-property expectation.

#### Scenario: Negative applicability and structural controls fail closed
r[valence.evidence_ir.semantic_merge_evidence_profile.negative]
- GIVEN role substitution, missing observables or assumptions, wrong candidate or model, changed proof bytes, stale consumer expectations, unsupported versions, dropped non-claims, Pijul domain substitution, or recorded-to-property promotion
- WHEN validation or applicability checking runs
- THEN Valence MUST fail closed with deterministic diagnostics.

### Requirement: Frozen producer-consumer integration fixture is labeled bounded integration evidence
r[valence.evidence_ir.semantic_merge_evidence_profile.integration_fixture] Valence MUST include a frozen end-to-end fixture binding one language-owned semantic-merge checker receipt and one Kiln consumer expectation explicitly labeled as bounded integration evidence.

#### Scenario: Frozen Octet-to-Kiln bounded integration fixture validates and rejects stale expectations
- GIVEN `fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json` labeled `bounded_integration_evidence`
- WHEN Valence validates the envelope and checks applicability against the Kiln expectation
- THEN the matching expectation MUST pass and a perturbed stale candidate expectation MUST fail closed.
