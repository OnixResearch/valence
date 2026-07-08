# Design: Evidence model property invariant tests

## Approach

Add a focused test module around pure evidence-model helpers. Each invariant is expressed as deterministic generated cases or compact tables so failures are reproducible and reviewable.

## Core invariants

- Display name, path, and alias churn does not change content identity when canonical content and dependency identities are unchanged.
- Canonical content or dependency identity changes alter parent identity.
- Graph export and graph identity are stable under node, edge, and binding input reordering.
- Ambiguous aliases and stale bindings fail closed.
- Cache reuse hits only when every cache-key field matches and policy allows replay.

## Failure model

Every invariant has at least one negative case. Diagnostics remain deterministic and sorted.
