# Content Bound Requirements Specification

## Purpose

Defines the `content-bound-requirements` capability.

## Requirements

### Requirement: Provenance uses content-bound requirement references

r[valence.provenance.content_bound_requirement_ref]

Valence MUST define a versioned requirement reference that binds repository, revision, requirement, specification, policy, registry, and requirement-row identities.

#### Scenario: A current typed reference passes

- GIVEN a requirement reference matches one row in a supplied current Cairn registry
- WHEN the pure Valence validator evaluates the reference
- THEN validation MUST pass with every identity domain preserved
- AND the result MUST remain identity and linkage evidence only

#### Scenario: Same-named requirement from another repository fails

- GIVEN two repositories use the same requirement ID
- WHEN a reference names one repository but supplies the other repository's registry
- THEN validation MUST fail with a repository-domain issue
- AND the shared text ID MUST NOT bypass the mismatch

### Requirement: Registry validation fails on stale or ambiguous links

r[valence.provenance.requirement_registry_validation]

Valence MUST require exact registry membership and MUST reject stale, absent, duplicate, or identity-substituted requirement links.

#### Scenario: Stale specification identity fails

- GIVEN a reference retains an old requirement-row or accepted-specification BLAKE3
- WHEN validation uses the current registry
- THEN validation MUST fail with a deterministic freshness issue
- AND no strict profile MUST count the stale link

#### Scenario: Duplicate registry row fails

- GIVEN a supplied registry contains the same requirement ID more than once
- WHEN Valence validates any reference against that registry
- THEN validation MUST fail before graph insertion
- AND issue output MUST identify the duplicate ID without unbounded source text

#### Scenario: Digest role substitution fails

- GIVEN a valid BLAKE3 value from one role is supplied as another identity role
- WHEN Valence validates the reference
- THEN validation MUST fail with a domain-substitution issue
- AND matching digest syntax MUST NOT make the substitution valid

### Requirement: Stack trace context binds a selected stack

r[valence.provenance.stack_trace_context]

Valence MUST define a canonical stack trace context that binds a sorted selected revision set, typed requirement references, evidence graph identity, optional release and lifecycle links, and non-claims.

#### Scenario: Complete selected stack passes

- GIVEN unique repository entries, current typed requirement references, and complete profile-required links
- WHEN the stack trace context is validated
- THEN Valence MUST emit one deterministic context identity
- AND repeated input ordering changes MUST NOT change that identity

#### Scenario: Conflicting repository revisions fail

- GIVEN a stack revision set contains two revisions for one repository identity
- WHEN the stack trace context is validated
- THEN validation MUST fail with a duplicate-repository issue
- AND no downstream release or runtime link MUST consume the context

#### Scenario: Present optional link is invalid

- GIVEN a profile does not require a release link but the context supplies one with a stale identity
- WHEN the context is validated
- THEN validation MUST fail rather than ignore the present invalid link
- AND absence and invalid presence MUST remain distinct states

### Requirement: Legacy requirement strings remain lower-fidelity evidence

r[valence.provenance.legacy_requirement_id_boundary]

Valence MUST keep existing string-based requirement IDs readable without treating them as content-bound strict-profile evidence.

#### Scenario: Legacy artifact remains readable

- GIVEN a valid v1 provenance artifact contains only `requirement_ids`
- WHEN the compatibility profile validates it
- THEN Valence MUST preserve the original artifact identity and recorded-only role
- AND validation MUST NOT invent repository, revision, specification, or registry facts

#### Scenario: Strict profile rejects legacy-only linkage

- GIVEN a strict profile requires content-bound requirement references
- WHEN an artifact supplies only legacy string IDs
- THEN validation MUST fail with a missing-content-bound-reference issue
- AND the failure MUST not claim that the underlying requirement is unsatisfied
