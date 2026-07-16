# Radicle Cob Evidence Specification

## Purpose

Defines the `radicle-cob-evidence` capability.

## Requirements

### Requirement: Radicle evidence sidecar profile is typed and versioned

r[valence.radicle_cob_evidence.sidecar_profile] Valence MUST define a versioned `org.onixresearch.valence.evidence` sidecar COB profile with closed typed roots and events for target references, artifact attachments, supersession, revocation, observed target state, relationship vocabulary, actor facts, profile identity, and required non-claims.

#### Scenario: Valid sidecar root and event decode

r[valence.radicle_cob_evidence.sidecar_profile.valid]
- GIVEN a sidecar root identifies a repository RID, target COB type, target object ID, target key, supported profile, and required non-claims and an event uses a supported closed operation shape
- WHEN Valence decodes the already-loaded values
- THEN it MUST preserve every typed field for validation and deterministic reduction.

#### Scenario: Unknown or malformed sidecar shape fails

r[valence.radicle_cob_evidence.sidecar_profile.invalid]
- GIVEN a sidecar uses an unsupported type/profile/schema, unknown control field, malformed target, unsupported relationship or operation, duplicate field, missing non-claim, or mismatched Radicle `related` target
- WHEN Valence decodes or validates it
- THEN validation MUST fail with deterministic field-scoped diagnostics.

### Requirement: COB evidence decisions remain in a pure core

r[valence.radicle_cob_evidence.pure_core] Valence MUST implement target validation, attachment validation, event authorization decisions, logical-sidecar reduction, conflict reporting, identity material, binding-receipt material, and snapshot material as pure deterministic functions over already-loaded inputs.

#### Scenario: Pure inputs produce stable reports

r[valence.radicle_cob_evidence.pure_core.valid]
- GIVEN equivalent profile, target, artifact, signer, authorization, and event inputs arrive in different collection orders
- WHEN the pure core validates and reduces them
- THEN it MUST emit equivalent canonically ordered reports and identical BLAKE3 identities.

#### Scenario: Core attempts ambient access

r[valence.radicle_cob_evidence.pure_core.boundary]
- GIVEN COB evidence evaluation runs in `valence-core`
- WHEN the implementation is checked for effects
- THEN it MUST NOT read Git or files, inspect environment or clocks, execute processes, access keys or network state, print output, or depend on ambient mutable state.

### Requirement: Radicle integration uses a separate in-tree adapter crate

r[valence.radicle_cob_evidence.adapter_boundary] Valence MUST provide an in-tree `valence-radicle` crate that owns Radicle/Git storage, signature verification, embedded-byte measurement, mutation orchestration, profile loading, and operator presentation without adding `radicle-cob` or Git storage types to `valence-core` or the existing `valence` CLI.

#### Scenario: Adapter supplies normalized core inputs

r[valence.radicle_cob_evidence.adapter_boundary.valid]
- GIVEN a local repository contains a supported target and sidecar history
- WHEN `valence-radicle` inspects it
- THEN the adapter MUST verify and normalize shell-owned facts before invoking the pure `cob_evidence` core.

#### Scenario: Integration leaks shell effects into core

r[valence.radicle_cob_evidence.adapter_boundary.invalid]
- GIVEN an implementation adds Radicle storage, Git access, signing, native repository handles, network calls, or `radicle-cob` public types to `valence-core`
- WHEN package-boundary and purity checks run
- THEN the change MUST fail admission.

### Requirement: Attachment identity is typed canonical BLAKE3 material

r[valence.radicle_cob_evidence.attachment_identity] Valence MUST compute the logical target key and immutable attachment ID with BLAKE3 over versioned typed canonical material while excluding display-only paths, aliases, timestamps, traversal order, signatures, and the future COB entry OID.

#### Scenario: Equivalent attachment material is stable

