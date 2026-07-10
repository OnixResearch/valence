## Tasks

- [ ] [serial] r[valence.evidence_ir.proof_carrying_ir_lineage.profile] Define the pure proof-carrying IR lineage profile and row DTOs.
- [ ] [depends:pcir-profile] r[valence.evidence_ir.proof_carrying_ir_lineage.validation] Validate source/IR refs, node ids, proof-obligation ids, transform steps, dialect identities, sidecar refs, roles, hashes, and non-claims.
- [ ] [depends:pcir-validation] r[valence.evidence_ir.proof_carrying_ir_lineage.opaque_dialects] Preserve unknown dialect identity and payload hashes without interpreting semantics.
- [ ] [depends:pcir-validation] r[valence.evidence_ir.proof_carrying_ir_lineage.positive] Add positive fixtures for valid lineage, unknown dialect metadata, and proof sidecar binding.
- [ ] [depends:pcir-validation] r[valence.evidence_ir.proof_carrying_ir_lineage.negative] Add negative fixtures for stale sidecar, cross-artifact substitution, role promotion, and semantic-equivalence overclaim.
- [ ] [depends:pcir-fixtures] r[valence.evidence_ir.proof_carrying_ir_lineage.shell] Keep parser/checker/file effects in `valence-cli` or producers, not `valence-core`.
- [ ] [depends:pcir-shell] r[valence.evidence_ir.proof_carrying_ir_lineage.final_validation] Run Valence tests, core purity smoke, fixture validation, Cairn gates, and focused stack evidence checks.