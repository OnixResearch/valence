# Artifact Auth Shell Verification Specification

## Purpose

Defines the `artifact-auth-shell-verification` capability.

## Requirements

### Requirement: Valence verifies a dedicated exact standalone signature

r[valence.artifact_auth_shell.exact_verification] Valence MUST map standalone statements in a pure core, require a protocol-verified actor-matched observation before dedicated signing, sign only canonical `artifact_auth.statement.v1` bytes through the existing Radicle signer, and verify the independently reconstructed statement through pinned `artifact-auth-ed25519`.

#### Scenario: Entry signatures cannot substitute

- GIVEN a valid Radicle entry signature without a valid dedicated signature over the exact standalone statement
- WHEN the Radicle shell evaluates standalone authentication
- THEN standalone verification SHALL fail without weakening protocol verification.

### Requirement: Public shell evidence is bounded

r[valence.artifact_auth_shell.evidence] Valence MUST expose statement, full-key, and signature refs, lowercase signature hex, stable cryptographic failure class, and dual-run compatibility while excluding private keys, SSH-agent authority, repository handles, credentials, and ambient authorization.

#### Scenario: Tamper or substitution fails closed

- GIVEN statement, key, signature, preimage, currentness, or carrier identity drift
- WHEN shell verification runs
- THEN it SHALL report a bounded failure or compatibility blocker and SHALL NOT report unexplained parity.

### Requirement: Radicle and Valence authority remain external

r[valence.artifact_auth_shell.authority] Valence MUST retain Radicle protocol verification, actor authorization, repository mutation, COB reduction, Evidence IR, Git measurement/storage, lifecycle, and release interpretation while this shell pilot remains diagnostic.

#### Scenario: Dedicated verification passes without authority admission

- GIVEN a dedicated exact standalone signature verifies
- WHEN dual-run compatibility is evaluated
- THEN protocol authority and rollback SHALL remain active, standalone authority SHALL remain false, and every Valence-owned gate SHALL still apply.
