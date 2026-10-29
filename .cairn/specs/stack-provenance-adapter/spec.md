# Stack Provenance Adapter Specification

## Purpose

Defines the `stack-provenance-adapter` capability.

## Requirements

### Requirement: Stack provenance adapter sidecar
r[valence.stack_provenance_adapter] Valence MUST provide a stack provenance adapter sidecar that records a validated Valence provenance chain plus the source artifact identities used to assemble that chain.

#### Scenario: Adapter sidecar carries chain and source identities
r[valence.stack_provenance_adapter.sidecar]
- GIVEN normalized Octet, Trellis, Cairn, and Mantle evidence inputs
- WHEN Valence builds a stack provenance sidecar
- THEN the sidecar MUST include a `valence.provenance-chain.v1` chain, source artifact identities, claim scope, diagnostics, and explicit non-claims.

#### Scenario: Adapter sidecar is deterministic
r[valence.stack_provenance_adapter.deterministic]
- GIVEN equivalent normalized evidence inputs in any supported input order
- WHEN Valence builds a stack provenance sidecar
- THEN the sidecar SHOULD serialize deterministically with stable ordering for links, artifact identities, and diagnostics.

### Requirement: Mantle external evidence verification
r[valence.stack_provenance_adapter.mantle_external_evidence] Valence MUST verify stack provenance sidecars included in Mantle release bundles through Mantle's generic external-evidence role and digest metadata, without requiring Mantle to interpret Valence semantics.

#### Scenario: Bundled sidecar digest matches
r[valence.stack_provenance_adapter.mantle_external_evidence.digest]
- GIVEN a Mantle release bundle contains an external evidence entry with role `stack-provenance-trace`
- WHEN Valence verifies the stack provenance sidecar
- THEN Valence MUST confirm the bundled sidecar bytes match the BLAKE3 digest recorded by Mantle.

#### Scenario: Missing Mantle role fails
r[valence.stack_provenance_adapter.mantle_external_evidence.missing_role]
- GIVEN a Mantle release bundle has no external evidence entry for the requested stack provenance role
- WHEN Valence verifies stack provenance for that release
- THEN verification MUST fail with a diagnostic naming the missing role.

### Requirement: Stack traceability validation
r[valence.stack_provenance_adapter.validation] Valence stack provenance verification MUST fail closed when chain identity, artifact identity, binary binding, parent linkage, or evidence-tier boundaries are missing or inconsistent.

#### Scenario: Binary hash mismatch fails
r[valence.stack_provenance_adapter.validation.binary_mismatch]
- GIVEN a provenance link names a binary hash not present in the Mantle release binary artifacts or explicitly scoped release artifacts
- WHEN Valence verifies the stack provenance sidecar
- THEN verification MUST fail with a diagnostic naming the binary hash mismatch.

#### Scenario: Parent artifact missing fails
r[valence.stack_provenance_adapter.validation.missing_parent]
- GIVEN a provenance link or proof-card-derived record names a parent artifact identity that is absent from the adapter inputs
- WHEN Valence verifies the stack provenance sidecar
- THEN verification MUST fail with a diagnostic naming the missing parent artifact.

#### Scenario: Manual-review evidence is not promoted
r[valence.stack_provenance_adapter.validation.no_tier_promotion]
- GIVEN imported Trellis or Octet evidence is marked `manual_review_required`
- WHEN Valence verifies a sidecar that claims the evidence as `formal_proof`
- THEN verification MUST fail with a diagnostic naming the evidence-tier promotion.

#### Scenario: Boundary weakening fails
r[valence.stack_provenance_adapter.validation.boundary]
- GIVEN a stack provenance sidecar weakens the required identity-and-linkage-only non-claim boundary
- WHEN Valence verifies the sidecar
- THEN verification MUST fail with a diagnostic naming the weakened boundary.

### Requirement: Stack provenance adapter CLI
r[valence.stack_provenance_adapter.cli] Valence SHOULD provide CLI commands for building stack provenance sidecars from local artifacts and verifying sidecars bundled in Mantle release evidence by external-evidence role.

