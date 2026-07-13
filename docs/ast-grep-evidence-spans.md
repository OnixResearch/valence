# ast-grep evidence spans

The `valence.ast-grep-evidence.v1` adapter accepts already-loaded ast-grep finding metadata and records syntax-aware source-shape evidence. Producers must bind the ast-grep version, rule ID, canonical rule-bundle BLAKE3 identity, canonical scan-scope BLAKE3 identity, finding identity, and an exact source range.

Ast-grep zero-based line and column positions are mapped to Valence's one-based `EvidenceSpan` coordinates. Byte offsets are preserved without widening the finding to a whole-file claim. The shared span validator rejects missing, reversed, overflowing, or otherwise malformed locations.

Findings may use only `recorded_only` or `boundary` verification roles. Replacement previews and transformed metavariables are review metadata. They cannot establish behavior preservation, semantic equivalence, source correctness, proof discharge, or release eligibility. Valence does not execute ast-grep or assess whether a pattern is correct, complete, or semantically meaningful.
