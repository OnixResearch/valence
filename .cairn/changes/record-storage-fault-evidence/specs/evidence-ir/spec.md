# Evidence IR Delta

## ADDED Requirements

### Requirement: Storage-fault evidence profile
r[valence.storage_fault_evidence.profile] Valence MUST provide a versioned closed profile for bounded saved storage-fault campaign evidence.

#### Scenario: Complete campaign artifact is supplied
- GIVEN a well-formed artifact with all required identities, roles, linkage, and non-claims
- WHEN pure validation runs
- THEN Valence accepts the artifact for canonical identity construction

### Requirement: Typed storage-fault identities
r[valence.storage_fault_evidence.identity] The profile MUST distinguish candidate, workload, kernel, filesystem, mount, device, fault, schedule, oracle, result, and producer-receipt identities.

#### Scenario: Digest role is substituted
- GIVEN a filesystem identity in the candidate identity field
- WHEN validation runs
- THEN Valence rejects the nominal role mismatch

### Requirement: Exact cohort linkage
r[valence.storage_fault_evidence.linkage] The profile MUST bind all selected case artifacts to one exact declared campaign cohort.

#### Scenario: Candidate identity differs
- GIVEN one linked result for another candidate BLAKE3
- WHEN validation runs
- THEN Valence rejects the mixed cohort

### Requirement: Storage-fault role policy
r[valence.storage_fault_evidence.roles] Raw campaign observations and producer verdicts MUST use `RecordedOnly`, while assumptions and non-claims MUST use `Boundary`.

#### Scenario: Campaign result uses Property
- GIVEN a raw producer result classified as `Property`
- WHEN validation runs
- THEN Valence rejects the role promotion

### Requirement: Capability-rooted shell
r[valence.storage_fault_evidence.shell] The CLI MUST read saved artifacts through explicit capability roots and MUST NOT execute producers or storage operations.

#### Scenario: Producer executable is absent
- GIVEN complete saved input artifacts
- WHEN CLI validation runs
- THEN validation can complete without a producer executable

### Requirement: Storage evidence privacy
r[valence.storage_fault_evidence.privacy] The profile MUST reject credentials, secret keys, absolute host paths, raw database contents, full environment dumps, and unbounded logs.

#### Scenario: Artifact contains an absolute host path
- GIVEN a campaign artifact with an absolute host path field
- WHEN validation runs
- THEN Valence rejects it before canonical output

### Requirement: Required storage non-claims
r[valence.storage_fault_evidence.nonclaims] Artifacts MUST state that they do not prove arbitrary devices, kernels, filesystems, schedules, database correctness, consensus correctness, or release eligibility.

#### Scenario: Non-claim is weakened
- GIVEN an artifact that omits the arbitrary-device non-claim
- WHEN validation runs
- THEN Valence rejects it

### Requirement: Storage evidence validation
r[valence.storage_fault_evidence.validation] Valence MUST include positive complete fixtures and negative tamper, role, cohort, freshness, duplicate, privacy, and overclaim fixtures.

#### Scenario: Duplicate case is supplied
- GIVEN two artifacts with the same case identity in one campaign set
- WHEN validation runs
- THEN Valence rejects the duplicate
