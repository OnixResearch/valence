## Phase 1: Session model

- [ ] [serial] r[valence.evidence_ir.structured_refactoring] Define typed refactoring session inputs for alias/name/path and requirement binding migrations.
- [ ] [serial] r[valence.evidence_ir.structured_refactoring.plan] Add pure dry-run planning over evidence graph/session inputs.

## Phase 2: Safety behavior

- [ ] [serial] r[valence.evidence_ir.structured_refactoring.blockers] Block plans for ambiguous aliases, missing identities, dangling edges, unsupported mutation surfaces, or required semantic-equivalence claims.
- [ ] [serial] r[valence.evidence_ir.structured_refactoring.receipts] Define before/after migration receipt fields.
- [ ] [parallel] r[valence.evidence_ir.structured_refactoring.tests] Add positive rename/path migration tests and negative blocker tests.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.structured_refactoring.docs] Document dry-run-first behavior, mutation boundaries, and non-claims.
- [ ] [serial] r[valence.evidence_ir.structured_refactoring] Run refactoring planner tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