#### Scenario: Build command writes sidecar
r[valence.stack_provenance_adapter.cli.build]
- GIVEN local Octet, Trellis, Cairn, and Mantle evidence inputs
- WHEN the operator runs the stack provenance build command with an output path
- THEN Valence SHOULD write the canonical stack provenance sidecar or fail with deterministic diagnostics.

#### Scenario: Verify command checks Mantle-bundled sidecar
r[valence.stack_provenance_adapter.cli.verify]
- GIVEN a Mantle release bundle contains a stack provenance external evidence sidecar
- WHEN the operator runs the stack provenance verify command for that role
- THEN Valence SHOULD verify the sidecar digest, schema, boundary, and stack traceability links.

### Requirement: Adapter boundary documentation
r[valence.stack_provenance_adapter.boundary] Valence MUST document that stack provenance proves identity and linkage only and that Mantle treats the sidecar as opaque external evidence.

#### Scenario: Boundary is visible to release operators
r[valence.stack_provenance_adapter.boundary.operator]
- GIVEN a release operator inspects stack provenance adapter documentation or verification output
- WHEN the supported claim is described
- THEN the output MUST state that the adapter does not prove behavioral correctness, semantic equivalence, whole-program safety, verifier soundness, or release eligibility by itself.

### Requirement: Built binary provenance thread
r[valence.stack_provenance_adapter.built_binary_thread] Valence MUST provide a verification path that accepts a built binary, an Octet Valence provenance export, a Valence stack sidecar, and Mantle release evidence metadata, then reports success only when the measured binary bytes are linked through the sidecar and release metadata.

#### Scenario: Measured binary digest is bound to the sidecar
r[valence.stack_provenance_adapter.built_binary_thread.binary_digest]
- GIVEN a local built binary and a candidate Valence stack provenance sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST compute the BLAKE3 digest of the binary bytes and require it to match the sidecar `binary_hash` and every provenance link `binary_hash`.

#### Scenario: Mantle release binary identity is required
r[valence.stack_provenance_adapter.built_binary_thread.mantle_binary_identity]
- GIVEN a Mantle release manifest or release evidence manifest view and a measured binary digest
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST require a Mantle release binary identity or explicitly scoped release artifact identity that matches the measured digest.

#### Scenario: Octet export remains verified evidence facts
r[valence.stack_provenance_adapter.built_binary_thread.octet_export]
- GIVEN an Octet Valence provenance export used to build or verify the sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN Valence MUST require the export to have valid verification status, no diagnostics, matching parent artifact identities, and evidence tiers that are not promoted beyond the Octet export.

#### Scenario: Full thread reaches the built binary
r[valence.stack_provenance_adapter.built_binary_thread.e2e]
- GIVEN a built binary, a verified Octet provenance export, a Valence sidecar, and Mantle external evidence metadata for that sidecar
- WHEN Valence verifies the built-binary provenance thread
- THEN the report MUST show source/provenance/artifact/binary linkage through the measured binary digest without claiming behavioral correctness.

#### Scenario: Stale binary bytes fail closed
r[valence.stack_provenance_adapter.built_binary_thread.stale_binary]
- GIVEN a valid sidecar and Mantle manifest for one binary digest
- WHEN the operator verifies different binary bytes against that sidecar and manifest
- THEN verification MUST fail with a diagnostic naming the stale or mismatched binary digest.

#### Scenario: Missing Mantle binary identity fails closed
r[valence.stack_provenance_adapter.built_binary_thread.missing_mantle_binary]
- GIVEN a sidecar whose digest matches Mantle external evidence metadata but no Mantle binary identity matches the measured binary
- WHEN Valence verifies the built-binary provenance thread
- THEN verification MUST fail with a diagnostic naming the missing Mantle binary identity.

#### Scenario: Sidecar digest remains mandatory
r[valence.stack_provenance_adapter.built_binary_thread.sidecar_digest]
- GIVEN a built binary and sidecar whose binary hash matches the measured bytes
- WHEN the sidecar bytes do not match the BLAKE3 digest recorded by Mantle external evidence metadata
- THEN verification MUST fail with a diagnostic naming the sidecar digest mismatch.

#### Scenario: Built-binary thread does not broaden the claim
r[valence.stack_provenance_adapter.built_binary_thread.non_claims]
- GIVEN a successful built-binary thread verification report
- WHEN an operator reads the supported claim
- THEN the report MUST state that the check proves measured-byte identity and linkage only and does not prove behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, verifier soundness, or whole-program safety.

