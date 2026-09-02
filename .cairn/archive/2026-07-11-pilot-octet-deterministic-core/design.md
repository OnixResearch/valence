## Context

Octet strict profiles are an admission layer over ordinary Rust. `deterministic-core` raises selected source/HIR lints to deny, requires explicit core source scopes, and requires deny-mode function-address evidence. It does not yet infer transitive MIR effects or dependency closure.

A first run against the complete production `valence-core` library produced 134 deny findings across unrelated modules because Cargo and general lint floors operate at target granularity. The pilot therefore needs a target that compiles the exact production identity source without claiming whole-crate admission.

## Decisions

### 1. Compile exact production source through a focused harness

**Choice:** Add the non-publishable `valence-shared-identity-pilot` workspace package. Its library path points directly at `crates/valence-core/src/shared_identity.rs`, which loads the same canonical-profile child module used by `valence-core`.

**Rationale:** The harness gives Cargo a narrow target without copying or forking product logic. General profile lint floors evaluate only that target, while ordinary `valence-core` continues compiling the same source in its existing module position.

### 2. Address two production source files

**Choice:** Declare `crates/valence-core/src/shared_identity.rs` and `crates/valence-core/src/shared_identity/canonical_hash_profile.rs` in `core_source_scopes`.

**Rationale:** Splitting the canonical hash profile preserves the public `as_str` API while avoiding ambiguous v1 function locators for same-named methods in one file. Test bodies live in `shared_identity/tests.rs`, remain covered by ordinary tests, and are outside production `--lib` plus function-address admission.

### 3. Make deterministic core APIs return outcomes

**Choice:** Replace mutable issue accumulator inputs in shared digest parsing/non-claim validation with returned `Result`/`Vec` values, and return a typed issue from canonical JSON serialization instead of panicking.

**Rationale:** These changes remove genuine `mutating_input_in_pure` and `no_unwrap` findings while strengthening the functional-core boundary. Downstream Valence modules explicitly translate shared identity results into their own issue vocabularies.

### 4. Check in the reviewed baseline and admitted bundle

**Choice:** Store the observe bundle under `evidence/octet/deterministic-core/baseline/`, use its `function-addresses.json` as the deny baseline, and retain the admitted profile bundle plus verification report beside it.

**Rationale:** Baseline refresh becomes an explicit source review. The admitted bundle makes exact toolchain, profile/config BLAKE3 identities, selected source envelopes, function records, warning debt, and non-claims reviewable.

### 5. Require exact replay and stale-source rejection

**Choice:** Rerun the identical admitted command and byte-compare `status.json` plus `function-addresses.json`. Separately, run a temporary workspace with a changed `blake3_hex` body against the unchanged baseline.

**Rationale:** Exact replay checks the identities this pilot claims. The negative case proves changed selected source cannot silently inherit prior function-address admission.

### 6. Preserve downstream findings rather than hide them

**Choice:** Record that the pilot exposed and required Octet fixes for standalone linker closure and local/duplicate `concrete_construction_in_core` findings.

**Rationale:** Dogfooding should improve the producer. Valence does not carry broad lint suppressions to compensate for checker defects. Eight remaining findings are non-required advisory style debt and remain visible in the admitted summary.

## Risks / Trade-offs

- A passing profile covers configured direct source/HIR checks only; calls into dependencies and other Valence modules are not transitive effect proof.
- The focused package is an admission harness, not a separately published product crate or proof that all `valence-core` code passed.
- Function addresses are source-token identities and do not establish HIR/MIR equivalence or semantic call-graph closure.
- The artifact records the local pinned Rust toolchain and Octet build used for this pilot; it is not a cross-toolchain or cross-machine reproducibility claim.
- Checking in a baseline creates intentional review work whenever selected function content changes.
- Valence's canonical Cairn checkout currently rejects the repository's older generated policy; lifecycle validation uses Octet's compatible flake-provided Cairn CLI and records the canonical-CLI blocker without changing the unrelated compatibility change.
