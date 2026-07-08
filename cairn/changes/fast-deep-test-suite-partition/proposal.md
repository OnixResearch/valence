# Change: Fast and deep test suite partitioning

## Why

Valence needs fast deterministic tests for every edit and deeper integration checks for cross-repo and enforcement behavior. The harness should make this split explicit so CI, local development, and nightly jobs run the right evidence gates.

## What changes

- Define fast, deep, and nightly suite lanes with stable command names or documentation.
- Keep fast tests hermetic and independent of live sibling repositories or host-specific sandbox features.
- Move optional cross-repo and enforcement checks into deep/nightly lanes with explicit skip/unavailable reporting.

## Out of scope

- No weakening of required fast gates.
- No hidden network or host-specific assumptions in ordinary workspace tests.
