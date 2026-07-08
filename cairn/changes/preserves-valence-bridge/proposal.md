## Why

Molten uses canonical Preserves envelopes for runtime communication and evidence, while Valence owns canonical Evidence IR. A narrow bridge lets Molten keep Preserves as its runtime wire/storage form while Valence validates evidence identity and non-claims.

## What Changes

- Add a Preserves-to-Valence adapter contract.
- Map Preserves canonical bytes and refs into Valence artifact identity fields.
- Reject unsupported schema roles, stale BLAKE3 refs, missing non-claims, and semantic overclaims.
