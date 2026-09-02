## Why

Valence's current formal-proof chain validates an Octet receipt, proof card, provenance link, role counts, and Cairn readiness summary. That shape is adequate for a direct verifier receipt but does not represent a translation-based proof path where stale or mismatched intermediate artifacts can change what the proof assistant checked.

An Aeneas/Lean proof passes through exact Rust source, Charon-produced LLBC, Aeneas-generated Lean definitions, hand-maintained models and theorem source, and a Lean checker receipt. Valence needs a provider-aware identity/linkage profile so downstream evidence can distinguish a fully linked theorem chain from a valid-looking final receipt attached to different intermediate bytes.

## What Changes

- Extend the formal-proof chain with a typed `aeneas-lean` translation profile over frozen Octet adapter inputs.
- Model source, effective Charon extraction contract, LLBC, Aeneas `translation.json`, generated Lean, external model, maintained theorem, theorem-adequacy/trust-closure decision, and Lean-check artifacts as distinct typed nodes connected by exact input/output edges.
- Preserve `translation.json` declaration metadata including source spans, generated files, opacity, failure/divergence, recursion, and loop-parent links while treating `def_id` as a within-run join key only.
- Compute a stack-owned BLAKE3 chain identity over canonical stage and edge material while preserving digest roles and durable source/model/theorem identities independently of tool-local declaration numbering.
- Enforce role policy: extraction, translation, manifest, model, and adequacy artifacts remain `recorded_only` or `boundary`; only a passing, artifact-verified Octet theorem receipt with an approved statement hash, requirement ID, claim class, and complete linked trust closure may contribute a `property` node.
- Add a pure validation core and a read-only CLI adapter surface that never executes Charon, Aeneas, Lean, or live sibling repositories and never independently decides theorem truth or product adequacy.
- Add positive frozen fixtures plus negative fixtures for weakened extraction options, errorful LLBC, stale manifests, unstable `def_id` reuse, stale stages, missing edges, digest-role substitution, theorem/adequacy/receipt mismatch, tier promotion, unsupported profiles, and weakened non-claims.

## Impact

- **Core**: formal-proof chain DTOs, canonical chain identity, role/link validation, deterministic diagnostics, and fixture adapters.
- **CLI**: bounded file loading and report rendering around the pure validator.
- **Fixtures**: an Octet-owned frozen Aeneas adapter input and local tamper variants; no live cross-repo dependency in ordinary tests.
- **Downstream**: Cairn may consume the resulting property/recorded-only/boundary summary through its existing generic evidence-chain readiness path; Mantle may carry the sidecar as opaque external evidence.
- **Claims**: validation proves canonical identity, exact linkage, role policy, and non-claim conformance only. It does not prove Rust/Aeneas/Lean semantics, translation correctness, theorem truth, verifier soundness, requirement satisfaction, release eligibility, or whole-program correctness.
