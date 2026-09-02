# Evidence IR Specification

## Purpose

Adds an iddqd-backed internal source index for evidence-codebase ingestion while preserving Evidence IR index and query JSON compatibility.

## Requirements

### Requirement: Normalized source identity index
r[valence.evidence_ir.iddqd_evidence_source_index.source_index] Valence MUST index normalized evidence sources by locator ids derived from the source values rather than by duplicated temporary map keys.

#### Scenario: Unique source provenance lookup passes
r[valence.evidence_ir.iddqd_evidence_source_index.fixtures.positive]
- GIVEN evidence artifacts normalize to unique source locators and provenance records reference supplied artifact bytes by locator and BLAKE3 digest
- WHEN Valence builds the evidence-codebase index
- THEN provenance lookup MUST compare against the indexed source identity and preserve stable public index output.

#### Scenario: Duplicate locator conflict fails closed
r[valence.evidence_ir.iddqd_evidence_source_index.fixtures.negative]
- GIVEN evidence artifacts normalize to conflicting duplicate locators, stale provenance hashes, missing referenced artifacts, malformed JSON/JSONL, or overbroad correctness claims
- WHEN Valence builds the evidence-codebase index
- THEN duplicate locator conflicts MUST produce deterministic issues or caveats according to build mode and MUST NOT silently overwrite a prior source identity.

### Requirement: Duplicate source policy
r[valence.evidence_ir.iddqd_evidence_source_index.duplicate_policy] Valence MUST make duplicate normalized source locator behavior explicit for strict and advisory index builds.

#### Scenario: Advisory exact duplicate remains bounded
r[valence.evidence_ir.iddqd_evidence_source_index.schema_compatibility]
- GIVEN advisory mode receives exact duplicate source artifacts with the same normalized locator and BLAKE3 digest
- WHEN sources are normalized and indexed
- THEN Valence MAY collapse the duplicate while keeping the public Evidence IR index, query report, and non-claim schemas stable.

### Requirement: Final validation evidence
r[valence.evidence_ir.iddqd_evidence_source_index.final_validation] The change MUST include positive and negative evidence-codebase-index coverage plus focused Valence validation evidence before archive.

#### Scenario: Source-index fixtures cover conflict handling
r[valence.evidence_ir.iddqd_evidence_source_index.final_validation.fixtures]
- GIVEN valid and invalid evidence source fixtures
- WHEN focused evidence-codebase-index tests and Cairn gates run
- THEN unique source identities MUST pass and conflicting duplicate locators or stale provenance identities MUST fail closed.
