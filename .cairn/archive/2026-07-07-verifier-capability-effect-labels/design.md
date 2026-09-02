## Context

Current Valence reports often describe what was checked but not what effects the checker used. Effect tracking should be typed and explicit, inspired by Unison abilities but implemented as ordinary Rust DTOs and policies.

## Decisions

### 1. Capability vocabulary is narrow and typed

Start with `Pure`, `ReadsFilesystem`, `ReadsClock`, `RunsProcess`, `UsesNetwork`, `UsesRemoteBuilder`, and `UsesSecrets`. Unknown capabilities fail closed or are treated conservatively according to policy.

### 2. Capability sets are receipt evidence

A verifier report should carry the capability set used to produce the evidence. This does not prove the verifier actually stayed within the set unless enforced by a specific runner; it is still useful policy evidence.

### 3. Pure checks get stronger replay/cache semantics

Only capability sets marked pure should be eligible for pure-result caching without environmental provenance.

### 4. Risky effects can be policy-denied

Gates should be able to reject `UsesNetwork`, `UsesSecrets`, or `UsesRemoteBuilder` for contexts that require local deterministic evidence.

## Risks / Trade-offs

- Capability labels can overclaim. Receipts must include a non-claim unless execution sandboxing enforces them.
- Unknown capabilities must not silently pass in strict modes.
