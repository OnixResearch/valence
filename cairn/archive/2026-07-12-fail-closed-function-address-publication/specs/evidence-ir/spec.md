# Evidence IR Specification

## Purpose

Ensures rejected function-address evidence cannot produce publication-shaped graph artifacts and successful multi-surface output remains coherently identity-bound.

## Requirements

### Requirement: Function-address publication is gated by validation
r[valence.evidence_ir.function_address_publication.validity_gate] Valence MUST derive publishable function-address receipt, graph, bundle, and graph-root material only from a passing typed validation result.

#### Scenario: Valid input produces publication material
r[valence.evidence_ir.function_address_publication.fixtures.positive]
- GIVEN function-address input passes every profile, role, digest, span, policy, edge, and non-claim check
- WHEN the pure publication planner runs
- THEN it MUST return a validated artifact and versioned graph publication bound to that artifact.

#### Scenario: Invalid input produces diagnostics only
r[valence.evidence_ir.function_address_publication.invalid_no_write]
- GIVEN function-address input contains a role promotion, overclaim, stale identity, malformed span, invalid edge, or other validation issue
- WHEN publication is requested
- THEN Valence MUST return diagnostics without publishable receipt, graph, bundle, or graph-root material and MUST NOT create, truncate, or replace requested destinations.

### Requirement: Function-address graph inputs are unambiguous
r[valence.evidence_ir.function_address_publication.graph_validation] Valence MUST reject empty graph locators and exact duplicate dependency edges before constructing the canonical graph report.

#### Scenario: Duplicate edge fails closed
r[valence.evidence_ir.function_address_publication.fixtures.negative]
- GIVEN two dependency rows have the same typed source, target, and edge kind
- WHEN graph validation runs
- THEN Valence MUST report a duplicate-edge diagnostic rather than allowing counts or identity to depend on repetition.

#### Scenario: Recursive self-edge remains bounded
- GIVEN a validated recursive function has a self-dependency edge
- WHEN graph validation runs
- THEN Valence MAY preserve the typed self-edge and MUST NOT infer termination, correctness, or call-graph semantics from it.

### Requirement: Canonical graph reports bind validation context
r[valence.evidence_ir.function_address_publication.canonical_graph] Valence MUST publish a versioned graph report containing the current policy identity, validated artifact identity, complete typed rows, deterministic summary counts, a passing validity marker, required non-claims, and a BLAKE3 report identity over named self-hash-excluded canonical material.

#### Scenario: Equivalent edge order has one identity
- GIVEN equivalent validated edge sets arrive in different input orders
- WHEN canonical graph reports are built
- THEN rows MUST sort by source, target, and edge kind and the report bytes and BLAKE3 identity MUST match.

#### Scenario: Bare row array is not canonical readiness evidence
- GIVEN a legacy JSON array contains graph rows without policy, artifact, verdict, or non-claim bindings
- WHEN canonical publication eligibility is evaluated
- THEN Valence MUST NOT present that array as the versioned readiness graph report.

### Requirement: Canonical bundle replacement is atomic
r[valence.evidence_ir.function_address_publication.atomic_bundle] Valence MUST support a single versioned function-address publication bundle whose receipt and graph report are written and replaced as one same-directory atomic artifact through the declared output capability.

#### Scenario: Pre-commit failure preserves prior bundle
- GIVEN a valid prior bundle exists and staging, flushing, or destination validation fails for its replacement
- WHEN bundle publication runs
- THEN Valence MUST report the I/O failure and MUST leave the prior bundle unchanged.

### Requirement: Split outputs are explicit compatibility surfaces
r[valence.evidence_ir.function_address_publication.compatibility_outputs] Valence MAY emit separate compatibility receipt and graph files only when each carries the same publication, policy, artifact, and passing-verdict identities and the interface does not claim portable cross-file crash atomicity.

#### Scenario: Compatibility outputs share publication identity
- GIVEN valid input requests both compatibility outputs
- WHEN staging succeeds
- THEN both files MUST carry the same publication identity and each file MUST be individually atomically replaced.

#### Scenario: Staging failure replaces neither compatibility file
- GIVEN one compatibility destination cannot be staged or validated
- WHEN split publication runs
- THEN Valence MUST fail before replacing either existing destination.

### Requirement: Publication boundaries are documented
r[valence.evidence_ir.function_address_publication.docs] Valence documentation MUST identify the canonical bundle, versioned graph contract, invalid diagnostic behavior, compatibility-output limits, and identity/linkage-only non-claim.

#### Scenario: Consumer does not treat failed graph as readiness input
- GIVEN a Mantle or Cairn consumer reads the function-address publication guidance
- WHEN upstream validation fails
- THEN the guidance MUST make clear that no publication-shaped graph artifact is eligible for binding or readiness evaluation.

### Requirement: Function-address publication remediation is verified
r[valence.evidence_ir.function_address_publication.final_validation] The change MUST include positive, negative, order-variance, overwrite-preservation, and failure-injection evidence for core and CLI publication paths.

#### Scenario: Reproduced invalid-output flaw is closed
- GIVEN the role-promoted overclaim input and pre-existing sentinel output files
- WHEN focused tests and repository validation run
- THEN validation MUST fail, no graph root or graph file MUST be published, and every sentinel file MUST remain unchanged.
