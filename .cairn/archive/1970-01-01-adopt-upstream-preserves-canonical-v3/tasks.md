# Tasks

## 1. Upstream canonical binary codec

- [x] [serial] [task:codec] Implement pure bounded Preserves 0.996.3 text parser, canonical binary encoder, and strict canonical binary decoder in `crates/valence-core/src/preserves_canonical.rs` with explicit limits and rejections. r[valence.evidence_ir.preserves_canonical_v3.profile]

## 2. Profile versioning, exclusion, migration, and bridge

- [x] [serial] [task:profile] [after:codec] Switch active `preserves_evidence` validation to `preserves-canonical-binary-v3`, retain `preserves-packed-structural-v2` only for explicit historical reproduction with the `(x)` 19-byte regression, and enforce v3 structural self-hash exclusion. r[valence.evidence_ir.preserves_canonical_v3.historical_v2] r[valence.evidence_ir.preserves_canonical_v3.self_hash_exclusion]
- [x] [serial] [task:migration] [after:profile] Implement `preserves-integrity-migration-v2` receipts binding exact original bytes, old/new profiles, exclusion rules, and identities, plus opaque-vs-canonical bridge checks and the pinned Kamacite interop fixture. r[valence.evidence_ir.preserves_canonical_v3.migration] r[valence.evidence_ir.preserves_canonical_v3.opaque_vs_canonical]

## 3. Documentation and verification

- [x] [serial] [task:docs] [after:migration] Update `docs/preserves-integrity-migration.md` and run workspace format, build, test, strict Clippy, core-purity smoke, and Cairn gates. r[valence.evidence_ir.preserves_canonical_v3.profile]