### Requirement: Built binary provenance thread CLI
r[valence.stack_provenance_adapter.built_binary_thread.cli] Valence SHOULD expose a CLI command for verifying a built binary against the Octet export, Valence sidecar, and Mantle release metadata.

#### Scenario: CLI verifies a built binary thread
r[valence.stack_provenance_adapter.built_binary_thread.cli.verify]
- GIVEN paths to a built binary, Octet Valence provenance export, Valence stack sidecar, and Mantle release metadata
- WHEN the operator runs `valence stack-provenance verify-built-binary`
- THEN the command SHOULD run the pure verifier and exit successfully only when all binary, sidecar, export, and release metadata bindings match.

#### Scenario: CLI emits deterministic diagnostics
r[valence.stack_provenance_adapter.built_binary_thread.cli.diagnostics]
- GIVEN stale binary bytes, missing Mantle binary identity, sidecar digest mismatch, Octet export mismatch, or evidence-tier promotion
- WHEN the operator runs `valence stack-provenance verify-built-binary`
- THEN the command SHOULD emit stable diagnostics suitable for release automation and human review.

### Requirement: Nickel contract metadata hardening
r[valence.stack_provenance_adapter.nickel_contracts] Valence MUST provide Nickel contracts that validate static metadata shapes for the built-binary provenance thread before Rust performs measured-byte verification.

#### Scenario: Contracts cover stack provenance metadata inputs
r[valence.stack_provenance_adapter.nickel_contracts.metadata_inputs]
- GIVEN an Octet Valence provenance export, a Valence stack sidecar, and a Mantle release manifest view
- WHEN the metadata is evaluated against the Nickel contracts
- THEN the contracts MUST validate required object shapes, fixed schema strings, required role names, claim scope, BLAKE3 digest syntax, and required non-claims.

#### Scenario: Static invariants are enforced
r[valence.stack_provenance_adapter.nickel_contracts.static_invariants]
- GIVEN metadata with a wrong schema string, malformed BLAKE3 digest, missing `stack-provenance-trace` role, missing identity-and-linkage non-claim, or manual-review evidence promoted to `formal_proof`
- WHEN the metadata is evaluated against the Nickel contracts
- THEN contract evaluation MUST fail before Rust built-binary verification is treated as eligible to run.

#### Scenario: Built-binary thread input envelope is contracted
r[valence.stack_provenance_adapter.nickel_contracts.thread_input]
- GIVEN a built-binary thread input envelope containing Octet export metadata, sidecar metadata, Mantle release metadata, and binary artifact locator metadata
- WHEN the envelope is evaluated against the Nickel contract
- THEN the contract MUST validate static metadata and MUST NOT claim to measure local binary bytes.

#### Scenario: Contracts include positive and negative fixtures
r[valence.stack_provenance_adapter.nickel_contracts.fixtures]
- GIVEN the checked-in Nickel contracts and fixture data
- WHEN fixture validation runs
- THEN valid fixtures MUST pass and invalid fixtures for schema, digest, role, non-claim, tier-promotion, parent-linkage, and overclaim failures MUST fail deterministically.

#### Scenario: Contract gate is deterministic
r[valence.stack_provenance_adapter.nickel_contracts.gate]
- GIVEN a repository checkout with Nickel tooling available
- WHEN the contract validation command or test target runs
- THEN it SHOULD evaluate all contract fixtures without network access and report stable diagnostics.

#### Scenario: Runtime boundary stays in Rust
r[valence.stack_provenance_adapter.nickel_contracts.rust_boundary]
- GIVEN metadata has passed Nickel contract validation
- WHEN Valence verifies the built-binary provenance thread
- THEN Rust MUST remain responsible for reading binary bytes, computing BLAKE3, and comparing measured identity across the sidecar, provenance links, Octet export, and Mantle release metadata.

#### Scenario: CLI preflight is bounded
r[valence.stack_provenance_adapter.nickel_contracts.cli]
- GIVEN an optional CLI or test-harness preflight for contract validation
- WHEN the preflight succeeds
- THEN the result MAY permit Rust verification to proceed but MUST NOT be reported as measured-byte identity proof by itself.

