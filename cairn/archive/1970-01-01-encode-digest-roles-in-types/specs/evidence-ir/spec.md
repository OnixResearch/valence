# Evidence IR Specification Delta

## ADDED Requirements

### Requirement: Digest roles are encoded in Rust types

r[valence.evidence_ir.nominal_digest_roles] Valence MUST represent statically known BLAKE3 evidence roles with distinct Rust types over one private checked digest implementation.

#### Scenario: Same-role digest is accepted

r[valence.evidence_ir.nominal_digest_roles.same_role]
- GIVEN a typed binary digest has valid normalized BLAKE3 content
- WHEN an API requires the binary digest role
- THEN the API MUST accept the value without another runtime role argument.

#### Scenario: Cross-role digest does not compile

r[valence.evidence_ir.nominal_digest_compile_time]
- GIVEN binary and receipt digests use distinct marker-role instantiations
- WHEN source passes a receipt digest to an API that requires a binary digest
- THEN the source MUST fail compilation with a type mismatch.

#### Scenario: Malformed input fails before core use

r[valence.evidence_ir.nominal_digest_roles.validation]
- GIVEN untrusted text has an unsupported prefix, wrong length, non-hex content, or unsupported spelling
- WHEN an adapter constructs a typed digest
- THEN construction MUST fail with a deterministic digest diagnostic
- AND the invalid value MUST NOT enter the typed core.

### Requirement: Heterogeneous digest wire data has an explicit adapter

r[valence.evidence_ir.nominal_digest_wire] Valence MUST preserve serialized role tags at wire boundaries and MUST convert admitted wire records into a closed enum of role-specific typed digests.

#### Scenario: Heterogeneous wire digest converts by role

r[valence.evidence_ir.nominal_digest_wire.convert]
- GIVEN a supported wire role and a valid digest spelling for that role
- WHEN wire admission runs
- THEN Valence MUST construct the matching role-specific digest variant.

#### Scenario: Wire role and digest disagree

r[valence.evidence_ir.nominal_digest_wire.validation]
- GIVEN a wire record uses an unsupported role, conflicting role metadata, or a digest from another required domain
- WHEN wire admission runs
- THEN Valence MUST fail closed with a deterministic role or domain diagnostic.

### Requirement: Typed digest migration preserves public identity

r[valence.evidence_ir.nominal_digest_migration] Valence MUST migrate shared identity, artifact references, stack provenance, function-address policy, and graph digest seams through explicit compatibility adapters.

#### Scenario: Public wire bytes remain stable

r[valence.evidence_ir.nominal_digest_roles.compatibility]
- GIVEN an existing supported evidence fixture is serialized before and after the typed migration
- WHEN its public JSON and canonical material are compared
- THEN the bytes and BLAKE3 identities MUST remain unchanged unless a separate versioned schema change approves the difference.

#### Scenario: Runtime role checks move to admission

r[valence.evidence_ir.nominal_digest_migration.admission]
- GIVEN an adapter receives heterogeneous untrusted evidence
- WHEN it admits a role-specific digest
- THEN runtime role checking MUST occur at that adapter
- AND downstream role-specific core APIs MUST use the typed role.

### Requirement: Typed digest claims remain bounded

r[valence.evidence_ir.nominal_digest_docs] Valence documentation and reports MUST state that digest newtypes prove local syntax, normalization, and role separation only.

#### Scenario: Boundary remains visible

r[valence.evidence_ir.nominal_digest_final_checks]
- GIVEN a typed digest passes construction and role checking
- WHEN Valence reports the supported claim
- THEN it MUST NOT claim measured-byte truth, evidence truth, producer authority, behavioral correctness, verifier soundness, or release eligibility.
