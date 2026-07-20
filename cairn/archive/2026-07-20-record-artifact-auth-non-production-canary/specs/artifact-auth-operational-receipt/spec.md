# Artifact-auth operational receipt

## ADDED Requirements

### Requirement: Non-production canary evidence is durably archived
r[valence.artifact_auth_operational_receipt.canary_archive] Valence MUST preserve its landed non-production artifact-auth canary as a self-contained public Cairn evidence bundle binding the exact product and artifact-auth revisions, harness source, RecordedOnly Evidence IR, signed COB identities, materialized successful fresh-process replay, actor-authorization removal, expected denial, typed manifest, and BLAKE3 inventory; the bundle MUST exclude private signing material and mutable Git or capability state.

#### Scenario: Complete public canary bundle validates
GIVEN the exact public artifacts from the bounded Valence canary run
WHEN the manifest, inventory, Evidence IR identity, signed COB linkage, positive replay, and authorization-removal denial are checked
THEN every archived regular file is content-bound and the observed authorized-to-unauthorized transition remains reviewable without secret state

#### Scenario: Unsafe or incomplete archive fails closed
GIVEN a missing, drifted, symlinked, oversized, malformed, overclaiming, or secret-bearing candidate archive member
WHEN the canary archive is validated
THEN the evidence package is rejected before it can support a later admission review

### Requirement: Canary evidence remains RecordedOnly
r[valence.artifact_auth_operational_receipt.canary_authority] Valence MUST label the archived canary as bounded non-production RecordedOnly evidence and MUST keep `legacy_authoritative = true`, `standalone_authority_admitted = false`, and `rollback_available = true`; the archive MUST NOT claim live Radicle membership, global revocation freshness, production signer custody, protocol authority, COB reduction authority, lifecycle handoff, release interpretation, or production rollout.

#### Scenario: Passing canary remains observational
GIVEN a complete archived signed attachment, materialization, replay, and authorization-removal denial record
WHEN an operator reviews the canary result
THEN the Evidence IR remains RecordedOnly, Radicle protocol behavior remains authoritative, and a separate reviewed authority-admission change is still required
