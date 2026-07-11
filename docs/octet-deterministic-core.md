# Octet deterministic-core pilot

Valence dogfoods Octet's opt-in `deterministic-core` profile against the production shared-identity implementation. The pilot records direct source/HIR policy and source-token function-address evidence; it is not a separate Rust language or a whole-crate verification claim.

<!-- r[verify valence.strict_rust_admission.boundaries.non_claims] -->

## Admitted scope

The non-publishable workspace package `valence-shared-identity-pilot` points its library target at the exact production source `crates/valence-core/src/shared_identity.rs`. That source loads `crates/valence-core/src/shared_identity/canonical_hash_profile.rs`, so the focused target compiles product code rather than a copied fixture.

Root workspace metadata selects only this package with `cargo_check_args = ["--lib"]`. Root `dylint.toml` declares both production files in `core_source_scopes`. Test bodies remain in `crates/valence-core/src/shared_identity/tests.rs`; ordinary Valence tests compile them, while the Octet production target and function-address scope do not include them.

This distinction matters:

- Cargo and required profile lints cover the focused harness target.
- Function-address deny mode covers the two configured production source files.
- The rest of `valence-core`, `valence-cli`, tests, dependencies, build scripts, and proc macros are not admitted as deterministic cores by this receipt.

## Replay

Set `OCTET_ROOT` to a reviewed Octet checkout containing strict-profile admission and downstream linker/lint-precision hardening:

```bash
export OCTET_ROOT="${OCTET_ROOT:-../octet}"
```

The packaged runner includes its linker closure, so these commands do not require entering Valence's development shell.

### Review or refresh the baseline

Do not refresh the baseline automatically. Run observe mode, review source-envelope and function-address diffs, and commit the replacement only when the source change is accepted:

```bash
nix run "path:$OCTET_ROOT#cargo-octet" -- check \
  --function-addresses observe \
  --function-address-path crates/valence-core/src/shared_identity.rs \
  --function-address-path crates/valence-core/src/shared_identity/canonical_hash_profile.rs \
  --artifact-dir evidence/octet/deterministic-core/baseline
```

The reviewed deny baseline is `evidence/octet/deterministic-core/baseline/function-addresses.json`.

### Run admission and verify evidence

```bash
nix run "path:$OCTET_ROOT#cargo-octet" -- check \
  --profile deterministic-core \
  --function-address-baseline evidence/octet/deterministic-core/baseline/function-addresses.json \
  --artifact-dir evidence/octet/deterministic-core/admission

nix run "path:$OCTET_ROOT#cargo-octet" -- artifact verify \
  --artifact-dir evidence/octet/deterministic-core/admission \
  --output-format json
```

The checked-in admitted run has no error findings and no function-address diagnostics. Eight non-required advisory style findings remain visible; warning-only status is not represented as a zero-warning claim.

## Findings from dogfooding

The pilot found and resolved three distinct classes of issue:

1. Checking all of `valence-core` made unrelated target-granularity debt part of the profile. The focused package now compiles only the exact production identity source.
2. Shared identity parsing mutated caller-owned issue vectors and canonical JSON hashing assumed serialization could not fail. The pure core now returns typed outcomes, and downstream modules translate them explicitly.
3. Octet's first downstream package lacked a linker and over-reported local/nested core construction. Octet now carries the linker closure and limits that lint to one direct external ADT constructor finding.

No broad lint suppression was added to make the profile pass.

## Evidence and limits

- `evidence/octet/deterministic-core/baseline/` is the reviewed observe bundle.
- `evidence/octet/deterministic-core/admission/` is the profile-bound deny bundle.
- `evidence/octet/deterministic-core/admission-verification.json` is the artifact-integrity report.
- `evidence/octet/deterministic-core/validation.md` records exact positive, repeat, and stale-source outcomes.

A passing pilot does not prove transitive effect absence, dependency closure, panic freedom in dependencies, semantic equivalence, compiler correctness, verifier soundness, whole-program correctness, cross-toolchain reproducibility, cross-machine reproducibility, or release eligibility.

## Conservative effect closure

<!-- r[verify valence.strict_rust_admission.effect_closure_policy.roles] -->
<!-- r[verify valence.strict_rust_admission.effect_closure_policy.calls] -->

The separate closure rail preserves the same two-file source universe and adds conservative source-token call/effect propagation. Human-authored policy lives in:

- `policy/octet-effect-closure-contracts.ncl` — copied typed workspace-policy contract;
- `policy/octet-effect-closure.ncl` — selected paths, baseline, and non-claims;
- `policy/octet-effect-closure-call-summaries.ncl` — 78 exact reviewed call-site summaries;
- `octet-effect-closure.json` — generated runtime input.

Both selected source files have the explicit `core` role. Each non-local call summary is keyed by a BLAKE3 call-site ID derived from its caller function address, normalized call shape, and within-function occurrence. The checked-in policy binds reviewed calls to the recorded `core`, `alloc`, or `std` toolchain identity, `blake3 1.8.5`, or `serde_json 1.0.150` provider identity for `x86_64-unknown-linux-gnu`. Toolchain API evidence binds the exact provider identity; Cargo-package summaries bind the reviewed `Cargo.lock` BLAKE3 identity. These are reviewed assertions, not verification of provider implementations.

Refresh generated JSON only after reviewing the Nickel diff:

```bash
nickel typecheck policy/octet-effect-closure.ncl
nickel export --format json policy/octet-effect-closure.ncl > octet-effect-closure.json
```

### Run and verify closure admission

<!-- r[verify valence.strict_rust_admission.effect_closure_final.admitted] -->

```bash
nix run "path:$OCTET_ROOT#cargo-octet" -- admit \
  --profile deterministic-core \
  --closure \
  --artifact-dir evidence/octet/effect-closure/admission \
  -- --target x86_64-unknown-linux-gnu

nix run "path:$OCTET_ROOT#cargo-octet" -- admit verify \
  --artifact-dir evidence/octet/effect-closure/admission \
  --output-format json
```

The final checked-in bundle admits 16 function-addressed nodes, records 78 reviewed call sites across five provider identities, permits only the inferred `alloc` effect, and has zero closure diagnostics. Its receipt identity is `b3:7787045860bfd88d8c82bf8c30f6253f989d673cc7c11003fe11b4e838705ff7`. Repeating the identical command produced byte-identical `effect-closure.json` and `status.json`.

### Fail-closed and tamper evidence

<!-- r[verify valence.strict_rust_admission.effect_closure_initial.denied] -->
<!-- r[verify valence.strict_rust_admission.effect_closure_final.negative] -->

`evidence/octet/effect-closure/initial-denied/` preserves the no-summary run. The direct deterministic-core check passed, then closure denied 78 unknown call edges and exposed each stable call-site identity for review. `evidence/octet/effect-closure/tamper-verification.json` records that changing a copied closure `summary.txt` produced nonzero verification with both stale-summary and provenance-hash diagnostics. `evidence/octet/effect-closure/parent-tamper-verification.json` separately records that changing the strict-profile parent's `check/summary.txt` is rejected by nested parent verification.

Closure v1 does not provide complete Rust name resolution, macro expansion, trait or dynamic-dispatch completeness, MIR call-graph completeness, dependency implementation verification, runtime reachability or ordering, panic freedom, semantic determinism, compiler correctness, verifier soundness, whole-program correctness, cross-target portability, signer authenticity, authorization, or release eligibility. BLAKE3 receipts do not defend against coherent replacement of every artifact and trust root. A target, toolchain, feature, package, lockfile, policy, function-address, or source change requires a fresh denied review queue and new exact admission evidence.
