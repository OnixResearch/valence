# Evidence artifact envelope

Valence treats the Evidence IR envelope as a closed control-plane schema. The
accepted top-level fields are `schema`, `artifact_kind`, `stable_id`,
`canonical_hash`, `source_span`, `verification_role`, `payload`, and
`non_claims`. `source_span` is also closed. Unknown, duplicate, misspelled, or
misplaced control fields are rejected before hash or claim-policy checks; they
are never ignored or interpreted as authority.

The kind-specific `payload` value is intentionally open. Producers may add
payload fields without widening the outer schema, but every payload key and
value is included in the artifact's canonical BLAKE3 identity. Object keys are
encoded in recursive lexical order, independent of `serde_json` map-order
feature unification.

This boundary complements typed claim policy: strict decoding proves only that
accepted control fields were recognized and canonical content was hash-bound.
It does not prove evidence truth, behavioral correctness, semantic equivalence,
verifier soundness, requirement satisfaction, or release eligibility.
