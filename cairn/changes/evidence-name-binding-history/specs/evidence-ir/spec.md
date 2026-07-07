# Evidence IR Specification

## Purpose

Extends `evidence-ir` with explicit name-binding history for aliases, paths, and human-readable labels.

## Requirements

### Requirement: Evidence name-binding history
r[valence.evidence_ir.name_binding_history] Valence MUST model human-readable names, aliases, and paths as bindings to content-addressed evidence identities rather than as identities themselves.

#### Scenario: Binding records map aliases to identities
r[valence.evidence_ir.name_binding_history.records]
- GIVEN an evidence content identity and a human-readable alias, source path, or requirement name
- WHEN Valence records the association
- THEN it MUST store a binding record that keeps the alias separate from the content identity.

#### Scenario: Bindings are carried in receipts when review-relevant
r[valence.evidence_ir.name_binding_history.receipts]
- GIVEN a verifier receipt, sidecar, or report relies on an alias for human review
- WHEN Valence renders the receipt
- THEN it SHOULD include the alias binding history needed to audit the content identity.

#### Scenario: Alias resolution is deterministic
r[valence.evidence_ir.name_binding_history.resolve]
- GIVEN an alias with exactly one current content identity binding
- WHEN Valence resolves the alias
- THEN it MUST return that content identity deterministically.

#### Scenario: Ambiguous aliases fail closed
r[valence.evidence_ir.name_binding_history.ambiguous]
- GIVEN an alias is bound to multiple content identities
- WHEN identity-level verification depends on that alias
- THEN Valence MUST fail closed with a deterministic ambiguity diagnostic.

#### Scenario: Renames preserve identity linkage
r[valence.evidence_ir.name_binding_history.rename]
- GIVEN a definition or artifact is renamed without changing canonical content
- WHEN Valence verifies provenance through the new alias
- THEN provenance SHOULD resolve to the same content identity and retain prior binding history.

#### Scenario: Stale bindings are reported
r[valence.evidence_ir.name_binding_history.stale]
- GIVEN a binding claims an alias points to a content identity that is absent from the evidence graph
- WHEN Valence validates bindings
- THEN validation MUST fail closed with a stale-binding diagnostic.

#### Scenario: Name-binding boundary is documented
r[valence.evidence_ir.name_binding_history.docs]
- GIVEN documentation or receipts describe name bindings
- WHEN the supported claim is stated
- THEN the output MUST state that aliases are review metadata and do not prove semantic equivalence or requirement satisfaction.
