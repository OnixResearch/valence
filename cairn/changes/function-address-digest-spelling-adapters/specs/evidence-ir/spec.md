# Evidence IR Specification

## Purpose

Normalize function-address digest spellings at explicit evidence boundaries without weakening typed digest domains.

## Requirements

### Requirement: Digest spelling adapters are explicit
r[valence.evidence_ir.function_address_digest_spelling.contract] Valence MUST define explicit function-address digest spelling adapters for supported boundary inputs such as Octet `b3:<hex>` and Kamacite bare BLAKE3 hex.

#### Scenario: Supported spelling normalizes
r[valence.evidence_ir.function_address_digest_spelling.positive]
- GIVEN a function-address evidence input uses a supported spelling at a declared adapter boundary
- WHEN Valence normalizes the digest
- THEN the normalized digest MUST preserve the BLAKE3 value and typed function-address digest domain.

### Requirement: Spelling normalization does not replace domain validation
r[valence.evidence_ir.function_address_digest_spelling.domain] Valence MUST validate digest domain separately from spelling normalization.

#### Scenario: Cross-domain substitution fails
r[valence.evidence_ir.function_address_digest_spelling.negative]
- GIVEN a syntactically valid BLAKE3 digest is supplied under the wrong function-address digest domain
- WHEN Valence validates the evidence input
- THEN validation MUST fail with a domain-substitution diagnostic.

### Requirement: Canonical spelling is documented
r[valence.evidence_ir.function_address_digest_spelling.docs] Valence MUST document the canonical digest spelling emitted in Valence function-address receipts and graph summaries.

#### Scenario: Stack smoke uses normalized digests
r[valence.evidence_ir.function_address_digest_spelling.validation]
- GIVEN Octet, Kamacite, Mantle, and Cairn exchange function-address evidence
- WHEN the stack smoke runs
- THEN Valence MUST normalize allowed spellings deterministically and reject malformed or unsupported spelling variants.
