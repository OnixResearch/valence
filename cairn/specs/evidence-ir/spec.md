# Evidence Ir Specification

## Purpose

Defines the `evidence-ir` capability.

## Requirements

### Requirement: Evidence IR v1 canonical form
r[valence.evidence_ir.canonical_form] The system MUST provide a canonical, serializable `EvidenceArtifact` type carrying `schema`, `artifact_kind`, `stable_id`, `canonical_hash`, `source_span`, `verification_role`, `payload`, and `non_claims` fields.

#### Scenario: Canonical form round-trips through serialization
r[valence.evidence_ir.canonical_form.round_trip]
- GIVEN a built EvidenceArtifact
- WHEN it is serialized to JSON and deserialized
- THEN all fields are preserved

#### Scenario: Canonical hash is stable for same core inputs
r[valence.evidence_ir.canonical_form.hash_stable]
- GIVEN two artifacts built from identical core inputs
- WHEN their canonical hashes are compared
- THEN they are equal

#### Scenario: Canonical hash excludes display fields
r[valence.evidence_ir.canonical_form.hash_excludes_display]
- GIVEN an artifact with a source span
- WHEN the source span path is changed
- THEN the canonical hash is unchanged

### Requirement: Evidence span provenance threading
r[valence.evidence_ir.span_threading] The system MUST provide an `EvidenceSpan` type with a `source_id` field that is the stable identifier threading source to receipt to binary.

#### Scenario: Explain resolves a known source ID
r[valence.evidence_ir.span_threading.explain_known]
- GIVEN a spans manifest containing an ID
- WHEN `explain_span` is called with that ID
- THEN the corresponding span is returned

#### Scenario: Explain returns none for unknown ID
r[valence.evidence_ir.span_threading.explain_unknown]
- GIVEN a spans manifest
- WHEN `explain_span` is called with an absent ID
- THEN None is returned

### Requirement: Verification role classification
r[valence.evidence_ir.verification_role] The system MUST classify every artifact with a `VerificationRole` of `Property`, `RecordedOnly`, or `Boundary`.

#### Scenario: Roles serialize to snake_case
r[valence.evidence_ir.verification_role.serialize]
- GIVEN each VerificationRole variant
- WHEN serialized to JSON
- THEN it appears as `"property"`, `"recorded_only"`, or `"boundary"`

### Requirement: Non-claim boundary enforcement
r[valence.evidence_ir.non_claim_boundary] The system MUST reject artifacts with missing or weakened non-claim boundaries, and MUST reject overclaim markers in payload or non_claims.

#### Scenario: Checker fails closed for missing non_claims
r[valence.evidence_ir.non_claim_boundary.missing]
- GIVEN an artifact with empty non_claims
- WHEN checked
- THEN the report is invalid with `non_claims_missing`

#### Scenario: Checker fails closed for weakened non_claims
r[valence.evidence_ir.non_claim_boundary.weakened]
- GIVEN an artifact whose non_claims omit the required boundary text
- WHEN checked
- THEN the report is invalid with `non_claims_weakened`

#### Scenario: Checker rejects overclaim in payload
r[valence.evidence_ir.non_claim_boundary.overclaim_payload]
- GIVEN an artifact whose payload contains "proves semantic equivalence"
- WHEN checked
- THEN the report is invalid with `overclaim_detected`

#### Scenario: Checker rejects overclaim in non_claims
r[valence.evidence_ir.non_claim_boundary.overclaim_non_claims]
- GIVEN an artifact whose non_claims contains "proves whole-program correctness"
- WHEN checked
- THEN the report is invalid with `overclaim_detected`

### Requirement: Function-address digest spelling adapters are explicit

r[valence.evidence_ir.function_address_digest_spelling.contract] Valence MUST define explicit function-address digest spelling adapters for supported boundary inputs such as Octet `b3:<hex>` and Kamacite bare BLAKE3 hex.

#### Scenario: Supported spelling normalizes
r[valence.evidence_ir.function_address_digest_spelling.positive]
- GIVEN a function-address evidence input uses a supported spelling at a declared adapter boundary
- WHEN Valence normalizes the digest
- THEN the normalized digest MUST preserve the BLAKE3 value and typed function-address digest domain.

### Requirement: Function-address spelling normalization does not replace domain validation

r[valence.evidence_ir.function_address_digest_spelling.domain] Valence MUST validate digest domain separately from spelling normalization.

#### Scenario: Cross-domain substitution fails
r[valence.evidence_ir.function_address_digest_spelling.negative]
- GIVEN a syntactically valid BLAKE3 digest is supplied under the wrong function-address digest domain
- WHEN Valence validates the evidence input
- THEN validation MUST fail with a domain-substitution diagnostic.

### Requirement: Function-address canonical spelling is documented

r[valence.evidence_ir.function_address_digest_spelling.docs] Valence MUST document the canonical digest spelling emitted in Valence function-address receipts and graph summaries.

#### Scenario: Stack smoke uses normalized digests
r[valence.evidence_ir.function_address_digest_spelling.validation]
- GIVEN Octet, Kamacite, Mantle, and Cairn exchange function-address evidence
- WHEN the stack smoke runs
- THEN Valence MUST normalize allowed spellings deterministically and reject malformed or unsupported spelling variants.

### Requirement: Function-address profile policy identity is deterministic

r[valence.evidence_ir.function_address_policy_identity.material] Valence MUST define deterministic canonical material for the function-address Evidence IR profile covering profile name/version, accepted digest domains, accepted roles, required non-claims, graph row contract, and validation boundary text.

r[valence.evidence_ir.function_address_policy_identity.digest] Valence MUST compute and expose a BLAKE3 policy/profile identity from that material.

#### Scenario: Current policy identity propagates
r[valence.evidence_ir.function_address_policy_identity.positive]
- GIVEN function-address evidence validates under the current profile
- WHEN Valence emits the evidence artifact, receipt, or graph summary
- THEN the output MUST carry the current policy identity and preserve its typed policy-profile domain.

#### Scenario: Stale policy identity fails closed
r[valence.evidence_ir.function_address_policy_identity.negative]
- GIVEN function-address evidence omits the policy identity, uses a placeholder, supplies malformed BLAKE3 text, or substitutes another digest domain
- WHEN Valence validates the evidence input
- THEN validation MUST fail with deterministic policy identity diagnostics.

#### Scenario: Placeholder policy hashes are rejected
r[valence.evidence_ir.function_address_policy_identity.replace_placeholder]
- GIVEN a fixture or smoke artifact still contains a placeholder function-address policy hash
- WHEN Valence validates it
- THEN validation MUST reject the stale identity rather than binding Mantle or Cairn to placeholder policy material.

### Requirement: Artifact reference core contract
r[valence.evidence_ir.artifact_ref_core.contract] Valence MUST define typed artifact reference contracts for artifact roles, digest spelling profiles, path metadata confinement, hash/size metadata, and bounded non-claim text.

#### Scenario: Valid artifact ref validates
r[valence.evidence_ir.artifact_ref_core.fixtures.positive]
- GIVEN an artifact reference uses a supported role, digest spelling profile, confined metadata path, matching hash/size metadata, and required non-claim boundary
- WHEN artifact ref validation runs
- THEN validation MUST accept the ref and preserve its role-specific type.

#### Scenario: Invalid artifact ref fails closed
r[valence.evidence_ir.artifact_ref_core.fixtures.negative]
- GIVEN an artifact reference has malformed digest text, unsupported prefix, role mismatch, path escape, stale hash/size metadata, missing non-claims, or overclaim text
- WHEN artifact ref validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid ref field.

### Requirement: Artifact roles remain distinct
r[valence.evidence_ir.artifact_ref_core.roles] Artifact ref validation MUST preserve role separation between binary artifacts, sidecars, receipts, source artifacts, release bundle members, and compatibility refs.

#### Scenario: Same digest cannot erase role
r[valence.evidence_ir.artifact_ref_core.roles.same_digest]
- GIVEN two artifact refs have the same digest bytes but different declared roles
- WHEN they are validated or compared
- THEN validation MUST preserve the role distinction and MUST NOT silently treat one role as the other.

### Requirement: Artifact ref validation is pure
r[valence.evidence_ir.artifact_ref_core.validation] Valence MUST implement artifact ref validation as pure deterministic logic over in-memory refs, roles, path metadata, and declared hash/size metadata.

#### Scenario: Shell owns artifact measurement
r[valence.evidence_ir.artifact_ref_core.validation.shell]
- GIVEN a CLI or downstream crate reads artifact bytes, resolves paths, or measures byte digests
- WHEN artifact ref validation is invoked
- THEN filesystem access, byte measurement, path resolution, stdout/stderr, and receipt writing MUST remain outside the artifact ref core.

### Requirement: Compatibility profiles preserve existing spellings
r[valence.evidence_ir.artifact_ref_core.adapters] Valence SHOULD provide compatibility profiles for existing public digest spellings and field names.

#### Scenario: Public field is adapter-backed
r[valence.evidence_ir.artifact_ref_core.adapters.shape]
- GIVEN an existing stack evidence surface exposes a raw hex, `b3:`, or `blake3:` artifact ref field
- WHEN an adapter validates the surface
- THEN the adapter SHOULD preserve that public field while requiring an explicit compatibility profile.

### Requirement: Artifact ref core remains bounded
r[valence.evidence_ir.artifact_ref_core.docs] Passing artifact ref validation MUST NOT prove artifact truth, behavioral correctness, semantic equivalence, verifier soundness, release eligibility, lifecycle readiness, runtime authority, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.artifact_ref_core.final_validation]
- GIVEN an artifact ref validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only artifact ref syntax, role separation, metadata confinement, and declared hash/size consistency.

### Requirement: CLI validates function-address Evidence IR

r[valence.evidence_ir.function_address_cli.command] Valence MUST provide a CLI command that validates function-address Evidence IR inputs through the pure `function_address_profile` core.

#### Scenario: Valid Kamacite-backed evidence passes
r[valence.evidence_ir.function_address_cli.positive]
- GIVEN function-address evidence records, source spans, roles, digest domains, and Kamacite receipt identity all match
- WHEN the Valence CLI validates the input
- THEN it MUST emit a passing recorded-only Valence function-address receipt.

### Requirement: CLI emits deterministic function-address receipt and graph outputs

r[valence.evidence_ir.function_address_cli.receipt] Valence MUST emit deterministic function-address receipt/sidecar output suitable for Mantle binding and Cairn readiness input.

#### Scenario: Graph output is stable
r[valence.evidence_ir.function_address_cli.graph]
- GIVEN the same typed function-address input is validated twice
- WHEN graph-summary output is requested
- THEN Valence MUST emit stable graph rows and BLAKE3 graph summary identity.

### Requirement: CLI preserves function-address recorded-only boundary

r[valence.evidence_ir.function_address_cli.negative] Valence MUST reject function-address evidence that uses the wrong digest domain, lacks spans, has stale Kamacite identity, declares unknown roles, contains overclaims, or promotes recorded-only evidence to property/formal-proof status.

#### Scenario: CLI preserves core boundary
r[valence.evidence_ir.function_address_cli.validation]
- GIVEN Valence CLI validation succeeds
- WHEN an operator reads the receipt
- THEN the receipt MUST state that function-address evidence records identity/linkage only and does not prove Rust semantic correctness or release eligibility.

### Requirement: CLI shell for Evidence IR
r[valence.evidence_ir.cli] The system MUST expose `valence check <FILE>`, `valence explain --id <ID> --manifest <FILE>`, and `valence version` commands with stable exit codes 0, 1, and 2.

#### Scenario: Check accepts valid artifact
r[valence.evidence_ir.cli.check_valid]
- GIVEN a library-emitted valid artifact written to a file
- WHEN `valence check <FILE>` is run
- THEN it exits 0 and reports `valid: true`

