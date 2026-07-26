# Radicle Forge Operations Specification

## Purpose

Define bounded native issue/patch metadata migration and exact-revision CI-admission evidence without transferring canonical-ref, protocol, source-host, or release authority to Valence.

## ADDED Requirements

### Requirement: Typed forge-operations profile

r[valence.radicle_forge_ops.profile] Valence MUST keep a typed Nickel-authored forge-operations profile and checked generated runtime JSON defining source kinds, bounds, state mappings, review treatment, attribution markers, admission vocabulary, and required non-claims.

#### Scenario: Reviewed profile is current

- GIVEN the Nickel profile and checked runtime export
- WHEN profile freshness and positive fixtures run
- THEN the runtime bytes MUST match the reviewed export and valid bounded issue and pull-request records MUST pass.

#### Scenario: Invalid or weakened profile is rejected

- GIVEN unknown source kinds, unbounded values, authority-promoting review mappings, missing non-claims, or unsupported state mappings
- WHEN negative profile fixtures run
- THEN validation MUST fail before any migration plan or admission receipt is produced.

### Requirement: Pure deterministic migration core

r[valence.radicle_forge_ops.migration_core] Valence MUST validate closed normalized issue and pull-request records and derive deterministic ordered native Radicle operation plans with BLAKE3 source and plan identities without filesystem, Git, Radicle, clock, network, environment, process, or signing access.

#### Scenario: Equivalent normalized inputs replay

- GIVEN identical profile, normalized records, and existing source-target mappings
- WHEN migration planning is repeated
- THEN source identities, operation ordering, plan identity, decisions, and issue codes MUST match exactly.

#### Scenario: Malformed or ambiguous input fails closed

- GIVEN unknown fields, duplicate source keys or events, malformed OIDs or locators, unsupported states, missing patch base/head facts, bound overflow, or conflicting existing mappings
- WHEN migration planning runs
- THEN no executable plan MUST be emitted and deterministic field-scoped issue codes MUST identify the rejection.

### Requirement: Source attribution without impersonation

r[valence.radicle_forge_ops.attribution] Valence MUST preserve supplied source actor, timestamp, URL, state, labels, comments, and review metadata as explicit attributed recorded facts while distinguishing the importer signer and refusing to represent imported reviews as Radicle approvals.

#### Scenario: Source attribution is rendered

- GIVEN a bounded source comment or review from a named or deleted source actor
- WHEN a native operation plan is rendered
- THEN the body MUST contain the fixed source-system, source-key, actor-or-tombstone, timestamp, URL, and event identity markers and MUST identify the importer as the actual signer.

#### Scenario: Imported approval is not promoted

- GIVEN a source pull-request review marked approved
- WHEN patch operations are planned or executed
- THEN Valence MUST render recorded attribution only and MUST NOT emit a Radicle review, accept, merge, delegate, or canonical-ref action.

### Requirement: Native built-in collaboration objects

r[valence.radicle_forge_ops.native_cobs] The process-isolated `valence-radicle-native` shell MUST be preview-first and, on explicit execution with the matching importer signer and a revalidated Valence plan, write only built-in `xyz.radicle.issue` or `xyz.radicle.patch` COB refs in one selected local repository and verify the resulting built-in state before reporting success. It MUST NOT load `valence-core` in the upstream Radicle `serde_json/preserve_order` process.

#### Scenario: Native issue and patch imports succeed

- GIVEN valid plans, a matching signer, locally present patch base/head objects, and no conflicting prior mapping
- WHEN issue and patch imports execute
- THEN built-in evaluators MUST load the expected title, attributed content, labels, state, target, base/head, revision, and source markers from refs confined to the importer's built-in COB namespace.

#### Scenario: Partial or unsafe mutation is rejected

- GIVEN signer drift, an absent Git object, invalid COB history, conflicting or partial prior mapping, wrong built-in type, unsupported lifecycle state, or any extra ref mutation
- WHEN execution or post-write verification runs
- THEN the operation MUST fail without claiming a completed import and MUST retain a bounded diagnostic suitable for repair review.

### Requirement: Source-to-target binding receipt

r[valence.radicle_forge_ops.binding] Valence MUST emit and offline-verify a BLAKE3-bound binding receipt linking the profile, deterministic plan, source record identities, importer, built-in target type/object, written entry OIDs, observed state, and required non-claims.

#### Scenario: Saved binding replays offline

