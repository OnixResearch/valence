## Tasks

- [ ] [serial] Define the state-machine exhaustiveness receipt contract and Evidence IR kind. r[valence.evidence_ir.state_machine_evidence.contract]
- [ ] [serial] Add positive fixtures for valid Octet state-machine receipts with source spans, state/event identities, coverage summary digests, status, diagnostics, artifact BLAKE3 identities, and non-claims. r[valence.evidence_ir.state_machine_evidence.fixtures.positive]
- [ ] [serial] Add negative fixtures for missing non-claims, malformed digests, stale coverage summaries, unknown tiers, missing parent Octet export identities, and overclaim text. r[valence.evidence_ir.state_machine_evidence.fixtures.negative]
- [ ] [serial] Implement the pure receipt validation core and keep file loading/report rendering in the CLI or adapter shell. r[valence.evidence_ir.state_machine_evidence.validation]
- [ ] [serial] Preserve optional Verus or Kani proof links as separate evidence records without promoting the state-machine receipt tier. r[valence.evidence_ir.state_machine_evidence.proof_links]
- [ ] [serial] Document Octet producer responsibilities, Valence non-claim boundaries, and consumer interpretation rules. r[valence.evidence_ir.state_machine_evidence.docs]
- [ ] [serial] Run focused core tests, formatting, and Cairn validation/gates. r[valence.evidence_ir.state_machine_evidence.final_validation]
