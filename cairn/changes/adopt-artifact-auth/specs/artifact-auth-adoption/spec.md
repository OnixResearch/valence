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
