## Tasks

- [x] [serial] Define typed artifact ref roles, digest spelling profiles, path metadata, and hash/size contracts. r[valence.evidence_ir.artifact_ref_core.contract]
- [x] [serial] Add positive fixtures for Valence, Cairn, Mantle, Basalt, Octet, and Molten/Aspen artifact refs. r[valence.evidence_ir.artifact_ref_core.fixtures.positive]
- [x] [serial] Add negative fixtures for malformed refs, unsupported prefixes, role mixups, path escapes, stale size/hash metadata, missing non-claims, and overclaims. r[valence.evidence_ir.artifact_ref_core.fixtures.negative]
- [x] [serial] Implement pure artifact ref validation and compatibility profiles. r[valence.evidence_ir.artifact_ref_core.validation]
- [x] [serial] Add adapters that preserve current public artifact fields. r[valence.evidence_ir.artifact_ref_core.adapters]
- [x] [serial] Document whether the implementation lands standalone or as part of `shared-evidence-identity-core`. r[valence.evidence_ir.artifact_ref_core.docs]
- [x] [serial] Run focused artifact ref fixture tests, `valence-core` tests, and Cairn validation/gates. r[valence.evidence_ir.artifact_ref_core.final_validation]
