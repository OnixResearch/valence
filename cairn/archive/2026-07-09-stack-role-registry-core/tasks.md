## Tasks

- [x] [serial] Define canonical stack role, schema role, verification role, alias, and compatibility contracts. r[valence.evidence_ir.stack_role_registry_core.contract]
- [x] [serial] Add positive fixture rows for canonical stack roles, schema/role pairs, verification roles, and approved aliases. r[valence.evidence_ir.stack_role_registry_core.fixtures.positive]
- [x] [serial] Add negative fixture rows for unknown roles, duplicate required roles, stale aliases, schema/role mismatches, missing verification roles, missing non-claims, and overclaims. r[valence.evidence_ir.stack_role_registry_core.fixtures.negative]
- [x] [serial] Implement pure registry validation and role/schema matching in `valence-core`. r[valence.evidence_ir.stack_role_registry_core.validation]
- [x] [serial] Add compatibility adapters that preserve existing public string encodings. r[valence.evidence_ir.stack_role_registry_core.adapters]
- [x] [serial] Document downstream migration order and bounded non-claims. r[valence.evidence_ir.stack_role_registry_core.docs]
- [x] [serial] Run focused `valence-core` tests, adapter fixture tests, and Cairn validation/gates. r[valence.evidence_ir.stack_role_registry_core.final_validation]
