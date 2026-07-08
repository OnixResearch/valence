# Provenance Specification

## Purpose

Adds canonical `valence-core` ownership and drift contracts for the provenance capability.

## Requirements

### Requirement: Canonical Valence core manifest
r[valence.provenance.canonical_core_manifest] Valence MUST publish a canonical manifest for Valence-owned `valence-core` modules, schema families, fixture roots, compatibility dispositions, and required non-claim boundaries.

#### Scenario: Canonical rows pass
r[valence.provenance.canonical_core_manifest.positive]
- GIVEN a manifest row names a Valence-owned module or schema family with a supported compatibility disposition and required non-claim text
- WHEN the manifest validator evaluates the row
- THEN the row MUST pass and preserve the canonical Valence path and schema identity.

#### Scenario: Unsupported rows fail closed
r[valence.provenance.canonical_core_manifest.negative]
- GIVEN a manifest row omits the canonical Valence path, weakens required non-claims, or uses an unsupported compatibility disposition
- WHEN the manifest validator evaluates the row
- THEN validation MUST fail with deterministic diagnostics naming the invalid field.

### Requirement: Canonical core drift gate
r[valence.provenance.canonical_core_drift] Valence MUST provide a deterministic drift gate that compares declared legacy or consumer-visible `valence-core` rows against the canonical Valence-owned module/schema identity.

#### Scenario: Matching legacy copy is accepted
r[valence.provenance.canonical_core_drift.match]
- GIVEN a legacy Octet-hosted row declares byte or schema identity that matches the canonical Valence manifest
- WHEN the drift gate runs
- THEN it MUST report the row as compatible without changing the evidence tier.

#### Scenario: Stale legacy copy is rejected
r[valence.provenance.canonical_core_drift.stale]
- GIVEN a legacy Octet-hosted row diverges from canonical Valence identity without a bounded migration exception
- WHEN the drift gate runs
- THEN it MUST fail closed with a stale-copy diagnostic.

### Requirement: Dependency direction remains explicit
r[valence.provenance.canonical_core_dependency_direction] Valence documentation and manifests MUST state that Valence owns Evidence IR semantics while Octet owns Octet-specific scan, lint, proof-rail, and publication orchestration.

#### Scenario: Consumer guidance points to Valence
r[valence.provenance.canonical_core_dependency_direction.consumer]
- GIVEN a downstream consumer wants `valence-core` semantics
- WHEN they read Valence or Octet integration guidance
- THEN the guidance MUST point at standalone Valence as the canonical semantic dependency.

#### Scenario: Cutover keeps non-claims visible
r[valence.provenance.canonical_core_dependency_direction.non_claims]
- GIVEN a cutover report or compatibility receipt is emitted
- WHEN a reviewer inspects the supported claim
- THEN it MUST state that the cutover proves ownership and identity compatibility only, not behavioral correctness, semantic equivalence, release eligibility, verifier soundness, or requirement satisfaction.

### Requirement: Validation evidence
r[valence.provenance.canonical_core_validation] The cutover change MUST include focused positive and negative tests plus Cairn validation evidence before archive.

#### Scenario: Focused validation covers pass and fail paths
r[valence.provenance.canonical_core_validation.focused]
- GIVEN canonical, legacy-compatible, stale, and weakened-boundary fixtures
- WHEN focused validation runs
- THEN pass fixtures MUST pass, fail fixtures MUST fail closed, and the receipt MUST identify the manifest and input hashes.
