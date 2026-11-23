# Tasks

## 1. Profile implementation and chain registration

- [x] [serial] [task:core] Implement `crates/valence-core/src/semantic_merge_evidence.rs` and register `semantic-merge` in `crates/valence-core/src/evidence_chain.rs` with four-role bindings, separated digest domains, six-state outcomes, artifact-role separation, projections, and consumer applicability checks. r[valence.evidence_ir.semantic_merge_evidence_profile.registry] r[valence.evidence_ir.semantic_merge_evidence_profile.subjects_and_domains] r[valence.evidence_ir.semantic_merge_evidence_profile.outcomes_and_roles] r[valence.evidence_ir.semantic_merge_evidence_profile.applicability]

## 2. Fixtures and documentation

- [x] [serial] [task:fixtures] [after:core] Add positive, negative, round-trip/projection, and frozen Octet-to-Kiln bounded integration fixtures (`fixtures/semantic-merge/bounded-integration-octet-kiln-v1.json`) plus `docs/semantic-merge-evidence-profile.md`. r[valence.evidence_ir.semantic_merge_evidence_profile.negative] r[valence.evidence_ir.semantic_merge_evidence_profile.integration_fixture]

## 3. Verification

- [x] [serial] [task:verify] [after:fixtures] Run formatting, workspace build/tests, strict Clippy, core-purity smoke, license-boundary check, and Cairn validation and gates. r[valence.evidence_ir.semantic_merge_evidence_profile.registry]