r[valence.radicle_cob_evidence.attachment_identity.stable]
- GIVEN two attachments have identical target, relationship, artifact identity, observed target-state identity, profile identity, and non-claim material but differ only in display metadata or input ordering
- WHEN their identities are computed
- THEN their target keys and attachment IDs MUST be equal.

#### Scenario: Identity-relevant material changes

r[valence.radicle_cob_evidence.attachment_identity.change]
- GIVEN the target, relationship, artifact stable ID or canonical hash, observed target state, profile identity, payload mode, or required non-claim changes
- WHEN the attachment identity is recomputed
- THEN the BLAKE3 attachment ID MUST change or validation MUST fail closed.

### Requirement: Radicle and Valence digest domains remain distinct

r[valence.radicle_cob_evidence.digest_domains] Valence MUST preserve separate typed domains for repository RIDs, COB object IDs, COB entry OIDs, target tip OIDs, embed OIDs, Valence artifact hashes, target keys, attachment IDs, profile hashes, materialized-state hashes, and receipt hashes.

#### Scenario: Valid protocol and evidence identifiers coexist

r[valence.radicle_cob_evidence.digest_domains.valid]
- GIVEN a valid attachment contains Radicle/Git protocol identifiers and Valence-owned BLAKE3 identifiers in their declared roles
- WHEN validation runs
- THEN it MUST preserve both domains without reinterpreting or replacing either identity.

#### Scenario: Cross-domain substitution fails

r[valence.radicle_cob_evidence.digest_domains.invalid]
- GIVEN syntactically valid identifier text is supplied under another digest role or a rehash is presented as the original protocol or artifact identity
- WHEN validation runs
- THEN it MUST fail with a deterministic digest-domain diagnostic.

### Requirement: Attachments support bounded embeds and verified references

r[valence.radicle_cob_evidence.payload_modes] Valence MUST support policy-bounded embedded evidence bytes and reference-only artifact links while requiring exact agreement among measured bytes, declared Valence identity, Git embed identity, artifact role, media class, size metadata, and safe locator policy when those fields apply.

#### Scenario: Valid embed and reference pass

r[valence.radicle_cob_evidence.payload_modes.valid]
- GIVEN one attachment embeds a small canonical Valence artifact within the reviewed policy and another supplies a valid typed reference to a verified evidence registry artifact
- WHEN payload validation runs
- THEN both attachments MUST preserve their distinct payload modes and exact artifact identities.

#### Scenario: Tampered or unsafe payload fails

r[valence.radicle_cob_evidence.payload_modes.invalid]
- GIVEN embedded bytes are missing, tampered, oversized, undeclared, or hash-mismatched or a reference uses an unsafe locator, stale registry identity, unsupported media class, or inconsistent size/hash metadata
- WHEN payload validation runs
- THEN the attachment MUST be rejected with deterministic diagnostics.

### Requirement: COB binding receipts avoid circular identity

r[valence.radicle_cob_evidence.binding_receipt] Valence MUST construct a post-write binding receipt only after the referenced Valence artifact and resulting COB entry identities exist, and the receipt MUST bind target, attachment, artifact, entry, optional embed, profile, supported claim, and non-claim material with BLAKE3.

#### Scenario: Acyclic binding verifies

r[valence.radicle_cob_evidence.binding_receipt.valid]
- GIVEN a validated Valence artifact is referenced by a successfully written and re-read COB entry
- WHEN the adapter constructs and verifies the binding receipt
- THEN the receipt MUST link both immutable identities without changing either one.

#### Scenario: Circular or stale binding fails

r[valence.radicle_cob_evidence.binding_receipt.invalid]
- GIVEN an artifact identity depends on the future COB entry, a receipt cites another entry or attachment, written bytes cannot be re-read exactly, or the stated receipt hash is stale
- WHEN binding verification runs
- THEN it MUST fail and MUST NOT emit a passing binding receipt.

### Requirement: Evidence attachment never promotes verification roles

