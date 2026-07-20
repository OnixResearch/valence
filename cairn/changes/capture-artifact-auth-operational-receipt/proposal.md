# Change: Capture Valence artifact-auth operational receipts

## Why

Valence can sign and independently verify exact standalone bytes after Radicle protocol verification and actor authorization, but the evidence is transient and currentness context is caller text. Valence needs a receipt bound to the validated authorization profile and persisted through its existing capability-rooted artifact plus signed Git COB path.

## What Changes

- Derive generation from the observed COB sequence and currentness from the validated authorization profile/operation.
- Build a deterministic Evidence IR artifact containing the standalone carrier and bounded shell outcome.
- Persist that artifact through the existing capability-rooted artifact path and signed Radicle COB attachment flow, then materialize and independently replay it.
- Preserve Radicle/Valence protocol authority and standalone rollback.

## Scope

No SSH-agent discovery, Radicle network synchronization, global key revocation, Git transport, lifecycle, or release authority moves into the receipt.