#### Scenario: Check rejects overclaim artifact
r[valence.evidence_ir.cli.check_overclaim]
- GIVEN an artifact with an overclaim written to a file
- WHEN `valence check <FILE>` is run
- THEN it exits 1 and reports `overclaim_detected`

#### Scenario: Explain resolves known ID
r[valence.evidence_ir.cli.explain_known]
- GIVEN a spans manifest file containing an ID
- WHEN `valence explain --id <ID> --manifest <FILE>` is run
- THEN it exits 0 and prints the span details

#### Scenario: Explain reports not_found for unknown ID
r[valence.evidence_ir.cli.explain_unknown]
- GIVEN a spans manifest file
- WHEN `valence explain --id <unknown> --manifest <FILE>` is run
- THEN it exits 1 and prints `not_found`

### Requirement: Stable v1 contract preservation
r[valence.evidence_ir.v1_contract_preservation] The system MUST preserve the byte-identical stable v1 contract strings from octet's valence-core for `function_object`, `evidence_codebase_index`, and `evidence_index` modules.

#### Scenario: Function Object v1 schema string preserved
r[valence.evidence_ir.v1_contract_preservation.function_object_schema]
- GIVEN the function_object module
- WHEN `FUNCTION_OBJECT_SCHEMA` is read
- THEN it equals `octet.function-object.v1`

#### Scenario: Evidence codebase index schema preserved
r[valence.evidence_ir.v1_contract_preservation.index_schema]
- GIVEN the evidence_codebase_index module
- WHEN `INDEX_SCHEMA_VERSION` is read
- THEN it equals `octet-evidence-codebase-index/v1`

### Requirement: Content-addressed evidence definitions
r[valence.evidence_ir.content_addressed_definitions] Valence MUST identify evidence-relevant definitions by a BLAKE3 hash over a typed canonical value, not by a human-readable name or path.

#### Scenario: Canonical content defines identity
r[valence.evidence_ir.content_addressed_definitions.canonical_identity]
- GIVEN an evidence-relevant definition projected into its typed canonical value
- WHEN Valence computes its evidence-definition identity
- THEN the identity MUST be the BLAKE3 hash of that canonical value.

#### Scenario: Dependency identities are part of canonical content
r[valence.evidence_ir.content_addressed_definitions.dependencies]
- GIVEN an evidence-relevant definition depends on other evidence definitions, artifacts, verifier inputs, or requirements with known content identities
- WHEN Valence builds the canonical value
- THEN dependency references SHOULD use content identities rather than display names.

#### Scenario: Names and paths are metadata
r[valence.evidence_ir.content_addressed_definitions.metadata_boundary]
- GIVEN the display name, source path, or alias for an evidence definition changes while canonical content and dependency identities are unchanged
- WHEN Valence recomputes the evidence-definition identity
- THEN the identity MUST remain stable.

#### Scenario: Rename and path churn fixtures stay stable
r[valence.evidence_ir.content_addressed_definitions.rename_stability]
- GIVEN positive fixtures that vary display names and source paths only
- WHEN the identity gate runs
- THEN Valence MUST report the same evidence-definition identity for each fixture.

#### Scenario: Dependency changes alter identity
r[valence.evidence_ir.content_addressed_definitions.dependency_change]
- GIVEN a fixture where a dependency content identity changes
- WHEN Valence recomputes the parent evidence-definition identity
- THEN the parent identity MUST change or the gate MUST fail closed.

#### Scenario: Identity claim remains bounded
r[valence.evidence_ir.content_addressed_definitions.non_claims]
- GIVEN documentation or receipts mention content-addressed evidence-definition identity
- WHEN the supported claim is described
- THEN the output MUST state that content identity proves canonical content and dependency identity only, not behavioral correctness, semantic equivalence, verifier soundness, release eligibility, or requirement satisfaction.

### Requirement: Evidence name-binding history
r[valence.evidence_ir.name_binding_history] Valence MUST model human-readable names, aliases, and paths as bindings to content-addressed evidence identities rather than as identities themselves.

#### Scenario: Binding records map aliases to identities
r[valence.evidence_ir.name_binding_history.records]
- GIVEN an evidence content identity and a human-readable alias, source path, or requirement name
- WHEN Valence records the association
- THEN it MUST store a binding record that keeps the alias separate from the content identity.

#### Scenario: Bindings are carried in receipts when review-relevant
r[valence.evidence_ir.name_binding_history.receipts]
- GIVEN a verifier receipt, sidecar, or report relies on an alias for human review
- WHEN Valence renders the receipt
- THEN it SHOULD include the alias binding history needed to audit the content identity.

#### Scenario: Alias resolution is deterministic
r[valence.evidence_ir.name_binding_history.resolve]
- GIVEN an alias with exactly one current content identity binding
- WHEN Valence resolves the alias
- THEN it MUST return that content identity deterministically.

#### Scenario: Ambiguous aliases fail closed
r[valence.evidence_ir.name_binding_history.ambiguous]
- GIVEN an alias is bound to multiple content identities
- WHEN identity-level verification depends on that alias
- THEN Valence MUST fail closed with a deterministic ambiguity diagnostic.

#### Scenario: Renames preserve identity linkage
r[valence.evidence_ir.name_binding_history.rename]
- GIVEN a definition or artifact is renamed without changing canonical content
- WHEN Valence verifies provenance through the new alias
- THEN provenance SHOULD resolve to the same content identity and retain prior binding history.

#### Scenario: Stale bindings are reported
r[valence.evidence_ir.name_binding_history.stale]
- GIVEN a binding claims an alias points to a content identity that is absent from the evidence graph
- WHEN Valence validates bindings
- THEN validation MUST fail closed with a stale-binding diagnostic.

#### Scenario: Name-binding boundary is documented
r[valence.evidence_ir.name_binding_history.docs]
- GIVEN documentation or receipts describe name bindings
- WHEN the supported claim is stated
- THEN the output MUST state that aliases are review metadata and do not prove semantic equivalence or requirement satisfaction.

### Requirement: Queryable evidence-codebase graph
r[valence.evidence_ir.queryable_graph] Valence MUST provide a typed evidence graph that indexes evidence identities and linkage relationships across definitions, artifacts, requirements, verifiers, receipts, binaries, sidecars, and aliases.

#### Scenario: Graph contains typed nodes and edges
r[valence.evidence_ir.queryable_graph.nodes]
- GIVEN evidence artifacts, provenance chains, stack sidecars, receipts, binaries, and alias bindings
- WHEN Valence builds the evidence graph
- THEN the graph MUST contain typed nodes and typed edges for their identity and linkage relationships.

#### Scenario: Graph export is deterministic
r[valence.evidence_ir.queryable_graph.export]
- GIVEN equivalent graph inputs in different orders
- WHEN Valence exports the graph
- THEN the exported graph MUST be byte-stable or canonical-value stable.

#### Scenario: Core queries are pure
r[valence.evidence_ir.queryable_graph.queries]
- GIVEN a graph value in memory
- WHEN Valence queries dependency closure, reverse dependencies, requirement coverage, binary traceability, verifier evidence, or alias resolution
- THEN the query MUST return deterministic results without filesystem or process I/O.

#### Scenario: Equivalent inputs have stable graph identity
r[valence.evidence_ir.queryable_graph.determinism]
- GIVEN positive fixtures containing the same nodes and edges in different input order
- WHEN the graph determinism gate runs
- THEN Valence MUST compute the same graph identity and query results.

#### Scenario: Broken graph edges fail closed
r[valence.evidence_ir.queryable_graph.fail_closed]
- GIVEN duplicate node identities, dangling edges, or ambiguous aliases
- WHEN Valence validates the graph
- THEN validation MUST fail closed with deterministic diagnostics.

#### Scenario: Graph boundary is documented
r[valence.evidence_ir.queryable_graph.docs]
- GIVEN documentation or query output describes the evidence graph
- WHEN the supported claim is stated
- THEN the output MUST state that the graph indexes evidence identity and linkage only, not behavioral correctness, verifier soundness, release eligibility, or requirement satisfaction.

### Requirement: Verifier capability labels
r[valence.evidence_ir.verifier_capabilities] Valence MUST model verifier effects with typed capability labels so receipts distinguish pure checks from checks that depend on external effects.

#### Scenario: Capability sets serialize deterministically
r[valence.evidence_ir.verifier_capabilities.serialization]
- GIVEN a verifier capability set
- WHEN Valence serializes it into a report or receipt
- THEN the capabilities MUST be typed, sorted deterministically, and round-trip through parsing.

#### Scenario: Receipts carry capabilities
r[valence.evidence_ir.verifier_capabilities.receipts]
- GIVEN a verifier report or receipt is emitted
- WHEN the verifier used known capabilities
- THEN the report SHOULD include its capability set as additive evidence metadata.

#### Scenario: Pure checks are identified
r[valence.evidence_ir.verifier_capabilities.pure]
- GIVEN a verifier capability set contains only `Pure`
- WHEN Valence evaluates replay or cache eligibility
- THEN the result MAY be treated as pure-cache eligible subject to matching input and verifier identities.

#### Scenario: Denied capabilities fail policy
r[valence.evidence_ir.verifier_capabilities.policy]
- GIVEN policy denies network, secret, remote-builder, process, clock, or filesystem capabilities for a gate
- WHEN a verifier report carries a denied capability
- THEN Valence MUST fail closed with a deterministic capability-policy diagnostic.

#### Scenario: Capability tests cover positive and negative cases
r[valence.evidence_ir.verifier_capabilities.tests]
- GIVEN capability fixtures and policies
- WHEN the capability gate runs
- THEN valid pure/local capabilities MUST pass and unknown or denied capabilities MUST fail deterministically.

#### Scenario: Capability boundary is documented
r[valence.evidence_ir.verifier_capabilities.docs]
- GIVEN documentation or receipts describe verifier capabilities
- WHEN the supported claim is stated
- THEN the output MUST state that labels are receipt evidence unless a sandboxed runner enforces them, and that labels do not prove verifier soundness or requirement satisfaction.

### Requirement: Dependency-closure verifier cache
r[valence.evidence_ir.dependency_closure_cache] Valence MUST define verifier cache keys that include canonical input identity, verifier identity, policy identity, dependency closure identity, capability set identity, and cache schema version.

#### Scenario: Cache key includes all identity inputs
r[valence.evidence_ir.dependency_closure_cache.key]
- GIVEN a verifier result is considered for reuse
- WHEN Valence computes the cache key
- THEN the key MUST include canonical input hash, verifier identity, policy hash, dependency closure hash, capability set hash, and cache schema version.

#### Scenario: Cache eligibility is bounded by capabilities
r[valence.evidence_ir.dependency_closure_cache.eligibility]
- GIVEN a verifier capability set is pure or explicitly replay-safe
- WHEN Valence evaluates cache eligibility
- THEN the result MAY be eligible only if all cache-key inputs match.

#### Scenario: Cache decisions are deterministic
r[valence.evidence_ir.dependency_closure_cache.decisions]
- GIVEN a cache lookup and current verifier context
- WHEN Valence compares cache keys
- THEN it MUST return a deterministic hit, miss, or blocked decision with issue codes.

#### Scenario: Matching keys hit
r[valence.evidence_ir.dependency_closure_cache.hit]
- GIVEN a cached result and current verifier context with identical cache-key fields
- WHEN Valence performs a cache lookup
- THEN it SHOULD report a cache hit and reuse the result only if policy allows reuse.

#### Scenario: Changed key fields miss
r[valence.evidence_ir.dependency_closure_cache.miss]
- GIVEN any cache-key field changes
- WHEN Valence performs a cache lookup
- THEN it MUST report a cache miss rather than reuse the stale result.

