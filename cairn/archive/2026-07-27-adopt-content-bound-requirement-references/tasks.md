# Tasks: Adopt content-bound requirement references

## Core model

- [x] [serial] Define typed requirement reference, stack revision set, stack trace context, and issue DTOs in `valence-core` r[valence.provenance.content_bound_requirement_ref]
- [x] [serial] Implement pure canonical hashing with distinct identity domains and deterministic ordering r[valence.provenance.stack_trace_context]
- [x] [serial] Implement exact Cairn registry membership, repository, revision, specification, policy, and row validation r[valence.provenance.requirement_registry_validation]

## Compatibility and shell

- [x] [parallel] Preserve legacy string requirement IDs as recorded-only v1 compatibility input without silent promotion r[valence.provenance.legacy_requirement_id_boundary]
- [x] [serial] Add capability-rooted requirement-reference and stack-context validation commands r[valence.provenance.requirement_registry_validation]
- [x] [parallel] Add `serde_path_to_error` only to the CLI shell and emit bounded deterministic field-path diagnostics r[valence.provenance.requirement_registry_validation]

## Verification

- [x] [parallel] Add positive core fixtures for one repository, a selected stack, and optional complete links r[valence.provenance.stack_trace_context]
- [x] [parallel] Add negative fixtures for stale registries, wrong revisions, duplicate repositories, domain substitution, missing rows, and weakened non-claims r[valence.provenance.requirement_registry_validation]
- [x] [parallel] Add CLI tests for malformed nested input, unsafe paths, legacy compatibility, and no-write failure behavior r[valence.provenance.legacy_requirement_id_boundary]

## Documentation and closeout

- [x] [parallel] Document producer rollout order, strict-profile behavior, compatibility, and non-claims r[valence.provenance.content_bound_requirement_ref]
- [x] [serial] Run formatting, workspace tests, strict Clippy, core purity smoke, Cairn validation, traceability coverage, and focused Nix checks r[valence.provenance.stack_trace_context]
- [x] [serial] Run proposal, design, and tasks gates before sync and archive r[valence.provenance.legacy_requirement_id_boundary]
