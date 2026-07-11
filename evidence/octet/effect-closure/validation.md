# Valence Octet effect-closure validation

Validation date: 2026-07-11

## Scope and tool

The retained runs used `/nix/store/8ny5smh17x2g5njainiszzyp4sw4r3rx-cargo-octet-0.1.0/bin/cargo-octet` built from reviewed exact Octet commit `15965f21e4e535a35c757561f789020b70414a80`. Both runs selected only:

- `crates/valence-core/src/shared_identity.rs`;
- `crates/valence-core/src/shared_identity/canonical_hash_profile.rs`.

Both direct checks used explicit Cargo target `x86_64-unknown-linux-gnu`, profile identity `b3:217467f897307ee25135762e38bde9d321e5c3cca5ac7b4322adea7b37f95539`, and configuration identity `b3:47531b6882b7e11f0091faf7321a2ebf42219f93567a21bec33fa1deb5d1e7c3`.

## Initial fail-closed result

The exact command is retained in `initial-denied/command.txt`. The direct deterministic-core parent passed before closure ran. With zero reviewed call summaries, closure exited 1 and retained:

- status `denied`;
- two source envelopes and 16 function nodes;
- zero reviewed providers;
- 90 deterministic diagnostics, including 78 `call_edge_unknown` findings plus propagated denied-`unknown` findings;
- receipt identity `b3:aea12687550918254723cea648ca2b33e3ec21bf2beef214e107e0b8a2f331ed`.

The 78 unknown edges include apparent `Ok`, `Err`, `Some`, and `Vec::new` calls. Octet intentionally grants no name-based intrinsic exemption because source tokens cannot prove those names were not shadowed.

## Final admission

`policy/octet-effect-closure-call-summaries.ncl` contains 78 sorted exact call-site summaries. Each binds the function-address-derived call-site ID, occurrence, reviewed callee label, target, effects, evidence identity, and one exact provider from `core`, `alloc`, `std`, `blake3 1.8.5`, or `serde_json 1.0.150`.

The exact command is retained in `admission/command.txt`. It exited 0 and retained:

- status `admitted`;
- two source envelopes and 16 function nodes;
- five used provider identities;
- only the allowed `alloc` closure effect;
- zero closure diagnostics;
- receipt identity `b3:82109fdac487d19a941ccc4a3b9e34746daed013f5d7a3bcf0befa63dbdd8300`.

`admission-verification.json` reports `status: valid`, the same receipt identity, and no diagnostics. Verification reran the ordinary artifact verifier over the complete strict-profile parent before checking closure receipt, policy, graph, status, summary, and provenance linkage.

## Repeat and negative evidence

Repeating the exact final command under unchanged inputs produced byte-identical `admission/effect-closure.json` and `admission/status.json` under `cmp`.

Two copied-bundle negatives exited 1:

- `tamper-verification.json`: appending to closure `summary.txt` produced stale-summary and provenance-hash diagnostics;
- `parent-tamper-verification.json`: appending to `check/summary.txt` produced a strict-profile parent artifact hash mismatch.

These BLAKE3 checks establish stored identity/linkage consistency only. They provide no signer authenticity, authorization, or resistance to coherent replacement of every artifact and trust root.

## Repository checks

| Command | Outcome |
| --- | --- |
| `nix develop -c cargo fmt --check` | PASS |
| `nix develop -c cargo test --workspace` | PASS |
| `nix develop -c cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Nickel typecheck/export plus `cmp` against `octet-effect-closure.json` | PASS |
| `nix build path:$PWD#checks.x86_64-linux.effect-closure-policy --no-link -L` | PASS |
| `nix flake check path:$PWD -L` | PASS on `x86_64-linux`; Nix reported `aarch64-linux` omitted as incompatible |

## Lifecycle checks and known blockers

- `nix run path:../octet#cairn -- validate --root .`: PASS with 21 accepted specs, 13 active changes, and no issues.
- Compatible proposal, design, and tasks gates for `admit-octet-effect-closure`: PASS with no issues.
- Compatible sync dry-run and execute: PASS. Sync did not merge the full-spec-shaped delta into the existing accepted capability, so all 11 `valence.strict_rust_admission.effect_closure*` requirement/scenario IDs were explicitly promoted into `cairn/specs/strict-rust-admission/spec.md` and revalidated.
- Compatible archive dry-run and `CAIRN_ARCHIVE_DATE=2026-07-11 ... archive --execute`: PASS; archived at `cairn/archive/2026-07-11-admit-octet-effect-closure/`.
- Post-archive compatible validation: PASS with 20 accepted specs, 12 active changes, and no issues.
- Canonical `nix run path:../cairn#cairn -- validate --root .`: existing blocker, `failed to parse policy cairn-policy/generated/cairn-policy.json: policy missing field function_address_gate`.
- Compatible `tracey coverage`: existing repository-wide failure with 505 requirements, 224 referenced IDs, three dangling IDs (`valence.evidence_ir.shared_identity.validation`, `valence.example`, `valence.unknown`), and `valid: false`. New closure IDs remain among the broader missing set; no Tracey success is claimed.

## Claim boundary

The result covers conservative source-token propagation under 78 reviewed call assertions for two production files and exact recorded inputs. It does not establish complete Rust name resolution, macro expansion, trait/dynamic dispatch, MIR closure, dependency implementation behavior, runtime reachability or ordering, panic freedom, semantic determinism, compiler correctness, verifier soundness, whole-program correctness, cross-target portability, authenticity, authorization, or release eligibility.
