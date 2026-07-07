## Phase 1: Binding model

- [ ] [serial] r[valence.evidence_ir.name_binding_history] Define evidence name-binding records from alias/name/path to content identity.
- [ ] [serial] r[valence.evidence_ir.name_binding_history.receipts] Add receipt or sidecar surfaces for binding history where operator review needs it.

## Phase 2: Resolution behavior

- [ ] [serial] r[valence.evidence_ir.name_binding_history.resolve] Resolve aliases to content identities deterministically.
- [ ] [serial] r[valence.evidence_ir.name_binding_history.ambiguous] Fail closed on ambiguous aliases when identity-level verification is requested.
- [ ] [parallel] r[valence.evidence_ir.name_binding_history.rename] Add positive rename/path-churn tests showing provenance follows content identity.
- [ ] [parallel] r[valence.evidence_ir.name_binding_history.stale] Add negative tests for stale or conflicting bindings.

## Phase 3: Evidence

- [ ] [parallel] r[valence.evidence_ir.name_binding_history.docs] Document names as metadata pointers and hashes as identity.
- [ ] [serial] r[valence.evidence_ir.name_binding_history] Run focused evidence/provenance tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
