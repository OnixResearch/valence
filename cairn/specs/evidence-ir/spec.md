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
