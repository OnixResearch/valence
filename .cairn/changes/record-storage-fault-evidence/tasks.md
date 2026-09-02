# Tasks: Record storage-fault evidence

## Profile and core

- [ ] [serial] Record current Evidence IR, role, chain, capability-root, and fixture baselines. r[valence.storage_fault_evidence.profile]
- [ ] [serial] Add the closed versioned storage-fault envelope and typed identity roles. r[valence.storage_fault_evidence.profile] r[valence.storage_fault_evidence.identity]
- [ ] [serial] Add pure candidate, workload, cohort, fault, schedule, oracle, result, and receipt linkage. r[valence.storage_fault_evidence.linkage]
- [ ] [serial] Enforce `RecordedOnly` campaign results and `Boundary` assumptions and non-claims. r[valence.storage_fault_evidence.roles]
- [ ] [serial] Add bounded public fields and reject secret, raw-path, raw-database, and unbounded-log fields. r[valence.storage_fault_evidence.privacy]

## Shell and fixtures

- [ ] [serial] Add capability-rooted CLI validation and deterministic Evidence IR output. r[valence.storage_fault_evidence.shell]
- [ ] [parallel] Add complete single-node, persistent-consensus, Octet, and Cairn positive fixtures. r[valence.storage_fault_evidence.validation]
- [ ] [parallel] Add tamper, role, mixed-cohort, stale, duplicate, missing-boundary, secret, path, and overclaim fixtures. r[valence.storage_fault_evidence.validation]

## Closeout

- [ ] [serial] Document producer ownership, role meanings, field policy, and non-claims. r[valence.storage_fault_evidence.nonclaims]
- [ ] [serial] Run formatting, workspace tests, Clippy, core purity, Cairn gates, and Nix checks. r[valence.storage_fault_evidence.validation]

## Verification Coverage

- `Scenario: Complete campaign artifact is supplied` -> positive profile fixture
- `Scenario: Candidate identity differs` -> stale-cohort fixture
- `Scenario: Campaign result uses Property` -> role rejection fixture
- `Scenario: Artifact contains an absolute host path` -> privacy rejection fixture
- `Scenario: Non-claim is weakened` -> boundary rejection fixture