#### Scenario: Contract boundary is documented
r[valence.stack_provenance_adapter.nickel_contracts.docs]
- GIVEN an operator reads the built-binary thread documentation or contract validation output
- WHEN the supported claim is described
- THEN the output MUST state that Nickel proves metadata shape/static invariants only and does not prove measured-byte identity, behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, or whole-program safety.

### Requirement: Preserves canonical stack-provenance values
r[valence.stack_provenance_adapter.preserves_values] Valence MUST provide canonical Preserves value projections for stack-provenance metadata surfaces used by built-binary thread verification.

#### Scenario: Stack provenance metadata has canonical values
r[valence.stack_provenance_adapter.preserves_values.metadata_surfaces]
- GIVEN an Octet Valence provenance export, a Valence stack sidecar, a Mantle release manifest view, a built-binary thread input envelope, or a built-binary thread verification report
- WHEN Valence projects the metadata to Preserves
- THEN the projection MUST produce a deterministic canonical Preserves value for the metadata surface.

#### Scenario: Preserves content hashes are named distinctly
r[valence.stack_provenance_adapter.preserves_values.hashes]
- GIVEN a canonical Preserves value for stack-provenance metadata
- WHEN Valence computes metadata identity evidence
- THEN Valence MUST compute a BLAKE3 hash over canonical Preserves bytes and expose it as `preserves_content_hash` or an equally explicit metadata-value hash name that cannot be confused with raw binary or sidecar byte digests.

#### Scenario: Preserves does not replace raw byte identity
r[valence.stack_provenance_adapter.preserves_values.boundary]
- GIVEN stack-provenance metadata has a valid Preserves content hash
- WHEN Valence verifies the built-binary provenance thread
- THEN Rust MUST still measure raw binary bytes with BLAKE3 and Mantle external evidence MUST still bind the raw sidecar byte digest.

#### Scenario: Preserves corpus covers valid metadata
r[valence.stack_provenance_adapter.preserves_values.corpus]
- GIVEN checked-in positive corpus fixtures for Octet export metadata, sidecar metadata, Mantle release metadata, built-binary thread input metadata, and verification report metadata
- WHEN the Preserves corpus gate runs
- THEN every positive fixture MUST project to the expected canonical Preserves value and `preserves_content_hash`.

#### Scenario: Preserves corpus catches metadata drift
r[valence.stack_provenance_adapter.preserves_values.gate]
- GIVEN stale parent linkage, evidence-tier promotion, missing required non-claims, overclaim wording, or changed report diagnostics
- WHEN the Preserves corpus gate evaluates negative or drift fixtures
- THEN the gate MUST fail deterministically unless the expected corpus update is made intentionally.

#### Scenario: Verification reports can be compared semantically
r[valence.stack_provenance_adapter.preserves_values.report_diff]
- GIVEN two built-binary thread verification reports rendered through different JSON formatting or field ordering
- WHEN Valence compares their Preserves projections
- THEN equivalent report values SHOULD compare equal and changed diagnostics or identity fields SHOULD produce a deterministic Preserves diff or hash mismatch.

#### Scenario: Receipts can include Preserves metadata hashes
r[valence.stack_provenance_adapter.preserves_values.receipts]
- GIVEN release or test automation records stack-provenance metadata identity evidence
- WHEN the receipt includes Preserves evidence
- THEN the receipt SHOULD include `preserves_content_hash` values for metadata surfaces without replacing raw binary BLAKE3 digests, Mantle sidecar byte digests, or the bounded identity-and-linkage claim.

#### Scenario: Preserves boundary is documented
r[valence.stack_provenance_adapter.preserves_values.docs]
- GIVEN an operator reads stack-provenance documentation or Preserves corpus output
- WHEN the supported claim is described
- THEN the output MUST state that Preserves proves canonical metadata value identity only and does not prove measured-byte identity, behavioral correctness, semantic equivalence, requirement satisfaction, release eligibility, verifier soundness, or whole-program safety.

### Requirement: Typed stack-provenance boundary values
r[valence.stack_provenance_adapter.typed_boundary_values] Valence MUST parse high-risk stack-provenance metadata strings into typed internal boundary values before using them for verification decisions.

