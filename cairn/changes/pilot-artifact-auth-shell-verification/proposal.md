# Pilot exact artifact-auth verification in the Radicle shell

## Why

Valence's pure adapter currently trusts an explicitly supplied standalone cryptographic observation. Radicle entry signatures cover protocol-owned bytes and cannot prove the different `artifact_auth.statement.v1` preimage. The Radicle shell must demonstrate a dedicated actor-key signature over the exact mapped standalone statement before authority admission can be considered.

## What changes

- Expose one pure Valence mapping for the exact signer-specific standalone statement.
- Add purpose-bounded dedicated statement signing through the existing Radicle signer interface and exact verification through the pinned `artifact-auth-ed25519` package.
- Emit bounded public statement/key/signature identities, signature hex, and dual-run diagnostics.
- Add real MemorySigner positive evidence and tamper, wrong-preimage, wrong-key, malformed, unknown-currentness, carrier-drift, and false-parity negatives.
- Keep Radicle protocol verification authoritative, standalone authority unadmitted, and rollback available.

## Impact

The pilot changes `valence-core`, `valence-radicle`, exact Cargo/Nix package checks, tests, and operator documentation. It does not alter COB entry signatures, actor authorization, repository mutation, Evidence IR, lifecycle, or release authority.
