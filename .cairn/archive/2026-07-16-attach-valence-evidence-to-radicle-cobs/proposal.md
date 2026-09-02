## Why

Valence can identify and validate evidence artifacts, while Radicle Collaborative Objects can carry signed, local-first collaboration history. There is currently no typed boundary that binds an immutable Valence artifact to the exact COB target and entry that recorded it. Ad hoc labels, comments, or copied JSON would lose digest-domain separation, role semantics, non-claim enforcement, deterministic concurrency behavior, and a portable handoff for Cairn.

Valence needs a bounded Radicle adapter that preserves Valence as the evidence-identity authority and Radicle as the collaborative transport. The adapter must not move Git, signing, repository access, or live network behavior into `valence-core`, and attaching evidence must never promote its verification role or authorize a lifecycle disposition by itself.

## What Changes

- Add a pure `cob_evidence` profile to `valence-core` for typed target references, evidence attachments, relationship events, deterministic reduction, BLAKE3 attachment identities, binding receipts, and snapshot artifacts. r[valence.radicle_cob_evidence.pure_core] r[valence.radicle_cob_evidence.attachment_identity]
- Add an in-tree `valence-radicle` adapter crate rather than a new sibling project. It owns local Git/Radicle storage access, COB signatures, embedded bytes, preview-first mutation, and CLI presentation while keeping `radicle-cob` out of `valence-core`. r[valence.radicle_cob_evidence.adapter_boundary]
- Define a versioned, Nickel-authored sidecar profile using the custom COB type `org.onixresearch.valence.evidence`. A logical sidecar is keyed by repository RID, target type, and target object ID; concurrent physical sidecar roots for the same target are reduced together instead of requiring coordinated creation. r[valence.radicle_cob_evidence.sidecar_profile] r[valence.radicle_cob_evidence.convergence]
- Attach canonical Valence artifact references and optionally bounded embedded bytes. Preserve Radicle/Git OIDs as protocol identities and BLAKE3 values as Valence-owned identities without cross-domain substitution. r[valence.radicle_cob_evidence.payload_modes] r[valence.radicle_cob_evidence.digest_domains]
- Avoid hash cycles by creating and validating the Valence artifact first, recording its identity in a COB entry second, and emitting a separate binding receipt over both identities third. r[valence.radicle_cob_evidence.binding_receipt]
- Preserve attachment state as `recorded_only`, enforce explicit non-claims, and separate signature validity from policy authorization. A valid attachment cannot waive a lint, resolve a Cairn change, or promote evidence to `property`. r[valence.radicle_cob_evidence.role_policy] r[valence.radicle_cob_evidence.authority]
- Emit deterministic saved snapshot/binding artifacts suitable for Valence evidence-chain and future Cairn policy consumption without requiring Cairn gates to contact a Radicle node or inspect live refs. r[valence.radicle_cob_evidence.snapshot_handoff]
- Document replicated-data privacy, supersession/revocation retention, jj-colocated Git safety, dependency pinning, and explicit non-goals. r[valence.radicle_cob_evidence.repository_safety] r[valence.radicle_cob_evidence.privacy_retention]

## Impact

- **Core**: `valence-core` gains pure attachment/profile DTOs, validators, reducers, and deterministic receipt construction over already-loaded inputs.
- **Adapter crate**: a new `crates/valence-radicle/` package depends on an exact reviewed `radicle-cob` release and exposes local, preview-first attach/snapshot/verify operations without changing the existing `valence` binary dependency closure.
- **Contracts**: Nickel owns the sidecar type name, schema/profile versions, relationship and actor-role vocabularies, embed policy, receipt fields, and required non-claims; Rust consumes reviewed generated profile data.
- **COB shape**: sidecar COBs relate to existing issue, patch, or custom COB targets and hold evidence links rather than replacing the target schema or Cairn lifecycle files.
- **Downstream**: Cairn may consume saved Valence snapshot/binding artifacts as optional lifecycle-policy input; it remains the authority for gates, exemptions, and release readiness.
- **Testing**: positive round trips plus negative fixtures for malformed targets, tampered embeds, stale or cross-domain hashes, unauthorized actors, role promotion, duplicate events, concurrent conflicts, weakened non-claims, hash cycles, unsafe repository mutation, and stale snapshot handoff.
- **Non-goals**: no Radicle node daemon, peer discovery, seeding policy, remote fetch, key custody service, forge UI, blanket repository rollout, raw lint-finding fan-out, or live network access from Valence or Cairn validation.
