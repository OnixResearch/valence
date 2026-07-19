# Design: exact artifact-auth verification in the Radicle shell

## Completion contract

Completion requires an existing Radicle signer to create a separate signature over canonical standalone statement bytes and `artifact-auth-ed25519` to verify an independently reconstructed statement. Real positive evidence and fail-closed tamper, wrong-preimage/key, malformed, currentness, carrier, protocol-rejection, and false-parity evidence must pass while authority remains unadmitted. Reusing an entry signature/boolean, fixture-only observation, exposing key material, or promoting standalone success is false completion.

## Core and shell

`valence-core` owns deterministic statement/policy mapping over already loaded protocol observations. `valence-radicle` owns signer interaction, public-key byte extraction, dedicated signature creation, exact standalone verification, carrier integrity, and orchestration. Core remains free of Git, repository, SSH-agent, filesystem, JSON, clocks, and signing effects.

## Dedicated signature

The shell first requires a protocol-verified observation and an exact actor/signer match. It maps and canonicalizes the standalone statement, then invokes the existing `radicle_crypto::Signer` on those bytes as a separate operation. Verification reconstructs the statement and uses the pinned standalone verifier over raw actor-key and dedicated signature bytes. The Radicle COB entry signature is neither read nor reused.

## Evidence and authority

The carrier records BLAKE3 statement, key, and signature refs plus lowercase signature hex. Verification recomputes all carrier identities. The report retains raw Radicle/Git identities and dual-run diagnostics. Radicle protocol verification, actor authorization, repository mutation, COB reduction, Evidence IR, lifecycle, and release interpretation remain authoritative; standalone authority stays false.

## Approach registry

| Family | State | Reason |
| --- | --- | --- |
| Reuse Radicle entry signature | Falsified | Different preimage and protocol framing. |
| Export actor private key | Rejected | Widens SSH-agent/key authority. |
| Verify a standalone fixture key only | Blocked | Does not exercise the product signer shell. |
| Dedicated exact-statement signature through existing signer | Selected | Exercises real product signing without replacing protocol authority. |
