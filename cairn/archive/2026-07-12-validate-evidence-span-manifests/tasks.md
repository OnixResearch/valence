## Phase 1: Span integrity core

- [x] [serial] Define pure span structural validation and deterministic field-scoped issue types. r[valence.evidence_ir.span_integrity.contract]
- [x] [serial] Enforce identifier/path/label shape, byte ordering, one-based coordinates, and source-position ordering. r[valence.evidence_ir.span_integrity.coordinates]
- [x] [parallel] Add positive fixtures for ordinary and zero-width valid spans. r[valence.evidence_ir.span_integrity.fixtures.positive]
- [x] [parallel] Add negative fixtures for blank fields, control characters, reversed bytes, zero coordinates, reversed positions, and padded labels. r[valence.evidence_ir.span_integrity.fixtures.negative]

## Phase 2: Manifest and artifact integration

- [x] [serial] Validate every manifest row and reject duplicate source IDs before lookup. r[valence.evidence_ir.span_integrity.manifest_uniqueness]
- [x] [serial] Replace the `Option`-only explain boundary with typed found, not-found, and invalid-manifest outcomes and deterministic CLI diagnostics. r[valence.evidence_ir.span_integrity.explain_result]
- [x] [serial] Apply the same span validator to artifact construction and `check_evidence_artifact`. r[valence.evidence_ir.span_integrity.artifact_validation]
- [x] [parallel] Add CLI regressions proving malformed and duplicate manifests never print a selected span and remain distinct from `not_found`. r[valence.evidence_ir.span_integrity.fixtures.negative]

## Phase 3: Documentation and evidence

- [x] [serial] Document coordinate conventions, unique-ID lookup, zero-width behavior, and the source-byte correspondence non-claim. r[valence.evidence_ir.span_integrity.docs]
- [x] [serial] Run focused span/artifact/CLI tests, workspace tests, and Cairn validation/gates. r[valence.evidence_ir.span_integrity.final_validation]
