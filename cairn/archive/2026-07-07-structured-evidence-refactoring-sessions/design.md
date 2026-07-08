## Context

Evidence identity surfaces are spread across artifacts, aliases, provenance chains, stack sidecars, manifests, and specs. Manual rename/migration work can create broken intermediate states.

## Decisions

### 1. Refactoring sessions are data

A session records requested rename or binding migration operations, expected affected identities, and policy constraints. The planner is pure over graph/session inputs.

### 2. Dry-run before mutation

Every session must support a dry-run impact report that lists affected nodes, edges, aliases, receipts, and files before any mutation is allowed.

### 3. Execute only unambiguous plans

Execution is blocked if aliases are ambiguous, target identities are missing, semantic equivalence would be required, or unsupported mutation surfaces are involved.

### 4. Receipts capture migration evidence

Executed sessions should produce receipts with before/after identity and binding manifests.

## Risks / Trade-offs

- This can drift into a broad migration framework. Start with alias/name/path refactors that preserve content identity.
- Mutations are risky. Keep imperative shell thin and require dry-run evidence.
