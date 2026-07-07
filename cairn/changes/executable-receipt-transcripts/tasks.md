## Phase 1: Transcript model

- [ ] [serial] r[valence.evidence_ir.executable_transcripts] Define transcript schema for inputs, verifier action, expected reports, issue codes, and hashes.
- [ ] [serial] r[valence.evidence_ir.executable_transcripts.runner] Add pure transcript evaluation helpers for in-memory verifier scenarios.

## Phase 2: Fixtures and gates

- [ ] [parallel] r[valence.evidence_ir.executable_transcripts.positive] Add positive transcript fixtures for valid Evidence IR and stack-provenance workflows.
- [ ] [parallel] r[valence.evidence_ir.executable_transcripts.negative] Add negative transcript fixtures for malformed inputs, overclaims, stale hashes, and denied capabilities.
- [ ] [serial] r[valence.evidence_ir.executable_transcripts.refresh] Add or document an intentional refresh workflow for expected output/hash changes.

## Phase 3: Documentation and validation

- [ ] [parallel] r[valence.evidence_ir.executable_transcripts.docs] Document transcript claims and non-claims.
- [ ] [serial] r[valence.evidence_ir.executable_transcripts] Run transcript tests, stack-provenance tests, `cargo fmt --check`, `cargo test --workspace`, and Cairn validation/gates.
