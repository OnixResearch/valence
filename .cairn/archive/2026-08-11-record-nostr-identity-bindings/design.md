# Design: Record Nostr identity binding evidence

## Goals

- Preserve exact producer and protocol identities.
- Keep external protocol hashes separate from Valence identities.
- Prevent authentication or binding records from becoming authority or proof.
- Keep validation in a pure core with a capability-rooted shell.

## Evidence profile

Add a versioned `nostr-identity-evidence` profile to `valence-core`.

The profile accepts already-loaded producer artifacts:

- a UCAN `VerifiedNostrCredential` record;
- a Basalt `AuthenticatedSubject` or terminal binding decision;
- an optional Basalt authorization-composition decision;
- exact producer schema and implementation identities;
- explicit freshness, replay, policy, and non-claim fields.

The control envelope is closed. Unknown fields, duplicate fields, stale schema identities, and weakened non-claims fail before identity construction.

## Digest domains

The profile defines distinct typed roles:

- Nostr event SHA-256 identity;
- Nostr x-only public key;
- UCAN adapter profile BLAKE3 identity;
- Basalt policy BLAKE3 identity;
- stable subject and binding identities;
- request BLAKE3 identity;
- Valence artifact canonical BLAKE3 identity;
- optional authorization and outcome receipt BLAKE3 identities.

Valence can recompute bounded carrier identities from supplied canonical bytes. It does not reinterpret a rehash as the original protocol identity.

The external SHA-256 role exists only for Nostr interoperability. Every Valence-owned canonical artifact and receipt remains BLAKE3.

## Pure validation core

The pure core validates:

- schema and profile identities;
- field and digest roles;
- credential-to-binding event identity equality;
- canonical Nostr key equality;
- request, audience, purpose, subject, binding, and policy linkage;
- optional authorization composition linkage;
- verification role and required non-claims;
- redaction and bounded public-field policy.

The core does not read files, clocks, environment, relays, DNS, signers, replay stores, or private keys.

## Shell boundary

Add a focused CLI surface under the existing Valence shell. It reads producer artifacts through declared capability roots.

The shell writes a validated Evidence IR artifact and a deterministic report. It performs no relay connection, signer request, DNS lookup, or authorization effect.

The initial change does not add a `valence-nostr` network adapter crate. A future transport adapter requires a separate change.

## Role policy

All Nostr authentication, subject-binding, and authorization-composition evidence uses `RecordedOnly`.

The profile preserves any linked authorization decision as metadata. It never converts that decision into Valence `Property` evidence or lifecycle readiness.

A valid Nostr signature establishes only the producer-declared key-control result. Valence validation establishes only identity, shape, linkage, role, and non-claim facts.

## Privacy and retention

The profile accepts public keys and bounded public identifiers. It rejects private keys, NIP-46 secrets, credentials, proof bytes, challenges, encrypted payloads, and secret metadata.

NIP-05 names and display labels remain optional metadata outside canonical subject authority. Relay URLs are omitted unless a reviewed profile needs bounded provenance.

## Verification

Positive fixtures cover exact credential, binding, optional authorization, rotation, and digest-role linkage.

Negative fixtures cover tampering, schema drift, digest substitution, mismatched keys, requests, subjects, policies, roles, and receipt identities.

Privacy fixtures reject secret-bearing fields. Claim fixtures reject identity, authorization, proof, completeness, freshness, and release overclaims.
