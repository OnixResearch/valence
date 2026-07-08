# Evidence IR Specification

## Purpose

Adds a Valence compatibility adapter for Cairn content-addressed Rust evidence receipts.

## Requirements

### Requirement: Cairn content-addressed compatibility manifest
r[valence.evidence_ir.cairn_content_addressed_compat.manifest] Valence MUST define a compatibility manifest for supported Cairn content-addressed Rust evidence receipt families, including schema, hash field, hash-material exclusion profile, Valence artifact kind, verification role, and required non-claim boundary.

#### Scenario: Manifest row admits a supported family
r[valence.evidence_ir.cairn_content_addressed_compat.manifest.supported]
- GIVEN a Cairn content-addressed receipt family is supported by Valence compatibility validation
- WHEN the manifest row is inspected
- THEN it MUST name the Cairn schema, the hash field, the hash-material profile, the Valence artifact kind, the verification role, and the bounded non-claim text.

#### Scenario: Unsupported family fails closed
r[valence.evidence_ir.cairn_content_addressed_compat.manifest.unsupported]
- GIVEN a Cairn receipt family has no compatibility manifest row
- WHEN Valence compatibility validation runs
- THEN validation MUST fail closed with an unsupported-family diagnostic.

### Requirement: Compatible receipts map to Evidence IR identity
r[valence.evidence_ir.cairn_content_addressed_compat.validation] Valence MUST map compatible Cairn content-addressed receipts into Evidence IR artifact identity and linkage rows without changing Cairn's receipt ownership.

#### Scenario: Function Object source-token receipt maps
r[valence.evidence_ir.cairn_content_addressed_compat.fixtures.positive]
- GIVEN a compatible Cairn source-token Function Object receipt has a valid BLAKE3 object hash, required caveats, and no correctness claim
- WHEN Valence compatibility validation runs
- THEN it MUST report a Valence Evidence IR identity/linkage row for that receipt.

#### Scenario: Invalid receipt fails closed
r[valence.evidence_ir.cairn_content_addressed_compat.fixtures.negative]
- GIVEN a Cairn receipt has a stale hash, unsupported schema, missing required nested field, ambient entropy in hash material, unknown pointer target, or overbroad identity/correctness claim
- WHEN Valence compatibility validation runs
- THEN validation MUST fail with deterministic diagnostics naming the failed compatibility rule.

### Requirement: Migration exceptions are explicit
r[valence.evidence_ir.cairn_content_addressed_compat.migration_exceptions] Valence MUST report legacy or divergent Cairn receipt shapes through explicit migration exceptions rather than silently normalizing them as canonical Valence DTOs.

#### Scenario: Divergent receipt is not silently upgraded
r[valence.evidence_ir.cairn_content_addressed_compat.migration_exceptions.visible]
- GIVEN a Cairn receipt uses a legacy shape that is useful as manual-review evidence but does not match Valence's current canonical DTO
- WHEN compatibility validation runs
- THEN the report MUST mark the row as a migration exception and MUST NOT claim byte-identical DTO compatibility.

### Requirement: Compatibility boundary remains bounded
r[valence.evidence_ir.cairn_content_addressed_compat.docs] The Cairn compatibility adapter MUST prove identity/linkage compatibility only.

#### Scenario: Boundary is visible
r[valence.evidence_ir.cairn_content_addressed_compat.final_validation]
- GIVEN a Cairn compatibility report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that Valence does not prove Cairn lifecycle readiness, source-code correctness, verifier soundness, semantic equivalence, release eligibility, or whole-program safety.