#### Scenario: Effectful cache reuse is blocked by default
r[valence.evidence_ir.dependency_closure_cache.boundary]
- GIVEN a verifier used filesystem, clock, process, network, remote-builder, secret, or unknown capabilities
- WHEN no policy marks that capability set replay-safe
- THEN cache reuse MUST be blocked.

#### Scenario: Cache boundary is documented
r[valence.evidence_ir.dependency_closure_cache.docs]
- GIVEN documentation or receipts describe verifier caching
- WHEN the supported claim is stated
- THEN the output MUST state that cache hits prove matching cache-key identities only, not verifier soundness, correctness, or requirement satisfaction.

### Requirement: Executable receipt transcripts
r[valence.evidence_ir.executable_transcripts] Valence MUST support checked transcript fixtures that bind verifier inputs to expected reports, diagnostics, and content hashes.

#### Scenario: Transcript fixture has stable shape
r[valence.evidence_ir.executable_transcripts.schema]
- GIVEN a transcript fixture
- WHEN Valence parses it
- THEN it MUST include schema version, scenario identity, inputs, verifier action, expected report fields, expected issue codes, expected hashes, and non-claims.

#### Scenario: Transcript runner evaluates pure scenarios
r[valence.evidence_ir.executable_transcripts.runner]
- GIVEN a transcript scenario that can run through pure verifier functions
- WHEN the transcript gate runs
- THEN Valence MUST evaluate it without filesystem or process I/O.

#### Scenario: Positive transcripts pass
r[valence.evidence_ir.executable_transcripts.positive]
- GIVEN a valid Evidence IR or stack-provenance transcript fixture
- WHEN the transcript gate runs
- THEN expected reports, issue codes, and hashes MUST match.

#### Scenario: Negative transcripts fail as expected
r[valence.evidence_ir.executable_transcripts.negative]
- GIVEN malformed input, overclaim, stale hash, or denied capability transcript fixtures
- WHEN the transcript gate runs
- THEN each fixture MUST produce its expected deterministic failure diagnostics.

#### Scenario: Transcript refresh is explicit
r[valence.evidence_ir.executable_transcripts.refresh]
- GIVEN verifier output or hash expectations intentionally change
- WHEN maintainers refresh transcript fixtures
- THEN the refresh MUST produce reviewable fixture diffs and MUST NOT update expectations implicitly during normal validation.

#### Scenario: Transcript boundary is documented
r[valence.evidence_ir.executable_transcripts.docs]
- GIVEN documentation or receipt output describes executable transcripts
- WHEN the supported claim is stated
- THEN the output MUST state that transcripts prove fixture behavior for checked inputs only, not general correctness, verifier soundness, or requirement satisfaction.

### Requirement: Structured evidence refactoring sessions
r[valence.evidence_ir.structured_refactoring] Valence MUST provide a typed refactoring session model for evidence alias, name, path, and requirement-binding migrations that preserve content identity.

#### Scenario: Session input is typed
r[valence.evidence_ir.structured_refactoring.session]
- GIVEN a requested evidence rename or binding migration
- WHEN Valence parses the session
- THEN the session MUST record source bindings, target bindings, intended content identities, policy constraints, and non-claims.

#### Scenario: Dry-run plan reports impact
r[valence.evidence_ir.structured_refactoring.plan]
- GIVEN a valid refactoring session and evidence graph
- WHEN Valence plans the session
- THEN it MUST produce a deterministic dry-run report listing affected nodes, edges, aliases, receipts, and mutation surfaces.

#### Scenario: Unsafe plans are blocked
r[valence.evidence_ir.structured_refactoring.blockers]
- GIVEN a session has ambiguous aliases, missing identities, dangling edges, unsupported mutation surfaces, or would require semantic-equivalence claims
- WHEN Valence plans or executes it
- THEN the session MUST be blocked with deterministic diagnostics.

#### Scenario: Migration receipts include before and after identities
r[valence.evidence_ir.structured_refactoring.receipts]
- GIVEN a refactoring session executes
- WHEN Valence records the migration receipt
- THEN the receipt SHOULD include before/after identity manifests, binding changes, policy identity, and mutation diagnostics.

#### Scenario: Positive and negative tests cover refactors
r[valence.evidence_ir.structured_refactoring.tests]
- GIVEN refactoring fixtures
- WHEN tests run
- THEN content-preserving alias/path migrations MUST pass and ambiguous, dangling, or semantic-equivalence-requiring migrations MUST fail.

#### Scenario: Refactoring boundary is documented
r[valence.evidence_ir.structured_refactoring.docs]
- GIVEN documentation or receipts describe structured refactoring
- WHEN the supported claim is stated
- THEN the output MUST state that sessions preserve and rewrite identity bindings only, not semantic equivalence, correctness, or requirement satisfaction.

### Requirement: Typed durable evidence storage
r[valence.evidence_ir.typed_durable_storage] Valence MUST persist durable evidence values with typed storage metadata that identifies schema, value type, codec, content hash, and migration history where applicable.

#### Scenario: Durable envelope carries type and schema identity
r[valence.evidence_ir.typed_durable_storage.envelope]
- GIVEN Valence persists an evidence value through the durable storage path
- WHEN the value is encoded
- THEN the envelope MUST include schema version, value type identity, codec identity, content hash, and non-claims.

#### Scenario: Decode validates before constructing DTOs
r[valence.evidence_ir.typed_durable_storage.decode]
- GIVEN a persisted evidence value is loaded
- WHEN Valence decodes it
- THEN Valence MUST validate schema identity, value type identity, codec identity, and content hash before constructing the current DTO.

#### Scenario: Legacy values require explicit migrations
r[valence.evidence_ir.typed_durable_storage.migration]
- GIVEN a persisted value uses an older supported schema
- WHEN Valence migrates it to a current DTO
- THEN the migration SHOULD record source identity, target identity, migration tool identity, diagnostics, and non-claims.

#### Scenario: Valid envelopes and migrations pass
r[valence.evidence_ir.typed_durable_storage.tests]
- GIVEN positive durable storage fixtures
- WHEN the durable storage gate runs
- THEN valid current envelopes and supported legacy migrations MUST pass deterministically.

#### Scenario: Mismatches fail closed
r[valence.evidence_ir.typed_durable_storage.fail_closed]
- GIVEN schema, type, codec, content hash, or migration metadata is missing or mismatched
- WHEN Valence decodes the persisted value
- THEN decoding MUST fail closed with deterministic diagnostics.

#### Scenario: Durable storage boundary is documented
r[valence.evidence_ir.typed_durable_storage.docs]
- GIVEN documentation or receipts describe typed durable evidence storage
- WHEN the supported claim is stated
- THEN the output MUST state that typed storage proves decode identity and compatibility only, not evidence truth, correctness, verifier soundness, or requirement satisfaction.

### Requirement: Executable transcript fixture runner
r[valence.evidence_ir.transcript_fixture_runner] Valence MUST provide a test harness that discovers checked-in executable transcript fixtures, evaluates them deterministically, and reports stable pass/fail diagnostics.

#### Scenario: Transcript fixtures load from pass and fail roots
r[valence.evidence_ir.transcript_fixture_runner.fixtures]
- GIVEN pass and fail transcript fixture directories
- WHEN the transcript fixture runner starts
- THEN it MUST load typed transcript fixtures with schema identity, scenario identity, inputs, verifier action, expected report fields, expected issue codes, expected hashes, and non-claims.

#### Scenario: Transcript execution stays pure
r[valence.evidence_ir.transcript_fixture_runner.pure]
- GIVEN a transcript fixture that targets a supported verifier action
- WHEN the runner evaluates the fixture
- THEN it MUST call pure in-memory verifier functions and MUST NOT perform filesystem, process, network, or clock I/O inside the evaluation core.

#### Scenario: Positive transcript fixtures pass
r[valence.evidence_ir.transcript_fixture_runner.positive]
- GIVEN positive transcript fixtures for valid Evidence IR or stack-provenance workflows
- WHEN the runner evaluates them
- THEN observed validity, issue codes, and expected hashes MUST match the fixture expectations.

#### Scenario: Negative transcript fixtures fail as expected
r[valence.evidence_ir.transcript_fixture_runner.negative]
- GIVEN negative transcript fixtures for malformed input, overclaims, stale hashes, or denied capabilities
- WHEN the runner evaluates them
- THEN each fixture MUST produce the expected deterministic failure diagnostics.

#### Scenario: Refresh is explicit
r[valence.evidence_ir.transcript_fixture_runner.refresh]
- GIVEN verifier output or expected hashes intentionally change
- WHEN maintainers refresh transcript expectations
- THEN the refresh MUST be explicit, reviewable, and MUST NOT update fixtures during normal validation.

#### Scenario: Fixture discovery order is stable
r[valence.evidence_ir.transcript_fixture_runner.stability]
- GIVEN equivalent transcript fixture sets discovered in different filesystem orders
- WHEN the runner renders its report
- THEN the report MUST be byte-stable or canonical-value stable.

### Requirement: Evidence model property invariants
r[valence.evidence_ir.property_invariants] Valence MUST provide deterministic property and invariant tests for pure evidence-model values that cover identity stability, graph determinism, alias failure, and cache-key equality.

#### Scenario: Rename-only churn preserves content identity
r[valence.evidence_ir.property_invariants.rename_stability]
- GIVEN evidence definitions whose display names, aliases, or paths change without changing canonical content or dependency identities
- WHEN identity invariants run
- THEN the evidence definition identity MUST remain stable.

#### Scenario: Dependency changes alter identity
r[valence.evidence_ir.property_invariants.dependency_sensitivity]
- GIVEN an evidence definition dependency content identity changes
- WHEN identity invariants run
- THEN the evidence definition identity MUST change.

#### Scenario: Graph identity is deterministic
r[valence.evidence_ir.property_invariants.graph_determinism]
- GIVEN equivalent graph nodes and edges in different input order
- WHEN graph invariants run
- THEN graph export identity and query results MUST remain deterministic.

#### Scenario: Ambiguous aliases fail closed
r[valence.evidence_ir.property_invariants.alias_fail_closed]
- GIVEN an alias resolves to multiple current content identities or a stale binding
- WHEN alias invariants run
- THEN resolution MUST fail closed with deterministic diagnostics.

#### Scenario: Cache keys compare every field
r[valence.evidence_ir.property_invariants.cache_key_equality]
- GIVEN cached and current verifier keys differ in any identity field or capability class
- WHEN cache invariants run
- THEN Valence MUST report a miss or block rather than a cache hit.

### Requirement: Malformed input corpus gates
r[valence.evidence_ir.malformed_input_corpus] Valence MUST provide a checked-in malformed-input corpus that exercises fail-closed diagnostics for Evidence IR and evidence-model boundaries.

#### Scenario: Corpus runner uses pure validation cores
r[valence.evidence_ir.malformed_input_corpus.runner]
- GIVEN malformed corpus fixtures that target pure validators
- WHEN the corpus gate runs
- THEN the runner MUST evaluate them without process, filesystem, network, or clock I/O inside the validation core.

#### Scenario: Diagnostics are normalized
r[valence.evidence_ir.malformed_input_corpus.diagnostics]
- GIVEN a malformed input produces multiple issue codes
- WHEN the corpus report is rendered
- THEN issue codes and unexpected-success diagnostics MUST be sorted deterministically.

#### Scenario: Digest and graph failures are covered
r[valence.evidence_ir.malformed_input_corpus.digest_and_graph]
- GIVEN malformed BLAKE3 strings, duplicate graph nodes, or dangling graph edges
- WHEN the corpus gate runs
- THEN each case MUST fail closed with deterministic diagnostics.

