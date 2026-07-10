## Phase 1: Function-address Evidence IR profile

- [x] [serial] Capture the Valence function-address evidence profile requirement delta before implementation. r[valence.evidence_ir.function_address_profile.contract]
- [x] [serial] Add pure core DTOs/validators for function-address profile records, identity domains, roles, verification-role classification, and non-claim policy. r[valence.evidence_ir.function_address_profile.contract] r[valence.evidence_ir.function_address_profile.validation]
- [x] [parallel] Add positive fixtures for Octet-only and Octet-plus-Kamacite function-address evidence with spans, dependencies, roles, and graph-report rows. r[valence.evidence_ir.function_address_profile.fixtures.positive] r[valence.evidence_ir.function_address_profile.graph]
- [x] [parallel] Add negative fixtures for stale identities, wrong digest domains, unsupported profile versions, unknown roles, missing non-claims, overclaims, and attempted formal-proof promotion. r[valence.evidence_ir.function_address_profile.fixtures.negative]
- [x] [parallel] Add documentation and CLI/graph-report updates that preserve Octet/Kamacite/Valence/Mantle/Cairn boundaries. r[valence.evidence_ir.function_address_profile.docs]
- [x] [serial] Run focused core tests, CLI/contract checks if exposed, graph-report checks, formatting, Cairn validation, and proposal/design/tasks gates before sync/archive. r[valence.evidence_ir.function_address_profile.final_validation]
