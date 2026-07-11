## Context

Octet strict profiles are an admission layer over ordinary Rust. `deterministic-core` raises selected source/HIR lints to deny, requires an explicit core source scope, and requires deny-mode function-address evidence. It does not yet infer transitive MIR effects or dependency closure.

Valence's `crates/valence-core/src/shared_identity.rs` is a narrow functional core over already-loaded values and bytes. Octet still invokes Cargo at crate granularity, so general profile lints inspect the production `valence-core` library while core-specific lint/path and function-address policy use the narrower source scope.

## Decisions

### 1. Pilot one identity module

**Choice:** Declare only `crates/valence-core/src/shared_identity.rs` in `core_source_scopes`.

**Rationale:** This module owns deterministic BLAKE3 normalization, canonical JSON hashing, digest-role separation, and non-claim checks without shell I/O. A single reviewed file keeps the first baseline understandable and makes source-address drift easy to audit.

### 2. Keep Cargo scope explicit and production-only

**Choice:** Add workspace Octet metadata selecting package `valence-core` with `cargo_check_args = ["--lib"]`.

**Rationale:** Tests intentionally contain fixture I/O and panic-based assertions. The admission target is production core code, not test harness behavior. General required lints still evaluate the compiled `valence-core` library; the receipt must not imply that only one file was compiled.

### 3. Check in the reviewed address baseline, not ambient operator state

**Choice:** Store the observe-mode `function-addresses.json` under `evidence/octet/deterministic-core/` and use it as the deny-mode baseline.

**Rationale:** The baseline becomes a reviewable source artifact. Source changes invalidate recorded function addresses and require an explicit baseline refresh rather than silent acceptance.

### 4. Preserve profile evidence and replay observations

**Choice:** Keep the final profile-bound bundle and a concise validation report under `evidence/octet/deterministic-core/`, then verify the bundle with Octet.

**Rationale:** A checked-in bundle makes the pilot's exact tool/profile/config identities reviewable. A second run compares profile/config identities and function-address records rather than claiming byte-identical logs or cross-machine reproducibility.

### 5. Treat stale source as the negative case

**Choice:** Replay the profile against a temporary workspace copy whose selected module has a semantic source change while retaining the reviewed baseline.

**Rationale:** The expected failure directly tests the admission property: changed function content cannot silently inherit prior function-address evidence. The temporary mutation is not committed.

## Risks / Trade-offs

- A passing profile covers configured direct source/HIR checks only; calls into dependencies and other modules are not transitive effect proof.
- General profile lints compile the whole production library even though function-address scope is one file.
- The artifact records the local pinned Rust toolchain and Octet build used for this pilot; it is not a cross-toolchain reproducibility claim.
- Checking in a baseline creates intentional review work whenever selected function content changes.
- Valence's canonical Cairn checkout currently rejects the repository's older generated policy; lifecycle validation uses Octet's compatible flake-provided Cairn CLI and records the canonical-CLI blocker without changing the unrelated compatibility change.
