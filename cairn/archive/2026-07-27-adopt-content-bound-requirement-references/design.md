# Design: Adopt content-bound requirement references

## Goal

Make requirement linkage portable across repositories without treating an ID string as sufficient identity evidence.

## Core types

`valence-core` adds these versioned values:

- `RequirementRefV1` for repository, revision, requirement, specification, policy, row, and registry identities;
- `StackRevisionEntryV1` and `StackRevisionSetV1` for a sorted selected stack;
- `StackTraceContextV1` for requirement refs, evidence graph root, release refs, lifecycle refs, and explicit non-claims;
- typed issue codes for domain, freshness, membership, duplication, and compatibility failures.

Every BLAKE3 field has a semantic role. The validator rejects substitution between requirement-row, specification, policy, registry, graph, artifact, and receipt identities.

## Registry validation

The pure core receives parsed reference values and a parsed Cairn registry. It performs no file access or process execution.

Validation requires exact repository identity, revision kind and value, registry identity, policy identity, and requirement-row identity. The requirement ID must occur exactly once.

A stack revision set uses sorted repository entries and one canonical BLAKE3 identity. Duplicate repositories and conflicting revisions fail closed.

## Stack trace context

A trace context can carry several requirement references and optional release, lifecycle, and runtime links. The selected profile declares which roles are required.

Optional fields do not become authority. Present optional fields still require complete validation.

The context hash excludes display-only paths. It includes every semantic reference, required role, and non-claim.

## Compatibility

Existing `ProvenanceLink.requirement_ids` remains readable under the v1 compatibility profile. It stays recorded-only and cannot satisfy a strict content-bound requirement role.

New producers use a versioned payload with `requirement_refs`. No validator silently invents repository or revision facts for a legacy string.

## CLI shell

The CLI adds bounded validation commands for requirement references and stack contexts. File reads remain under explicit capability roots.

The CLI can use `serde_path_to_error` to report safe structured field paths. Raw values, absolute private roots, and unbounded payload excerpts remain excluded.

## Failure behavior

Validation fails for malformed hashes, unknown revision kinds, wrong repositories, stale registry or policy identities, duplicate rows, absent requirements, domain substitution, or weakened non-claims.

Issue ordering is deterministic. One invalid edge cannot be hidden by another valid edge.

## Verification

Core tests cover valid single and multi-repository contexts. Negative tests mutate one identity domain at a time.

CLI tests cover malformed nested input, capability-root rejection, legacy compatibility, and no-write failure behavior. The `content-bound-requirements` Cairn trace profile keeps this change separate from known gaps in older profiles.

## Rollout

Cairn publishes registries first. Octet emits typed producer links, Mantle binds them to releases, and OnixOS carries them into runtime operations.
