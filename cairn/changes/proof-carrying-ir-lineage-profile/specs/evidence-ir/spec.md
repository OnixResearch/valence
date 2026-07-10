# Evidence IR Specification

## Purpose

Add a proof-carrying IR lineage profile for recorded IR/proof sidecar linkage.

## Requirements

### Requirement: Proof-carrying IR lineage profile
r[valence.evidence_ir.proof_carrying_ir_lineage.profile] Valence MUST define a pure Evidence IR profile for proof-carrying IR lineage rows that bind source artifact refs, IR artifact refs, node or region ids, proof-obligation ids, transform step ids, dialect identities, proof sidecar refs, verification roles, hashes, and non-claim boundaries.

#### Scenario: Valid lineage validates
r[valence.evidence_ir.proof_carrying_ir_lineage.positive]
- GIVEN a producer provides proof-carrying IR lineage rows with matching source refs, IR refs, proof-obligation ids, transform step ids, sidecar artifact refs, roles, hashes, and non-claims
- WHEN Valence validates the profile
- THEN Valence MUST accept the rows as bounded identity and linkage evidence.

### Requirement: Opaque dialect preservation
r[valence.evidence_ir.proof_carrying_ir_lineage.opaque_dialects] Valence MUST preserve unknown dialect identity, version, payload hash, and producer metadata without interpreting dialect semantics.

#### Scenario: Unknown dialect remains recorded-only
- GIVEN an IR lineage row names an unknown dialect with a valid opaque payload hash and non-claim boundary
- WHEN Valence validates the row
- THEN validation MAY accept the row as recorded-only linkage evidence and MUST NOT infer dialect semantics or proof discharge.

### Requirement: Lineage validation fails closed on substitution or overclaim
r[valence.evidence_ir.proof_carrying_ir_lineage.validation] Valence MUST reject stale, substituted, role-promoted, or overclaiming proof-carrying IR lineage rows.

#### Scenario: Invalid lineage is rejected
r[valence.evidence_ir.proof_carrying_ir_lineage.negative]
- GIVEN a lineage row has a stale proof sidecar, mismatched source artifact, cross-artifact IR ref, unsupported role promotion, missing proof-obligation id, missing non-claims, or semantic-equivalence overclaim
- WHEN Valence validation runs
- THEN validation MUST fail with deterministic diagnostics and MUST NOT count the row as property or formal-proof evidence.

### Requirement: Valence core remains parser and checker agnostic
r[valence.evidence_ir.proof_carrying_ir_lineage.shell] Valence core MUST NOT parse tMIR formats, invoke solvers, check proofs, read files, render JSON output, or execute external commands for this profile.

#### Scenario: Producer owns IR parsing
- GIVEN a producer emits already-parsed proof-carrying IR lineage rows
- WHEN Valence validates them
- THEN Valence core MUST operate only on typed in-memory rows while producers or CLI shells own parsing, file I/O, checker execution, and presentation.

### Requirement: Lineage profile preserves Valence non-claims
r[valence.evidence_ir.proof_carrying_ir_lineage.final_validation] Valence MUST state that passing proof-carrying IR lineage validation proves only identity, linkage, role, hash, and non-claim conformance.

#### Scenario: Valid lineage does not prove compiler correctness
- GIVEN a proof-carrying IR lineage profile validates
- WHEN downstream stack evidence consumes it
- THEN the profile MUST NOT prove source-to-IR correctness, transform correctness, proof obligation discharge, solver soundness, compiler correctness, codegen correctness, machine-code correctness, semantic equivalence, or release eligibility.