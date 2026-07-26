## MODIFIED Requirements

### Requirement: Valence adopts one immutable reviewed source

r[valence.artifact_auth_adoption.source] Valence MUST consume `artifact-auth-core` and `artifact-auth-ed25519` from one immutable reviewed `OnixResearch/onix-artifact` revision with aligned Cargo and Nix identities, exact source-workspace membership, compatible licensing, and a consumer graph limited to those two authentication packages.

#### Scenario: Unified source identity is admissible

- GIVEN Cargo and Nix resolve revision `e41340bec587b6d049b5cc518ec7db925dde84be`, the complete source profile agrees, and authentication entry-point bytes match the reviewed identities
- WHEN Valence evaluates dependency admission
- THEN it SHALL accept only the two authentication packages and SHALL reject floating, duplicate, mismatched, sibling-path, graph-widening, byte-drifted, or license-incompatible source selections.

### Requirement: Unified Artifact workspace source transport r[valence.artifact_auth_adoption.radicle_transport]

Valence MUST consume the two authentication packages from the immutable unified Artifact repository at exact revision `e41340bec587b6d049b5cc518ec7db925dde84be` while retaining the predecessor Radicle identity as historical evidence only.

#### Scenario: Exact unified source is admitted

- GIVEN the repository, revision, Nix NAR hash, source archive identity, package profile, and authentication source identities agree
- WHEN Valence evaluates the dependency source
- THEN the source MAY be used without admitting transfer packages or Artifact-owned authority into Valence.

#### Scenario: Mixed or widened source is rejected

- GIVEN a different repository, revision, NAR hash, package set, source identity, or executable predecessor dependency
- WHEN source admission runs
- THEN the dependency MUST be rejected.

### Requirement: Cargo and Nix lock agreement r[valence.artifact_auth_adoption.lock_agreement]

Cargo manifests, `Cargo.lock`, `flake.nix`, and `flake.lock` MUST identify the same unified Artifact repository and reviewed Git revision while preserving the locked Nix content identity and exact two-package Valence consumer graph.

#### Scenario: Owning tools preserve identity

- GIVEN regenerated Cargo and Nix locks
- WHEN agreement validation runs
- THEN both authentication packages, the exact revision, the unified repository, and the expected Nix content hash MUST match.

#### Scenario: Lock drift fails closed

- GIVEN a predecessor, floating, local, mismatched, duplicate, missing, or widened source selection
- WHEN agreement validation runs
- THEN validation MUST fail.

### Requirement: Behavioral parity checks r[valence.artifact_auth_adoption.behavior]

The source migration MUST preserve existing Valence and artifact-auth behavior under focused positive and negative mapping, shell, workspace, no-std, formatting, and Clippy checks without Rust implementation changes.

#### Scenario: Existing positive and negative behavior passes

- GIVEN the unified source at the reviewed commit
- WHEN the focused and workspace suites run
- THEN existing acceptance and rejection behavior MUST pass without Rust implementation changes.

#### Scenario: Behavioral drift blocks acceptance

- GIVEN any changed fixture result, feature graph, package version, source entry point, or authority boundary
- WHEN parity is evaluated
- THEN the migration MUST not be accepted.

### Requirement: No mixed or automatic source fallback r[valence.artifact_auth_adoption.fallback]

Valence MUST use only the admitted unified source in executable manifests and locks and MUST NOT automatically select the predecessor repository, a sibling path, or a floating source.

#### Scenario: Unified source is unavailable

- GIVEN the admitted source cannot serve the exact object
- WHEN dependency resolution runs
- THEN resolution MUST fail visibly rather than selecting another source.

#### Scenario: Historical evidence remains bounded

- GIVEN documentation and archived receipts name the predecessor GitHub and Radicle sources
- WHEN active-source validation runs
- THEN historical evidence MAY remain while executable manifests and locks contain only the admitted unified source.

### Requirement: Typed source migration evidence r[valence.artifact_auth_adoption.radicle_evidence]

Valence MUST emit typed Nickel/JSON evidence with a BLAKE3 sidecar that binds the unified source, source and consumer package sets, source-byte parity, test observations, rollback boundary, and non-claims while retaining the prior Radicle receipt as historical evidence.

#### Scenario: Complete migration evidence passes

- GIVEN matching source, package, parity, lock, and test observations
- WHEN receipt validation runs
- THEN it MUST accept deterministically.

#### Scenario: Missing linkage or overclaim fails

- GIVEN source drift, package widening, missing negative evidence, automatic rollback, absolute evidence paths, or weakened non-claims
- WHEN receipt validation runs
- THEN it MUST fail closed.
