# Stack Provenance Adapter Specification

## Purpose

Repairs the CLI capability boundary so authority roots are declared independently from untrusted file locators.

## Requirements

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
