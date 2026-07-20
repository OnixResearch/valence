# Design: Valence artifact-auth operational receipt

## Goal and completion evidence

Complete when Valence derives nonzero generation from the observed COB sequence, derives currentness and context identity from a validated authorization profile, signs exact standalone bytes, creates a deterministic RecordedOnly Evidence IR artifact, writes it under a capability root, attaches it through signed Git COB mutation, materializes it, and independently replays the carrier.

False completion includes `MemorySigner` evidence without Git persistence, arbitrary currentness strings, raw file writes outside capability roots, unsigned Git objects, authorization-hash omission, or authority promotion.

## Portfolio registry

| Family | Mechanism | State | Evidence or blocker |
|---|---|---|---|
| authorization-profile | Validated current profile binds actor, role, operation, and full profile hash | active | Existing mutation planning and reduction already fail on stale authorization hashes. |
| ssh-agent-loaded | Treat loaded key as current | falsified | Agent readiness proves availability, not authorization or revocation. |
| radicle-membership | Infer key status from network membership | blocked | No bounded current membership/revocation API exists here. |
| cob-history | Persist receipt as Evidence IR attachment in signed Git COB history | active | Existing execute/materialize path verifies written signatures and attachment identities. |

## Boundaries and audit

Pure generation/currentness derivation and receipt validation remain in deterministic code; capability-root file I/O, SSH-agent effects, and Git mutation remain shell-owned. Audit sequence overflow, stale authorization hash, actor removal, operation drift, wrong full key, attachment substitution, malformed receipt, Git signature failure, carrier drift, and false parity.
