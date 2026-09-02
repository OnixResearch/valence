# Evidence IR Build-Service Delta

## ADDED Requirements

### Requirement: valence.evidence_ir.build_service_profile

r[valence.evidence_ir.build_service_profile]

Valence SHALL define a versioned build-service evidence profile that can link service requests, durable attempts, transitions, authority decisions, machine classes, resource decisions, observations, retries, usage, results, reuse, orchestration jobs, and external statuses.

#### Scenario: Complete successful build is represented

- **GIVEN** a concrete request, one terminal attempt, an authority decision, a resource decision, bounded observations, a signed result, and an external Check Run
- **WHEN** a producer constructs the profile records
- **THEN** Valence SHALL assign each record a declared kind and canonical identity
- **AND** it SHALL validate every required typed link

#### Scenario: Optional record is absent

- **GIVEN** a build without a retry, reuse edge, or external status
- **WHEN** the profile validates required links for the claimed observation classes
- **THEN** the profile MAY remain valid
- **AND** it SHALL NOT invent absent observations

### Requirement: valence.evidence_ir.build_service_identity

r[valence.evidence_ir.build_service_identity]

Valence SHALL use canonical encoding and domain-separated BLAKE3 for stack-owned build-service record identities. Set-like fields, units, timestamps, integer widths, optional fields, and versions SHALL have unambiguous encodings.

#### Scenario: Equivalent records are reordered

- **GIVEN** two semantically equivalent records whose set-like source fields have different input order
- **WHEN** canonical encoding runs
- **THEN** both SHALL produce the same canonical bytes and identity

#### Scenario: Unsupported or ambiguous encoding appears

- **GIVEN** an unknown version, duplicate set member, absent unit, ambiguous timestamp, overflow, or noncanonical ordering
- **WHEN** validation runs
- **THEN** it SHALL fail with a stable path-based diagnostic

### Requirement: valence.evidence_ir.build_service_roles

r[valence.evidence_ir.build_service_roles]

Valence SHALL assign verification roles to individual facts and links. A role on one field or edge SHALL NOT promote unrelated build-service claims.

#### Scenario: Event signature verifies

- **GIVEN** a completion event with a valid configured producer signature
- **WHEN** Valence records its signature role
- **THEN** the event identity MAY be `Signed`
- **AND** build output behavior, isolation, and release eligibility SHALL remain unverified

#### Scenario: Producer reports resource use

- **GIVEN** a bounded measurement without independent validation
- **WHEN** Valence records it
- **THEN** the measurement SHALL be `RecordedOnly` or another exact accepted role
- **AND** it SHALL NOT become a future resource guarantee

### Requirement: valence.evidence_ir.resource_retry_links

r[valence.evidence_ir.resource_retry_links]

Valence SHALL link each resource decision to declared requirements, eligible-set identity, selected class, policy, compatible observations, and ordered reasons. An OOM escalation edge SHALL also link positive OOM evidence and a strictly increasing eligible class.

#### Scenario: OOM retry is well formed

- **GIVEN** predecessor and successor attempts, trusted positive OOM evidence, increasing eligible class, policy, and limit state
- **WHEN** the retry edge validates
- **THEN** Valence SHALL link those facts without claiming the retry was necessary or sufficient

#### Scenario: OOM evidence or class increase is absent

- **GIVEN** an edge labeled as OOM escalation without positive evidence or a larger eligible class
- **WHEN** validation runs
- **THEN** it SHALL reject the edge

### Requirement: valence.evidence_ir.usage_links

r[valence.evidence_ir.usage_links]

Valence SHALL link usage reservations and reconciliations to public unit schedules, project or account scopes, attempt identities, and outcomes without claiming billing fairness.

#### Scenario: Usage reconciliation links

- **GIVEN** one reservation and one completion reconciliation for an attempt
- **WHEN** records validate
- **THEN** Valence SHALL link both to the same attempt and unit schedule

#### Scenario: Duplicate charge identity conflicts

- **GIVEN** two different charge facts with the same canonical reconciliation identity
- **WHEN** validation runs
- **THEN** it SHALL reject the conflict

### Requirement: valence.evidence_ir.result_reuse_edges

r[valence.evidence_ir.result_reuse_edges]

Valence SHALL keep producer and consumer identities distinct and SHALL represent sharing policy, authorization, request and platform compatibility, output verification, producer signature, CAS observation, and usability as separate facts.

#### Scenario: Authorized cross-project reuse is linked

- **GIVEN** distinct producer and consumer projects and all required accepted facts
- **WHEN** the reuse edge validates
- **THEN** Valence SHALL link the consumer request to the producer result
- **AND** it SHALL preserve each fact's verification role

#### Scenario: Shared object is the only common fact

- **GIVEN** a consumer and producer that reference the same CAS object without required authority, signature, policy, platform, or result links
- **WHEN** the reuse edge validates
- **THEN** Valence SHALL reject usable-result linkage

### Requirement: valence.evidence_ir.external_status_links

r[valence.evidence_ir.external_status_links]

Valence SHALL link an external status to a declared orchestration job and build attempt using `External`, `RecordedOnly`, or another exact accepted role. External success SHALL NOT imply release eligibility.

#### Scenario: GitHub Check Run is linked

- **GIVEN** a Lattice job, Mantle attempt, and GitHub Check Run observation
- **WHEN** the external status record validates
- **THEN** it SHALL link the public identities and observed state
- **AND** it SHALL preserve the external-system trust boundary

#### Scenario: External status claims release eligibility

- **GIVEN** a Check Run record that promotes success directly to release eligibility
- **WHEN** profile non-claims validate
- **THEN** validation SHALL reject the claim without a separate Cairn decision

### Requirement: valence.evidence_ir.build_service_privacy

r[valence.evidence_ir.build_service_privacy]

Canonical build-service artifacts SHALL exclude raw OIDC tokens, UCAN bearer tokens, compatibility tickets, private claim sets, replay keys, private keys, provider credentials, secret values, secret-derived hashes, and raw logs.

#### Scenario: Evidence links a log

- **GIVEN** an authorized bounded log artifact
- **WHEN** build-service evidence references it
- **THEN** the record MAY contain its public content identity and authorized location metadata
- **AND** it SHALL NOT embed the log bytes

#### Scenario: Forbidden material is present

- **GIVEN** a canonical record field that contains forbidden bearer, secret, private-claim, provider, replay, or raw-log material
- **WHEN** privacy validation runs
- **THEN** validation SHALL fail with a stable path-based diagnostic

### Requirement: valence.evidence_ir.build_service_non_claims

r[valence.evidence_ir.build_service_non_claims]

Every build-service profile SHALL preserve explicit non-claims for build correctness, hermeticity, sandboxing, host isolation, identity beyond checked claims, authorization freshness, future resource sufficiency, accounting fairness, external status correctness, and release eligibility.

#### Scenario: All links validate

- **GIVEN** a fully linked profile
- **WHEN** a consumer evaluates its scope
- **THEN** the consumer SHALL receive the required non-claims with the linked observations
- **AND** linkage alone SHALL NOT upgrade them
