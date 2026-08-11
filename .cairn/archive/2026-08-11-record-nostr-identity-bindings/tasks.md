# Tasks: Record Nostr identity binding evidence

## Producer contracts and pure core

- [x] [serial] Pin reviewed UCAN and Basalt producer schemas and non-claims. r[valence.nostr_identity_evidence.producer_linkage]
- [x] [serial] Add the closed Nostr identity evidence envelope and typed fields. r[valence.nostr_identity_evidence.profile]
- [x] [serial] Add distinct Nostr SHA-256 and Valence BLAKE3 digest roles. r[valence.nostr_identity_evidence.digest_domains]
- [x] [serial] Add pure credential, binding, request, subject, policy, and authorization linkage. r[valence.nostr_identity_evidence.producer_linkage]
- [x] [parallel] Add exact-linkage, rotation, and optional-authorization positive fixtures. r[valence.nostr_identity_evidence.final_validation]

## Role, privacy, and shell

- [x] [serial] Enforce `RecordedOnly` for authentication, binding, and composition evidence. r[valence.nostr_identity_evidence.role_policy]
- [x] [parallel] Add property-promotion, authority, proof, freshness, completeness, and release overclaim fixtures. r[valence.nostr_identity_evidence.role_policy]
- [x] [serial] Add secret-field rejection and bounded public metadata policy. r[valence.nostr_identity_evidence.privacy]
- [x] [parallel] Add private-key, credential, proof, challenge, signer-secret, and encrypted-payload fixtures. r[valence.nostr_identity_evidence.privacy]
- [x] [serial] Add capability-rooted CLI validation and deterministic artifact output. r[valence.nostr_identity_evidence.shell_boundary]

## Closeout

- [x] [parallel] Add tamper, schema, digest, key, request, subject, policy, role, and receipt mismatch fixtures. r[valence.nostr_identity_evidence.final_validation]
- [x] [serial] Update profile, CLI, producer, privacy, digest, and non-claim documentation. r[valence.nostr_identity_evidence.final_validation]
- [x] [serial] Run format, workspace, Clippy, purity, Cairn, and Nix verification. r[valence.nostr_identity_evidence.final_validation]
