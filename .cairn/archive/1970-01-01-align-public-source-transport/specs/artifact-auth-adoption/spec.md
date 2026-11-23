# Artifact Auth Adoption Delta

## ADDED Requirements

### Requirement: Build inputs resolve anonymously over public HTTPS

r[valence.artifact_auth_adoption.public_transport] Valence MUST declare every Cargo Git dependency and flake input as an HTTPS URL to a public repository at its pinned immutable revision so that dependency fetch and flake evaluation succeed without SSH credentials, private network access, or a pre-populated cache.

#### Scenario: Anonymous clean clone resolves the dependency graph

- GIVEN a fresh clone of Valence and an empty Cargo home with SSH disabled
- WHEN `cargo fetch --locked` and flake evaluation run
- THEN every pinned Git source MUST resolve over anonymous HTTPS at the declared revision
- AND the consumer graph MUST remain limited to `artifact-auth-core` and `artifact-auth-ed25519`.

#### Scenario: Non-public transport blocks source admission

- GIVEN a Cargo manifest or flake input that names an SSH or otherwise credential-bearing URL
- WHEN source-admission validation runs
- THEN it MUST reject the cohort without falling back to another transport or revision.