#### Scenario: Public DTO shape remains compatible
r[valence.stack_provenance_adapter.typed_boundary_values.dto_parse]
- GIVEN an Octet Valence provenance export, Valence stack sidecar, Mantle release manifest view, or built-binary thread report using the existing JSON string fields
- WHEN Valence parses the metadata for verification
- THEN Valence MUST convert schema, role, claim-scope, status, digest, and evidence-tier fields into typed internal values without changing the public JSON field names or serialized string values.

#### Scenario: Digest roles are explicit
r[valence.stack_provenance_adapter.typed_boundary_values.digest_roles]
- GIVEN raw binary digests, sidecar byte digests, artifact digests, and Preserves metadata hashes are all BLAKE3-shaped strings
- WHEN Valence validates or compares them
- THEN the Rust core MUST use typed wrappers or explicit role names so cross-role comparisons are intentional and reviewable.

#### Scenario: Validation uses typed values
r[valence.stack_provenance_adapter.typed_boundary_values.validation]
- GIVEN stack-provenance metadata has been parsed into typed boundary values
- WHEN Valence validates schema, external evidence role, claim scope, verification status, digest syntax, and evidence-tier policy
- THEN validation MUST fail closed with deterministic diagnostics for invalid typed values before treating the built-binary thread as valid.

#### Scenario: Evidence tiers remain conservative
r[valence.stack_provenance_adapter.typed_boundary_values.evidence_tiers]
- GIVEN an unknown evidence tier, a manual-review record, or imported Trellis/manual-review evidence promoted to `formal_proof`
- WHEN Valence parses and validates the tier
- THEN the typed model MUST preserve unknown source strings but MUST NOT count them as formal proof or allow manual-review evidence promotion.

#### Scenario: Preserves projections use typed-normalized metadata
r[valence.stack_provenance_adapter.typed_boundary_values.preserves]
- GIVEN a metadata surface with typed boundary values
- WHEN Valence projects it into canonical Preserves metadata values
- THEN the projection SHOULD use typed-normalized values where available and MUST keep corpus hash changes explicit through the Preserves corpus gate.

#### Scenario: Layer boundary is preserved
r[valence.stack_provenance_adapter.typed_boundary_values.boundary]
- GIVEN stack-provenance metadata passes typed parsing
- WHEN Valence verifies the built-binary provenance thread
- THEN Nickel MUST remain the static metadata contract layer, Preserves MUST remain the canonical metadata value layer, and Rust MUST remain responsible for raw binary measurement and digest comparison.

#### Scenario: Typed parsing has positive and negative tests
r[valence.stack_provenance_adapter.typed_boundary_values.tests]
- GIVEN typed stack-provenance boundary parsers and fixtures
- WHEN focused tests run
- THEN valid metadata MUST parse successfully and malformed digests, wrong schema/version strings, invalid claim scopes, missing or wrong Mantle roles, invalid statuses, tier-promotion attempts, and unknown evidence tiers MUST be covered by deterministic negative tests.

#### Scenario: Typed boundary is documented
r[valence.stack_provenance_adapter.typed_boundary_values.docs]
- GIVEN an operator or downstream maintainer reads the stack-provenance documentation
- WHEN typed boundary values are described
- THEN the documentation MUST state that typed values harden internal validation only, do not change public JSON contracts, and do not broaden the measured-byte identity-and-linkage-only claim.

### Requirement: Stack provenance cross-repo fixtures
r[valence.stack_provenance_adapter.cross_repo_fixtures] Valence MUST provide minimal checked-in fixtures that exercise the Octet export to Valence sidecar to Mantle manifest to measured binary hash provenance thread.

#### Scenario: Fast fixtures avoid live sibling repositories
r[valence.stack_provenance_adapter.cross_repo_fixtures.fast]
- GIVEN the fast stack-provenance fixture suite
- WHEN it runs in ordinary workspace tests
- THEN it MUST use frozen local fixtures and MUST NOT require live Octet, Mantle, Trellis, or Cairn repository state.

