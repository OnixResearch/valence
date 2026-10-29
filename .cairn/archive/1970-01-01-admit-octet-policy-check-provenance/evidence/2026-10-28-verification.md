# Actual Octet policy-check provenance handoff

## Outcome and ownership

Valence now recognizes `policy_check` on Octet provenance records without relabeling producer bytes or extending proof-card admission. Valence owns this typed tier/sidecar boundary. Octet still owns static artifact checking, Mantle owns materialization, and Molten owns consumer/runtime/authority admission.

The current consumer is Molten's hosted-effects actor and installed-extension adoption. The durable contribution is interoperable, nonpromoting stack-sidecar construction from ordinary official Octet exports. Repeatability is covered by adjacent deterministic positive/negative regressions, complete workspace checks, and the actual capability-rooted CLI calls below.

## Observed failure before the fix

Official immutable-toolchain `cargo-octet artifact verify --emit-valence-provenance` accepted the genuine neutral effects artifact with `status=valid` and no diagnostics. The actual Molten-linked Valence helper then exited 1 with:

```text
Valence sidecar denied: StackProvenanceReport { valid: false, issue_codes: ["unknown_evidence_tier"] }
```

The unchanged export contains `policy_check` records. The prior parser recognized only `formal_proof` and `manual_review_required`. Neither the export nor its evidence tiers was rewritten to pass.

## Executed checks

All product commands ran from the isolated Jujutsu workspace based on published Valence `15e72b5a330b08139ac7c809ac31e7e87f825b22`.

The initial run below preceded the publication guard. That guard found an
unintended unlocked-test lockfile refresh and unrelated pilot formatting;
both were restored. The final restored-lock proof and formatter limitation are
recorded separately below; the initial clean full-format result is not a claim
about the final unchanged pilot.

| Check | Observed result |
|---|---|
| `cargo test -p valence-core stack_provenance_adapter` in the pinned shell | 23 passed, 0 failed; includes policy-record binary-thread admission, policy-proof-card rejection, unknown-tier rejection and frozen Preserves corpus freshness |
| `cargo build --locked -p valence-cli --bin valence` | exit 0 |
| Actual neutral CLI sidecar build | exit 0; `valid: true` |
| Actual callback CLI sidecar build | exit 0; `valid: true` |
| `cargo test --locked --workspace` | 369 passed, 0 failed, 0 ignored across reported test-result rows |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo fmt --check` | exit 0 |
| `nix flake check path:/home/brittonr/git/OnixResearch/.worktrees/valence-policy-check-20261028 --builders '' -L` | exit 0 |

Rust commands used `nix develop path:/home/brittonr/git/OnixResearch/.worktrees/valence-policy-check-20261028 --builders '' -c`, `CARGO_BUILD_JOBS=2`, debug info disabled, and the task-owned external Cargo target. Plain `nix develop` initially selected the enclosing operator Git repository and refused its untracked worktree flake. The explicit `path:` reference resolved that workspace-selection error; the operator repository was not staged or modified.

Exact argv, cwd, stdout, stderr and exit records are retained under `/home/brittonr/.cache/valence-policy-check-20261028/verification/`. These host paths are observations, not durable product dependencies. Rust-analyzer references timed out; the existing scoped tier matches were inspected and complete workspace compilation/Clippy checked their exhaustiveness. Adding the public enum variant requires downstream Rust consumers to recompile; no public JSON field or source-tier spelling changes.

## Actual immutable-toolchain exports and CLI calls

The official exporter is the toolchain's `cargo-octet` wrapper, which executes the immutable `/nix/store/prr45b7qnqb24ks9q0mkx956wfbdlk08-cargo-octet-0.1.0/bin/cargo-octet`. Both artifact verification reports were valid with no diagnostics.

For each export, the actual CLI ran:

```text
valence stack-provenance build
  --artifact-root /home/brittonr/.cache/aspen-hosted-effects-20261028
  --octet-export <neutral|callback>-valence-provenance.json
  --binary-hash <exact component digest below>
  --output-root /home/brittonr/.cache/valence-policy-check-20261028/verification
  --output <neutral|callback>-sidecar.json
```

The brackets above compactly describe two recorded invocations, not literal shell arguments. Both emitted `valid: true` and wrote the named sidecar.

| Subject | Exact component BLAKE3 | Official export BLAKE3 | Actual sidecar BLAKE3 |
|---|---|---|---|
| Neutral effects guest | `db83d1d091b1dc4dfe92a2dd0a9c9b300fcef8a94a6fdb39d6855f374b49b657` | `efada38ab1846ea4c598f281914476805c19dca72b2fde98909f51e6f2777398` | `ef8d9ba904e5992b168d8667c83af40bb5977b0e4de3527f8e815c21ff52ca8b` |
| Molten callback guest | `e7492c6e50b201cff0d836955aa7ce13ce97fed9e7d884e95fe91d0a4e904e2a` | `429fdb8eb1c154ee6437f7df51cef33393a522ad349047abe19ce4ae6f6ca161` | `438d34ee5e652718fc80e2d763860cb1f2f4bcf2fc185421458c76e44c7ae467` |

## Boundary

The actual handoff proves accepted tier vocabulary, metadata consistency and sidecar identity/linkage for these supplied artifacts. It does not prove producer truth, semantic equivalence, runtime execution, Wasmtime correctness, host isolation, capability authority, requirement satisfaction or release eligibility. Policy checks remain recorded static observations. Unknown tiers and policy-check proof cards remain denied. Molten must separately pin the published revision and exercise its actual consumers.

## Native lifecycle checks

Proposal, design and tasks gates passed before implementation. Native requirement
synchronization applied the one added requirement and four scenarios without
diagnostics (`receipt_hash=267bb0cf96193548decc120a6a00553f8bcc13de69b9ca19fad62b9265b69802`).
Strict Cairn validation then returned `valid=true`, no findings and no issues
(`receipt_hash=8bbe2810a9046265db74b552a576cb67d27e6d43787fccae76db75531e3fd637`).
These are lifecycle/structural observations, not runtime or release acceptance.


## Publication guard and final restored-lock proof

The publication guard restored the original `Cargo.lock` and unrelated
`pilots/schema-identity/src/lib.rs`; neither is part of this delivery. A fresh
locked verification then rebuilt the actual CLI, rebuilt both actual sidecars,
passed all 369 workspace cases, passed workspace Clippy with warnings denied,
passed the changed-source formatter check under `nightly-2026-03-21`, and passed
the complete Nix flake checks. Final argv/stdout/stderr/exit records are retained
under `/home/brittonr/.cache/valence-policy-check-20261028/verification-pinned/`.
Both final sidecar digests exactly match the table above.

The full `rustup run nightly-2026-03-21 cargo fmt --check` returned exit 1 for
the unchanged published pilot's existing three-line
`schema_identity_conformance::verify_fixture_json` wrapping at line 9. No other
formatter diff was reported. This is a retained pre-existing repository-wide
formatting limitation, not a clean full-formatter claim; unrelated pilot code
was deliberately not restyled. The changed provenance source's direct pinned
`rustfmt --check --edition 2024 --config skip_children=true` returned exit 0.

