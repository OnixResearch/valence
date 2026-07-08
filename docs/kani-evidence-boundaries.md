# Kani evidence roles and non-claims

Valence treats Kani output as scoped Evidence IR, not as generic formal proof.
Kani artifacts are accepted only when they preserve target identity, toolchain
identity, receipt status, role classification, and explicit non-claim labels.

## Contract evidence

Use `artifact_kind = "kani-contract-evidence-v1"` for Kani contract or harness
receipts. The payload must include:

- `contract_facets`: `precondition`, `postcondition`, `loop_invariant`,
  `loop_decreases`, `quantifier`, or `contract_harness`.
- `target`: crate target, source ID, checked target scope, and monomorphization
  or target instance.
- `toolchain`: Kani, Rust, CBMC, and solver identity.
- `receipt`: saved receipt hash and `passed`, `failed`, or `bounded_only`
  status.
- `claim_classification`: `passing_contract`,
  `partial_correctness_loop_contract`, or `bounded_harness`.

A `property` role requires a passing receipt. Bounded-only harnesses stay
`recorded_only`. Loop termination claims require a `loop_decreases` facet.
Valence rejects `all_monomorphizations` target scopes because they flatten a
scoped Kani run into an unreviewable universal claim.

## Stub evidence

Use `artifact_kind = "kani-stub-evidence-v1"` for Kani stub facts. The payload
must classify the stub as:

- `verified_contract_stub`: links to a passing contract artifact with matching
  original function, source scope, signature, and artifact hash.
- `plain_stub`: boundary/recorded-only evidence with explicit assumptions.
- `environmental_stub`: boundary evidence with explicit environmental model
  assumptions.

Plain and environmental stubs must not be promoted to `property` evidence.
Verified stubs fail closed if the contract link is missing, stale, failed, or
signature-mismatched.

## Required Kani non-claim labels

Every Kani artifact must carry the general Evidence IR non-claim plus these
labels:

- `kani.boundary.whole_program`
- `kani.boundary.verifier_soundness`
- `kani.boundary.aliasing`
- `kani.boundary.concurrency_async`
- `kani.boundary.ffi_stub`
- `kani.boundary.recursive_termination`
- `kani.boundary.bounded_only`

`valence check` prints `non_claim_count` and each `non_claim=` line so these
boundaries are visible in CLI output. This report is identity and boundary
validation only; it does not prove Kani, CBMC, solver, Rust-model, behavioral,
whole-program, or release-readiness correctness.
