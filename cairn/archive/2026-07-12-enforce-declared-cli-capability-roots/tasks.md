## Phase 1: Pure authority plan

- [x] [serial] Define typed role-root declarations, relative locators, and pure access-plan validation without filesystem I/O. r[valence.stack_provenance_adapter.declared_capability_roots.contract]
- [x] [serial] Reject omitted roots, absolute locators, platform prefixes, traversal, and operation/role mismatches before opening a capability handle. r[valence.stack_provenance_adapter.declared_capability_roots.relative_locators]
- [x] [parallel] Add positive pure-core fixtures for nested relative locators under explicitly declared input and output roots. r[valence.stack_provenance_adapter.declared_capability_roots.fixtures.positive]
- [x] [parallel] Add negative pure-core fixtures for authority derivation and locator escape attempts. r[valence.stack_provenance_adapter.declared_capability_roots.fixtures.negative]

## Phase 2: CLI shell cutover

- [x] [serial] Replace parent-derived ambient root opening with explicit role-root handles across `check`, `explain`, `stack-provenance`, and `function-address` commands. r[valence.stack_provenance_adapter.declared_capability_roots.shell_boundary]
- [x] [serial] Preserve typed separation between artifact, manifest, sidecar, binary, and output operations without requiring distinct physical directories. r[valence.stack_provenance_adapter.declared_capability_roots.role_separation]
- [x] [parallel] Add CLI integration tests proving valid in-root access succeeds and omitted roots, absolute locators, traversal, wrong-role access, and symlink escapes fail without outside reads or writes. r[valence.stack_provenance_adapter.declared_capability_roots.fixtures.negative]

## Phase 3: Compatibility and evidence

- [x] [serial] Document the root/locator CLI migration, removal of parent-derived authority, and bounded confinement non-claim. r[valence.stack_provenance_adapter.declared_capability_roots.docs]
- [x] [serial] Run focused CLI tests, the core-purity smoke, workspace tests, and Cairn validation/gates. r[valence.stack_provenance_adapter.declared_capability_roots.final_validation]