#### Scenario: Positive fixture verifies identity linkage
r[valence.stack_provenance_adapter.cross_repo_fixtures.positive]
- GIVEN matching Octet export, Valence sidecar, Mantle manifest, and measured binary byte fixtures
- WHEN Valence verifies the built-binary provenance thread
- THEN verification MUST accept the measured-byte identity and linkage.

#### Scenario: Negative fixtures fail closed
r[valence.stack_provenance_adapter.cross_repo_fixtures.negative]
- GIVEN fixtures with stale binary bytes, mismatched sidecar digest, missing Mantle role, parent artifact mismatch, tier promotion, or weakened boundaries
- WHEN Valence verifies the provenance thread
- THEN verification MUST fail closed with deterministic diagnostics.

#### Scenario: Deep sibling integration is separated
r[valence.stack_provenance_adapter.cross_repo_fixtures.deep]
- GIVEN maintainers run a deep or nightly integration suite
- WHEN live sibling repositories are used
- THEN that suite MUST be opt-in and MUST NOT be required for the fast fixture gate.

#### Scenario: Fixture claims remain bounded
r[valence.stack_provenance_adapter.cross_repo_fixtures.non_claims]
- GIVEN fixture reports describe stack-provenance success
- WHEN the supported claim is stated
- THEN the report MUST state that success proves measured-byte identity and linkage only, not correctness, release eligibility, semantic equivalence, verifier soundness, or requirement satisfaction.

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

### Requirement: Formal proof stack-chain fixture contract
r[valence.stack_provenance.formal_proof_chain_fixture.contract] Valence MUST define a frozen formal-proof stack-chain fixture contract that links Octet formal-proof receipts, proof cards, provenance exports, Valence sidecars, graph reports, and Cairn readiness input summaries with typed identities, verification roles, and non-claims.

#### Scenario: Fixture contract names every hop
r[valence.stack_provenance.formal_proof_chain_fixture.contract.hops]
- GIVEN a formal-proof stack-chain fixture is declared
- WHEN Valence validates the fixture contract
- THEN the fixture MUST name the Octet receipt, proof card, provenance record, Valence sidecar, graph report, Cairn readiness input summary, and required non-claim boundary.

### Requirement: Positive formal-proof chain fixture
r[valence.stack_provenance.formal_proof_chain_fixture.positive] Valence MUST accept a fixture where a scoped Octet-owned `formal_proof` receipt has matching proof-card, provenance, sidecar, graph, and Cairn readiness-input identities.

#### Scenario: Scoped property survives the chain
r[valence.stack_provenance.formal_proof_chain_fixture.positive.property]
- GIVEN a fixture contains one scoped Octet-owned passed formal-proof receipt with matching proof card, provenance record, sidecar, graph node, and Cairn readiness input summary
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST pass, preserve `verification_role = property` for that record, and report that only that property record counts as verified.

### Requirement: Negative formal-proof chain fixtures
r[valence.stack_provenance.formal_proof_chain_fixture.negative] Valence MUST reject fixtures with stale proof links, manual-review promotion, digest drift, unsupported edges, missing non-claims, or invalid verified counts.

#### Scenario: Manual review cannot become property
r[valence.stack_provenance.formal_proof_chain_fixture.negative.manual_review]
- GIVEN an imported Trellis or manual-review record is marked as `property` without an Octet-owned rerun receipt
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST fail with a deterministic tier-promotion diagnostic.

#### Scenario: Recorded-only evidence cannot be counted as verified
r[valence.stack_provenance.formal_proof_chain_fixture.negative.role_count]
- GIVEN a Cairn readiness input summary counts `recorded_only` or `boundary` evidence as verified
- WHEN Valence validates the stack-chain fixture
- THEN validation MUST fail with a deterministic verification-role-count diagnostic.

### Requirement: Stack-chain validation core
r[valence.stack_provenance.formal_proof_chain_fixture.validation] Valence MUST validate formal-proof stack-chain fixtures through a pure deterministic core over parsed in-memory fixture values.

#### Scenario: Core has no shell effects
r[valence.stack_provenance.formal_proof_chain_fixture.validation.core]
- GIVEN parsed fixture values and expected BLAKE3 identities
- WHEN the formal-proof stack-chain validator runs
- THEN it MUST return pass/fail diagnostics without reading files, invoking processes, inspecting environment, or contacting sibling repositories.

