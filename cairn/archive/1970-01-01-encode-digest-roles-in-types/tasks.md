# Tasks: Encode digest roles in Rust types

## Phase 1: Baseline and foundation

- [x] [serial] V1 Capture current shared-identity JSON, canonical material, BLAKE3 identities, and stack-provenance fixture outputs before core changes. r[valence.evidence_ir.nominal_digest_roles.compatibility]
- [x] [serial] I1 Add private `Digest<Role>` and borrowed `DigestRef<Role>` core types with marker roles, checked parsing, explicit accessors, and typed errors. r[valence.evidence_ir.nominal_digest_roles]
- [x] [parallel] V2 Add positive and negative parser tests for accepted spellings, malformed values, unsupported prefixes, wrong lengths, and non-hex input. r[valence.evidence_ir.nominal_digest_roles.validation]
- [x] [serial] I2 Add the closed wire DTO and `AnyDigest` adapter enum for heterogeneous serialized evidence. r[valence.evidence_ir.nominal_digest_wire]

## Phase 2: Core migration

- [x] [serial] I3 Migrate `shared_identity` constructors and validators from runtime expected-role arguments to typed role parameters. r[valence.evidence_ir.nominal_digest_roles]
- [x] [serial] I4 Consolidate stack-provenance local digest wrappers onto the shared generic type without weakening adapter boundaries. r[valence.evidence_ir.nominal_digest_migration]
- [x] [serial] I5 Migrate artifact-reference, function-address policy, graph summary, and Evidence IR helper seams in bounded compatibility-preserving steps. r[valence.evidence_ir.nominal_digest_migration]
- [x] [parallel] I6 Add explicit wire-to-core and core-to-wire adapters in `valence-cli` and producer-facing compatibility modules. r[valence.evidence_ir.nominal_digest_wire]

## Phase 3: Evidence and compatibility

- [x] [parallel] V3 Add compile-pass fixtures for same-role use and compile-fail fixtures for receipt, binary, policy, artifact, graph, sidecar, and source role substitutions. r[valence.evidence_ir.nominal_digest_compile_time]
- [x] [parallel] V4 Add serialization and canonical-identity golden tests proving the public wire and hash material remain unchanged. r[valence.evidence_ir.nominal_digest_roles.compatibility]
- [x] [parallel] V5 Add malformed-role, duplicate-role, stale-identity, and cross-role heterogeneous adapter tests. r[valence.evidence_ir.nominal_digest_wire.validation]
- [x] [parallel] V6 Run existing artifact-reference, function-address, stack-provenance, malformed-corpus, and downstream compatibility fixtures. r[valence.evidence_ir.nominal_digest_migration]

## Phase 4: Closeout

- [x] [serial] I7 Document owned and borrowed role types, wire adapters, consumer migration, and non-claims. r[valence.evidence_ir.nominal_digest_docs]
- [x] [serial] V7 Run formatting, workspace tests, Clippy, core purity smoke, Cairn validation, change gates, and the relevant Nix checks. r[valence.evidence_ir.nominal_digest_final_checks]

## Verification coverage

- `Scenario: Same-role digest is accepted` -> I1, V2, V3
- `Scenario: Cross-role digest does not compile` -> I1, V3
- `Scenario: Heterogeneous wire digest converts by role` -> I2, V5
- `Scenario: Wire role and digest disagree` -> I2, V5
- `Scenario: Shared identity uses typed roles` -> I3, V6
- `Scenario: Existing adapter wrappers converge` -> I4, V6
- `Scenario: Evidence surfaces retain role separation` -> I5, V6
- `Scenario: Public wire bytes remain stable` -> V1, I6, V4
- `Scenario: Canonical identities remain stable` -> V1, V4
- `Scenario: Malformed input fails before core use` -> V2, V5
- `Scenario: Typed digest claims remain bounded` -> I7, V7
