# Valence testing harness

Valence keeps harness logic in pure `valence-core` functions and leaves file discovery, refresh writes, process execution, and CI wiring to shells or integration tests.

## Suite lanes

Fast lane:

```bash
nix develop --option builders '' -c cargo test -p valence-core evidence_harness
```

Runs hermetic transcript fixtures, malformed corpus cases, property invariants, golden normalization, requirement coverage, capability-label policy checks, and suite-summary checks. It must not require live sibling repositories, network access, or host-specific sandbox support.

Hegel fast lane:

```bash
nix develop --option builders '' -c cargo test -p valence-core --test hegel_property_laws
```

Runs bounded Hegel RS generated-input checks over pure Valence cores. The named fast budget is `HEGEL_FAST_CASE_BUDGET`, Hegel uses deterministic or replayable settings, and failures should be copied into concrete regression fixtures before relying on them for future review.

Deep lane:

```bash
nix develop --option builders '' -c cargo test --workspace stack_provenance -- --nocapture
```

Runs stack-provenance-focused checks and opt-in integration-style probes when their fixtures/backends are available. Missing sandbox support is reported as `unavailable` or `skipped`, never as successful enforcement.

Hegel deep coverage may use larger budgets or stateful models, but it stays opt-in and must report pass, fail, skipped, or unavailable status rather than becoming a hidden prerequisite for ordinary validation.

<!-- r[verify valence.stack_provenance_adapter.cross_repo_fixtures.deep] -->
Optional sibling-repo integration may be added to this lane or the nightly lane. The fast lane remains backed by frozen checked-in fixtures rather than live `../octet`, `../mantle`, `../trellis`, or `../cairn` checkouts.

Nightly lane:

```bash
nix develop --option builders '' -c cargo clippy --workspace --all-targets -- -D warnings && nix flake check --option builders '' -L
```

Runs expensive workspace and Nix validation. CI may choose to schedule this separately from ordinary edit-time checks.

## Transcript fixtures

Checked-in transcript fixtures live under:

- `crates/valence-core/testdata/transcripts/pass/`
- `crates/valence-core/testdata/transcripts/fail/`

Normal validation loads the fixtures and runs them through pure transcript evaluators. Refresh is explicit: call `plan_transcript_fixture_refresh` from a review shell or test helper, inspect the returned expected reports/hashes, and only then update fixture files.

## Golden reports

Golden checks compare normalized structured report values, not raw stdout. Raw stdout snapshots are limited to CLI boundary tests. Normalization sorts object fields, removes declared volatile fields, emits a BLAKE3 `content_hash`, and provides semantic changed-field diffs for stale expectations.

## Boundary

Harness reports are evidence about deterministic fixture behavior, marker traceability, and suite scheduling only. They do not prove behavioral correctness, semantic equivalence, verifier soundness, requirement satisfaction, release eligibility, sandbox effectiveness, or whole-program safety.
