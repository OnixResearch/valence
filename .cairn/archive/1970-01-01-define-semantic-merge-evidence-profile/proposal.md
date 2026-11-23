# Change: Define typed evidence bindings for four-version semantic merge checks

## Why

Four-version semantic merge checkers (such as Octet's bounded Rust rail and Noble's language-owned verifier) and downstream CI consumers (such as Kiln) need a narrow, versioned Valence evidence-chain profile binding exact base `O`, variants `A` and `B`, candidate `M`, provider-native provenance, claim/observable/assumption selection, six-state outcomes, typed artifact roles, and consumer-supplied applicability checks without turning identity validation into behavioral proof.

## What Changes

- Register `evidence_kind = "semantic-merge"` (`valence.semantic-merge-evidence.v1`) in `valence-core::evidence_chain`, reusing existing canonical envelope identity, profile/policy root binding, parent-link validation, projection drift checking, generic non-claims, and graph-root emission.
- Add `crates/valence-core/src/semantic_merge_evidence.rs` implementing four explicit subject roles (`base`, `variant_a`, `variant_b`, `candidate`) with role-tagged BLAKE3 bindings (supporting identical digests across distinct roles), strictly separated provider-native provenance domains (including Pijul channel states vs change hashes), explicit six-state outcomes, distinct artifact roles preventing `RecordedOnly`-to-`Property` promotion, deterministic compatibility projections, and consumer-supplied `check_semantic_merge_applicability`.
- Document the reuse-versus-extension review, exact canonicalization, digest domains, typed-link/role/outcome rules, compatibility behavior, and claim boundary in `docs/semantic-merge-evidence-profile.md` and `docs/evidence-chain-profile.md`.
- Add positive, negative, round-trip/projection, and frozen end-to-end Octet-to-Kiln bounded integration evidence fixtures (`fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json`).

## Impact

`valence-core::evidence_chain` dispatches `semantic-merge` envelopes to the pure `semantic_merge_evidence` profile validator while keeping existing `function-address` and `trellis-proof` profiles unchanged.

## Out of Scope

No merge synthesis, no source mutation, no verifier execution, no adoption of OOPSLA 2018 SafeMerge semantics as a universal language contract, and no claim of evidence truth, checker soundness, or release eligibility.
