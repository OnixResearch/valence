## Why

Valence now owns the richer standalone Evidence IR, stack-provenance adapter, role policy, and non-claim enforcement surfaces, but `octet/crates/valence-core` still exists as an in-repository readiness seam. Downstream consumers can therefore accidentally bind to the older Octet-hosted crate and miss newer Valence-owned contracts.

This change makes standalone Valence the canonical owner of `valence-core` DTOs, contracts, compatibility fixtures, and provenance semantics while keeping Octet responsible for Octet-specific source scanning, command orchestration, and artifact publication.

## What Changes

- Define a canonical Valence-owned crate/API boundary for `valence-core`.
- Add a cutover manifest that maps Octet-hosted modules to Valence-owned modules, schema identities, fixtures, and compatibility status.
- Add drift checks that fail when Octet-hosted copies diverge from the Valence-owned canonical source without an explicit migration exception.
- Document the dependency direction: Octet emits Valence-ready evidence, but Valence owns Evidence IR semantics.
- Preserve non-claims: the cutover proves DTO/source ownership and compatibility only, not behavioral correctness, semantic equivalence of arbitrary code, or release eligibility.

## Impact

- **Valence** becomes the source of truth for Evidence IR crate semantics.
- **Octet** keeps CLI, lint, scan, proof-rail, and publication ownership while consuming Valence contracts.
- **Downstream consumers** get one canonical dependency target for `valence-core`.
- **Testing** needs positive compatibility fixtures and negative drift/stale-copy fixtures.
