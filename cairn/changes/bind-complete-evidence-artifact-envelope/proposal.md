## Why

`check_evidence_artifact` currently deserializes directly into `EvidenceArtifact` with Serde's default unknown-field behavior. Unknown top-level fields are discarded before canonical hash recomputation and before overclaim scanning. A valid artifact can therefore retain its old hash while adding an unbound field such as `claims: ["proves whole-program correctness"]`; the current CLI reports that modified envelope as valid.

Canonical artifact hashing also relies on `serde_json` map behavior after `to_value`. A downstream feature-unification change such as `preserve_order` must not be able to alter stack-owned identity bytes. Accepted fields and canonical ordering need an explicit contract rather than implicit serializer defaults.

## What Changes

- Add strict, field-aware decoding for the closed Evidence IR envelope and closed nested control-plane records.
- Reject unknown, duplicate, misspelled, or misplaced control fields before typed validation and hash comparison.
- Keep the kind-specific `payload` open but bind its complete canonical value into the artifact hash.
- Route canonical artifact identity through feature-independent key ordering with compatibility goldens for current valid hashes.
- Add exploit regression fixtures for unknown top-level claims, unknown span metadata, duplicate fields, and serializer feature unification.
- Coordinate with the active typed-claim remediation without treating unknown prose fields as authority.

## Impact

- **Evidence acceptance**: envelopes carrying previously ignored fields become invalid rather than silently losing data.
- **Identity**: current known-field artifacts retain their hashes; any unavoidable hash change requires an explicitly versioned migration instead of silent churn.
- **Architecture**: decoding, field inventory, and canonicalization remain pure functions over in-memory JSON.
- **Claims**: strict envelope binding proves only that accepted control fields were recognized and canonical content was hash-bound; it does not prove evidence truth or correctness.
