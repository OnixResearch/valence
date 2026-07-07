## Why

Unison supports structured refactoring without leaving the codebase broken. Valence should offer similar structured sessions for evidence renames, alias migrations, requirement ID moves, and provenance graph rewrites.

This would let operators preview impact, migrate bindings, and produce receipts instead of manually editing scattered JSON and specs.

## What Changes

- Define a structured refactoring session model for evidence identity/name changes.
- Add dry-run impact reports and deterministic mutation plans.
- Require blocked plans when identity ambiguity, dangling references, or unsupported semantic changes are detected.
- Record migration receipts and tests for rename/path churn.

## Impact

- **Files**: likely evidence graph/name-binding modules, CLI later, docs, fixtures.
- **Compatibility**: dry-run first; mutation commands should be opt-in and receipt-backed.
- **Validation**: plan/execute/blocker tests and Cairn gates.
