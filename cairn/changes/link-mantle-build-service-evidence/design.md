# Design: Mantle build-service evidence linkage

## Context

Remote build evidence crosses authority and lifecycle boundaries. Valence provides canonical identity and linkage. It does not become the runtime state owner or infer correctness from linked observations.

The profile is additive over existing Evidence IR. Generic identities, spans, roles, sidecars, and non-claims remain authoritative.

## Profile model

`BuildServiceEvidenceProfile` identifies the profile version, canonicalization version, producer policy, and required non-claims.

The profile can link these records:

- `ServiceRequestRecord`: canonical concrete request and idempotency identity;
- `AttemptRecord`: durable attempt, generation, parent or retry relation, and service policy;
- `AttemptTransitionRecord`: prior and next state, sequence, observed time, producer, and reason;
- `AuthorityDecisionRecord`: public subject projection, policy identity, requested operation, decision, and verifier evidence reference;
- `MachineClassRecord`: declared platform and resource class identity;
- `ResourceDecisionRecord`: declared minima, eligible-set identity, selected class, policy, and ordered reasons;
- `ResourceObservationRecord`: bounded measurements, collector, compatibility scope, and terminal outcome;
- `RetryEdgeRecord`: predecessor, successor, trigger evidence, limit state, and policy;
- `UsageRecord`: reservation or reconciliation identity, public units, window, account or project scope, and outcome;
- `BuildResultRecord`: request, attempt, platform, policy, outputs, producer signature reference, and CAS observation;
- `ReuseEdgeRecord`: producer result, consumer request, sharing policy, authorization decision, and usability outcome;
- `OrchestrationRecord`: Lattice workflow and matrix-job identities;
- `ExternalStatusRecord`: GitHub Check Run or another external status identity and observed state.

Not every deployment must emit every record. Profile validation defines which links are required for each claimed observation class.

## Canonical identity

Stack-owned record identity uses canonical bytes and BLAKE3 with a profile-specific domain separator. Set-like fields have canonical ordering. Optional fields have explicit presence markers. Units, timestamp representation, integer widths, and version fields are unambiguous.

Identity excludes transport envelopes, signatures, secret material, raw logs, and unstable display text. A signature wraps the canonical record identity and declared producer context.

## Verification roles

The profile uses existing Valence roles rather than creating stronger claims:

- `Verified`: a declared validator checked the bounded fact under an identified policy;
- `Linked`: canonical identities and required references are consistent;
- `Signed`: a configured producer signature verified;
- `RecordedOnly`: the producer reported the observation without independent validation;
- `External`: an external system reported the state under named trust assumptions.

A role applies to one field or edge, not the whole build. For example, a verified event signature does not verify build output behavior. A linked usage record does not prove fair billing.

## Authority and privacy projection

Authority evidence records only policy-approved public subject fields or their canonical public projection. It can link to Basalt verification evidence. It never embeds the raw OIDC token, UCAN bearer token, compatibility ticket, private claim set, replay key, private signing key, provider credential, or secret-derived hash.

Logs remain separate bounded artifacts. Evidence can link a content identity and authorized location, but the profile does not embed log bytes. Consumer access to a link remains an external authority decision.

## Resource and retry linkage

A resource decision links declared requirements, eligible-set identity, selected class, policy version, compatible-observation identities, and ordered reasons. This permits deterministic replay without treating history as authority.

An OOM retry edge links positive OOM evidence, predecessor and successor attempts, prior and next machine classes, limit state, and policy. Missing trigger evidence or non-increasing class selection is invalid for an OOM-escalation edge.

## Result reuse linkage

A reuse edge does not make a result usable. It records the producer result, consumer request, sharing policy, authorization decision, request and platform compatibility, output checks, CAS observation, and final usability outcome.

Validation requires producer and consumer identities to remain distinct. Shared object identity alone cannot satisfy signature, policy, authority, or availability links.

## External status linkage

Lattice can link a GitHub Check Run to one orchestration job and its current Mantle attempt. The external status uses `External` or `RecordedOnly` roles as appropriate. A terminal successful status cannot imply release eligibility without a separate Cairn decision.

## Validation and fixtures

The pure validator accepts a profile, records, links, and explicit trust facts. It returns stable path-based diagnostics. It performs no file, network, clock, signature-key discovery, or runtime service I/O.

Positive fixtures cover a successful build, a failed build, an OOM retry, same-project reuse, explicit cross-project reuse, and a GitHub Check Run. Negative fixtures cover dangling links, wrong kinds, duplicate identities, noncanonical order, unsupported versions, missing OOM evidence, non-increasing retry class, shared-object-only reuse, producer-consumer confusion, secret-bearing fields, raw logs, and overclaimed roles.

## Non-claims

Every profile instance preserves that linkage does not prove build correctness, hermeticity, sandboxing, host isolation, identity beyond checked claims, authorization freshness, future resource sufficiency, accounting fairness, external status correctness, or release eligibility.