### Requirement: Stack-chain ownership boundary
r[valence.stack_provenance.formal_proof_chain_fixture.docs] Valence documentation MUST state that the fixture proves stack identity and role-preserving linkage only.

#### Scenario: Fixture non-claims are visible
r[valence.stack_provenance.formal_proof_chain_fixture.docs.non_claims]
- GIVEN a formal-proof stack-chain fixture report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that the fixture does not prove whole-program correctness, verifier soundness, semantic equivalence, release eligibility, live sibling freshness, or requirement satisfaction outside the declared fixture scope.

### Requirement: Formal-proof chain validation evidence
r[valence.stack_provenance.formal_proof_chain_fixture.final_validation] The change MUST include positive and negative fixtures plus focused Valence and Cairn validation evidence before archive.

#### Scenario: Focused validation covers stack-chain health
r[valence.stack_provenance.formal_proof_chain_fixture.final_validation.fixtures]
- GIVEN positive and negative formal-proof stack-chain fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, and saved validation evidence MUST identify fixture and policy inputs.

### Requirement: Valence CLI filesystem authority is capability-rooted
r[valence.stack_provenance_adapter.cap_std_cli_boundary.root_wrappers] Valence MUST represent CLI input artifact, manifest, sidecar, binary, and output filesystem authority as typed capability roots opened by `valence-cli`.

#### Scenario: Valid CLI artifact path stays under the declared root
r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.positive]
- GIVEN `valence-cli` has opened an operator-declared input or output root
- WHEN stack-provenance or evidence-file commands read a valid relative artifact path, measure a binary, or write a sidecar/report through the typed root
- THEN the operation MUST remain confined to that declared root while preserving existing valid-input behavior.

#### Scenario: Invalid CLI artifact path cannot escape the root
r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, a symlink escape attempt, or an output escape attempt
- WHEN `valence-cli` resolves the path through the typed root
- THEN Valence MUST fail closed before reading, measuring, or writing outside the declared root.

### Requirement: Capability dependency stays outside Valence core
r[valence.stack_provenance_adapter.cap_std_cli_boundary.dependency] Valence MUST add `cap-std` only to `valence-cli` shell/adaptor code that owns filesystem effects.

#### Scenario: Evidence core remains pure
r[valence.stack_provenance_adapter.cap_std_cli_boundary.conversion]
- GIVEN `valence-core` Evidence IR, provenance, role, hash, and non-claim validation logic is reviewed
- WHEN cap-std adoption is complete
- THEN `valence-core` MUST receive loaded bytes, parsed DTOs, or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths directly.

### Requirement: Capability boundary is documented without evidence overclaims
r[valence.stack_provenance_adapter.cap_std_cli_boundary.docs] Valence MUST document that capability roots bound local CLI filesystem authority only and do not prove evidence truth, behavioral correctness, release eligibility, or verifier soundness.

#### Scenario: Closeout evidence includes CLI boundary validation
r[valence.stack_provenance_adapter.cap_std_cli_boundary.validation]
- GIVEN the cap-std CLI-boundary change is ready to archive
- WHEN focused CLI tests, core purity checks, Cairn validation, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible non-claim documentation.

### Requirement: CLI capability roots are explicitly declared
r[valence.stack_provenance_adapter.declared_capability_roots.contract] Valence MUST require an explicit typed capability root for every artifact, manifest, sidecar, binary, or output filesystem operation and MUST NOT derive that root from the target locator.

#### Scenario: Explicit in-root access succeeds
r[valence.stack_provenance_adapter.declared_capability_roots.fixtures.positive]
- GIVEN an operator explicitly declares an input root and supplies an existing relative artifact locator beneath it
- WHEN the CLI validates and opens the access plan
- THEN Valence MUST read the artifact through the declared capability handle.

### Requirement: File locators remain capability-relative
r[valence.stack_provenance_adapter.declared_capability_roots.relative_locators] Valence MUST validate file locators as relative paths beneath the declared root before performing I/O.