#### Scenario: Binding and capability failures are covered
r[valence.evidence_ir.malformed_input_corpus.bindings_and_capabilities]
- GIVEN stale bindings, ambiguous aliases, unknown capabilities, or denied capabilities
- WHEN the corpus gate runs
- THEN each case MUST fail closed with deterministic diagnostics.

#### Scenario: Boundary and storage failures are covered
r[valence.evidence_ir.malformed_input_corpus.boundary_and_storage]
- GIVEN weakened non-claim boundaries or durable storage envelope mismatches
- WHEN the corpus gate runs
- THEN each case MUST fail closed with deterministic diagnostics.

### Requirement: Stable golden report normalization
r[valence.evidence_ir.golden_report_normalization] Valence MUST compare structured harness reports through canonical normalized values rather than raw stdout snapshots except at CLI boundaries.

#### Scenario: Reports normalize deterministically
r[valence.evidence_ir.golden_report_normalization.stability]
- GIVEN semantically equivalent report values with different object field order or fixture discovery order
- WHEN Valence normalizes the reports
- THEN the normalized value and content hash MUST match.

#### Scenario: Semantic diffs name changed fields
r[valence.evidence_ir.golden_report_normalization.semantic_diff]
- GIVEN a normalized golden report and a changed candidate report
- WHEN the comparison fails
- THEN Valence SHOULD render deterministic changed-field diagnostics.

#### Scenario: Raw stdout snapshots stay at CLI boundaries
r[valence.evidence_ir.golden_report_normalization.raw_stdout_boundary]
- GIVEN a test is not checking CLI stdout/stderr contract behavior
- WHEN it records expected output
- THEN it SHOULD use structured normalized report values rather than raw stdout text.

#### Scenario: Unnormalizable reports fail closed
r[valence.evidence_ir.golden_report_normalization.fail_closed]
- GIVEN a report cannot be parsed or normalized as a structured value, or a golden hash is stale
- WHEN the golden gate runs
- THEN the gate MUST fail closed with deterministic diagnostics.

#### Scenario: Golden refresh is explicit
r[valence.evidence_ir.golden_report_normalization.refresh]
- GIVEN expected report output intentionally changes
- WHEN maintainers refresh golden values
- THEN the refresh MUST be explicit and produce reviewable diffs.

### Requirement: Capability enforcement harness
r[valence.evidence_ir.capability_enforcement_harness] Valence MUST separate pure capability-label policy tests from opt-in sandbox enforcement probes.

#### Scenario: Enforcement availability is explicit
r[valence.evidence_ir.capability_enforcement_harness.availability]
- GIVEN sandbox enforcement support is unavailable or unsupported for an effect
- WHEN enforcement probes are reported
- THEN the status MUST be `unavailable` or `skipped`, not successful enforcement.

#### Scenario: Label policy tests cover allowed and denied capabilities
r[valence.evidence_ir.capability_enforcement_harness.policy]
- GIVEN pure, local, unknown, or denied capability labels
- WHEN pure policy tests run
- THEN allowed labels MUST pass and unknown or denied labels MUST fail deterministically.

#### Scenario: Denied-effect probes are opt-in
r[valence.evidence_ir.capability_enforcement_harness.denied_effects]
- GIVEN filesystem, process, network, clock, remote-builder, or secret effect probes
- WHEN the deep or nightly enforcement lane runs
- THEN the probes MUST require explicit opt-in and report pass, fail, skipped, or unavailable status per effect.

#### Scenario: Missing sandbox support is not false success
r[valence.evidence_ir.capability_enforcement_harness.no_false_success]
- GIVEN an enforcement backend is missing or cannot block a requested denied effect
- WHEN the enforcement report is rendered
- THEN the report MUST NOT claim the effect was successfully blocked.

#### Scenario: Capability enforcement claims remain bounded
r[valence.evidence_ir.capability_enforcement_harness.non_claims]
- GIVEN capability labels or enforcement reports are emitted
- WHEN the supported claim is stated
- THEN the report MUST state that labels are metadata unless an enforcement backend actually blocks effects.

### Requirement: Requirement coverage evidence report
r[valence.evidence_ir.requirement_coverage_report] Valence MUST provide a deterministic report that maps accepted requirement IDs to implementation and verification evidence markers.

#### Scenario: Coverage rows are deterministic
r[valence.evidence_ir.requirement_coverage_report.rows]
- GIVEN accepted requirement IDs and discovered evidence markers
- WHEN Valence builds the coverage report
- THEN report rows MUST be sorted deterministically by requirement identity.

#### Scenario: Implementation and verification markers are distinct
r[valence.evidence_ir.requirement_coverage_report.markers]
- GIVEN source or test markers use `r[impl ...]` or `r[verify ...]`
- WHEN Valence classifies coverage evidence
- THEN implementation evidence and verification evidence MUST be reported separately.

#### Scenario: Coverage computation core is pure
r[valence.evidence_ir.requirement_coverage_report.shell]
- GIVEN in-memory requirement and marker sets
- WHEN coverage computation runs
- THEN it MUST produce report rows without filesystem or process I/O.

#### Scenario: Covered requirements report positive evidence
r[valence.evidence_ir.requirement_coverage_report.positive]
- GIVEN a requirement has both implementation and verification markers
- WHEN the coverage report is built
- THEN the report SHOULD mark that requirement as covered by traceability evidence.

#### Scenario: Missing coverage is reported deterministically
r[valence.evidence_ir.requirement_coverage_report.missing]
- GIVEN an accepted requirement lacks implementation or verification markers
- WHEN the coverage report is built
- THEN the missing evidence MUST be reported with deterministic diagnostics.

#### Scenario: Coverage claims remain bounded
r[valence.evidence_ir.requirement_coverage_report.non_claims]
- GIVEN coverage reports describe requirement evidence
- WHEN the supported claim is stated
- THEN the report MUST state that marker coverage proves traceability only, not correctness, verifier soundness, semantic equivalence, or requirement satisfaction.

### Requirement: Fast and deep test suite partitioning
r[valence.evidence_ir.test_suite_partition] Valence MUST define explicit test suite lanes for fast deterministic checks, deeper optional integration checks, and expensive/nightly validation.

#### Scenario: Fast lane is hermetic
r[valence.evidence_ir.test_suite_partition.fast]
- GIVEN the fast test lane
- WHEN developers run ordinary local validation
- THEN the lane MUST use hermetic checked-in fixtures and MUST NOT require live sibling repositories, network access, or host-specific sandbox enforcement.

#### Scenario: Deep lane contains optional integrations
r[valence.evidence_ir.test_suite_partition.deep]
- GIVEN cross-repo stack checks or sandbox enforcement probes are requested
- WHEN the deep lane runs
- THEN those checks SHOULD run there or in the nightly lane with explicit pass, fail, skipped, or unavailable reporting.

#### Scenario: Lane reporting is stable
r[valence.evidence_ir.test_suite_partition.reporting]
- GIVEN any suite lane completes
- WHEN the harness renders its summary
- THEN the summary MUST use deterministic ordering and stable status labels for pass, fail, skipped, and unavailable checks.

#### Scenario: CI and local commands are documented
r[valence.evidence_ir.test_suite_partition.ci]
- GIVEN maintainers need to run a suite lane locally or in CI
- WHEN they inspect the harness documentation
- THEN each lane MUST have a stable command or documented invocation.

#### Scenario: Suite partition claims remain bounded
r[valence.evidence_ir.test_suite_partition.non_claims]
- GIVEN suite-lane reports describe validation evidence
- WHEN the supported claim is stated
- THEN the report MUST state that lanes schedule evidence gates only and do not prove correctness, verifier soundness, semantic equivalence, or requirement satisfaction.

### Requirement: Hegel property receipts
r[valence.evidence_ir.hegel_property_receipts] Valence MUST model Hegel property-test runs as bounded generated-input evidence receipts with stable identity, runner settings, generator identity, status, diagnostics, and explicit non-claims.

#### Scenario: Receipt carries runner and property identity
r[valence.evidence_ir.hegel_property_receipts.schema]
- GIVEN a Hegel property run targets a Valence pure-core invariant
- WHEN Valence records the receipt
- THEN the receipt MUST include schema version, property ID, linked requirement IDs, checked module or function family, Hegel crate/backend identity, runner settings, case budget, phases, seed or derandomized mode, generator identity, status, issue codes, and non-claims.

#### Scenario: Passing receipt preserves bounded evidence
r[valence.evidence_ir.hegel_property_receipts.valid]
- GIVEN a Hegel property run passes for its configured generated cases and explicit examples
- WHEN Valence validates the receipt
- THEN validation MUST preserve the pass status as bounded generated-testing evidence without promoting it to formal proof.

#### Scenario: Broken receipt fails closed
r[valence.evidence_ir.hegel_property_receipts.fail_closed]
- GIVEN a Hegel receipt omits required identity fields, uses malformed BLAKE3 hashes, records ambiguous settings, or references a stale generator identity
- WHEN Valence validates the receipt
- THEN validation MUST fail closed with deterministic diagnostics naming the rejected field.

#### Scenario: Non-claim boundary is mandatory
r[valence.evidence_ir.hegel_property_receipts.non_claims]
- GIVEN a Hegel receipt omits the generated-testing non-claim boundary or claims exhaustive correctness, semantic equivalence, verifier soundness, requirement satisfaction, release eligibility, or formal proof
- WHEN Valence validates the receipt
- THEN validation MUST reject the receipt with a deterministic boundary or overclaim diagnostic.

### Requirement: Hegel property suites for pure cores
r[valence.evidence_ir.hegel_property_suites] Valence SHOULD provide Hegel-backed property suites for selected pure Evidence IR cores whose invariants benefit from generated-input coverage.

#### Scenario: Canonical identity laws are generated
r[valence.evidence_ir.hegel_property_suites.identity]
- GIVEN generated EvidenceArtifact, Function Object, or related canonical identity inputs that differ only in display metadata or supported field ordering
- WHEN Hegel property suites run
- THEN Valence SHOULD report identical canonical identities and BLAKE3 hashes for semantically equivalent canonical content.

#### Scenario: Graph and cache laws are generated
r[valence.evidence_ir.hegel_property_suites.graph]
- GIVEN generated evidence graph inputs, alias bindings, capability sets, and cache-key fields
- WHEN Hegel property suites run
- THEN Valence SHOULD preserve deterministic graph identity, deterministic query results, deterministic alias resolution, and cache hits only when every identity field matches.

#### Scenario: Generated negative inputs fail closed
r[valence.evidence_ir.hegel_property_suites.fail_closed]
- GIVEN generated duplicate identities, dangling graph edges, stale aliases, weakened non-claim boundaries, unsupported capabilities, or cache-key mismatches
- WHEN Hegel property suites run
- THEN Valence MUST return typed errors or deterministic diagnostics rather than panic, silently accept invalid input, or emit unstable output.

#### Scenario: Evaluation core stays pure
r[valence.evidence_ir.hegel_property_suites.pure_core]
- GIVEN a Hegel test needs to evaluate a Valence invariant
- WHEN the property invokes Valence logic
- THEN the evaluated core MUST operate on in-memory values and MUST NOT read files, inspect environment, spawn processes, access clocks, perform network I/O, or print.

### Requirement: Hegel suite lanes
r[valence.evidence_ir.hegel_suite_lanes] Valence MUST partition Hegel property coverage into explicit fast and deep lanes with named budgets, deterministic or replayable settings, and bounded claims.

#### Scenario: Fast lane is ordinary-test safe
r[valence.evidence_ir.hegel_suite_lanes.fast]
- GIVEN maintainers run ordinary local validation from the Nix devshell
- WHEN the fast Hegel lane runs
- THEN it MUST use small named case budgets, deterministic or replayable settings, and no live sibling repositories or network access.

