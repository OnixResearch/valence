# Artifact Auth Adoption Specification

## Purpose

Defines the `artifact-auth-adoption` capability.

## Requirements

### Requirement: Valence adopts one immutable reviewed source

r[valence.artifact_auth_adoption.source] Valence MUST consume one immutable reviewed `artifact-auth` revision with aligned Cargo and Nix identities and MUST bind the Valence mapping profile and checked projection from that same revision before implementation or cutover.

#### Scenario: Source identity is admissible

- GIVEN Cargo, Nix, the mapping profile, and its checked projection resolve revision `799459346d5416fbd7b9f55840a7371441b55afa`
- WHEN Valence evaluates dependency admission
- THEN it SHALL reject floating, duplicate, mismatched, sibling-path, product-dependent, or license-incompatible source selections.

### Requirement: Valence retains protocol, identity, and authorization authority

r[valence.artifact_auth_adoption.authority] Valence MUST retain Radicle protocol verification, actor authorization, Evidence IR identity/roles, COB reduction, Git measurement/storage, SSH-agent effects, lifecycle handoff, and release interpretation while treating standalone authentication as one bounded post-verification input.

#### Scenario: Authentication passes without actor or evidence authority

- GIVEN a standalone mapped decision passes over an already protocol-verified observation
- WHEN actor authorization, Evidence IR, lifecycle, or release interpretation runs
- THEN Valence MUST still require its product-owned checks and MUST NOT promote standalone success into product authority.

### Requirement: Cutover requires explained dual-run evidence

r[valence.artifact_auth_adoption.cutover] Valence MUST dual-run legacy and standalone projections over identical verified observations, classify every preimage, identity, decision, issue, and non-claim difference, reject unrelated-failure false parity, and preserve a bounded legacy rollback until standalone authority is explicitly admitted.

#### Scenario: Unexplained or lossy mapping blocks cutover

- GIVEN any unexplained, lossy, authority-widening, or source-identity difference
- WHEN Valence evaluates cutover admission
- THEN the legacy path SHALL remain authoritative and the exact blocker SHALL be recorded without weakening protocol, identity, authorization, lifecycle, or release gates.

### Requirement: Radicle-backed source transport r[valence.artifact_auth_adoption.radicle_transport]

Valence MUST consume the accepted public artifact-auth RID through the upload-pack-only Radicle HTTPS adapter at exact revision `799459346d5416fbd7b9f55840a7371441b55afa`.

#### Scenario: Exact public source is admitted

- GIVEN an accepted publication receipt, RID, URL, commit, and source archive identity
- WHEN Valence evaluates the dependency source
- THEN all identities MUST agree before the source is used.

#### Scenario: Unaccepted source is rejected

- GIVEN a different RID, URL, commit, visibility, or archive identity
- WHEN source admission runs
- THEN the dependency MUST be rejected.

### Requirement: Cargo and Nix lock agreement r[valence.artifact_auth_adoption.lock_agreement]

Cargo manifests, `Cargo.lock`, `flake.nix`, and `flake.lock` MUST identify the same Radicle HTTPS repository and reviewed Git revision while preserving the locked Nix content identity.

#### Scenario: Owning tools preserve identity

- GIVEN regenerated Cargo and Nix locks
- WHEN agreement validation runs
- THEN both artifact-auth packages, the exact revision, the public URL, and the expected Nix content hash MUST match.

#### Scenario: Lock drift fails closed

- GIVEN a stale GitHub source, mismatched revision, missing package, or changed Nix content hash
- WHEN agreement validation runs
- THEN validation MUST fail.

### Requirement: Behavioral parity checks r[valence.artifact_auth_adoption.behavior]

The transport cutover MUST preserve existing Valence and artifact-auth behavior under focused mapping, workspace, no-std, formatting, and Clippy checks.

#### Scenario: Existing positive and negative behavior passes

- GIVEN the Radicle-backed source at the reviewed commit
- WHEN the focused and workspace suites run
- THEN existing acceptance and rejection behavior MUST pass without Rust implementation changes.

#### Scenario: Behavioral drift blocks acceptance

- GIVEN any changed fixture result, feature graph, package version, or authority boundary
- WHEN parity is evaluated
- THEN the cutover MUST not be accepted.

### Requirement: No executable GitHub fallback r[valence.artifact_auth_adoption.fallback]

Valence MUST NOT retain an executable GitHub source fallback for artifact-auth.

#### Scenario: Radicle source is unavailable

- GIVEN the admitted Radicle HTTPS source cannot serve the exact object
- WHEN dependency resolution runs
- THEN resolution MUST fail visibly rather than selecting GitHub.

#### Scenario: Historical documentation remains bounded

- GIVEN documentation names historical GitHub provenance
- WHEN fallback validation runs
- THEN documentation MAY remain while manifests and locks contain no executable artifact-auth GitHub source.

### Requirement: Typed Radicle cutover evidence r[valence.artifact_auth_adoption.radicle_evidence]

Valence MUST emit typed Nickel/JSON evidence with a BLAKE3 sidecar binding publication, source, locks, test observations, rollback boundary, and non-claims.

#### Scenario: Complete cutover evidence passes

- GIVEN matching publication, source, lock, and test observations
- WHEN receipt validation runs
- THEN it MUST accept deterministically.

#### Scenario: Missing linkage or overclaim fails

- GIVEN missing publication linkage, drifted locks, missing negative evidence, or weakened non-claims
- WHEN receipt validation runs
- THEN it MUST fail closed.
