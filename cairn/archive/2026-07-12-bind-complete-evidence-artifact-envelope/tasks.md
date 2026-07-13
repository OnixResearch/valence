## Phase 1: Strict envelope core

- [x] [serial] Implement pure field inventory and strict typed decoding for the closed Evidence IR envelope. r[valence.evidence_ir.complete_envelope.strict_decode]
- [x] [serial] Reject unknown, duplicate, misspelled, and misplaced outer or closed-nested control fields with field-aware diagnostics. r[valence.evidence_ir.complete_envelope.unknown_fields]
- [x] [parallel] Add positive fixtures for current artifacts and open kind-specific payload extensions. r[valence.evidence_ir.complete_envelope.fixtures.positive]
- [x] [parallel] Add negative exploit fixtures for unknown top-level claims, typoed role/hash fields, unknown span fields, duplicate fields, and trailing JSON values. r[valence.evidence_ir.complete_envelope.fixtures.negative]

## Phase 2: Canonical identity hardening

- [x] [serial] Route Evidence IR and shared policy JSON identities through one recursive, feature-independent canonical byte encoder. r[valence.evidence_ir.complete_envelope.canonical_hash]
- [x] [parallel] Add a downstream consumer fixture enabling `serde_json/preserve_order` and assert canonical bytes and BLAKE3 goldens remain unchanged. r[valence.evidence_ir.complete_envelope.feature_matrix]
- [x] [serial] Check current valid artifact hashes for compatibility and require a versioned migration if any accepted identity would change. r[valence.evidence_ir.complete_envelope.compatibility]

## Phase 3: Integration and evidence

- [x] [serial] Update `check_evidence_artifact` and CLI diagnostics to preserve malformed, unknown, duplicate, and hash-mismatch distinctions without echoing untrusted values. r[valence.evidence_ir.complete_envelope.diagnostics]
- [x] [serial] Document the closed-envelope/open-payload boundary and coordination with typed claim policy. r[valence.evidence_ir.complete_envelope.docs]
- [x] [serial] Run focused exploit regressions, feature-matrix identity tests, workspace tests, and Cairn validation/gates. r[valence.evidence_ir.complete_envelope.final_validation]
