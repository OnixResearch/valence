# Validation evidence

## Completion contract

Goal: implement a real local `radicle-cob` sidecar adapter while preserving Valence evidence authority, `valence-core` purity, recorded-only linkage, deterministic concurrency, saved offline Cairn handoff, and jj/Git ref safety.

Completion requires executable positive and negative tests, exact dependency pinning, Nickel profile freshness, no Radicle/Git dependency in `valence-core`, real signed create/list/update round trips, saved receipt verification without repository/network access, passing focused traceability, workspace checks, and unblocked Cairn sync.

False completion includes a pure DTO-only prototype, raw custom refs that bypass `radicle-cob`, a live-node dependency in validation, self-authorized signatures, attachment role promotion, timestamp conflict resolution, branch/working-copy mutation, or task checkboxes without executable evidence.

## Portfolio search and adversarial audit

| Family | Mechanism | Result |
|---|---|---|
| `radicle-cob-local-git` | Exact-pinned `radicle-cob` CRU APIs over a local `git2` object/ref storage adapter, with `Evaluate` preserving signed transport facts for the pure reducer | **Validated** by signed create/list/update, reference/embed, revoke, malformed-root, and multi-root tests |
| `raw-git-special-refs` | Write a custom Git JSON history without `radicle-cob` | **Rejected** as false completion because it bypasses COB manifests, signatures, graph loading, and reviewed API compatibility |
| `live-radicle-node` | Depend on Radicle node/home/seeding APIs during mutation or snapshot validation | **Deferred** because it adds network/node lifecycle and violates saved offline handoff |
| `receipt-only` | Emit Valence snapshots without writing a real COB | **Rejected** because it does not prove adapter round-trip or repository safety |

Adversarial review found and corrected stale observed-target hashes, forged-but-well-shaped profile drift, snapshot verification without supplied profile/authorization content, malformed related roots being omitted, and noncausal supersede/revoke planning. The accepted implementation recomputes target-state identity, enforces the exact v1 profile, verifies offline snapshots against both reviewed inputs, retains malformed related roots as rejected evidence, and blocks disposition plans unless accepted predecessors are causal in the selected physical sidecar.

A secondary local-model review was advisory only and was rejected where it proposed non-existent APIs or untyped event shapes. Repository compilation and deterministic checks remained authoritative.

## Implemented evidence

- Pure core: `crates/valence-core/src/cob_evidence.rs`
- Adapter: `crates/valence-radicle/`
- Nickel source: `contracts/radicle-cob-evidence/profile.ncl`
- Generated runtime profile: `contracts/radicle-cob-evidence/generated/profile.json`
- Operator and boundary guide: `docs/radicle-cob-evidence.md`
- Accepted specification: `cairn/specs/radicle-cob-evidence/spec.md`
- Dedicated Tracey profile: `radicle-cob-evidence`

The executable fixture constructs an isolated local Git repository with a built-in issue-type target ref, signed authorized sidecars, reference-only and embedded artifacts, multiple independently created physical roots, malformed signed roots, and confined special-ref mutation. It does not use network access or mutate the Valence working repository.

## Checks

| Check | Result |
|---|---|
| Pre-change `nix develop -c cargo test --workspace` | PASS |
| `nix develop -c cargo test -p valence-core cob_evidence` | PASS, 11 focused core tests |
| `nix develop -c cargo test -p valence-radicle` | PASS, 5 adapter tests and 2 CLI tests |
| `nix develop -c cargo fmt --all -- --check` | PASS |
| `nix develop -c cargo test --workspace` | PASS |
| `nix develop -c cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Core purity search on `cob_evidence.rs` for filesystem/environment/process/clock/async/printing effects | PASS, no matches |
| `cargo tree -p valence-core` search for `radicle` or `git2` | PASS, no matches |
| `nix build .#checks.x86_64-linux.radicle-cob-evidence-profile --no-link -L` | PASS, positive/negative Nickel fixtures and generated-profile freshness |
| `nix build .#valence-radicle --no-link -L` | PASS |
| `nix run path:../cairn#cairn -- policy export --check` | PASS |
| Proposal/design/tasks gates | PASS; 14/14 tasks complete |
| `nix run path:../cairn#cairn -- validate --root .` | PASS after separating one pre-existing combined function-address requirement identity |
| `nix run path:../cairn#cairn -- tracey coverage --root . --profile radicle-cob-evidence` | PASS, 42/42 referenced |
| Cairn sync dry-run / execute | PASS; unblocked, accepted spec created, execute `mutated = true` |
| `nix flake check -L` | PASS on `x86_64-linux`; `aarch64-linux` reported omitted/incompatible |

The older default `valence-default` Tracey profile remains scoped to stack-provenance and has pre-existing missing/dangling references. No global default-profile Tracey success is claimed; the change-owned profile is complete.

## Receipt and claim boundaries

Cairn sync dry-run receipt hash: `d21d215e6fc554659e576adf3d26dcccfad2cee59745f6837c4c1ad3f6e673f1`.

Cairn sync execute receipt hash: `c060be305af0727a5d6d579259197aa4ad10a292e45d5b63dcaaa751c1c1fec7`.

Passing checks prove bounded schema/profile conformance, canonical BLAKE3 identity, typed Git/Radicle OID preservation, signature facts, supplied authorization-policy conformance, deterministic reduction, saved receipt integrity, local special-ref confinement, and executable fixture behavior only. They do not prove evidence truth, target correctness, ambient signer authority, peer availability, deletion from replicated history, confidentiality, requirement satisfaction, lifecycle readiness, release eligibility, or whole-system correctness.