- GIVEN a completed import receipt and the reviewed profile
- WHEN saved-binding verification runs without repository or network access
- THEN every identity, target role, entry ordering, observed state, and non-claim MUST validate.

#### Scenario: Tampered binding is rejected

- GIVEN any modified source identity, plan identity, importer, target, entry OID, state, or non-claim
- WHEN saved-binding verification runs
- THEN verification MUST return nonzero with deterministic mismatch or boundary diagnostics.

### Requirement: Exact-revision CI admission

r[valence.radicle_forge_ops.ci_admission] Valence MUST admit a patch revision only when an exact policy, patch/revision/head, typed successful CI event/result, matching signed bot-status observation, unique threshold-satisfying delegate approvals for that revision, current canonical predecessor, and descendant relation all agree.

#### Scenario: Exact successful revision is admitted

- GIVEN a non-delegate bot, a succeeded zero-exit exact-object result, a matching signed status observation, unique exact-revision delegate approvals meeting policy threshold, and a candidate head descending from the current canonical predecessor
- WHEN CI admission runs
- THEN Valence MUST emit a BLAKE3-bound admitted receipt naming every supplied identity and the external compare-and-swap claim boundary.

#### Scenario: Incomplete or stale evidence is denied

- GIVEN a failed, timed-out, cancelled, output-limited, malformed, stale-policy, unsigned, wrong-bot, wrong-object, wrong-revision, duplicate-approval, below-threshold, bot-delegate, non-descendant, or stale-canonical observation
- WHEN CI admission runs
- THEN admission MUST fail closed with no canonical mutation plan.

### Requirement: Capability-confined shell

r[valence.radicle_forge_ops.shell] The planner/binder and process-isolated native adapter shells MUST require explicit selected repository or capability roots, reject escaping locators, default every mutation to preview, require an explicit execute flag, and verify plan identity, signer identity, and intended refs before mutation.

#### Scenario: Preview has no effects

- GIVEN a valid migration or admission input without the execute flag
- WHEN the shell runs
- THEN it MUST emit the deterministic plan or report without changing Git refs, collaboration objects, source files, lifecycle files, or external state.

#### Scenario: Ambient or escaping inputs are rejected

- GIVEN an omitted repository/root, absolute locator, parent traversal, duplicate flag, unknown flag, or mismatched signer
- WHEN the shell parses or prepares the request
- THEN it MUST return nonzero before mutation.

### Requirement: Validation matrix

r[valence.radicle_forge_ops.validation] Valence MUST retain positive and negative profile, core, adapter, native-COB, receipt, admission, repository-confinement, and dependency-compatibility tests and pass focused plus repository-wide validation before lifecycle acceptance.

#### Scenario: Complete validation passes

- GIVEN the final implementation and retained fixtures
- WHEN formatting, workspace tests, all-target Clippy, core purity, Nickel freshness, native COB round trips, receipt replay, Cairn gates, Tracey coverage, and flake checks run
- THEN every selected check MUST pass and validation evidence MUST name the exact commands and outcomes.

#### Scenario: False completion fixtures fail

- GIVEN a custom-COB-only import, unverified shell replay, actor impersonation, imported approval promotion, result-only admission, protocol-level mandatory-CI claim, canonical-ref mutation in Valence, production mutation, or live CI widening
- WHEN the validation matrix runs
- THEN acceptance MUST fail.

### Requirement: Authority and non-claim boundaries

r[valence.radicle_forge_ops.boundaries] Valence MUST describe forge migration and CI admission as bounded identity, native-write observation, and external-guard input only and MUST preserve explicit non-claims for source export completeness, actor authenticity, timestamp preservation, semantic equivalence, approval equivalence, merge eligibility, protocol enforcement, canonical-ref authority, CI correctness, host sandboxing, release readiness, and whole-stack GitHub independence.

#### Scenario: Downstream authority remains external

- GIVEN a valid migration binding or admitted CI receipt
- WHEN OnixOS, onix-core, Cairn, a delegate, or an operator consumes it
- THEN the consumer MUST apply its own freshness, authorization, compare-and-swap, lifecycle, and release policy without treating Valence validity as authority to mutate a canonical ref.

#### Scenario: Overclaim is rejected

- GIVEN a receipt or document claiming protocol-enforced mandatory CI, source semantic equivalence, preserved source authorship signatures, automatic merge eligibility, bypass-proof delegate behavior, canonical mutation, or release readiness
- WHEN boundary validation runs
- THEN the artifact MUST be rejected or the unsupported claim MUST be recorded as an exact blocker.
