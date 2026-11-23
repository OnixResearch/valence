# Design: Four-version semantic merge evidence-chain profile

## Context

`valence-core::evidence_chain` (`valence.evidence-chain.v1`) already validates canonical envelope bytes, profile/policy digests, typed parent links, compatibility projections, generic non-claims, and deterministic graph roots. Octet #1 and Kiln #1 established bounded producer receipt and CI orchestration expectations for four-version semantic merge checks.

## Decisions

### Reuse generic evidence-chain machinery

Register `SEMANTIC_MERGE_CHAIN_KIND = "semantic-merge"` inside `evidence_chain::validate_registered_profile` rather than creating a separate provenance envelope. Enforce `envelope.verification_role == payload.verification_role` at dispatch so wrappers cannot upgrade `RecordedOnly` payloads to `Property`.

### Role-tagged four-version bindings and separated digest domains

Represent `base`, `variant_a`, `variant_b`, and `candidate` with distinct domains for `source-tree`, `semantic-subject`, `correspondence-evidence`, and provider-native provenance (`pijul-channel-state`, `pijul-change-set`, `pijul-change-hash`, `vcs-native-revision`). Hash each subject with its role tag so equal digests in distinct roles (`A == O`, `A == B`) remain valid and distinct while role swaps fail applicability.

### Six-state outcome lattice and artifact role separation

Represent `verified`, `validated_counterexample`, `unknown`, `unsupported`, `timeout_exhausted`, and `execution_error` explicitly. Keep `extraction_transcript`, `test_result`, `solver_output`, `independently_checked_proof`, and `explicit_assumption` in distinct typed roles. Nonverified outcomes produce valid structural envelopes with non-property roles and never satisfy `ExpectationMode::RequiredVerifiedProperty`.

### Consumer-supplied applicability and bounded integration fixture

Require consumers to pass `SemanticMergeExpectation` (expected subjects, property, profile, model, dependencies, assumptions, toolchain, policy, and optional proof digest) into `check_semantic_merge_applicability`. Check in a frozen Octet-to-Kiln fixture at `fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json` explicitly labeled `bounded_integration_evidence`.

## Rejected Alternatives

- Creating a separate non-chain provenance system: duplicates canonical envelope, projection, and graph-root logic already owned by `evidence_chain`.
- Collapsing merge outcomes into a Boolean pass/fail flag: hides counterexamples, unknown, unsupported, timeout, and execution errors.

## Validation

Run `cargo fmt --check`, `cargo build --workspace`, `cargo test --workspace`, strict Clippy, core-purity smoke, license-boundary check, and Cairn validation/gates.

## Risks and Claim Boundaries

Passing validation proves bounded four-version canonical identity, digest-domain separation, typed linkage, role/outcome separation, projection conformance, and consumer applicability only. It does not prove evidence truth, checker soundness, semantic correctness beyond the producer's checked relation, source mutation authority, or release eligibility.

## Rollback

Remove the `semantic-merge` dispatch arm, `semantic_merge_evidence` module, docs, and fixtures in one commit without affecting existing profiles.
