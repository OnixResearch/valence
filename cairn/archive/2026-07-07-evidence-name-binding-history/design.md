## Context

Names are useful pointers, not durable identities. A source path, requirement alias, function name, or release label can move while the underlying evidence content remains the same.

## Decisions

### 1. Binding records are separate from content identity

A binding record maps an alias to a content identity and records enough context to audit the binding. The content identity remains authoritative.

### 2. Bindings are append-only in receipts

Receipts should preserve prior aliases rather than overwrite them silently. A rename can add a new binding while keeping history for review.

### 3. Ambiguous aliases fail closed when used as identity

If an alias resolves to multiple content identities and a caller asks for identity-level verification, Valence should return a deterministic ambiguity diagnostic.

### 4. Stack semantics remain bounded

Name-binding history proves alias linkage only. It does not prove semantic equivalence between renamed definitions or that a requirement is satisfied.

## Risks / Trade-offs

- Alias history can grow. Keep binding records compact and sorted deterministically.
- Compatibility IDs may appear identity-like. Documentation and types must call them aliases when they are not content identities.
