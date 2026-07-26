# Forge operations validation — 2026-07-26

## Scope

This evidence covers deterministic normalized issue/pull-request planning, process-isolated built-in Radicle issue/patch mutation, offline source-to-target bindings, and exact-revision CI-admission receipts. No production repository was mutated, no canonical ref was changed, no source-host API was called, and live CI remained scoped to Bounded Exec.

## Architectural compatibility finding

The first direct integration of upstream `radicle = 0.24.0` into `valence-radicle` enabled `serde_json/preserve_order` in the same Cargo process. Nix testing correctly failed the pre-existing `function_object::tests::hash_payload_is_reviewable_and_excludes_display_fields` invariant (`schema` appeared before the expected sorted `body_hash`).

The rejected design would have changed Valence canonical JSON behavior. The accepted design moves native built-in mutation into the standalone, workspace-excluded `valence-radicle-native` crate. It has its own tool-generated `Cargo.lock`, no `valence-core` dependency, and an explicit JSON plan/observation process boundary. `valence-radicle` retains the default sorted-map backend and independently revalidates the plan before binding observations.

The standalone manifest pins the reviewed compatible set exactly where upstream semver ranges otherwise diverge:

- `radicle = 0.24.0` (Radicle 1.9.1 library generation);
- `radicle-cob = 0.20.0`;
- `radicle-crypto = 0.17.0`;
- `radicle-oid = 0.2.0`; and
- `git2 = 0.20.4`.

The explicit `radicle-oid` pin prevents resolution to `0.2.2`/`git2 0.21`, which is type-incompatible with `radicle-cob 0.20.0`'s `git2 0.20.4` adapter.

## Retained positive and negative evidence

- Nickel pass fixtures export; missing non-claim, authority-promoting review handling, and unbounded-record fixtures fail.
- The normalized source fixture deterministically emits the checked migration plan.
- Native preview validates the plan without opening the selected repository and writes no observation file.
- Built-in evaluator round trips create/reload a solved issue and archived patch, preserving imported review metadata as comments with zero Radicle reviews.
- Negative tests reject malformed/duplicate normalized records, conflicting mappings, plan tampering, signer mismatch, wrong operation type, absent patch Git objects, absent existing COBs, escaping output paths, duplicate outputs, and unintended changed refs.
- Binding replay succeeds offline; plan, observed-state, and non-claim tampering fail.
- Exact CI admission succeeds only for a zero-exit succeeded event/result, matching signed-bot observation, threshold unique delegate accepts for the exact revision, current canonical predecessor, and descendant candidate.
- Negative admission tests cover failed result, unsigned status, duplicate/wrong-revision approval, non-descendant/stale canonical state, bot-as-delegate, stale identity, and weakened non-claims.
- A static positive/negative core-purity test rejects filesystem, environment, process, network, clock, Git, or Radicle effect markers in the production pure module.

## Commands and outcomes

All commands ran from the isolated Valence working copy.

```text
nix develop -c cargo fmt --all --check
nix develop -c cargo fmt --manifest-path crates/valence-radicle-native/Cargo.toml --all --check
```

Outcome: passed.

```text
nix develop -c cargo test --workspace
nix develop -c cargo test --manifest-path crates/valence-radicle-native/Cargo.toml
```

Outcome: all workspace and standalone native tests passed. The standalone native suite includes real built-in issue/patch evaluator round trips.

```text
nix develop -c cargo clippy --workspace --all-targets -- -D warnings
nix develop -c cargo clippy --manifest-path crates/valence-radicle-native/Cargo.toml --all-targets -- -D warnings
```

Outcome: passed with warnings denied.

```text
nix build .#valence-radicle-native --no-link -L
nix build .#checks.x86_64-linux.radicle-forge-operations --no-link -L
nix flake check -L
```

Outcome: passed on `x86_64-linux`; the flake reported only the declared incompatible `aarch64-linux` checks as omitted. The forge-operations check proves Nickel freshness/pass/fail fixtures, deterministic CLI golden bytes, no-write native preview, admitted receipt verification, and failed-result denial.

```text
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- validate --root . \
  --policy /home/brittonr/git/OnixResearch/cairn/cairn-policy/generated/cairn-policy.json
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- gate proposal migrate-forge-metadata-and-bind-ci-admission --root . --policy <policy>
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- gate design migrate-forge-metadata-and-bind-ci-admission --root . --policy <policy>
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- gate tasks migrate-forge-metadata-and-bind-ci-admission --root . --policy <policy>
```

Outcome: all passed before sync/archive.

```text
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- policy export --check
nix run /home/brittonr/git/OnixResearch/cairn#cairn -- tracey coverage --root . --profile radicle-forge-operations
```

Outcome: generated policy fresh; dedicated Tracey coverage passed with 9/9 requirement IDs referenced. Cairn sync dry-run and execute created `cairn/specs/radicle-forge-operations/spec.md` before this coverage check.

## Claim boundary

This evidence establishes checked deterministic plans, observed native built-in COB behavior in temporary test repositories, offline bindings, and bounded admission decisions over supplied observations. It does not establish source export completeness, actor authenticity, original source signatures/timestamps, semantic equivalence, live signature/currentness truth, CI correctness, host sandboxing, protocol-enforced mandatory CI, bypass-proof delegate behavior, canonical mutation, merge eligibility, release readiness, or whole-stack GitHub independence.