#### Scenario: Absolute locator cannot select its own authority root
r[valence.stack_provenance_adapter.declared_capability_roots.fixtures.negative]
- GIVEN an operator supplies an absolute artifact locator without a separately declared root-relative locator
- WHEN capability planning runs
- THEN Valence MUST reject the locator before opening or reading its parent directory.

#### Scenario: Traversal and symlink escapes fail closed
- GIVEN a locator contains parent traversal or resolves through the declared handle outside its root
- WHEN the CLI attempts the operation
- THEN Valence MUST fail without reading, measuring, or writing the outside target.

### Requirement: Capability roles remain distinct
r[valence.stack_provenance_adapter.declared_capability_roots.role_separation] Valence MUST preserve typed operation roles for input artifacts, manifests, sidecars, binaries, and outputs even when an operator explicitly maps multiple roles to the same physical directory.

#### Scenario: Wrong-role operation is rejected
- GIVEN a handle is declared only for an input role
- WHEN an output write is planned through that role
- THEN pure access-plan validation MUST reject the operation with a deterministic role-mismatch diagnostic.

### Requirement: Filesystem authority remains in the shell
r[valence.stack_provenance_adapter.declared_capability_roots.shell_boundary] Valence MUST keep root opening, `cap-std` handles, reads, measurements, and writes in `valence-cli`, while pure access-plan validation remains deterministic and I/O-free.

#### Scenario: Core validation requires no filesystem
- GIVEN a role, operation, declared-root identity, and relative locator in memory
- WHEN the access plan is validated
- THEN the result MUST be testable without opening a directory, reading environment state, or touching the filesystem.

### Requirement: Capability-root migration is explicit
r[valence.stack_provenance_adapter.declared_capability_roots.docs] Valence documentation MUST explain the new root-plus-relative-locator contract and MUST state that confinement does not prove root trust, evidence truth, sandbox completeness, correctness, or release eligibility.

#### Scenario: Legacy ambient invocation is not silently preserved
- GIVEN an existing script passes an absolute file argument that previously selected its own parent root
- WHEN the script is migrated
- THEN documentation MUST require an explicit role root and relative locator rather than an ambient compatibility fallback.

### Requirement: Declared-root remediation is verified
r[valence.stack_provenance_adapter.declared_capability_roots.final_validation] The change MUST include positive and negative core and CLI evidence covering every file-taking command family.

#### Scenario: Regression rail covers public command paths
- GIVEN the declared-root implementation is complete
- WHEN focused CLI tests and repository validation run
- THEN explicit in-root operations MUST pass and omitted-root, absolute-locator, traversal, wrong-role, and symlink-escape cases MUST fail closed.

### Requirement: Policy-check provenance records retain their source tier
r[valence.stack_provenance_adapter.policy_check_records] Valence MUST admit the exact `policy_check` evidence tier on validated Octet provenance records, preserve that source string, and retain identity/linkage-only non-claims and recorded-only evidence semantics.

#### Scenario: Actual Octet policy-check export builds a sidecar
r[valence.stack_provenance_adapter.policy_check_records.admit]
- GIVEN a valid official Octet provenance export containing `policy_check` records and matching artifact identities
- WHEN Valence builds and validates a stack-provenance sidecar
- THEN admission MUST succeed without rewriting the export or promoting the policy-check evidence to formal proof.

#### Scenario: Unknown provenance tiers remain denied
r[valence.stack_provenance_adapter.policy_check_records.unknown]
- GIVEN a provenance record with an unrecognized evidence tier
- WHEN Valence validates the export
- THEN validation MUST fail with the existing unknown-tier diagnostic and MUST preserve the source string in recorded projections.

#### Scenario: Policy checks do not become proof-card tiers
r[valence.stack_provenance_adapter.policy_check_records.cards]
- GIVEN a proof card using `policy_check` instead of the existing formal-proof or manual-review tier
- WHEN Valence validates the export
- THEN the proof card MUST remain denied even though that tier is recognized for provenance records.

#### Scenario: Recognition does not increase verification authority
r[valence.stack_provenance_adapter.policy_check_records.nonclaims]
- GIVEN admitted policy-check records
- WHEN Valence projects their evidence or describes the resulting sidecar
- THEN the result MUST remain recorded-only identity/linkage evidence and MUST NOT establish runtime correctness, requirement satisfaction, authority or release eligibility.
