# Four-version semantic merge evidence-chain profile

Valence registers `evidence_kind = "semantic-merge"` (payload profile `valence.semantic-merge-evidence.v1`) as a narrowly scoped profile inside `valence.evidence-chain.v1`. It binds exact base `O`, variants `A` and `B`, candidate `M`, provider-native provenance, the producer's selected claim/observables/assumptions, explicit six-state outcomes, typed artifact roles, and consumer-supplied applicability expectations.

Valence performs no merge synthesis, source mutation, or verifier execution. This profile transports producer-specific claims (such as Octet's bounded Rust rail or Noble's language-owned verifier); it does not adopt Verified Three-Way Program Merge (OOPSLA 2018) semantics as a universal language contract.

## Reuse-versus-extension review

| Concern | Reused from `valence.evidence-chain.v1` / `evidence_ir.rs` | Narrowly added by `semantic_merge_evidence` |
| --- | --- | --- |
| Canonical envelope bytes & BLAKE3 identity | `EvidenceChainEnvelope.canonical_bytes`, `envelope_identity` (`kamacite-envelope`) | None |
| Profile root & policy root binding | `profile_root` (`profile-root`), `policy_identity` (`policy-profile`) | Consumer-supplied `expected_policy_identity` enforcement so producer self-description cannot select policy |
| Generic parent links & graph root | `parent_links`, self-link/duplicate checks, deterministic `graph_rows` & `graph_root` | Role-tagged subject/provenance/claim/artifact edges emitted by `validate_semantic_merge_evidence` |
| Compatibility projection & drift check | `compatibility_projection`, `projection_identity` (`compatibility-projection`) | `SemanticMergeProjection` (`valence.semantic-merge-projection.v1`) preserving all four role bindings, outcome, role, and scope |
| Verification role vocabulary | `VerificationRole` (`Property`, `RecordedOnly`, `Boundary`) | Envelope-to-payload role equality check plus per-artifact role separation preventing `RecordedOnly`-to-`Property` promotion |
| Four-version subjects & provider provenance | None (existing profiles are single-subject) | Exact 4-role quartet (`base`, `variant_a`, `variant_b`, `candidate`) with separate source-tree, semantic-subject, correspondence, and Pijul-native domains |
| Outcome lattice & consumer applicability | None (existing profiles lack 6-state merge outcomes and consumer expectations) | 6-state `SemanticMergeOutcome` and `check_semantic_merge_applicability` against consumer-supplied `SemanticMergeExpectation` |

## Canonicalization and digest domains

All digests are 64-character lowercase BLAKE3 hex (accepting optional `b3:` prefix) paired with an explicit non-interchangeable domain string:

- **Envelope domains (reused):** `kamacite-envelope`, `profile-root`, `policy-profile`, `compatibility-projection`.
- **Subject domains (distinct per concern):**
  - `source-tree`: measured source/canonical tree bytes.
  - `semantic-subject`: language-specific semantic subject / IR identity.
  - `correspondence-evidence`: source-to-IR correspondence receipt/report.
- **Provider-native provenance domains (strictly separate from content/program domains):**
  - `pijul-channel-state`: exact Pijul Merkle channel state (required whenever `provider == "pijul"`).
  - `pijul-change-set`: ordered admitted Pijul change-set observation.
  - `pijul-change-hash`: individual Pijul change hash (cannot stand for a full channel state or source tree).
  - `vcs-native-revision`: non-Pijul provider-native revision token.
- **Claim & artifact domains:** `property-spec`, `observable-domain`, `semantic-model`, `dependency-closure`, `summary-identity`, `assumption`, `checker-toolchain`, `extraction-transcript`, `test-result`, `solver-output`, `checked-proof`, `counterexample-witness`.

Each subject role computes a role-tagged binding digest via `BLAKE3(canonical_json({domain: "semantic-merge-role-binding.v1", role, source_tree, semantic_subject, correspondence, native_provenance}))`. Identical digests in different roles (`A == O` or `A == B`) remain valid and distinct; swapping roles changes the binding and fails applicability.

## Typed-link, role, and outcome rules

- **Four explicit roles:** `base`, `variant_a`, `variant_b`, `candidate`. Missing or duplicate roles fail closed.
- **Six explicit outcomes (never collapsed to a Boolean):**
  - `verified`: requires non-empty observables, non-zero `checked_inputs`, every observable covered with `admitted_inputs > 0` (no vacuous verification), and empty `missing_observables`.
  - `validated_counterexample`: requires non-empty observable name and a `counterexample-witness` digest.
  - `unknown`, `unsupported`: require non-empty diagnostic reason.
  - `timeout_exhausted`: requires non-empty `limit_kind` and non-zero `limit_value`.
  - `execution_error`: requires non-empty `backend` and `diagnostic`.
- **Well-formed nonverified envelopes:** any nonverified outcome is a valid structural envelope when its bindings, domains, assumptions, and non-claims are valid AND `verification_role != VerificationRole::Property`.
- **Artifact role separation:** `extraction_transcript`, `test_result`, `solver_output`, and `explicit_assumption` artifacts can never claim `VerificationRole::Property`. Only `independently_checked_proof` (`checked-proof`) may back `VerificationRole::Property` on a `verified` outcome.
- **Envelope role lock:** `evidence_chain::validate_evidence_chain` verifies `envelope.verification_role == payload.verification_role` so an envelope cannot promote a `RecordedOnly` payload.

## Consumer applicability and compatibility behavior

Consumers (such as Kiln CI gates) supply an explicit `SemanticMergeExpectation` specifying `mode` (`required_verified_property` or `advisory`), `expected_policy_identity`, expected quartet bindings, property/model/dependency/toolchain/assumption/observable pins, and optional pinned `expected_proof_digest`.

- Stale, swapped, omitted, duplicated-role, changed-proof, or wrong-context bindings fail closed with deterministic diagnostics.
- Under `required_verified_property`, **only** a valid, applicable envelope with `outcome = verified`, `verification_role = property`, and an `independently_checked_proof` artifact satisfies the expectation. Every nonverified outcome and every `RecordedOnly` / `Boundary` envelope fails `required_verified_property_unsatisfied`.
- Under `advisory`, a matching nonverified result passes applicability while setting `satisfies_required_property = false`.
- `project_semantic_merge` produces `valence.semantic-merge-projection.v1`, which is hash-bound into `projection_identity` so serialization and downstream projections preserve identities, outcomes, and scope.

## Bounded integration evidence fixture

`fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json` is checked in and explicitly labeled `classification = "bounded_integration_evidence"`. It connects one language-owned Octet checker receipt (`octet-merge-rail-receipt/v1` / `octet-merge-rail-bounded-v1`), its canonical `valence.evidence-chain.v1` / `valence.semantic-merge-evidence.v1` envelope with Pijul channel-state provenance and projection binding, and one Kiln consumer expectation (`required_verified_property`).

## Claim boundary and non-claims

Every `valence.semantic-merge-evidence.v1` payload MUST retain:

1. `semantic-merge evidence binds four-version identity, linkage, role, outcome, and applicability only`
2. `does not prove evidence truth, checker soundness, or semantic correctness beyond the producer's checked relation`
3. `grants no authority to synthesize merges, mutate source, or authorize release`

Passing validation proves bounded four-version canonical identity, digest-domain separation, typed linkage, outcome/role separation, projection conformance, and consumer applicability only.
