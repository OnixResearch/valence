# Proposal: Adopt content-bound requirement references

## Summary

Replace strict-profile requirement ID strings with typed references bound to a Cairn registry, repository revision, accepted specification, and policy identity.

## Motivation

Valence provenance links currently carry requirement IDs as strings. A valid string does not establish repository ownership, registry membership, specification freshness, or revision compatibility.

Valence owns canonical evidence linkage. It needs a versioned reference that preserves those domains without claiming requirement satisfaction.

## Scope

- Add typed requirement, stack revision set, and stack trace context models to `valence-core`.
- Validate typed references against a supplied content-bound Cairn requirement registry.
- Add strict profiles that reject missing, stale, duplicate, wrong-repository, and wrong-revision links.
- Add capability-rooted CLI validation with precise safe field-path diagnostics.
- Preserve legacy string-based v1 artifacts as lower-fidelity recorded-only inputs.

## Affected specs

- `content-bound-requirements`: typed references, registry validation, stack context, and legacy boundaries.

## Dependencies

This change consumes the reviewed registry schema produced by Cairn change `export-content-bound-requirement-registry`. The implementation must pin an exact accepted Cairn contract revision or frozen contract fixture.

## Non-goals

- Do not make Valence discover repository revisions or read VCS state from the core.
- Do not claim that registry membership proves requirement satisfaction.
- Do not rewrite existing v1 artifact hashes or silently upgrade string IDs.
- Do not require a live Cairn or Valence service.
- Do not make runtime, lifecycle, authority, or release decisions.

## Completion evidence

The change is complete when strict typed links validate deterministically, stale or cross-repository links fail, legacy links remain bounded, and CLI diagnostics name exact safe field paths.
