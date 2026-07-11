# Evidence IR Specification

## Purpose

Extend Evidence IR with a bounded ast-grep finding profile that preserves source-shape evidence spans and non-claims.

## Requirements

### Requirement: Ast-grep Evidence IR profile
r[valence.ast_grep_evidence_spans.profile] Valence MUST define an ast-grep Evidence IR adapter profile for source-shape findings, scan summaries, and optional rewrite candidates.

#### Scenario: Finding profile is selected
- GIVEN a producer submits an ast-grep finding artifact
- WHEN Valence validates the artifact profile
- THEN Valence MUST treat it as syntax-aware source-shape evidence and MUST require explicit non-claims.

### Requirement: Producer and bundle identity
r[valence.ast_grep_evidence_spans.identity] Valence MUST validate ast-grep tool identity, rule id, rule bundle BLAKE3 identity, scan scope identity, and finding identity for ast-grep Evidence IR artifacts.

#### Scenario: Bundle identity is stale
- GIVEN an ast-grep artifact names a rule bundle identity that does not match the declared bundle metadata
- WHEN Valence validates the artifact
- THEN validation MUST fail closed with deterministic identity diagnostics.

### Requirement: Source span mapping
r[valence.ast_grep_evidence_spans.spans] Valence MUST map ast-grep ranges into EvidenceSpan fields with source id, byte offsets, line range, column range, and optional labels.

#### Scenario: Finding carries a valid span
- GIVEN an ast-grep finding includes a file identity and valid byte, line, and column range
- WHEN Valence builds the Evidence IR artifact
- THEN the artifact MUST preserve that span without widening it to a whole-file claim.

### Requirement: Verification roles stay bounded
r[valence.ast_grep_evidence_spans.roles] Valence MUST permit ast-grep artifacts only as recorded-only or boundary evidence unless another checked profile explicitly narrows the role.

#### Scenario: Artifact requests property role
- GIVEN an ast-grep artifact requests a property/proof verification role
- WHEN Valence validates the artifact
- THEN validation MUST reject the role unless a separate checked profile authorizes that promotion.

### Requirement: Non-claim preservation
r[valence.ast_grep_evidence_spans.nonclaims] Valence MUST reject ast-grep artifacts that omit non-claims or claim semantic correctness, semantic equivalence, source correctness, proof discharge, or release eligibility.

#### Scenario: Rewrite candidate overclaims equivalence
- GIVEN an ast-grep rewrite candidate claims behavior preservation
- WHEN Valence validates the artifact
- THEN validation MUST fail closed and preserve diagnostics explaining the unsupported equivalence claim.

### Requirement: Positive and negative fixtures
r[valence.ast_grep_evidence_spans.fixtures] Valence MUST include positive and negative fixtures for ast-grep Evidence IR artifacts.

#### Scenario: Fixture suite runs
- GIVEN fixture validation is executed
- WHEN positive and negative ast-grep artifacts are evaluated
- THEN valid artifacts MUST pass and malformed, stale, unsupported-role, or overclaiming artifacts MUST fail deterministically.

### Requirement: Adapter validation evidence
r[valence.ast_grep_evidence_spans.validation] Valence MUST validate ast-grep adapter changes with adapter tests, malformed fixture tests, Cairn gates, and focused repository checks.

#### Scenario: Adapter change is reviewed
- GIVEN a change modifies ast-grep Evidence IR validation
- WHEN validation evidence is assembled
- THEN the evidence MUST include positive fixtures, negative fixtures, lifecycle gate results, and focused Valence test output.
