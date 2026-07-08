## ADDED Requirements

### Requirement: Canonical stack provenance identity
r[valence.stack_provenance_adapter.canonical_chain_identity] Valence stack provenance artifacts MUST use canonical IDs derived from deterministic content and BLAKE3 hashes for requirements, spans, receipts, proof artifacts, sidecars, release bundles, and lifecycle gates.

#### Scenario: Canonical IDs are deterministic
r[valence.stack_provenance_adapter.canonical_chain_identity.deterministic]
- GIVEN equivalent stack provenance artifacts with fields in any supported input order
- WHEN Valence canonicalizes and hashes them
- THEN the canonical IDs and BLAKE3 hashes MUST be identical.

#### Scenario: Paths are metadata only
r[valence.stack_provenance_adapter.canonical_chain_identity.paths_metadata]
- GIVEN two artifacts with the same canonical content and different display paths
- WHEN Valence computes canonical identity
- THEN the stable ID MUST be derived from canonical content rather than the display path.

#### Scenario: Stale content fails closed
r[valence.stack_provenance_adapter.canonical_chain_identity.stale]
- GIVEN an artifact whose stated canonical hash no longer matches its content
- WHEN Valence verifies the chain
- THEN verification MUST fail with a deterministic stale-content diagnostic.

### Requirement: Typed stack provenance edges
r[valence.stack_provenance_adapter.typed_edges] Valence stack provenance chains MUST use typed edges that explain why artifacts are linked, including local proof, source-shape check, provenance export, release bundle inclusion, lifecycle gate satisfaction, and manual-review input.

#### Scenario: Supported edge kinds pass
r[valence.stack_provenance_adapter.typed_edges.supported]
- GIVEN a chain containing only supported edge kinds with matching source and target artifacts
- WHEN Valence verifies the chain
- THEN the report MUST preserve each edge kind, producer role, consumer role, and claim scope.

#### Scenario: Unsupported edge kind fails closed
r[valence.stack_provenance_adapter.typed_edges.unsupported]
- GIVEN a chain containing an edge kind outside the supported vocabulary
- WHEN Valence verifies the chain
- THEN verification MUST fail with a deterministic unsupported-edge diagnostic.

#### Scenario: Missing parent fails closed
r[valence.stack_provenance_adapter.typed_edges.missing_parent]
- GIVEN an edge references a parent artifact absent from the canonical artifact set
- WHEN Valence verifies the chain
- THEN verification MUST fail with a deterministic missing-parent diagnostic.

### Requirement: Machine-checked non-claim policy
r[valence.stack_provenance_adapter.non_claim_policy] Valence MUST enforce stack provenance non-claim boundaries as machine-checked policy over producer roles, verification roles, edge kinds, evidence tiers, and claim scopes.

#### Scenario: Manual-review evidence is not promoted
r[valence.stack_provenance_adapter.non_claim_policy.manual_review]
- GIVEN a Trellis or Octet artifact marked manual-review or reference input
- WHEN a chain attempts to use it as downstream formal proof without a downstream verifier receipt
- THEN Valence verification MUST fail with a deterministic tier-promotion diagnostic.

#### Scenario: Mantle opaque sidecar remains opaque
r[valence.stack_provenance_adapter.non_claim_policy.mantle_opaque]
- GIVEN Mantle records a stack provenance sidecar as opaque external evidence
- WHEN Valence verifies the chain
- THEN the report MUST treat Mantle's role as bundle-local path, digest, role, schema, and non-claim recording rather than semantic verification.

#### Scenario: Cairn gate does not prove source correctness
r[valence.stack_provenance_adapter.non_claim_policy.cairn_gate]
- GIVEN a Cairn lifecycle gate receipt is linked into the provenance chain
- WHEN Valence verifies the chain
- THEN the report MUST state that the gate proves lifecycle policy conformance only and not source-code behavioral correctness.

#### Scenario: Weakened non-claim fails closed
r[valence.stack_provenance_adapter.non_claim_policy.weakened]
- GIVEN an artifact or edge omits or weakens required non-claims
- WHEN Valence verifies the chain
- THEN verification MUST fail with a deterministic non-claim diagnostic.

### Requirement: Stack provenance graph report
r[valence.stack_provenance_adapter.graph_report] Valence MUST provide a deterministic provenance graph report that lists artifacts, typed edges, trust boundaries, manual-review nodes, unresolved links, diagnostics, and the bounded supported claim.

#### Scenario: JSON report is machine readable
r[valence.stack_provenance_adapter.graph_report.json]
- GIVEN a verified or failed stack provenance chain
- WHEN Valence emits the JSON graph report
- THEN the report MUST include deterministic artifact, edge, boundary, manual node, unresolved link, diagnostic, and report hash fields.

#### Scenario: Markdown report is operator readable
r[valence.stack_provenance_adapter.graph_report.markdown]
- GIVEN the same graph report data
- WHEN Valence emits the Markdown projection
- THEN the report MUST summarize the same artifacts, edges, trust boundaries, manual nodes, failures, and non-claims for operator review.

### Requirement: Broken-chain fixture coverage
r[valence.stack_provenance_adapter.broken_chain_fixtures] Valence MUST include positive and negative fixtures for canonical stack provenance chain validation.

#### Scenario: Complete chain fixture passes
r[valence.stack_provenance_adapter.broken_chain_fixtures.positive]
- GIVEN a complete fixture linking Octet export facts, Valence sidecar, Mantle bundle metadata, and Cairn lifecycle gate evidence
- WHEN Valence verifies the chain
- THEN verification MUST pass and report identity-and-linkage-only success.

#### Scenario: Broken chain fixtures fail closed
r[valence.stack_provenance_adapter.broken_chain_fixtures.negative]
- GIVEN fixtures with missing parents, digest mismatches, stale policy, unsupported edge kinds, manual-review tier promotion, weakened non-claims, or span/source content drift
- WHEN Valence verifies the chain
- THEN each fixture MUST fail closed with deterministic diagnostics.