r[valence.radicle_cob_evidence.role_policy] Valence MUST classify attachment, binding, and materialized COB snapshot artifacts as `RecordedOnly`, preserve the referenced artifact's original role as linked metadata, and reject any role or claim promotion caused solely by attachment.

#### Scenario: Property artifact remains linked property metadata

r[valence.radicle_cob_evidence.role_policy.valid]
- GIVEN a valid `Property` evidence artifact is attached to a COB
- WHEN attachment and snapshot roles are evaluated
- THEN the referenced artifact MUST remain identifiable as `Property` while the attachment, binding, and snapshot remain `RecordedOnly`.

#### Scenario: Attachment requests authority or proof promotion

r[valence.radicle_cob_evidence.role_policy.invalid]
- GIVEN an attachment claims that its signature, target relation, embed, or existence proves target correctness, authorizes a waiver, satisfies a requirement, establishes lifecycle readiness, or promotes recorded evidence to property evidence
- WHEN role and non-claim validation run
- THEN validation MUST fail with no increased property count.

### Requirement: Signature validity and operation authorization are separate

r[valence.radicle_cob_evidence.authority] Valence MUST record cryptographic signature validity separately from policy authorization and MUST allow an event to affect accepted state only when its signature, signer identity, actor role, operation permission, and bound actor-policy/profile identity are all valid.

#### Scenario: Authorized signed event affects state

r[valence.radicle_cob_evidence.authority.valid]
- GIVEN a correctly signed event comes from an actor authorized for its exact attach, supersede, or revoke operation under the bound profile
- WHEN logical-sidecar reduction runs
- THEN the event MAY affect accepted state and MUST retain its signer and authorization disposition.

#### Scenario: Signed but unauthorized event is rejected

r[valence.radicle_cob_evidence.authority.invalid]
- GIVEN an event is unsigned, invalidly signed, signed by an unknown actor, validly signed but unauthorized for the operation, or evaluated under a stale or unknown actor-policy identity
- WHEN reduction runs
- THEN it MUST remain visible as a rejected event and MUST NOT affect accepted attachment state.

### Requirement: Logical sidecars converge deterministically

r[valence.radicle_cob_evidence.convergence] Valence MUST reduce all valid physical sidecar COB roots sharing one target key into a deterministic logical sidecar where attachment replay is idempotent and incompatible concurrent supersede, revoke, or authorization outcomes become explicit conflicts rather than implicit winners.

#### Scenario: Offline sidecars converge

r[valence.radicle_cob_evidence.convergence.valid]
- GIVEN peers independently create sidecar roots and valid attachment events for the same target and later union their histories
- WHEN Valence reduces the complete event set in any traversal order
- THEN accepted attachments, rejected events, conflicts, and the materialized-state hash MUST converge.

#### Scenario: Concurrent conflict cannot use hidden precedence

r[valence.radicle_cob_evidence.convergence.invalid]
- GIVEN concurrent events incompatibly supersede or revoke one attachment or produce contradictory authorization dispositions
- WHEN reduction runs
- THEN Valence MUST emit a deterministic conflict and MUST NOT choose a winner by timestamp, wall clock, input order, traversal order, or hash-order tie breaking.

### Requirement: Saved COB snapshots provide bounded downstream handoff

r[valence.radicle_cob_evidence.snapshot_handoff] Valence MUST emit a deterministic saved snapshot artifact that binds the target and observed target state, contributing sidecar object/tip identities, accepted attachments, rejected events, conflicts, actor-policy/profile identity, materialized-state hash, receipt hash, verification role, supported claim, and explicit non-claims.

#### Scenario: Valid snapshot is replayable input

r[valence.radicle_cob_evidence.snapshot_handoff.valid]
- GIVEN local sidecar histories, embeds, signer facts, authorization inputs, and profile data validate
- WHEN `valence-radicle` materializes a snapshot
- THEN the saved artifact MUST validate without live Radicle access and MUST be suitable as optional Valence evidence-chain or future Cairn policy input.

#### Scenario: Live or stale handoff fails closed

