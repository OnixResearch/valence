# Proposal: Record Nostr identity binding evidence

## Why

The UCAN adapter can verify Nostr key control. Basalt can bind that credential to a stable subject and compose authorization.

Downstream evidence needs typed identities and explicit non-claims. Raw Nostr events or Basalt receipts cannot become Valence property evidence by attachment alone.

## What Changes

- Add a versioned Nostr identity evidence profile to `valence-core`.
- Link UCAN adapter credentials, Basalt subject bindings, and optional authorization decisions.
- Preserve Nostr SHA-256 event identities separately from Valence BLAKE3 identities.
- Classify authentication, binding, and composition records as `RecordedOnly`.
- Add capability-rooted CLI validation over already-loaded artifacts.
- Add positive and negative digest, linkage, role, redaction, and freshness fixtures.

## Impact

Valence can record which reviewed adapter and policy produced an identity statement. The statement remains bounded to the exact request and supplied observations.

Valence does not prove the human identity behind a key. It does not grant authority or establish Nostr network completeness.

## Dependencies

Implementation depends on reviewed schemas from these changes:

- `OnixResearch/ucan`: `add-nostr-identity-adapter`
- `OnixResearch/basalt`: `bind-verified-nostr-identities`

Valence pins exact producer schema identities. It does not depend on live sibling paths or network services.

## Non-goals

- Do not connect to Nostr relays or NIP-46 signers.
- Do not resolve NIP-05 names or fetch profile metadata.
- Do not reimplement Basalt policy or UCAN authorization.
- Do not promote authentication, signature, relay, or binding evidence to `Property`.
