## Phase 1: Translation-chain model

- [ ] [serial] Extend the formal-proof chain with a closed `aeneas-lean` profile and typed source, effective Charon contract, LLBC, `translation.json`, generated Lean, model/trust input, maintained theorem, adequacy decision, package-lock, checker, Octet receipt, proof-card, and provenance nodes. r[valence.provenance.aeneas_translation_chain.profile]
- [ ] [serial] Define typed required edges and deterministic graph validation for extraction policy, stage order, manifest declaration/file links, endpoint roles, missing links, duplicate links, dangling nodes, and cycles. r[valence.provenance.aeneas_translation_chain.linkage]
- [ ] [serial] Validate reviewed effective Charon options, `has_errors = false`, `skip_borrowck = false`, exact target metadata, and matching extraction-profile identity from the frozen Octet input. r[valence.provenance.aeneas_translation_chain.charon_contract]
- [ ] [serial] Preserve manifest names/spans/files/opacity/failure/divergence/recursion metadata while rejecting `def_id` as a cross-run or durable stable identity. r[valence.provenance.aeneas_translation_chain.translation_manifest]
- [ ] [serial] Compute canonical role-separated BLAKE3 chain identity over durable source/profile/model/theorem/evidence material while excluding display-only paths, labels, and run-local declaration numbering. r[valence.provenance.aeneas_translation_chain.identity]

## Phase 2: Role and claim policy

- [ ] [serial] Preserve extraction, translation, manifest, model, adequacy, and trust-closure artifacts as recorded-only or boundary evidence and admit only a matching artifact-verified Octet theorem receipt as property evidence. r[valence.provenance.aeneas_translation_chain.role_policy]
- [ ] [serial] Require theorem/property ID, approved statement BLAKE3, requirement ID, claim class, adequacy policy/decision, and trust-closure agreement across theorem, checker, Octet receipt, proof card, and provenance nodes without independently deciding theorem truth or product adequacy. r[valence.provenance.aeneas_translation_chain.theorem_link] r[valence.provenance.aeneas_translation_chain.adequacy_link]
- [ ] [parallel] Add negative role and claim fixtures for direct Lean-result promotion, translation/manifest/adequacy property roles, missing Octet verification, vacuous-statement approval drift, undeclared trust input, weakened non-claims, and overclaim text. r[valence.provenance.aeneas_translation_chain.role_policy] r[valence.provenance.aeneas_translation_chain.adequacy_link] r[valence.provenance.aeneas_translation_chain.non_claims]

## Phase 3: Frozen adapter fixtures and CLI shell

- [ ] [serial] Import a frozen Octet Aeneas adapter fixture only after recording its producer revision, schema, and BLAKE3 identity. r[valence.provenance.aeneas_translation_chain.fixture]
- [ ] [parallel] Add tamper variants for errorful extraction, `skip_borrowck`, target/profile drift, stale source, swapped LLBC, stale or mismatched `translation.json`, invalid cross-run `def_id`, stale generated Lean, missing model/trust input, theorem/adequacy mismatch, wrong digest domain, unsupported profile, duplicate edge, dangling edge, and cycle. r[valence.provenance.aeneas_translation_chain.fixture] r[valence.provenance.aeneas_translation_chain.linkage] r[valence.provenance.aeneas_translation_chain.charon_contract] r[valence.provenance.aeneas_translation_chain.translation_manifest]
- [ ] [serial] Add a read-only CLI adapter that confines file access and measurement in the shell while passing parsed values to the pure core. r[valence.provenance.aeneas_translation_chain.cli]

## Phase 4: Generic stack handoff and validation

- [ ] [serial] Emit the canonical chain identity, extraction/manifest/adequacy linkage summary, and existing property/recorded-only/boundary summary for generic Cairn and Mantle consumption without provider-specific downstream commands. r[valence.provenance.aeneas_translation_chain.handoff]
- [ ] [serial] Document execution ownership, role counting, fixture refresh, downstream handoff, and translation/verifier non-claims. r[valence.provenance.aeneas_translation_chain.non_claims]
- [ ] [serial] Run positive and negative chain fixtures, core/CLI tests, canonical identity goldens, workspace tests, Clippy, and Cairn validation/gates. r[valence.provenance.aeneas_translation_chain.final_validation]