#### Scenario: Deep lane is opt-in
r[valence.evidence_ir.hegel_suite_lanes.deep]
- GIVEN larger generated case budgets, stateful Hegel models, or shrink-heavy properties are useful
- WHEN maintainers run the deep lane
- THEN the lane MUST be opt-in and report pass, fail, skipped, or unavailable status without becoming a hidden prerequisite for starter validation.

#### Scenario: Lane docs preserve non-claims
r[valence.evidence_ir.hegel_suite_lanes.docs]
- GIVEN documentation or suite reports describe Hegel coverage
- WHEN the supported claim is stated
- THEN the output MUST say Hegel checks bounded generated inputs only and does not prove correctness, semantic equivalence, verifier soundness, requirement satisfaction, release eligibility, or whole-program safety.

### Requirement: State-machine exhaustiveness receipt contract
r[valence.evidence_ir.state_machine_evidence.contract] Valence MUST define a canonical receipt contract for Octet-produced state-machine exhaustiveness evidence.

#### Scenario: Valid receipt names the checked transition domain
r[valence.evidence_ir.state_machine_evidence.contract.domain]
- GIVEN an Octet state-machine exhaustiveness receipt names a transition core, source span, state type identity, event type identity, variant inventory digest, coverage summary digest, status, diagnostics, artifact identity, evidence tier, and required non-claims
- WHEN Valence validates the receipt
- THEN validation MUST preserve those identities as bounded source-shape evidence.

#### Scenario: Missing domain identity fails
r[valence.evidence_ir.state_machine_evidence.contract.missing_domain]
- GIVEN a receipt omits the transition core, state type, event type, source span, or coverage summary identity
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic missing-domain diagnostic.

### Requirement: Positive state-machine evidence fixtures
r[valence.evidence_ir.state_machine_evidence.fixtures.positive] Valence MUST include positive fixtures for valid Octet state-machine exhaustiveness receipts.

#### Scenario: Valid Octet receipt passes
r[valence.evidence_ir.state_machine_evidence.fixtures.positive.valid]
- GIVEN a fixture contains a valid Octet-produced state-machine exhaustiveness receipt with source spans, domain identities, coverage summary digest, passing status, artifact BLAKE3 identity, parent export identity, and non-claims
- WHEN the state-machine evidence fixture suite runs
- THEN the fixture MUST pass and produce deterministic Evidence IR output.

### Requirement: Negative state-machine evidence fixtures
r[valence.evidence_ir.state_machine_evidence.fixtures.negative] Valence MUST reject malformed, stale, unsupported, or overclaiming state-machine exhaustiveness receipts.

#### Scenario: Missing non-claims fail
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.non_claims]
- GIVEN a state-machine receipt omits the bounded source-shape non-claim boundary
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic non-claim diagnostic.

#### Scenario: Stale coverage summary fails
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.stale]
- GIVEN a state-machine receipt carries a malformed or stale coverage summary digest or artifact BLAKE3 identity
- WHEN Valence validates the receipt
- THEN validation MUST fail with a deterministic digest diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.state_machine_evidence.fixtures.negative.overclaim]
- GIVEN a state-machine receipt claims behavioral correctness, semantic equivalence, verifier soundness, release eligibility, protocol completeness, liveness, or exhaustive Rust coverage
- WHEN Valence validates the receipt
- THEN validation MUST reject the receipt with a deterministic overclaim diagnostic.

### Requirement: Pure receipt validation core
r[valence.evidence_ir.state_machine_evidence.validation] Valence MUST validate state-machine exhaustiveness receipts through a pure core over parsed receipt values.

#### Scenario: Core owns validation decisions
r[valence.evidence_ir.state_machine_evidence.validation.core]
- GIVEN parsed state-machine receipt values
- WHEN validation runs
- THEN the pure core MUST decide acceptance or rejection without filesystem, process, network, clock, environment, or stdout side effects.

### Requirement: State-machine evidence documentation
r[valence.evidence_ir.state_machine_evidence.docs] Valence documentation MUST describe Octet producer responsibilities, Evidence IR validation boundaries, and consumer interpretation rules for state-machine exhaustiveness receipts.

#### Scenario: Consumer boundary is visible
r[valence.evidence_ir.state_machine_evidence.docs.boundary]
- GIVEN a consumer reads Valence documentation for state-machine evidence
- WHEN the supported claim is stated
- THEN the documentation MUST say Valence validates receipt shape, identity, linkage, and non-claims only, not Rust semantics or release readiness.

### Requirement: Final validation
r[valence.evidence_ir.state_machine_evidence.final_validation] The change MUST include focused core tests, positive and negative fixtures, formatting checks, and Cairn validation/gate evidence before archive.

#### Scenario: Validation covers receipt health
r[valence.evidence_ir.state_machine_evidence.final_validation.checks]
- GIVEN the receipt contract, fixtures, validation core, proof-link handling, and documentation are complete
- WHEN final validation runs
- THEN focused tests, fixture checks, formatting, and Cairn gates MUST pass or record explicit pre-existing blockers.

### Requirement: ChaosControl Evidence IR export contract
r[valence.evidence_ir.chaoscontrol_export.contract] Valence MUST support an adapter contract for selected ChaosControl evidence families without taking ownership of VMM, replay, or determinism semantics.

#### Scenario: Supported ChaosControl export passes
r[valence.evidence_ir.chaoscontrol_export.fixtures.positive]
- GIVEN an export row maps a ChaosControl replay verdict, dogfood receipt, assertion summary, or readiness report to a Valence Evidence IR artifact with schema, stable ID, canonical hash, verification role, payload digest, and required non-claims
- WHEN adapter validation runs
- THEN the row MUST pass and preserve the ChaosControl artifact identity.

#### Scenario: Invalid ChaosControl export fails
r[valence.evidence_ir.chaoscontrol_export.fixtures.negative]
- GIVEN an export row has malformed hash, unsupported schema, missing non-claims, or attempts to promote recorded evidence to property evidence
- WHEN adapter validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Adapter validation boundary
r[valence.evidence_ir.chaoscontrol_export.validation] Valence MUST validate identity, role, schema, and non-claim boundaries for ChaosControl exports without claiming replay or VMM correctness.

#### Scenario: Boundary is visible
r[valence.evidence_ir.chaoscontrol_export.docs]
- GIVEN a ChaosControl export validates
- WHEN a report renders the supported claim
- THEN it MUST state that Valence proves evidence identity/linkage only and does not prove universal determinism, hosted-product parity, replay completeness, or VMM correctness.

### Requirement: Final validation
r[valence.evidence_ir.chaoscontrol_export.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers export health
r[valence.evidence_ir.chaoscontrol_export.final_validation.fixtures]
- GIVEN valid and invalid ChaosControl export fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.

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

### Requirement: Oxide primitive reference inventory
r[valence.evidence_ir.oxide_primitives.reference_inventory] Valence MUST maintain a reference-only inventory for each Oxide primitive or attestation repository used to shape Evidence IR primitives.

#### Scenario: Reference intake records boundaries
r[valence.evidence_ir.oxide_primitives.reference_inventory.boundaries]
- GIVEN an Oxide repository is evaluated for Valence adoption
- WHEN the intake record is written
- THEN it MUST name the source repository, license posture, MSRV/no-std posture when relevant, intended adaptation, and explicit non-adoption boundaries.

#### Scenario: Reference intake rejects hidden authority
r[valence.evidence_ir.oxide_primitives.reference_inventory.reject_hidden_authority]
- GIVEN an external repository has a useful implementation pattern
- WHEN Valence records it as a reference
- THEN the record MUST NOT treat that repository as proof of Evidence IR correctness, release readiness, or attestation truth.

### Requirement: Typed evidence identifiers
r[valence.evidence_ir.oxide_primitives.typed_ids] Valence MUST distinguish stack-owned BLAKE3 evidence identities from UUID-compatible external identifiers through typed wrappers or equivalent type-level roles.

#### Scenario: BLAKE3 identity remains canonical
r[valence.evidence_ir.oxide_primitives.typed_ids.blake3_canonical]
- GIVEN a Valence artifact has both a stack-owned canonical identity and an external UUID-compatible identifier
- WHEN identity validation runs
- THEN the canonical evidence identity MUST remain the BLAKE3-bound value and the UUID-compatible identifier MUST remain external metadata.

#### Scenario: Role mismatch fails closed
r[valence.evidence_ir.oxide_primitives.typed_ids.role_mismatch]
- GIVEN an identifier for one evidence role is supplied where another role is required
- WHEN typed validation compares the identifiers
- THEN validation MUST fail closed with deterministic diagnostics.

### Requirement: Derived-key evidence collections
r[valence.evidence_ir.oxide_primitives.identity_maps] Valence SHOULD support in-memory evidence collections whose lookup keys are derived from values while rejecting duplicate or inconsistent keys.

#### Scenario: Duplicate derived key is rejected
r[valence.evidence_ir.oxide_primitives.identity_maps.duplicate_rejection]
- GIVEN two evidence records derive the same canonical key
- WHEN the collection is constructed
- THEN construction MUST reject the duplicate instead of silently overwriting either record.

#### Scenario: Borrowed and owned lookup forms agree
r[valence.evidence_ir.oxide_primitives.identity_maps.lookup_consistency]
- GIVEN a collection supports borrowed and owned lookup forms for the same key profile
- WHEN both lookup forms target the same evidence record
- THEN both lookups SHOULD resolve to the same record or both fail with the same deterministic issue code.

### Requirement: Semantic evidence diffs
r[valence.evidence_ir.oxide_primitives.semantic_diffs] Valence SHOULD provide semantic diff reports for reviewable Evidence IR migrations without claiming semantic equivalence.

#### Scenario: Structural diff is review evidence
r[valence.evidence_ir.oxide_primitives.semantic_diffs.review_only]
- GIVEN two Evidence IR values differ structurally
- WHEN Valence emits a semantic diff report
- THEN the report SHOULD identify added, removed, and changed typed fields and MUST state that it is review evidence only.

#### Scenario: Stale baseline is rejected
r[valence.evidence_ir.oxide_primitives.semantic_diffs.stale_baseline]
- GIVEN a stored diff baseline no longer matches the current before-value identity
- WHEN a diff refresh is requested
- THEN Valence MUST reject the refresh until the caller supplies the current baseline identity.

### Requirement: Attestation vocabulary mapping
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary] Valence MAY map attestation vocabulary from CoRIM, VM attestation, DICE, or certificate-chain references into bounded Evidence IR metadata.

#### Scenario: Attestation metadata has explicit scope
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary.scope]
- GIVEN an attestation-shaped artifact is imported
- WHEN Valence normalizes its metadata
- THEN the normalized record MUST include issuer, subject, scope, digest binding, and non-claim boundary fields.

#### Scenario: Missing boundary fails closed
r[valence.evidence_ir.oxide_primitives.attestation_vocabulary.missing_boundary]
- GIVEN imported attestation metadata omits scope or non-claim fields
- WHEN Valence validates the record
- THEN validation MUST fail closed and MUST NOT infer trust from repository origin.

### Requirement: Fixture-backed adoption
r[valence.evidence_ir.oxide_primitives.validation] Valence MUST require positive and negative fixture coverage before adopting any Oxide-inspired primitive into maintained core behavior.

#### Scenario: Positive fixtures cover accepted behavior
r[valence.evidence_ir.oxide_primitives.validation.positive]
- GIVEN typed IDs, derived-key collections, semantic diffs, and attestation metadata are accepted
- WHEN the fixture matrix runs
- THEN valid examples MUST pass with stable BLAKE3-bound identities and deterministic reports.

#### Scenario: Negative fixtures cover failure behavior
r[valence.evidence_ir.oxide_primitives.validation.negative]
- GIVEN malformed IDs, duplicate keys, stale diffs, missing issuers, weakened non-claims, or overclaim text are supplied
- WHEN the fixture matrix runs
- THEN invalid examples MUST fail with deterministic diagnostics and no partial acceptance.

