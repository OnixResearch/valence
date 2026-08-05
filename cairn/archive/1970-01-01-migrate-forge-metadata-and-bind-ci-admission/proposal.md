# Migrate forge metadata and bind CI admission

## Why

Onix-owned Git source is available through Radicle, but issue and pull-request metadata still lacks a deterministic, reviewable migration path. The existing Radicle CI runner also publishes bounded observations without producing a typed input that an external canonical-ref guard can enforce. Treating prose comments, source-host exports, or successful process exits as authority would permit metadata loss, actor impersonation, stale-result admission, and false mandatory-CI claims.

## What changes

- Add a pure forge-migration core that validates closed normalized issue and pull-request records, derives BLAKE3 source identities, and emits deterministic native Radicle issue/patch operation plans. r[valence.radicle_forge_ops.migration_core]
- Extend `valence-radicle` with preview-first execution against one explicitly selected local Git/Radicle repository. It writes only built-in `xyz.radicle.issue` or `xyz.radicle.patch` COB refs, verifies every resulting entry, and emits a source-to-target binding receipt. r[valence.radicle_forge_ops.native_cobs] r[valence.radicle_forge_ops.binding]
- Preserve source authors, timestamps, states, labels, comments, reviews, base/head objects, and source URLs as attributed recorded metadata. The importer signs its own operations and never impersonates source actors or converts historical review text into Radicle approval authority. r[valence.radicle_forge_ops.attribution]
- Add a pure CI-admission core that binds an exact patch/revision/head, successful typed CI event/result, signed bot-status observation, unique delegate approvals, current canonical predecessor, and descendant relation into an admission receipt for an external canonical-ref guard. r[valence.radicle_forge_ops.ci_admission]
- Keep network access, source-host API collection, key custody, node lifecycle, canonical-ref mutation, deployment, and release decisions outside Valence. r[valence.radicle_forge_ops.boundaries]

## Impact

- **Core:** `valence-core` gains deterministic forge-migration and CI-admission DTOs, validators, plan/receipt identity, and fail-closed diagnostics over already-loaded inputs.
- **Adapter:** `valence-radicle` gains local preview/execute and saved-receipt verification commands using exact reviewed Radicle collaboration formats.
- **Contracts:** Nickel owns bounds, source kinds, state/review mappings, required non-claims, and admission-policy defaults; checked runtime JSON remains freshness-checked.
- **Downstream:** OnixOS may consume the portable policy and onix-core may require the admission receipt before its own compare-and-swap shell mutates a canonical ref. Neither receives Valence evidence authority as canonical-ref authority.
- **Compatibility:** Existing Evidence IR, sidecar COB, issue, patch, CI result, and publication identities remain unchanged.
- **Live scope:** No source-host API is contacted and live CI remains limited to Bounded Exec. Production collaboration objects and canonical refs are not mutated by repository validation.
