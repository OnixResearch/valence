# Change: Add Hegel property evidence

## Why

Valence already has deterministic fixture, corpus, and harness checks, but many pure Evidence IR laws are stronger when exercised over generated inputs. Hegel can provide shrinking and reproducible counterexamples for canonical identity, graph determinism, non-claim enforcement, alias resolution, and cache-key laws, as long as Valence keeps the claim boundary explicit.

## What changes

- Add Valence-owned Hegel property receipt semantics for bounded generated-input test runs.
- Add Hegel-backed property suites for selected pure `valence-core` invariants.
- Partition Hegel coverage into fast and deep lanes with named budgets and deterministic/replayable settings.
- Document that Hegel evidence is generated testing evidence only, not formal proof, verifier soundness, requirement satisfaction, or release eligibility.

## Out of scope

- No production dependency on Hegel in `valence-core` or `valence-cli`.
- No filesystem, process, network, clock, or environment access in pure property validation cores.
- No claim that generated testing is exhaustive or equivalent to Verus/Kani/formal proof evidence.
- No Octet rail implementation in this change; Octet consumes this receipt contract through its own change package.