### Requirement: Preserves evidence core contract
r[valence.evidence_ir.preserves_evidence_core.contract] Valence MUST define pure Preserves evidence contracts for canonicalization profiles, envelope schema/version metadata, `preserves_content_hash`, self-hash exclusion profiles, and required non-claims.

#### Scenario: Valid Preserves evidence validates
r[valence.evidence_ir.preserves_evidence_core.fixtures.positive]
- GIVEN a Preserves evidence envelope uses a supported schema, supported canonicalization profile, matching content hash, declared self-hash exclusion profile, and required non-claim boundary
- WHEN Preserves evidence validation runs
- THEN validation MUST accept the envelope and report the normalized content identity.

#### Scenario: Invalid Preserves evidence fails closed
r[valence.evidence_ir.preserves_evidence_core.fixtures.negative]
- GIVEN a Preserves evidence envelope has malformed Preserves text, a stale content hash, an unknown schema, a wrong exclusion profile, missing non-claims, or overclaim text
- WHEN Preserves evidence validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid field.

### Requirement: Preserves self-hash exclusion profiles
r[valence.evidence_ir.preserves_evidence_core.self_hash_exclusion] Valence MUST require named self-hash exclusion profiles for receipt-shaped Preserves evidence.

#### Scenario: Self-reference is excluded only by profile
r[valence.evidence_ir.preserves_evidence_core.self_hash_exclusion.profile]
- GIVEN a receipt-shaped Preserves value includes fields that name its own content hash or receipt hash
- WHEN the declared exclusion profile builds hash material
- THEN only fields declared by that profile MUST be excluded and ordinary content fields MUST remain included.

### Requirement: Preserves evidence validation is pure
r[valence.evidence_ir.preserves_evidence_core.validation] Valence MUST implement Preserves evidence identity validation as pure deterministic logic over in-memory Preserves values, envelope metadata, canonical bytes, and hash profiles.

#### Scenario: Shell owns Preserves loading
r[valence.evidence_ir.preserves_evidence_core.validation.shell]
- GIVEN a CLI or downstream crate loads Preserves text, generated contracts, or evidence files from disk
- WHEN Preserves evidence validation is invoked
- THEN file reads, Nickel evaluation, stdout/stderr, process execution, and receipt writing MUST remain outside the Preserves evidence core.

### Requirement: Compatibility adapters preserve public fields
r[valence.evidence_ir.preserves_evidence_core.adapters] Valence SHOULD expose compatibility adapters for existing Cairn, Mantle, Octet, and Valence Preserves evidence shapes.

#### Scenario: Adapter keeps existing field names
r[valence.evidence_ir.preserves_evidence_core.adapters.shape]
- GIVEN a downstream Preserves evidence record uses current public field names
- WHEN an adapter validates that record
- THEN the adapter SHOULD preserve the public DTO shape while routing identity checks through the shared core.

### Requirement: Preserves evidence core remains bounded
r[valence.evidence_ir.preserves_evidence_core.docs] Passing Preserves evidence validation MUST NOT prove receipt truth, release eligibility, lifecycle readiness, semantic equivalence, behavioral correctness, verifier soundness, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.preserves_evidence_core.final_validation]
- GIVEN a Preserves evidence validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves only canonical Preserves content identity and bounded envelope compatibility.

### Requirement: Preserves bridge contract
r[valence.evidence_ir.preserves_bridge.contract] Valence MUST support an adapter contract that maps canonical Preserves evidence envelopes to Valence Evidence IR artifact identity fields.

#### Scenario: Canonical Preserves envelope maps to Valence artifact
r[valence.evidence_ir.preserves_bridge.fixtures.positive]
- GIVEN a Preserves envelope has a supported schema label, canonical byte digest, artifact role, verification role, and required non-claims
- WHEN bridge validation runs
- THEN validation MUST pass and preserve the canonical Preserves digest.

#### Scenario: Invalid Preserves envelope fails
r[valence.evidence_ir.preserves_bridge.fixtures.negative]
- GIVEN a Preserves envelope has stale digest, unsupported role, malformed schema label, or missing non-claims
- WHEN bridge validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Bridge validation boundary
r[valence.evidence_ir.preserves_bridge.validation] The bridge MUST validate identity, schema, role, and non-claim mapping only.

#### Scenario: Boundary is visible
r[valence.evidence_ir.preserves_bridge.docs]
- GIVEN a Preserves bridge report passes
- WHEN the supported claim is rendered
- THEN it MUST state that Valence does not prove Molten runtime authority, transport correctness, or Preserves application semantics.

### Requirement: Final validation
r[valence.evidence_ir.preserves_bridge.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers bridge health
r[valence.evidence_ir.preserves_bridge.final_validation.fixtures]
- GIVEN valid and invalid bridge fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.

### Requirement: Shared evidence identity contract
r[valence.evidence_ir.shared_identity.contract] Valence MUST provide pure evidence-identity contracts for BLAKE3 digest spellings, digest roles, canonical hash profiles, self-referential hash-field exclusion, and reusable non-claim boundary checks.

#### Scenario: Supported digest spellings normalize by profile
r[valence.evidence_ir.shared_identity.digest_profiles]
- GIVEN a raw BLAKE3 hex digest, a `b3:` digest, or a `blake3:` digest accepted by a compatibility profile
- WHEN Valence parses the value for a declared role
- THEN parsing MUST produce a normalized digest value or fail closed if the spelling is unsupported for that profile.

#### Scenario: Digest roles are not interchangeable
r[valence.evidence_ir.shared_identity.digest_roles]
- GIVEN a binary digest, sidecar digest, artifact digest, receipt digest, and Preserves metadata digest have the same underlying bytes
- WHEN typed validation compares evidence roles
- THEN validation MUST preserve the declared role and MUST NOT silently treat one role as another.

### Requirement: Canonical hash profiles
r[valence.evidence_ir.shared_identity.canonical_hashes] Valence MUST compute content identities through explicit canonical hash profiles rather than ambient display serialization.

#### Scenario: Canonical JSON hash is stable
r[valence.evidence_ir.shared_identity.fixtures.positive]
- GIVEN two equivalent typed JSON values with different map insertion order
- WHEN the shared identity core hashes them through the declared canonical JSON profile
- THEN the computed BLAKE3 identity MUST be stable for both values.

#### Scenario: Unsupported canonical profile fails closed
r[valence.evidence_ir.shared_identity.fixtures.negative]
- GIVEN a caller requests an unknown canonicalization profile or supplies malformed digest text
- WHEN the shared identity core validates the input
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Self-hash exclusion profiles
r[valence.evidence_ir.shared_identity.self_hash_exclusion] Valence MUST support named self-hash exclusion profiles for receipt-shaped values whose own digest fields are excluded from their hash material.

#### Scenario: Receipt hash excludes self-reference
r[valence.evidence_ir.shared_identity.self_hash_exclusion.receipt]
- GIVEN a receipt-shaped value has `receipt_hash` or `preserves_content_hash` fields
- WHEN the declared receipt exclusion profile computes hash material
- THEN those self-referential fields MUST be excluded and ordinary content fields MUST remain included.

#### Scenario: Unknown exclusion is rejected
r[valence.evidence_ir.shared_identity.self_hash_exclusion.unknown]
- GIVEN a caller requests an unknown exclusion profile or tries to exclude undeclared fields
- WHEN the shared identity core builds hash material
- THEN validation MUST fail closed with a deterministic issue code.

### Requirement: Compatibility adapters preserve existing contracts
r[valence.evidence_ir.shared_identity.compatibility_adapters] Valence SHOULD expose compatibility adapters for existing stack surfaces while preserving public field names, digest spellings, and bounded claim text.

#### Scenario: Downstream adapter preserves public shape
r[valence.evidence_ir.shared_identity.compatibility_adapters.shape]
- GIVEN an existing Cairn, Mantle, Basalt, Octet, or Molten/Aspen evidence surface uses a supported digest spelling and field name
- WHEN a compatibility adapter validates that surface
- THEN the adapter SHOULD preserve the public DTO shape while routing digest and non-claim validation through shared identity primitives.

### Requirement: Shared identity remains bounded
r[valence.evidence_ir.shared_identity.docs] Shared identity helpers MUST NOT claim evidence truth, behavioral correctness, semantic equivalence, verifier soundness, release eligibility, lifecycle readiness, or whole-program safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.shared_identity.final_validation]
- GIVEN a shared identity validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that shared identity proves only syntax, canonical content identity, role separation, and non-claim boundary conformance for the selected profile.

### Requirement: Stack role registry core contract
r[valence.evidence_ir.stack_role_registry_core.contract] Valence MUST define pure stack role registry contracts for canonical stack roles, schema role bindings, verification role vocabulary, compatibility aliases, and bounded non-claim text.

#### Scenario: Canonical role row validates
r[valence.evidence_ir.stack_role_registry_core.fixtures.positive]
- GIVEN a registry row names a canonical stack role, supported schema, verification role, compatibility class, and non-claim boundary
- WHEN role registry validation runs
- THEN validation MUST accept the row and preserve the public role encoding.

#### Scenario: Invalid role row fails closed
r[valence.evidence_ir.stack_role_registry_core.fixtures.negative]
- GIVEN a registry row has an unknown role, duplicate required role, stale alias, schema/role mismatch, missing verification role, missing non-claim, or authority overclaim
- WHEN role registry validation runs
- THEN validation MUST fail with deterministic diagnostics naming the invalid role or schema field.

### Requirement: Compatibility adapters preserve public role strings
r[valence.evidence_ir.stack_role_registry_core.adapters] Valence SHOULD provide adapters for existing stack role strings without silently broadening their meaning.

#### Scenario: Legacy role spelling maps through an alias row
r[valence.evidence_ir.stack_role_registry_core.adapters.alias]
- GIVEN a downstream evidence record uses an approved legacy role spelling
- WHEN the adapter parses the record
- THEN parsing MUST require a compatibility alias row that names the canonical replacement, owner, status, and non-claim boundary.

### Requirement: Role registry validation is pure core
r[valence.evidence_ir.stack_role_registry_core.validation] Stack role registry validation MUST be pure deterministic logic over in-memory registry rows and evidence rows.

#### Scenario: Shell owns policy loading
r[valence.evidence_ir.stack_role_registry_core.validation.shell_boundary]
- GIVEN a CLI or downstream crate loads role policy from files or generated artifacts
- WHEN registry validation is invoked
- THEN file reads, environment access, process execution, stdout/stderr, and receipt writing MUST remain outside the role registry core.

### Requirement: Role registry remains bounded
r[valence.evidence_ir.stack_role_registry_core.docs] Passing stack role registry validation MUST NOT prove evidence truth, verifier soundness, source-code correctness, release eligibility, lifecycle readiness, runtime authority, or whole-system safety.

#### Scenario: Boundary is visible
r[valence.evidence_ir.stack_role_registry_core.final_validation]
- GIVEN a role registry validation report passes
- WHEN an operator reads the supported claim
- THEN the report MUST state that validation proves role/schema/vocabulary compatibility only.

### Requirement: Verification role vocabulary contract
r[valence.evidence_ir.verification_role_vocabulary.contract] Valence MUST define the canonical vocabulary for Evidence IR verification roles, tier mappings, counting behavior, and non-claim boundaries.

#### Scenario: Known roles pass
r[valence.evidence_ir.verification_role_vocabulary.fixtures.positive]
- GIVEN a role row declares a known role, allowed tier mapping, counting behavior, and non-claims
- WHEN vocabulary validation runs
- THEN validation MUST pass and preserve the role identity.

