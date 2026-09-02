## Design

The adapter is a compatibility layer, not a receipt rewrite. It accepts parsed Cairn content-addressed receipt values and reports whether each record can be mapped to Valence Evidence IR identity/linkage concepts. It does not call Cairn, read repository state, or decide lifecycle readiness.

### Decisions

1. **Family manifest first.** Each supported Cairn receipt family is described by schema, hash field, hash-material exclusion profile, Valence artifact kind, verification role, and required non-claims.

2. **Function Object compatibility is explicit.** Cairn's source-token Function Object receipt shape and Valence's current `FunctionObjectV1` shape must be compared through a compatibility adapter. Byte-identical hash material is required only where the manifest marks a row as compatible.

3. **Legacy divergence is visible.** If a Cairn receipt shape is useful as historical or manual-review evidence but does not match Valence's current DTO, the adapter reports a migration exception rather than normalizing it silently.

4. **Ambient entropy remains rejected.** Display paths, temporary paths, timestamps, usernames, hostnames, and environment-derived values must not become identity hash material unless a future reviewed profile explicitly admits them.

5. **No policy promotion.** A compatible Cairn content-addressed receipt proves at most identity, linkage, and bounded non-claim conformance for the selected family. It does not prove source-code correctness, verifier soundness, release eligibility, or lifecycle readiness.

### Validation shape

A pure core validates in-memory receipt values and compatibility manifest rows. A CLI or test shell may load fixtures. Positive tests cover compatible Function Object source-token receipts, name-pointer drift records, and export bundle object ordering/linkage. Negative tests cover wrong hash, unsupported schema, missing required nested fields, ambient entropy in hash material, overbroad correctness claims, unknown pointer targets, and unsupported family rows.
