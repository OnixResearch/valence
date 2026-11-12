# Tasks

## 1. Public source transport

- [x] [serial] [task:transport] Switch the two Cargo declarations, the `artifact` flake input, and the source-pin expectation to anonymous HTTPS at revision `5f3962d7471d1547e044f57daeda72f56621fd52`; regenerate only the affected Cargo and Nix lock entries. r[valence.artifact_auth_adoption.public_transport]
- [x] [serial] [task:admission] [after:transport] Confirm source admission accepts the HTTPS cohort and rejects a credential-bearing URL without changing the NAR hash or consumer package graph. r[valence.artifact_auth_adoption.public_transport]

## 2. Public release metadata

- [x] [serial] [task:license] Add the missing workspace license rule and AGPL text for the schema pilot so the license-boundary tool passes. r[valence.artifact_auth_adoption.public_transport]
- [x] [serial] [task:docs] Replace personal absolute paths in active documentation, agent notes, and rendered index text with portable placeholders; link the security policy from the README; pin the README revision statement. r[valence.artifact_auth_adoption.public_transport]
- [x] [serial] [task:ci] [after:transport] Add explicit least-privilege `contents: read` permissions to the flake-check workflow. r[valence.artifact_auth_adoption.public_transport]

## 3. Verification

- [x] [serial] [task:anonymous] [after:admission] Prove anonymous resolution from a fresh clone with an empty Cargo home and SSH disabled, including `cargo fetch --locked` and anonymous `ls-remote` for every flake input URL. r[valence.artifact_auth_adoption.public_transport]
- [x] [serial] [task:checks] [after:license] [after:docs] [after:ci] Run formatting, workspace build, workspace tests, strict Clippy, the core-purity smoke, the license-boundary tool, Cairn validation, and the proposal/design/tasks gates. r[valence.artifact_auth_adoption.public_transport]
