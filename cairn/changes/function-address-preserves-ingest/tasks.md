## Tasks

- [ ] [serial] Add pure function-address Preserves receipt validation input to `valence-core`. r[valence.evidence_ir.function_address_preserves_ingest.core]
- [ ] [serial] Add CLI shell support for reading Kamacite Preserves receipts and passing canonical bytes to the core. r[valence.evidence_ir.function_address_preserves_ingest.cli]
- [ ] [serial] Bind JSON compatibility projections to the canonical Preserves receipt hash. r[valence.evidence_ir.function_address_preserves_ingest.json_projection]
- [ ] [serial] Add positive fixtures for valid Preserves receipts producing Valence receipt and graph rows. r[valence.evidence_ir.function_address_preserves_ingest.positive]
- [ ] [serial] Add negative fixtures for wrong hash, schema mismatch, projection drift, domain substitution, missing spans, role promotion, dependency overclaims, and weakened non-claims. r[valence.evidence_ir.function_address_preserves_ingest.negative]
- [ ] [serial] Run Valence tests, core-purity smoke, and the Preserves-backed stack smoke. r[valence.evidence_ir.function_address_preserves_ingest.validation]
