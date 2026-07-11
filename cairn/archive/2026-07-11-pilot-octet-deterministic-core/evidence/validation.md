# Valence Octet deterministic-core pilot validation

Validation date: 2026-07-11

## Baseline

Before product source changes:

| Command | Outcome |
| --- | --- |
| `nix develop -c cargo test -p valence-core` | PASS. |
| `nix develop -c cargo clippy -p valence-core --all-targets -- -D warnings` | PASS. |
| `nix run path:/home/brittonr/git/OnixResearch/octet#cairn -- validate --root .` | PASS; the compatible pinned CLI validated the 12 then-active changes and 19 accepted specs. |
| `nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .` | Existing compatibility blocker: the canonical CLI rejected `cairn-policy/generated/cairn-policy.json` because it lacks the newer `function_address_gate` field. The unrelated active `restore-cairn-policy-compatibility` change was preserved. |

## Dogfood findings

1. A direct standalone packaged Octet run failed while compiling `serde_derive`, recording Cargo exit 101 and no function-address artifact. The same package reached lint execution inside `nix develop`, isolating a missing linker in the Octet runtime closure.
2. Checking the full `valence-core --lib` target under `deterministic-core` produced 134 deny findings in unrelated modules, so it was not a narrow identity-core admission.
3. A non-publishable focused package pointing at the exact shared-identity source reduced that result to 63 errors. Fifty-eight were duplicate/local `concrete_construction_in_core` false positives, two were mutable issue accumulator APIs, two were recursive traversal advisories, and one was an infallible-assumption `expect`.
4. Observe-mode function scanning found a same-file `as_str` locator collision and macro-context caveats from inline test bodies.

Octet commits `865690c` and `51d9b43` package the linker, classify one direct external constructor instead of local/nested parents, verify a proc-macro consumer, sync the accepted contract, and archive producer evidence. Valence then:

- added `valence-shared-identity-pilot`, which compiles the production source by path;
- split the canonical profile method into its own production source file without changing the public API;
- moved test bodies out of the admitted production files while retaining ordinary unit coverage;
- changed shared digest parsing/non-claim validation to return outcomes instead of mutating caller vectors;
- changed canonical JSON hashing to return a typed issue instead of panicking;
- retained no broad lint suppression.

## Final Valence checks

| Command | Outcome |
| --- | --- |
| `nix develop -c cargo fmt --check` | PASS. |
| `nix develop -c cargo test --workspace` | PASS, including positive and negative shared-identity tests and CLI integration tests. |
| `nix develop -c cargo clippy --workspace --all-targets -- -D warnings` | PASS. |
| `nix flake check path:$PWD -L` | PASS; all checks passed on `x86_64-linux` (the flake reports `aarch64-linux` as omitted/incompatible). |
| Core-purity search from `AGENTS.md` | PASS for production code. The only I/O match is `std::fs::read_to_string` in a `#[cfg(test)]` negative-fixture helper; the `async` text match is a documented non-claim constant. |
| `nix run path:/home/brittonr/git/OnixResearch/octet#cairn -- validate --root .` | PASS. |
| Compatible Cairn proposal/design/tasks gates | PASS with no issues. |

## Baseline bootstrap

Exact command:

```bash
nix run path:/home/brittonr/git/OnixResearch/octet#cargo-octet -- check \
  --function-addresses observe \
  --function-address-path crates/valence-core/src/shared_identity.rs \
  --function-address-path crates/valence-core/src/shared_identity/canonical_hash_profile.rs \
  --artifact-dir evidence/octet/deterministic-core/baseline
```

Outcome: warning-only bootstrap produced 16 supported function records across two source envelopes, no unsupported or duplicate locator diagnostics, and the reviewed baseline `evidence/octet/deterministic-core/baseline/function-addresses.json`.

## Profile admission and artifact verification

Exact commands:

```bash
nix run path:/home/brittonr/git/OnixResearch/octet#cargo-octet -- check \
  --profile deterministic-core \
  --function-address-baseline evidence/octet/deterministic-core/baseline/function-addresses.json \
  --artifact-dir evidence/octet/deterministic-core/admission
nix run path:/home/brittonr/git/OnixResearch/octet#cargo-octet -- artifact verify \
  --artifact-dir evidence/octet/deterministic-core/admission \
  --output-format json
```

Observed result:

- status `warning-only`, process exit zero;
- eight non-required advisory style findings, zero errors;
- profile `deterministic-core`;
- config identity `b3:c26d00917e4feb3e9a7f8307222758fc925b98cda6bc7e088781815bcad4c646`;
- profile identity `b3:5ef5d578922af5664cdfaa792b0a4db164dbdbdf573d2b40bb647f5088f87657`;
- function-address mode `deny`, 16 records, zero diagnostics;
- function-address record root `b3:68ef940eef5a36448ec6292468e50807e7dd7ada1b6769d4ca21d8dbbd10a869`;
- artifact verification `status: valid`, checked scope `profile:deterministic-core`, empty diagnostics.

## Repeat replay

The identical admission command ran a second time. `cmp` confirmed byte-identical first/second `status.json` and `function-addresses.json`, proving the selected profile/config identities and 16-record set repeated under unchanged local inputs.

## Stale-source negative case

A temporary minimal workspace retained the checked-in baseline but changed `blake3_hex` from one expression to an explicit intermediate digest binding. It ran:

```bash
nix run path:/home/brittonr/git/OnixResearch/octet#cargo-octet -- check \
  --profile deterministic-core \
  --function-address-baseline evidence/function-addresses.json \
  --artifact-dir artifacts
```

The command exited with the expected lint exit code one. `status.json` reported `lint-failure`, and the summary contained deterministic `function_address_stale` findings. No temporary mutation was committed.

## Boundary

This pilot establishes direct selected source/HIR policy, source-token function identity, local replay for named artifacts, and artifact-integrity validation only. It does not admit all of `valence-core` or any dependency closure and does not prove transitive effect absence, panic freedom in dependencies, semantic equivalence, behavioral correctness, compiler correctness, verifier soundness, whole-program correctness, cross-toolchain reproducibility, cross-machine reproducibility, or release eligibility.

## Tracey baseline

`nix run path:/home/brittonr/git/OnixResearch/octet#cairn -- tracey coverage --root . --json` remains blocked by the repository-wide Tracey baseline: 505 requirements, 224 references, three dangling references, and `valid: false`. Valence has no `.config/tracey/config.styx`; the report lists all 13 newly accepted `valence.strict_rust_admission.*` IDs among the broader missing set. Establishing Tracey configuration and closing repository-wide coverage remain separate work and are not hidden by this pilot.

## Lifecycle completion

- Proposal, design, and tasks gates passed with no issues using the compatible pinned Cairn CLI from Octet.
- Sync dry-run was unblocked and execute completed. That CLI created the accepted capability wrapper but did not promote the full-spec-shaped delta requirements, so the requirement sections were manually merged into `cairn/specs/strict-rust-admission/spec.md`; a targeted search confirmed the accepted IDs and Cairn validation passed.
- Archive dry-run was unblocked.
- `CAIRN_ARCHIVE_DATE=2026-07-11 ... archive --execute` moved the package to `cairn/archive/2026-07-11-pilot-octet-deterministic-core/`.
- Post-archive compatible Cairn validation passed with 20 accepted specs, 12 remaining active changes, and no issues.
- The canonical Cairn checkout compatibility blocker described in the baseline section remains unchanged and belongs to the preserved `restore-cairn-policy-compatibility` change.
