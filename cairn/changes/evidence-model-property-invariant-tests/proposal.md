# Change: Evidence model property invariant tests

## Why

The evidence model depends on identity and fail-closed invariants that are stronger than individual example tests. The harness should exercise rename stability, dependency sensitivity, graph determinism, alias ambiguity, and cache-key equality across generated or table-driven inputs.

## What changes

- Add invariant/property tests for Evidence IR identity, bindings, graph export, capability policy, and cache decisions.
- Keep invariants pure and deterministic, with no mocks or external processes.
- Record positive and negative invariants so regressions fail closed.

## Out of scope

- No randomized non-reproducible tests.
- No semantic-equivalence or correctness claims beyond the modeled identity/linkage/cache invariants.
