# Bounded Valence effect-closure lifecycle validation

Validation date: 2026-07-11

The complete replay record is retained at `evidence/octet/effect-closure/validation.md`; initial, admitted, verification, repeat, and tamper artifacts are under the same directory.

## Outcome

- The initial no-summary run passed its direct parent and denied closure with exit 1, 78 unknown call edges, 90 total diagnostics, and receipt `b3:aea12687550918254723cea648ca2b33e3ec21bf2beef214e107e0b8a2f331ed`.
- The final policy contains 78 exact summaries over `core`, `alloc`, `std`, `blake3`, and `serde_json`; no name-based intrinsic exemption is used.
- The final explicit-target run admitted two production files and 16 functions with only `alloc`, zero diagnostics, and receipt `b3:82109fdac487d19a941ccc4a3b9e34746daed013f5d7a3bcf0befa63dbdd8300`.
- Stored verification was valid with no diagnostics; identical replay produced byte-identical receipt/status artifacts.
- Separate closure-summary and strict-profile-parent-summary tamper copies both exited 1 with deterministic integrity diagnostics.

## Checks

- `nix develop -c cargo fmt --check`: PASS.
- `nix develop -c cargo test --workspace`: PASS.
- `nix develop -c cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- Nickel typecheck/export/freshness comparison: PASS.
- `nix build path:$PWD#checks.x86_64-linux.effect-closure-policy --no-link -L`: PASS.
- `nix flake check path:$PWD -L`: PASS on `x86_64-linux`; `aarch64-linux` was reported omitted/incompatible.
- Compatible Cairn validation plus proposal/design/tasks gates: PASS.
- Compatible sync dry-run/execute: PASS, followed by explicit promotion of all 11 effect-closure IDs because the full-spec-shaped delta was not merged into the existing accepted capability.
- Compatible archive dry-run: PASS and unblocked.
- `CAIRN_ARCHIVE_DATE=2026-07-11 ... archive --execute`: PASS; moved the package to `cairn/archive/2026-07-11-admit-octet-effect-closure/`.
- Post-archive compatible validation: PASS with 20 accepted specs, 12 remaining active changes, and no issues.

## Existing blockers

- Canonical sibling Cairn validation cannot parse `cairn-policy/generated/cairn-policy.json` because `function_address_gate` is missing; the compatible Octet-pinned CLI is used for this lifecycle.
- Tracey remains invalid repository-wide: 505 requirements, 224 references, and three dangling IDs. No change-local Tracey success is claimed.

The receipt is conservative selected source-token and reviewed-summary evidence. It is not complete Rust/MIR closure, dependency implementation verification, runtime determinism, whole-program correctness, signer authenticity, authorization, or release eligibility.
