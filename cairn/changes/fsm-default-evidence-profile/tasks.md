## Tasks

- [x] [serial] Define the `fsm-default-exhaustive` Evidence IR profile fields, role vocabulary, identity-domain rules, and required non-claims. r[valence.evidence_ir.fsm_default_profile.contract]
- [x] [serial] Implement pure validation over parsed FSM default-exhaustive evidence records without filesystem, process, environment, network, clock, or stdout access. r[valence.evidence_ir.fsm_default_profile.validation]
- [x] [parallel] Add positive fixtures for Octet-only default-exhaustive receipts and Octet-plus-Kamacite linked receipts with source spans, universe identities, coverage identities, denial coverage identities, artifact identities, and non-claims. r[valence.evidence_ir.fsm_default_profile.fixtures.positive]
- [x] [parallel] Add negative fixtures for stale universes, malformed coverage digests, missing denial coverage, missing non-claims, identity-domain substitution, unsupported profile versions, unknown roles, and overclaims. r[valence.evidence_ir.fsm_default_profile.fixtures.negative]
- [x] [serial] Preserve formal proof links as separate evidence records without promoting the default-exhaustive FSM receipt tier. r[valence.evidence_ir.fsm_default_profile.proof_links]
- [x] [parallel] Document Octet, Kamacite, and Valence ownership boundaries for default-exhaustive FSM evidence. r[valence.evidence_ir.fsm_default_profile.docs]
- [x] [serial] Run focused core tests, fixture validation, formatting, Cairn validation, and proposal/design/tasks gates before archive. r[valence.evidence_ir.fsm_default_profile.final_validation]
