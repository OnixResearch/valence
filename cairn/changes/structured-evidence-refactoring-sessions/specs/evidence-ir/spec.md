# Evidence IR Specification

## Purpose

Extends `evidence-ir` with structured evidence refactoring sessions for safe alias/name/path migrations.

## Requirements

### Requirement: Structured evidence refactoring sessions
r[valence.evidence_ir.structured_refactoring] Valence MUST provide a typed refactoring session model for evidence alias, name, path, and requirement-binding migrations that preserve content identity.

#### Scenario: Session input is typed
r[valence.evidence_ir.structured_refactoring.session]
- GIVEN a requested evidence rename or binding migration
- WHEN Valence parses the session
- THEN the session MUST record source bindings, target bindings, intended content identities, policy constraints, and non-claims.

#### Scenario: Dry-run plan reports impact
r[valence.evidence_ir.structured_refactoring.plan]
- GIVEN a valid refactoring session and evidence graph
- WHEN Valence plans the session
- THEN it MUST produce a deterministic dry-run report listing affected nodes, edges, aliases, receipts, and mutation surfaces.

#### Scenario: Unsafe plans are blocked
r[valence.evidence_ir.structured_refactoring.blockers]
- GIVEN a session has ambiguous aliases, missing identities, dangling edges, unsupported mutation surfaces, or would require semantic-equivalence claims
- WHEN Valence plans or executes it
- THEN the session MUST be blocked with deterministic diagnostics.

#### Scenario: Migration receipts include before and after identities
r[valence.evidence_ir.structured_refactoring.receipts]
- GIVEN a refactoring session executes
- WHEN Valence records the migration receipt
- THEN the receipt SHOULD include before/after identity manifests, binding changes, policy identity, and mutation diagnostics.

#### Scenario: Positive and negative tests cover refactors
r[valence.evidence_ir.structured_refactoring.tests]
- GIVEN refactoring fixtures
- WHEN tests run
- THEN content-preserving alias/path migrations MUST pass and ambiguous, dangling, or semantic-equivalence-requiring migrations MUST fail.

#### Scenario: Refactoring boundary is documented
r[valence.evidence_ir.structured_refactoring.docs]
- GIVEN documentation or receipts describe structured refactoring
- WHEN the supported claim is stated
- THEN the output MUST state that sessions preserve and rewrite identity bindings only, not semantic equivalence, correctness, or requirement satisfaction.
