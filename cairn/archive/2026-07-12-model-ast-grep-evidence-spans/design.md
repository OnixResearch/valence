## Design

1. Add an Evidence IR adapter profile for ast-grep findings, scans, and rewrite candidates.
2. Require producer identity fields: ast-grep tool version, rule id, rule bundle BLAKE3 identity, scan scope identity, and finding identity.
3. Map ast-grep ranges into Valence evidence spans with source id, byte offsets, line and column ranges, and optional labels.
4. Permit verification roles only when they preserve the bounded claim: `RecordedOnly` for candidate source-shape findings and `Boundary` for policy/gate decisions about the finding.
5. Preserve replacement text or transformed metavariables only as review metadata, not as semantic-equivalence evidence.
6. Reject missing non-claims, unsupported roles, stale bundle identity, malformed spans, and behavioral or release-readiness overclaims.
7. Keep parsing and validation in pure core over already-loaded DTOs; CLI shell owns file reads and presentation.

## Non-Goals

- No execution of ast-grep from Valence.
- No validation that an ast-grep pattern is correct, complete, or semantically meaningful.
- No claim that a rewrite candidate preserves behavior.
