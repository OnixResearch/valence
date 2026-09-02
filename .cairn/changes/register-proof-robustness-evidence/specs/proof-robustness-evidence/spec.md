# Proof Robustness Evidence Specification

## Purpose

Define canonical Valence identity, linkage, and role mapping for Octet proof-robustness evidence without promoting stability observations into proof.

## ADDED Requirements

### Requirement: Producer schema identity is exact

r[valence.proof_robustness_evidence.schema]

Valence MUST admit proof-robustness evidence only from a registered Octet producer schema with exact schema, producer revision, profile, fixture, and BLAKE3 identities.

#### Scenario: Registered producer fixture is supplied

- GIVEN every registered producer and schema identity matches the supplied Octet fixture
- WHEN intake validation runs
- THEN Valence MAY parse and validate the proof-robustness layers

#### Scenario: Producer schema is absent or stale

- GIVEN the schema, producer revision, profile, fixture, or digest differs from the registry
- WHEN intake validation runs
- THEN Valence MUST fail closed before role assignment

### Requirement: Robustness layers keep independent identities

r[valence.proof_robustness_evidence.layers]

Valence MUST preserve independent identities and typed links for the robustness profile, parent proof, child runs, resource observations, audit facts, proof-use facts, classification, terminal snapshot, receipt, and non-claims.

#### Scenario: Complete layer graph is supplied

- GIVEN each required layer has one supported identity and every parent edge matches
- WHEN graph validation runs
- THEN Valence MUST produce a deterministic acyclic evidence graph

#### Scenario: A child or layer is swapped

- GIVEN a child receipt, audit fact, proof-use fact, classification, or snapshot belongs to another profile, proof, source, verifier, or generation
- WHEN graph validation runs
- THEN Valence MUST reject the graph with a deterministic linkage diagnostic

### Requirement: Robustness role mapping is closed

r[valence.proof_robustness_evidence.roles]

Valence MUST map seed runs, resource observations, proof-use facts, robustness classifications, and terminal snapshots to `RecordedOnly`. It MUST map assumptions, trust boundaries, unknown coverage, unsupported facts, and non-claims to `Boundary`.

#### Scenario: Complete role mapping passes

- GIVEN every robustness layer uses its admitted role
- WHEN role validation runs
- THEN Valence MUST preserve the mapping without promotion
- AND a separately validated parent proof MAY retain its existing `Property` role

#### Scenario: Robustness layer requests Property

- GIVEN a seed run, resource observation, audit classification, proof-use fact, robustness classification, or snapshot uses `Property`
- WHEN role validation runs
- THEN Valence MUST reject the artifact

### Requirement: Only complete terminal snapshots can enter the sidecar

r[valence.proof_robustness_evidence.validation]

A proof-robustness sidecar MUST bind a current terminal snapshot and every declared child run. Queued, running, stale, cancelled, missing, or cross-generation state MUST NOT enter as complete robustness evidence.

#### Scenario: Current terminal snapshot matches the receipt

- GIVEN the snapshot generation, child set, profile, proof parent, and receipt identities match
- WHEN sidecar validation runs
- THEN Valence MUST admit the snapshot as `RecordedOnly`

#### Scenario: Snapshot is live, stale, or incomplete

- GIVEN the snapshot is queued, running, stale, cancelled, cross-generation, or missing a declared child
- WHEN sidecar validation runs
- THEN Valence MUST reject it as incomplete robustness evidence

### Requirement: CLI intake remains capability confined

r[valence.proof_robustness_evidence.shell]

The Valence shell MUST read proof-robustness artifacts only through explicit capability-relative roots, remeasure supplied bytes, and pass typed in-memory inputs to the pure core.

#### Scenario: Relative artifacts are complete

- GIVEN every locator stays under its declared root and every measured digest matches
- WHEN CLI intake runs
- THEN the shell MAY invoke pure validation and emit a bounded report

#### Scenario: Artifact access is unsafe

- GIVEN a locator is absolute, escapes its root, resolves through a forbidden link, is missing, exceeds bounds, or has a stale digest
- WHEN CLI intake runs
- THEN intake MUST fail before evidence admission

### Requirement: Valence preserves the producer boundary

r[valence.proof_robustness_evidence.boundary]

Valence MUST NOT recompute Octet metrics, classify proof robustness, execute a verifier, assign lifecycle policy, or claim solver independence, verifier soundness, proof completeness, semantic equivalence, whole-program correctness, or release eligibility.

#### Scenario: Consumer promotes robustness evidence

- GIVEN a sidecar, report, or stack projection presents robustness evidence as proof or release authority
- WHEN non-claim validation runs
- THEN Valence MUST reject the promotion

### Requirement: Proof robustness fixtures cover success and failure

r[valence.proof_robustness_evidence.fixtures]

The suite MUST include positive complete role and linkage fixtures plus negative stale, swapped, role-confused, incomplete, unsupported, excessive, and overclaim fixtures.

#### Scenario: Fixture suite runs

- GIVEN every declared positive and negative fixture is available
- WHEN focused validation runs
- THEN every positive fixture MUST pass deterministically
- AND every negative fixture MUST fail at its declared boundary