#### Scenario: Unknown role fails
r[valence.evidence_ir.verification_role_vocabulary.fixtures.negative]
- GIVEN a consumer declares an unknown role, role-counting drift, tier promotion, or missing non-claims
- WHEN vocabulary validation runs
- THEN validation MUST fail with deterministic diagnostics.

### Requirement: Vocabulary constants
r[valence.evidence_ir.verification_role_vocabulary.constants] Valence SHOULD provide generated constants or drift validation so consumers do not copy role literals unchecked.

#### Scenario: Consumer constants match
r[valence.evidence_ir.verification_role_vocabulary.constants.match]
- GIVEN a consumer validates local constants against the vocabulary digest
- WHEN constants match
- THEN validation SHOULD pass and record the vocabulary digest.

### Requirement: Role docs
r[valence.evidence_ir.verification_role_vocabulary.docs] Documentation MUST state which roles count as verified property evidence and which remain recorded-only, boundary, or manual-review context.

#### Scenario: Counting boundary is visible
r[valence.evidence_ir.verification_role_vocabulary.docs.counting]
- GIVEN a report summarizes verification roles
- WHEN the supported claim is rendered
- THEN it MUST state that only configured property roles count as verified property evidence.

### Requirement: Final validation
r[valence.evidence_ir.verification_role_vocabulary.final_validation] The change MUST include positive and negative fixtures plus focused validation evidence before archive.

#### Scenario: Fixture suite covers role health
r[valence.evidence_ir.verification_role_vocabulary.final_validation.fixtures]
- GIVEN valid and invalid role fixtures
- WHEN focused validation runs
- THEN valid fixtures MUST pass and invalid fixtures MUST fail closed.

### Requirement: FSM default-exhaustive evidence profile
r[valence.evidence_ir.fsm_default_profile.contract] Valence MUST define an `fsm-default-exhaustive` Evidence IR profile for bounded state-machine evidence that binds closed state/event universes, transition coverage, invalid-transition denial coverage, producer identities, source spans, artifact identities, roles, and required non-claims.

#### Scenario: Valid profile preserves producer boundaries
r[valence.evidence_ir.fsm_default_profile.contract.boundaries]
- GIVEN an FSM default-exhaustive evidence record names Octet lint/provenance evidence and optionally Kamacite FSM spec evidence
- WHEN Valence validates the record
- THEN validation MUST preserve Octet as source-shape producer, Kamacite as portable schema/receipt producer when present, and Valence as identity/linkage/non-claim verifier.

#### Scenario: Closed universes are bound
r[valence.evidence_ir.fsm_default_profile.contract.universes]
- GIVEN an FSM default-exhaustive evidence record declares closed state and event universe identities
- WHEN Valence validates the record
- THEN validation MUST require state universe identity, event universe identity, variant inventory identity, transition coverage identity, and invalid-transition denial coverage identity.

### Requirement: Pure FSM evidence validation
r[valence.evidence_ir.fsm_default_profile.validation] Valence MUST validate FSM default-exhaustive evidence through pure deterministic logic over parsed in-memory values.

#### Scenario: Core has no shell side effects
r[valence.evidence_ir.fsm_default_profile.validation.pure]
- GIVEN parsed FSM default-exhaustive evidence values
- WHEN validation runs
- THEN the pure core MUST decide acceptance or rejection without filesystem, process, network, clock, environment, or stdout side effects.

### Requirement: Positive FSM default-exhaustive fixtures
r[valence.evidence_ir.fsm_default_profile.fixtures.positive] Valence MUST include positive fixtures for valid FSM default-exhaustive evidence records.

#### Scenario: Octet-only receipt passes
r[valence.evidence_ir.fsm_default_profile.fixtures.positive.octet_only]
- GIVEN a fixture contains an Octet-produced FSM default-exhaustive receipt with source spans, closed universe identities, transition coverage, denial coverage, artifact BLAKE3 identity, parent export identity, and non-claims
- WHEN the FSM default-exhaustive fixture suite runs
- THEN the fixture MUST pass and produce deterministic Evidence IR output.

#### Scenario: Octet plus Kamacite linked receipt passes
r[valence.evidence_ir.fsm_default_profile.fixtures.positive.kamacite_link]
- GIVEN a fixture contains Octet source-shape evidence linked to a Kamacite FSM spec and receipt identity with preserved non-claims
- WHEN Valence validates the linked evidence
- THEN validation MUST preserve both producer identities and report the linkage without treating Kamacite or Octet evidence as runtime correctness proof.

### Requirement: Negative FSM default-exhaustive fixtures
r[valence.evidence_ir.fsm_default_profile.fixtures.negative] Valence MUST reject malformed, stale, unsupported, or overclaiming FSM default-exhaustive evidence records.

#### Scenario: Missing denial coverage fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.denial_coverage]
- GIVEN an FSM default-exhaustive record omits invalid-transition denial coverage identity
- WHEN Valence validates the record
- THEN validation MUST fail with a deterministic missing-denial-coverage diagnostic.

#### Scenario: Stale universe or coverage identity fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.stale]
- GIVEN an FSM default-exhaustive record carries malformed or stale universe, inventory, coverage, denial coverage, artifact, or parent export identity
- WHEN Valence validates the record
- THEN validation MUST fail with deterministic digest or identity-domain diagnostics.

#### Scenario: Missing non-claims fail
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.non_claims]
- GIVEN an FSM default-exhaustive record omits the required bounded evidence non-claims
- WHEN Valence validates the record
- THEN validation MUST fail with a deterministic non-claim diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.fsm_default_profile.fixtures.negative.overclaim]
- GIVEN an FSM default-exhaustive record claims behavioral correctness, protocol completeness, liveness, verifier soundness, release eligibility, or exhaustive Rust coverage
- WHEN Valence validates the record
- THEN validation MUST reject the record with a deterministic overclaim diagnostic.

### Requirement: Formal proof links remain separate
r[valence.evidence_ir.fsm_default_profile.proof_links] Valence MUST preserve optional formal proof links for FSM invariants as separate evidence records and MUST NOT promote the FSM default-exhaustive receipt tier.

#### Scenario: Linked proof does not promote FSM receipt
r[valence.evidence_ir.fsm_default_profile.proof_links.no_promotion]
- GIVEN a valid FSM default-exhaustive record links to a Verus, Kani, or Trellis proof receipt
- WHEN Valence builds or verifies the evidence graph
- THEN the FSM receipt MUST remain bounded source-shape/schema evidence while the proof receipt retains its own verifier identity and tier.

### Requirement: FSM default-exhaustive documentation
r[valence.evidence_ir.fsm_default_profile.docs] Valence documentation MUST describe Octet, Kamacite, and Valence ownership boundaries for FSM default-exhaustive evidence.

#### Scenario: Consumer boundary is visible
r[valence.evidence_ir.fsm_default_profile.docs.boundary]
- GIVEN a consumer reads Valence documentation for FSM default-exhaustive evidence
- WHEN the supported claim is stated
- THEN the documentation MUST say Valence validates identity, linkage, role, digest, coverage-record shape, and non-claims only, not Rust semantics, FSM semantics, liveness, or release readiness.

### Requirement: FSM default-exhaustive validation evidence
r[valence.evidence_ir.fsm_default_profile.final_validation] The FSM default-exhaustive profile change MUST include focused positive and negative tests plus Cairn validation evidence before archive.

#### Scenario: Focused validation covers pass and fail paths
r[valence.evidence_ir.fsm_default_profile.final_validation.focused]
- GIVEN implementation is complete
- WHEN the change is prepared for archive
- THEN saved evidence MUST identify focused core tests, positive and negative fixture coverage, and Cairn proposal/design/tasks validation results.

### Requirement: Function-address profile contract

r[valence.evidence_ir.function_address_profile.contract] Valence MUST define a function-address Evidence IR profile that validates Kamacite function-address receipt identity, Octet producer identity, source-envelope identities, function-address identities, optional dependency-closure identities, source spans, artifact roles, verification roles, and explicit non-claims.

#### Scenario: Valid profile preserves producer boundaries
r[valence.evidence_ir.function_address_profile.contract.boundaries]
- GIVEN a function-address evidence record names Octet producer facts and a Kamacite function-address receipt
- WHEN Valence validates the record
- THEN validation MUST preserve Octet as source-shape producer, Kamacite as portable receipt owner, and Valence as identity/linkage/non-claim verifier.

#### Scenario: Profile keeps identity domains distinct
r[valence.evidence_ir.function_address_profile.contract.identity_domains]
- GIVEN a record contains source-envelope identity, function address, dependency-closure identity, Kamacite receipt identity, Valence projection identity, sidecar digest, and release-binary identity
- WHEN Valence validates the record
- THEN each BLAKE3 identity MUST remain in its declared domain and MUST NOT be substituted for another domain.

### Requirement: Profile validation fails closed

r[valence.evidence_ir.function_address_profile.validation] Valence MUST fail closed when function-address profile records contain malformed BLAKE3 digests, stale identity links, unsupported profile versions, unknown roles, missing required source-span linkage, missing non-claims, weakened non-claims, or overclaim text.

#### Scenario: Stale receipt identity fails
r[valence.evidence_ir.function_address_profile.validation.stale_receipt]
- GIVEN a Valence profile record declares a Kamacite receipt identity that does not match the supplied receipt metadata
- WHEN Valence validates the record
- THEN validation MUST fail with a stale-receipt-identity diagnostic.

#### Scenario: Wrong domain substitution fails
r[valence.evidence_ir.function_address_profile.validation.domain_substitution]
- GIVEN a function address is supplied where a source-envelope identity or release-binary identity is required
- WHEN Valence validates the record
- THEN validation MUST fail with an identity-domain-substitution diagnostic.

#### Scenario: Overclaim fails
r[valence.evidence_ir.function_address_profile.validation.overclaim]
- GIVEN a function-address profile record claims behavioral correctness, Rust semantic correctness, compiler correctness, verifier soundness, whole-program safety, or release eligibility
- WHEN Valence validates the record
- THEN validation MUST fail with an overclaim diagnostic.

### Requirement: Positive function-address fixtures

r[valence.evidence_ir.function_address_profile.fixtures.positive] Valence MUST include positive fixtures for Octet-only producer records, Octet-plus-Kamacite linked receipts, dependency-edge records, source-span preservation, and optional Mantle release sidecar linkage.

#### Scenario: Octet and Kamacite linked receipt passes
r[valence.evidence_ir.function_address_profile.fixtures.positive.kamacite_link]
- GIVEN a fixture contains Octet producer evidence linked to a Kamacite function-address receipt with preserved spans, dependencies, roles, and non-claims
- WHEN Valence validates the linked evidence
- THEN validation MUST pass and report the linkage without treating the receipt as runtime correctness proof.

#### Scenario: Optional Mantle linkage is preserved
r[valence.evidence_ir.function_address_profile.fixtures.positive.mantle_link]
- GIVEN a function-address profile record includes optional Mantle release sidecar metadata
- WHEN Valence validates the record
- THEN the linkage MUST be preserved as bundle-local artifact context and MUST NOT become release-readiness proof.

### Requirement: Negative function-address fixtures

r[valence.evidence_ir.function_address_profile.fixtures.negative] Valence MUST include negative fixtures for malformed digests, stale identities, wrong identity domains, unsupported versions, unknown roles, missing non-claims, overclaims, and attempted formal-proof promotion.

#### Scenario: Invalid fixture suite fails closed
r[valence.evidence_ir.function_address_profile.fixtures.negative.suite]
- GIVEN invalid function-address profile fixtures cover the rejected classes
- WHEN the fixture suite runs
- THEN each fixture MUST fail with deterministic diagnostics naming the invalid field, role, identity domain, or non-claim boundary.

### Requirement: Verification-role preservation

