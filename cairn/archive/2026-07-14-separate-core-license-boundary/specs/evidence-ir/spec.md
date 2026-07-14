# Valence Evidence IR License Boundary Specification

## Purpose

Keep the canonical Evidence IR core available under file-level copyleft while retaining AGPL reciprocity for executable Valence surfaces.

## Requirements

### Requirement: Core and executable package licenses are separated

r[valence.evidence_ir.license_boundary.package_split] `valence-core` MUST declare `MPL-2.0`, while `valence-cli` and repository-owned executable pilot packages MUST declare or inherit `AGPL-3.0-or-later`.

#### Scenario: An evidence producer embeds the core

- GIVEN an external producer depends only on `valence-core`
- WHEN Cargo metadata is inspected
- THEN the core MUST report MPL-2.0 without inheriting the CLI's AGPL expression.

### Requirement: Complete license artifacts accompany source

r[valence.evidence_ir.license_boundary.artifacts] Valence MUST ship complete MPL-2.0 and AGPL-3.0-or-later texts and MUST map repository-owned packages to their selected terms.

#### Scenario: A source distribution is inspected offline

- GIVEN a recipient receives the core, CLI, and pilots
- WHEN package licensing is reviewed
- THEN both complete texts and an unambiguous package map MUST be present.

### Requirement: License metadata does not rewrite Evidence IR identity

r[valence.evidence_ir.license_boundary.documentation] Valence documentation MUST state that package licensing is outside canonical Evidence IR identity, that prior grants remain valid, and that third-party terms and evidence non-claims remain intact.

#### Scenario: Core package metadata changes

- GIVEN `valence-core` moves from AGPL to MPL without canonical evidence-byte changes
- WHEN an existing evidence ID is interpreted
- THEN Valence MUST NOT claim that package metadata alone changes the evidence identity.

### Requirement: Package mapping validation fails closed

r[valence.evidence_ir.license_boundary.validation] Focused validation MUST accept the intended core/CLI/pilot mapping and MUST reject a core that remains AGPL, a CLI that becomes permissive unintentionally, or missing complete license artifacts.

#### Scenario: Core and CLI mappings are reversed

- GIVEN malformed package metadata assigns AGPL to the core and MPL to the CLI
- WHEN license-boundary validation runs
- THEN validation MUST fail with package-specific diagnostics.

### Requirement: Focused verification proves distribution consistency

r[valence.evidence_ir.license_boundary.final_validation] The change MUST verify Cargo metadata, complete license artifacts, current boundary documentation, and Cairn lifecycle conformance.

#### Scenario: The final repository state is checked

- GIVEN package declarations, texts, and documentation agree
- WHEN focused checks and Cairn gates run
- THEN the boundary MUST pass without making stronger behavioral or release-readiness claims.
