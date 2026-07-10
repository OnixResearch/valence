## Tasks

- [ ] [serial] Register a Trellis proof evidence validator under the generic evidence-chain profile. r[valence.evidence_ir.trellis_proof_evidence_profile.registry]
- [ ] [serial] Validate source, proof IR, verifier receipt, assumptions, property IDs, requirement IDs, spans, digest domains, policy hashes, and non-claims. r[valence.evidence_ir.trellis_proof_evidence_profile.validation]
- [ ] [serial] Separate reference-only, formal-proof candidate, and accepted formal-proof role outcomes. r[valence.evidence_ir.trellis_proof_evidence_profile.roles]
- [ ] [serial] Emit graph rows for requirements, source spans, verifier receipts, assumptions, proof obligations, and downstream artifacts. r[valence.evidence_ir.trellis_proof_evidence_profile.graph]
- [ ] [serial] Add positive fixtures for recorded-only reference evidence, accepted formal-proof evidence, and graph output. r[valence.evidence_ir.trellis_proof_evidence_profile.positive]
- [ ] [serial] Add negative fixtures for stale source, missing assumptions, verifier mismatch, unsupported status, malformed IDs, unauthorized role promotion, domain substitution, and weakened non-claims. r[valence.evidence_ir.trellis_proof_evidence_profile.negative]
- [ ] [serial] Run Valence tests, core-purity smoke, and Trellis proof evidence stack smoke. r[valence.evidence_ir.trellis_proof_evidence_profile.final_validation]