r[valence.evidence_ir.function_address_profile.roles] Function-address profile validation MUST preserve verification-role boundaries. Function-address receipts MUST remain recorded-only or boundary evidence unless linked to a separate formal proof receipt with its own verifier identity and tier.

#### Scenario: Function address is not formal proof
r[valence.evidence_ir.function_address_profile.roles.no_promotion]
- GIVEN a function-address receipt is valid and no separate proof receipt is supplied
- WHEN Valence validates the profile
- THEN the receipt MUST NOT be counted as formal proof or behavioral correctness evidence.

#### Scenario: Separate proof link remains separate
r[valence.evidence_ir.function_address_profile.roles.proof_link]
- GIVEN a function-address record links to a separate Trellis, Verus, Kani, or other formal proof receipt
- WHEN Valence validates the profile
- THEN the proof receipt MUST retain its own verifier identity and tier without promoting the function-address receipt itself.

### Requirement: Graph-report integration

r[valence.evidence_ir.function_address_profile.graph] Valence SHOULD include function-address nodes and typed edges in stack graph reports when supplied, including edges to Octet producer evidence, Kamacite receipts, source spans, dependency identities, Mantle sidecars, and Cairn gate inputs.

#### Scenario: Graph report preserves typed edges
r[valence.evidence_ir.function_address_profile.graph.typed_edges]
- GIVEN a valid function-address profile record with producer, receipt, source, dependency, and sidecar edges
- WHEN Valence emits a graph report
- THEN the report SHOULD preserve the typed edges and identity domains without claiming release eligibility.

### Requirement: Documentation and final validation

r[valence.evidence_ir.function_address_profile.docs] Valence documentation MUST describe function-address profile ownership boundaries and non-claims. r[valence.evidence_ir.function_address_profile.final_validation] The change MUST include positive and negative fixtures, focused core tests, and lifecycle validation before archive.

#### Scenario: Boundary is visible
r[valence.evidence_ir.function_address_profile.docs.boundary]
- GIVEN an operator reads Valence function-address profile documentation
- WHEN the supported claim is described
- THEN the documentation MUST state that Valence validates identity, linkage, roles, digest domains, spans, and non-claims only; Octet owns Rust source analysis, Kamacite owns portable receipts, Mantle owns bundle-local binding, and Cairn owns lifecycle policy.

#### Scenario: Final validation covers fixtures
r[valence.evidence_ir.function_address_profile.final_validation.fixtures]
- GIVEN the function-address profile implementation is complete
- WHEN focused validation runs
- THEN valid fixtures MUST pass, invalid fixtures MUST fail closed, formatting and lifecycle validation MUST pass, and saved evidence MUST identify the checked profile identities.

### Requirement: Nickel contracts cover the supported Cairn policy field set
r[valence.evidence_ir.cairn_policy_compatibility.nickel_contract] Valence MUST keep its Nickel policy contract compatible with every required field in the supported Cairn policy schema before exporting generated JSON.

#### Scenario: Current policy contract exports
r[valence.evidence_ir.cairn_policy_compatibility.fixtures]
- GIVEN the reviewed Valence Nickel policy includes the complete supported field set
- WHEN Cairn exports and validates the policy
- THEN export and policy parsing MUST succeed without missing-field diagnostics.

#### Scenario: Missing newly required field fails in Nickel
- GIVEN a policy fixture omits a required supported Cairn field
- WHEN Nickel contract evaluation runs
- THEN evaluation MUST fail before stale generated JSON is accepted.

### Requirement: Function-address lifecycle policy is explicit
r[valence.evidence_ir.cairn_policy_compatibility.function_address_gate] Valence MUST declare a reviewed function-address gate mode, allowed claim scopes, accepted canonical Valence policy identities, required roles, required schemas, and bounded non-claim text in Nickel.

#### Scenario: Reviewed profile identity is accepted
- GIVEN the current canonical Valence function-address profile identity and required recorded-only role/schema set
- WHEN policy validation runs
- THEN the policy MUST accept the reviewed identity without promoting function-address evidence to property proof or release eligibility.

#### Scenario: Placeholder or malformed policy identity fails
- GIVEN the accepted identity list contains a fixture placeholder, malformed BLAKE3 value, or stale unreviewed profile identity
- WHEN policy validation runs
- THEN validation MUST fail with a deterministic function-address policy diagnostic.

### Requirement: Generated Cairn policy comes only from Nickel
r[valence.evidence_ir.cairn_policy_compatibility.generated_policy] Valence MUST regenerate `cairn-policy/generated/cairn-policy.json` from the contracted Nickel source and update policy field-set compatibility metadata in the same reviewed change.

#### Scenario: Direct generated-file drift is rejected
r[valence.evidence_ir.cairn_policy_compatibility.freshness]
- GIVEN generated JSON differs from a fresh export of the Nickel source
- WHEN policy freshness validation runs
- THEN validation MUST fail and identify the generated policy as stale.

### Requirement: Default lifecycle validation parses repository policy
r[valence.evidence_ir.cairn_policy_compatibility.validation] The supported Cairn binary MUST parse Valence's default generated policy and run lifecycle validation without requiring an external substitute policy.

#### Scenario: Documented validation command reaches lifecycle checks
- GIVEN the generated policy is fresh
- WHEN the repo-documented default `cairn validate --root .` command runs
- THEN Cairn MUST scan specs and changes and MUST NOT abort on policy parsing.

### Requirement: Policy compatibility boundary is documented
r[valence.evidence_ir.cairn_policy_compatibility.docs] Valence documentation MUST identify the supported Cairn policy contract, Nickel refresh workflow, selected function-address mode, and the boundary between lifecycle policy compatibility and product evidence truth.

#### Scenario: Parse success is not promoted to evidence truth
- GIVEN the policy parses and lifecycle validation runs
- WHEN compatibility evidence is reported
- THEN Valence MUST state that the result proves configuration schema/value compatibility only, not correctness, verifier soundness, or release eligibility.

### Requirement: Policy compatibility remediation is verified
r[valence.evidence_ir.cairn_policy_compatibility.final_validation] The change MUST include positive and negative Nickel fixtures, generated-policy freshness evidence, supported-Cairn parse evidence, and default lifecycle validation evidence.

#### Scenario: Reproduced parser failure is closed
- GIVEN the repository uses its default policy path with the supported Cairn checkout
- WHEN focused policy checks and repository validation run
- THEN the policy MUST parse, malformed function-address fixtures MUST fail, and active lifecycle changes MUST be evaluated.

### Requirement: Preserves hash material is structurally derived
r[valence.evidence_ir.preserves_integrity.canonical_hash] Valence MUST derive Preserves evidence hash material by parsing the envelope, removing only schema-declared top-level self-hash fields, canonically encoding the remaining typed value, and hashing those canonical bytes with BLAKE3.

#### Scenario: Canonical envelope identity is stable
r[valence.evidence_ir.preserves_integrity.fixtures.positive]
- GIVEN equivalent typed Preserves envelopes with different non-canonical renderings
- WHEN corrected-profile identity is computed
- THEN the canonical packed bytes and BLAKE3 identity MUST match.

#### Scenario: Payload field-name text remains hash-bearing
r[valence.evidence_ir.preserves_integrity.fixtures.negative]
- GIVEN payload content contains `receipt_hash` or `preserves_content_hash` as a key or string value
- WHEN that payload content changes
- THEN the canonical hash material MUST change and validation MUST reject reuse of the prior digest.

#### Scenario: Malformed structure fails closed
- GIVEN an envelope cannot be parsed, has duplicate or misplaced self-hash fields, or violates the declared schema
- WHEN identity validation runs
- THEN validation MUST fail with deterministic structural diagnostics and MUST NOT fall back to rendered-line filtering.

### Requirement: Corrected hash profiles are versioned
r[valence.evidence_ir.preserves_integrity.profile_version] Valence MUST expose a distinct corrected hash-profile identity and MUST reject a receipt whose declared profile does not match the algorithm used to derive its identity.

#### Scenario: Legacy digest cannot claim corrected profile
- GIVEN a receipt carries a digest derived by the vulnerable rendered-line profile
- WHEN it declares the corrected profile
- THEN validation MUST fail with a profile-and-digest mismatch diagnostic.

### Requirement: Legacy migration is explicit and byte-bound
r[valence.evidence_ir.preserves_integrity.migration] Valence MUST migrate a legacy Preserves evidence identity only through an explicit adapter that validates the original envelope bytes and emits a receipt binding the legacy identity, corrected identity, migration profile, and non-claims.

#### Scenario: Missing original bytes block migration
- GIVEN a legacy receipt identity without its original envelope bytes
- WHEN migration is requested
- THEN migration MUST fail closed and MUST NOT synthesize a corrected identity from metadata alone.

### Requirement: Evidence claim authority is typed
r[valence.evidence_ir.typed_claim_boundary.contract] Valence MUST represent supported claim class, verification role, prohibited claim capabilities, and required boundary identifiers as typed policy values rather than granting authority through prose matching.

#### Scenario: Bounded identity claim passes
r[valence.evidence_ir.typed_claim_boundary.fixtures.positive]
- GIVEN an artifact declares an allowed identity/linkage claim class, compatible verification role, required boundary identifiers, and no prohibited capabilities
- WHEN typed claim validation runs
- THEN validation MUST pass while preserving the bounded claim.

#### Scenario: Unknown typed value fails closed
r[valence.evidence_ir.typed_claim_boundary.fixtures.negative]
- GIVEN an artifact declares an unknown claim class, role, boundary identifier, or capability
- WHEN typed claim validation runs
- THEN validation MUST fail with a deterministic unsupported-value diagnostic.

### Requirement: Contradictory overclaims are rejected
r[valence.evidence_ir.typed_claim_boundary.validation] Valence MUST reject records whose typed fields or human-readable metadata assert a capability stronger than the supported typed claim, even when required non-claim phrases are also present.

#### Scenario: Contradictory non-claim text cannot pass
- GIVEN an artifact includes the required identity-only boundary text and also asserts correctness, semantic equivalence, authority, verifier soundness, or release eligibility
- WHEN claim validation runs
- THEN validation MUST fail with a deterministic prohibited-capability diagnostic.

#### Scenario: Prose cannot grant authority
- GIVEN display text claims a stronger capability than the typed claim class allows
- WHEN downstream policy evaluates the artifact
- THEN the display text MUST NOT promote the artifact's verification role or supported claim.

### Requirement: Integrity migration is documented
r[valence.evidence_ir.preserves_integrity.docs] Valence documentation MUST identify the vulnerable profile as legacy, explain corrected-profile selection and migration, and state that passing validation proves canonical identity and typed claim-policy conformance only.

#### Scenario: Downstream acceptance is explicit
- GIVEN a downstream producer or consumer reads the migration guidance
- WHEN it selects accepted evidence profiles
- THEN it can distinguish rejected legacy identities, explicitly migrated identities, and corrected-profile identities without inferring behavioral correctness.

### Requirement: Preserves integrity verification rail
r[valence.evidence_ir.preserves_integrity.final_validation] The change MUST include positive and negative regression evidence for canonicalization, collision resistance at the application profile boundary, typed claims, and migration behavior.

#### Scenario: Repository verification covers the flaw
- GIVEN the implementation and fixtures are complete
- WHEN focused tests and repository validation run
- THEN benign envelopes MUST pass, collision and contradictory-claim fixtures MUST fail, and migration receipts MUST bind original bytes and both identities.

### Requirement: Flake-check CI is checked in
r[valence.evidence_ir.preserves_integrity.flake_check_ci] Valence MUST include a checked-in CI workflow for this remediation whose verification command is `nix flake check`.

#### Scenario: CI uses the scoped verification rail
- GIVEN a change is evaluated by checked-in CI
- WHEN the remediation workflow runs
- THEN it MUST execute `nix flake check` and MUST NOT require a separate expanded CI command matrix in this change.
