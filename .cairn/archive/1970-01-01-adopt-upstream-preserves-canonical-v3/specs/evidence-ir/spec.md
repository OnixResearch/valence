# Evidence IR Delta: Upstream Preserves Canonical Binary v3

## ADDED Requirements

### Requirement: Active Preserves evidence uses upstream 0.996.3 canonical binary encoding
r[valence.evidence_ir.preserves_canonical_v3.profile] Valence MUST encode and validate active Preserves evidence under profile `preserves-canonical-binary-v3` using upstream Preserves `0.996.3` canonical binary bytes with BLAKE3 content identities, encoded-byte ordering for sets and dictionaries, and explicit fail-closed rejections for annotations, embedded values, IEEE 754 floats, duplicate set members or dictionary keys, non-minimal varints or integers, non-canonical binary ordering, trailing bytes, and resource limit breaches.

#### Scenario: Upstream Preserves values encode canonically and decode as one value
- GIVEN an admitted upstream Preserves value including records, sequences, sets, dictionaries, booleans, signed integers, strings, byte strings, symbols, and Unicode scalars
- WHEN Valence encodes canonical bytes under `preserves-canonical-binary-v3`
- THEN the output MUST decode as exactly one upstream Preserves `0.996.3` value with no trailing bytes and match normative reference bytes.

#### Scenario: Non-canonical or out-of-profile input fails closed
- GIVEN input containing annotations, embedded values, floats, duplicate keys/members, non-minimal varint or integer bytes, out-of-order set/dictionary bytes, trailing bytes, or exceeding configured limits
- WHEN Valence decodes or encodes under `preserves-canonical-binary-v3`
- THEN validation MUST fail closed with a deterministic error code.

### Requirement: Historical private v2 profile is explicitly separated from canonical Preserves
r[valence.evidence_ir.preserves_canonical_v3.historical_v2] Valence MUST retain `preserves-packed-structural-v2` only as explicitly identified historical private-format reproduction and MUST reject `preserves-packed-structural-v2` and `preserves-text-v1` on active canonical Preserves validation paths.

#### Scenario: Minimal historical regression reproduces v2 bytes and fails upstream decode
- GIVEN the minimal legacy input `(x)` under `none-structural-v2`
- WHEN historical v2 reproduction runs
- THEN it MUST reproduce the exact 19-byte private encoding while upstream canonical Preserves decoding rejects those bytes as trailing data after `#f` and active `validate_preserves_evidence` rejects `preserves-packed-structural-v2`.

### Requirement: Structural self-hash exclusion preserves payload collisions and rejects misplaced fields
r[valence.evidence_ir.preserves_canonical_v3.self_hash_exclusion] Valence MUST remove only exact top-level symbol self-hash fields declared by the selected v3 exclusion profile while keeping payload strings, payload string keys, and substring symbol names hash-bearing and failing closed on duplicate, nested, misplaced, or malformed envelopes.

#### Scenario: Payload name collisions change the canonical digest
- GIVEN two `preserves-canonical-binary-v3` envelopes that differ only in payload strings or payload string keys containing `receipt_hash` or `preserves_content_hash`
- WHEN canonical digests are computed under `receipt-self-hash-canonical-v3`
- THEN the digests MUST differ while nested or duplicate structural self-hash fields fail closed.

### Requirement: Versioned migration evidence binds exact originals, profiles, exclusions, and identities
r[valence.evidence_ir.preserves_canonical_v3.migration] Valence MUST emit and validate `preserves-integrity-migration-v2` receipts binding the exact original-bytes BLAKE3 hash, source profile, source exclusion rule, validated source identity, target profile `preserves-canonical-binary-v3`, target exclusion rule, target canonical identity, and required non-claims.

#### Scenario: Missing originals or unsupported profiles cannot migrate
- GIVEN empty original bytes, an unsupported source profile, or a mismatched declared source identity
- WHEN migration into `preserves-canonical-binary-v3` is requested
- THEN migration MUST fail closed without emitting a receipt.

### Requirement: Opaque-byte carrier checks stay distinct from canonical Preserves validation
r[valence.evidence_ir.preserves_canonical_v3.opaque_vs_canonical] Valence MUST keep opaque-byte carrier digest validation distinct from canonical Preserves binary parsing and validation, and verify interoperability against a pinned upstream producer fixture.

#### Scenario: Historical private bytes fail canonical bridge validation while pinned Kamacite bytes pass
- GIVEN a bridge row carrying private v2 bytes versus a pinned Kamacite canonical Preserves record
- WHEN canonical bridge validation runs
- THEN the private v2 bytes MUST fail canonical validation while the pinned Kamacite canonical binary bytes MUST pass.
