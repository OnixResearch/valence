# Nostr identity evidence

Valence records exact linkage between reviewed UCAN Nostr credentials and Basalt subject-binding receipts. The result is `RecordedOnly` Evidence IR.

## Producer pins

The profile pins these immutable producer revisions:

- UCAN integrated revision: `277ab768eb532ac0c715a3bbff2a7dc37a702f11`;
- UCAN credential revision: `c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026`;
- Basalt revision: `bb8fb5f4a763f532df96b6e65407163ac668834e`.

It also pins the Basalt schema, implementation-profile, and policy BLAKE3 identities from `contracts/nostr-identity-evidence/profile.ncl`.

## Validation

The pure core checks exact event, public-key, adapter-profile, adapter-implementation, request, audience, purpose, subject, binding, policy, and receipt linkage.

The Nostr event keeps the `nostr_event_sha256` protocol role. UCAN, Basalt, and Valence BLAKE3 values keep separate typed roles. Cross-role substitution fails.

An optional Basalt composition receipt must cite the exact authentication receipt, request, subject, policy, and producer identities.

## Role and privacy

Authentication, subject binding, and composition always remain `recorded_only`. Property, authority, boundary, proof, completeness, freshness, and release promotion fail.

Closed DTOs reject unknown fields. Public text rejects private-key, credential, proof, challenge, signer-secret, encrypted-payload, and NIP-46 markers.

Valence does not verify the Nostr signature. It does not contact relays, DNS, signers, clocks, or replay stores.

## CLI

Validate an input and optionally write the deterministic artifact:

```text
valence nostr-identity validate \
  --artifact-root INPUT_ROOT --input input.json \
  --output-root OUTPUT_ROOT --out artifact.json
```

Both paths are capability-rooted. Invalid input does not write an artifact.

## Non-claims

A valid report proves only bounded shape, identity, linkage, role, and required non-claim checks over supplied bytes.

It does not prove a human identity, producer trust, authorization, external freshness, policy publication, deployment, or release eligibility.
