## Why

The local trust stack (octet, mantle, trellis, cairn) needs a shared evidence
identity layer that is owned in one place, depended on via a single pinned
source per receipt, and capable of threading provenance from source to binary.
Today `valence-core` lives inside `octet/crates/valence-core/` as an
in-repository readiness seam. octet's flake already ships
`valenceCorePackageReadiness` gates verifying build, focused tests, API docs,
package listing, metadata, public modules, inventory sync, and boundary
non-claims — the crate is package-ready. Extracting it into a standalone
`valence` repo unblocks cross-repo work (Phase 2 provenance threading, Phase 3
property vocabulary) and gives the Evidence IR a home that all five repos can
depend on without the determinism tax that a projection tool like josh would
introduce.

This change also introduces the Evidence IR v1 spine: a canonical, serializable,
type-checked intermediate representation for evidence artifacts with span/
provenance threading and non-claim boundary enforcement. This is the bounded
version of the "valence language" decision — an IR, not a language. It gives
the stack canonical-evidence-like discipline (canonical machine form + human
projection + span survival through the chain) without a compiler, without
backends, and without mechanized type-soundness.

## What Changes

- Extract `valence-core` from `octet/crates/valence-core/` into a standalone
  `valence` repo. The three existing modules (`function_object`,
  `evidence_codebase_index`, `evidence_index`) are copied verbatim — the stable
  v1 contract strings remain byte-identical.
- Add `evidence_ir` module: the Evidence IR v1 spine. Defines
  `EvidenceArtifact` (unified canonical form), `EvidenceSpan` (source-span
  model with stable `source_id` threading), `VerificationRole`
  (`Property` | `RecordedOnly` | `Boundary`), `build_evidence_artifact`,
  `check_evidence_artifact` (validates schema + hash + non-claim boundary,
  rejects overclaims, fails closed), and `explain_span` (resolves a stable
  source ID to its span from a JSON manifest).
- Add `valence-cli` crate: the imperative shell (`valence check <FILE>`,
  `valence explain --id <ID> --manifest <FILE>`, `valence version`). All core
  logic lives in `valence-core`; the CLI owns only arg parsing, file reads,
  stdout/stderr, and exit codes.
- Add workspace scaffolding: `Cargo.toml` (workspace), `rust-toolchain.toml`
  (pinned nightly-2026-03-21, matching octet), `rustfmt.toml`, `clippy.toml`,
  `flake.nix`, `.gitignore`, `README.md`, `AGENTS.md`.
- Initialize native cairn lifecycle (`cairn/` layout) with `valence-default`
  policy (adapted from cairn's own policy).

## Impact

- **Files**: `crates/valence-core/src/{lib,function_object,evidence_index,evidence_codebase_index*,evidence_ir}.rs`,
  `crates/valence-cli/src/main.rs`, `crates/valence-cli/tests/cli_round_trip.rs`,
  `Cargo.toml`, `crates/valence-core/Cargo.toml`, `crates/valence-cli/Cargo.toml`,
  `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `flake.nix`,
  `.gitignore`, `README.md`, `AGENTS.md`, `cairn-policy/`,
  `cairn/changes/extract-valence-standalone-and-evidence-ir-spine/`.
- **Testing**: `cargo test --workspace` (47 tests: 40 core + 2 CLI unit + 5 CLI
  integration), `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --check`, `cairn validate --root .`, `cairn policy export --check`.
- **Non-claims**: Valence supports bounded local evidence replay. It does not
  claim behavioral correctness, semantic equivalence, exhaustive Rust coverage,
  pure-cache eligibility, cross-machine determinism, cross-toolchain
  determinism, external consumer compatibility, mechanized language soundness,
  or general HIR/THIR/MIR extraction. Every artifact carries an explicit
  `non_claims` boundary; the checker fails closed if the boundary is missing or
  weakened.
