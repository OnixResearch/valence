# Artifact Auth Operational Receipt Specification

## Purpose

Defines the `artifact-auth-operational-receipt` capability.

## Requirements

### Requirement: Product-derived generation and currentness
r[valence.artifact_auth_operational_receipt.currentness] Valence MUST derive nonzero standalone key generation from the checked COB event sequence and MUST derive key currentness/context from a valid current authorization profile for the exact actor and operation; SSH-agent availability or caller text alone MUST NOT produce `Current`.

#### Scenario: Authorized actor maps to current
GIVEN a protocol-verified event and a valid authorization profile permitting its actor and operation
WHEN Valence derives standalone key state
THEN generation is the checked successor of the event sequence and currentness is bound to the authorization hash

#### Scenario: Removed actor or overflow fails closed
GIVEN an unauthorized actor, stale authorization profile, or maximum event sequence
WHEN Valence derives standalone key state
THEN standalone signing is denied without mutation

### Requirement: RecordedOnly receipt identity
r[valence.artifact_auth_operational_receipt.identity] Valence MUST build a deterministic RecordedOnly Evidence IR artifact that binds exact standalone carrier refs and bytes, protocol-event identity, authorization hash, operation, shell outcome, authority flags, required non-claims, and canonical BLAKE3 identity.

#### Scenario: Deterministic receipt builds
GIVEN one exact shell outcome and authorization snapshot
WHEN Valence builds the operational receipt twice
THEN canonical artifact identity and bytes match

#### Scenario: Receipt overclaim or drift is rejected
GIVEN changed carrier, authorization, operation, role, non-claim, authority flag, or canonical hash
WHEN Valence validates the receipt
THEN validation fails closed

### Requirement: Capability-rooted Git persistence and replay
r[valence.artifact_auth_operational_receipt.persistence] Valence MUST write the receipt under an explicit capability root, attach it through the existing authorized signed Radicle COB mutation path, materialize the resulting state, and independently replay exact standalone verification from the persisted receipt.

#### Scenario: Persisted receipt replays
GIVEN an authorized signer and valid operational receipt
WHEN Valence writes, attaches, materializes, and replays the receipt
THEN the Git entry signature, attachment identity, authorization identity, and standalone verification all match

#### Scenario: Attachment or authorization drift fails
GIVEN tampered receipt bytes, wrong attachment identity, removed actor authorization, or changed operation
WHEN Valence materializes or replays the persisted receipt
THEN replay fails closed before evidence can be promoted

### Requirement: Protocol authority remains unchanged
r[valence.artifact_auth_operational_receipt.authority] Valence MUST keep `legacy_authoritative = true`, `standalone_authority_admitted = false`, and `rollback_available = true`; operational receipts MUST NOT replace Radicle protocol verification, actor authorization, COB reduction, Git/SSH effects, lifecycle handoff, or release interpretation.

#### Scenario: Passing receipt remains RecordedOnly
GIVEN a passing persisted and replayed receipt
WHEN Valence reports the outcome
THEN it remains RecordedOnly and protocol behavior remains authoritative

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
