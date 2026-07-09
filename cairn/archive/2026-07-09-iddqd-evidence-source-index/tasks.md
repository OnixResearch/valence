## Tasks

- [x] [serial] Add iddqd to `valence-core` and define an ordered normalized-source identity index. r[valence.evidence_ir.iddqd_evidence_source_index.source_index]
- [x] [serial] Replace duplicate source-hash map construction with pure identity-index construction and explicit duplicate diagnostics. r[valence.evidence_ir.iddqd_evidence_source_index.duplicate_policy]
- [x] [parallel] Add positive fixtures for unique locators, advisory exact duplicates, provenance lookup, and stable index hashes. r[valence.evidence_ir.iddqd_evidence_source_index.fixtures.positive]
- [x] [parallel] Add negative fixtures for conflicting duplicate locators, stale provenance hashes, missing referenced artifacts, malformed JSON/JSONL, and overbroad claims. r[valence.evidence_ir.iddqd_evidence_source_index.fixtures.negative]
- [x] [serial] Confirm Evidence IR index, query report, and non-claim JSON shapes remain stable. r[valence.evidence_ir.iddqd_evidence_source_index.schema_compatibility]
- [x] [serial] Run focused evidence-codebase-index tests, core purity smoke, workspace tests as needed, and Cairn validation/gates. r[valence.evidence_ir.iddqd_evidence_source_index.final_validation]
