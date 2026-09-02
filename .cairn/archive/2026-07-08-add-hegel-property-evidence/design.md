# Design: Add Hegel property evidence

## Approach

Hegel stays test-only. `valence-core` owns pure deterministic generators, property inputs, and receipt validation over in-memory values. Test modules invoke Hegel and translate each run into a bounded property receipt value. CLI and filesystem concerns remain outside the core.

## Receipt model

A Hegel property receipt records schema version, property ID, linked requirement IDs, checked module or function family, Hegel crate/backend identity, runner settings, case budget, phases, seed or derandomized mode, generator family identity, BLAKE3 generator/content hash where applicable, status, issue codes, minimal counterexample identity for failures, and explicit non-claims.

Receipt validation accepts passing and failing receipts as evidence records, but it must fail closed when required identity fields are missing, settings are ambiguous, hashes are malformed or stale, non-claims are missing, or a generated-test receipt attempts to claim formal proof or exhaustive correctness.

## Property suite targets

Initial Hegel suites should cover pure core invariants that are already deterministic over in-memory values:

- EvidenceArtifact canonical hash stability under display-path churn and JSON field-order variation.
- fail-closed non-claim and overclaim detection for generated payload/non-claim variants.
- evidence graph determinism for equivalent node/edge orderings and fail-closed duplicate or dangling edges.
- alias resolution and cache-key equality laws for positive and negative generated cases.

## Lane partition

The fast lane uses small named budgets and deterministic or replayable Hegel settings suitable for ordinary `cargo test`. The deep lane may use larger budgets or stateful Hegel models, but must remain opt-in and report skipped/unavailable states rather than becoming a hidden release prerequisite.

## Boundary

A passing Hegel receipt proves only that the property held for the bounded generated cases and explicit examples used by that run. It does not prove behavioral correctness, semantic equivalence, exhaustive Rust coverage, verifier soundness, requirement satisfaction, release eligibility, or whole-program safety.
