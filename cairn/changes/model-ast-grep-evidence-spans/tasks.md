## Tasks

- [ ] [serial] r[valence.ast_grep_evidence_spans.profile] Define the ast-grep Evidence IR adapter profile and DTO shape.
- [ ] [depends:ast-grep-evidence-profile] r[valence.ast_grep_evidence_spans.identity] Validate ast-grep tool identity, rule id, rule bundle BLAKE3 identity, scan scope identity, and finding identity.
- [ ] [depends:ast-grep-evidence-profile] r[valence.ast_grep_evidence_spans.spans] Map ast-grep ranges into Valence source spans with deterministic diagnostics for malformed spans.
- [ ] [depends:ast-grep-evidence-spans] r[valence.ast_grep_evidence_spans.roles] Enforce recorded-only or boundary roles and reject unsupported promotion.
- [ ] [depends:ast-grep-evidence-roles] r[valence.ast_grep_evidence_spans.nonclaims] Reject missing or weakened non-claims, semantic-equivalence claims, proof claims, and release-readiness claims.
- [ ] [depends:ast-grep-evidence-nonclaims] r[valence.ast_grep_evidence_spans.fixtures] Add positive and negative fixtures for valid findings, missing spans, stale bundle identity, unsupported roles, and overclaims.
- [ ] [depends:ast-grep-evidence-fixtures] r[valence.ast_grep_evidence_spans.validation] Run adapter tests, malformed fixture tests, Cairn gates, cargo tests, and focused Valence validation.