r[valence.radicle_cob_evidence.snapshot_handoff.invalid]
- GIVEN a consumer requires network access, unrecorded refs, ambient signer policy, stale target tips, stale profile identity, mismatched attachment state, missing rejected/conflict rows, or a weakened non-claim to reproduce the result
- WHEN snapshot verification runs
- THEN it MUST fail and MUST NOT emit a passing downstream disposition.

### Requirement: Repository mutation is preview-first and jj-safe

r[valence.radicle_cob_evidence.repository_safety] Valence MUST make attachment mutations preview-first and, on explicit execution, limit writes to intended local COB refs and declared output artifacts without changing heads, tags, tracked files, the jj working-copy commit, remote refs, or network state.

#### Scenario: Explicit local mutation is confined

r[valence.radicle_cob_evidence.repository_safety.valid]
- GIVEN an operator reviews a deterministic attachment plan and explicitly executes it in a colocated jj/Git repository
- WHEN the adapter writes and verifies the change
- THEN only the declared local COB refs and output receipt paths MUST change.

#### Scenario: Unsafe mutation is blocked

r[valence.radicle_cob_evidence.repository_safety.invalid]
- GIVEN a plan would escape an output root, change a branch or tag, mutate tracked lifecycle/source files, alter the jj working-copy commit, update a remote ref, contact the network, or write after a failed preflight
- WHEN preview or execution validation runs
- THEN the operation MUST fail before unsafe mutation and report `mutated = false` or its equivalent.

### Requirement: Privacy and retention limits remain explicit

r[valence.radicle_cob_evidence.privacy_retention] Valence MUST document and enforce that supersede or revoke changes accepted state without promising historical-byte deletion, that replicated COB data is not a secret-storage mechanism, and that private-repository selective replication is not an at-rest encryption guarantee.

#### Scenario: Safe evidence is attached with visible limits

r[valence.radicle_cob_evidence.privacy_retention.valid]
- GIVEN an attachment contains reviewable non-secret evidence within the embed policy
- WHEN it is attached, superseded, revoked, or snapshotted
- THEN operator output and documentation MUST preserve the retention and confidentiality non-claims.

#### Scenario: Sensitive or deletion-dependent evidence is rejected

r[valence.radicle_cob_evidence.privacy_retention.invalid]
- GIVEN evidence is marked secret, regulated, credential-bearing, deletion-dependent, or otherwise excluded by payload policy
- WHEN attachment planning runs
- THEN embedding MUST be rejected and reference-only handling MUST still require an explicitly allowed confidentiality profile.

### Requirement: COB evidence validation covers positive and negative paths

r[valence.radicle_cob_evidence.final_validation] The change MUST include deterministic positive fixtures and negative fixtures covering every profile, target, digest role, payload mode, identity, signature, authorization, role, non-claim, concurrency, binding, snapshot, repository-safety, privacy, and dependency-boundary rule before archive.

#### Scenario: Complete focused matrix passes

r[valence.radicle_cob_evidence.final_validation.positive]
- GIVEN the profile, pure core, adapter shell, local fixture repository, binding receipts, snapshots, documentation, and Cairn handoff are ready
- WHEN focused contract, core, adapter, convergence, jj-safety, workspace, Clippy, Cairn, and Nix checks run
- THEN valid fixtures MUST pass, repeated materialization MUST be byte-identical where declared, and every malformed fixture MUST fail for its expected deterministic issue code.

#### Scenario: Missing negative evidence blocks completion

r[valence.radicle_cob_evidence.final_validation.negative]
- GIVEN any declared malformed target, tampered embed, stale hash, digest substitution, role promotion, invalid signature, unauthorized operation, concurrent conflict, weakened non-claim, circular binding, stale snapshot, unsafe mutation, confidentiality violation, or dependency-boundary case lacks an executable rejection fixture
- WHEN completion gates run
- THEN the change MUST remain incomplete and MUST NOT be archived.
