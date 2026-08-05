# Build-service evidence profile

Valence profile `valence.build-service-evidence.v1` links the records one
remote build produces across Mantle, Basalt, Lattice, and OnixOS: service
requests, durable attempts, state transitions, authority decisions, machine
classes, resource decisions, resource observations, retry edges, usage
entries, build results, reuse edges, orchestration jobs, and external
statuses.

The Rust surface is `valence_core::build_service`. The Nickel shape contracts
live in `contracts/build-service-evidence/`.

## Canonical identity

Every record identity is BLAKE3 over canonical JSON material with a
kind-specific domain separator (`valence.build-service.<kind>.v1`). Set-like
fields (eligible classes, declared minima, compatible observations, output
lists) sort canonically before hashing, so equivalent records with different
input order produce the same identity. Ordered fields (reason codes) keep
their order. Optional fields serialize as explicit `null` absence markers.
Integers use fixed widths (`u32`/`u64`). Timestamps use strict
`YYYY-MM-DDTHH:MM:SSZ` text; calendar semantics stay producer-owned.

Identity excludes transport envelopes, signatures, secret material, raw logs,
and unstable display text. A signature wraps the canonical record identity
and declared producer context; it never enters the identity material.

## Verification roles, field by field

| Role | Meaning | Typical fields |
|---|---|---|
| `verified` | A declared validator checked the bounded fact under an identified policy | resource decisions, reuse authorization |
| `linked` | Canonical identities and required references are consistent | requests, attempts, classes, orchestration jobs |
| `signed` | A configured producer signature verified | build results |
| `recorded_only` | The producer reported the observation without independent validation | transitions, resource observations, usage |
| `external` | An external system reported the state under named trust assumptions | external statuses |

A role applies to one field or edge, never to the whole build. A `signed`
result does not verify build output behavior. A `linked` usage record does
not prove fair billing. An `external` Check Run success does not imply
release eligibility; external statuses may carry only `external` or
`recorded_only`.

## Required linkage rules

- Attempts reference a known request and, for retries, a known parent attempt.
- Transitions reference a known attempt, use declared states, unique sequence
  numbers per attempt, and strict UTC timestamps.
- Authority decisions reference a known attempt and record only the public
  subject projection plus the policy identity.
- Resource decisions reference a known attempt, a non-empty eligible class
  set, a selected class inside that set, compatible observations, and ordered
  reason codes.
- An `oom_escalation` retry edge must carry positive trigger evidence and a
  strictly increasing machine-class ordinal.
- Usage reservations and reconciliations reference a known attempt and a
  public unit schedule; two different charge facts with the same
  reconciliation identity conflict.
- A reuse edge keeps producer and consumer identities distinct. A `usable`
  outcome requires separate sharing-policy, authorization, request
  compatibility, platform compatibility, output-check, signature, and CAS
  facts. Shared object identity alone never satisfies them.
- External statuses link one orchestration job and one attempt.

## Privacy boundary

Canonical artifacts never contain raw OIDC tokens, UCAN bearer tokens,
compatibility tickets, private claim sets, replay keys, private signing keys,
provider credentials, secret values, secret-derived hashes, or raw logs.
Validation scans every string field and object key for forbidden markers and
fails closed with a stable path-based diagnostic. Logs remain separate
bounded artifacts; evidence may carry a log's public content identity and
authorized location metadata, never its bytes.

## Non-claims

Every profile instance carries the required boundary: build-service linkage
proves canonical identity and bounded observation linkage only; it does not
prove build correctness, hermeticity, sandboxing, host isolation, identity
beyond checked claims, authorization freshness, future resource sufficiency,
accounting fairness, external status correctness, or release eligibility.

## Consumer examples

Complete worked bundles live in `fixtures/build-service-evidence/`.

### Mantle (producer)

Mantle emits the service request, attempts, transitions, resource decisions,
observations, retry edges, usage records, and the signed build result. See
`fixtures/build-service-evidence/mantle-build.json` for a complete successful
build with one OOM retry. Mantle remains the owner of scheduling, execution,
store, cache, and accounting behavior; Valence validates linkage and identity
only.

### Basalt (authority producer)

Basalt emits authority decisions carrying only the public subject projection
and the policy identity, plus an optional verifier evidence reference. See
`fixtures/build-service-evidence/basalt-authority.json`. Raw OIDC tokens,
UCAN bearer tokens, tickets, and private claims never enter canonical
artifacts.

### Lattice (orchestration producer)

Lattice emits orchestration records binding a workflow and matrix-job
identity to the current Mantle attempt. See
`fixtures/build-service-evidence/lattice-orchestration.json`. Lattice keeps
its own workflow state; the record proves linkage, not scheduling truth.

### OnixOS (operator consumer)

OnixOS consumes validated bundles to review machine-class selection and
usage scope for a target. See
`fixtures/build-service-evidence/onixos-consumption.json` for the consumer
view with per-field roles intact. Consumption does not grant OnixOS any
stronger claim than the recorded roles.

### Cairn (lifecycle consumer)

Cairn consumes external-status and readiness inputs as bounded observations.
A terminal successful Check Run is `external` evidence only; release
readiness still requires a separate Cairn decision. See
`fixtures/build-service-evidence/cairn-check-run.json`.
