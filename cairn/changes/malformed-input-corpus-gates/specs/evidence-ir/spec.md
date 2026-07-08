# Evidence IR Specification

## Purpose

Extends `evidence-ir` with malformed-input corpus gates for deterministic fail-closed behavior.

## Requirements

### Requirement: Malformed input corpus gates
r[valence.evidence_ir.malformed_input_corpus] Valence MUST maintain a checked-in malformed-input corpus whose fixtures are expected to fail closed with deterministic diagnostics.

#### Scenario: Corpus runner expects failure
r[valence.evidence_ir.malformed_input_corpus.runner]
- GIVEN a malformed-input corpus fixture
- WHEN the corpus runner evaluates it
- THEN unexpected success MUST fail the harness and expected issue codes MUST be compared deterministically.

#### Scenario: Digest and graph errors are covered
r[valence.evidence_ir.malformed_input_corpus.digest_and_graph]
- GIVEN malformed digest, duplicate graph node, or dangling graph edge fixtures
- WHEN the corpus runner evaluates them
- THEN Valence MUST produce deterministic fail-closed diagnostics.

#### Scenario: Binding and capability errors are covered
r[valence.evidence_ir.malformed_input_corpus.bindings_and_capabilities]
- GIVEN stale binding, ambiguous alias, unknown capability, or denied capability fixtures
- WHEN the corpus runner evaluates them
- THEN Valence MUST produce deterministic fail-closed diagnostics.

#### Scenario: Boundary and storage errors are covered
r[valence.evidence_ir.malformed_input_corpus.boundary_and_storage]
- GIVEN weakened non-claim or durable-envelope mismatch fixtures
- WHEN the corpus runner evaluates them
- THEN Valence MUST produce deterministic fail-closed diagnostics.

#### Scenario: Diagnostics are stable
r[valence.evidence_ir.malformed_input_corpus.diagnostics]
- GIVEN the same malformed corpus fixtures in different discovery orders
- WHEN the corpus runner renders its report
- THEN issue codes and fixture results MUST be sorted deterministically.

#### Scenario: Corpus claims remain bounded
r[valence.evidence_ir.malformed_input_corpus.non_claims]
- GIVEN malformed corpus reports describe fail-closed behavior
- WHEN the supported claim is stated
- THEN the report MUST state that corpus coverage demonstrates checked failure handling for fixture inputs only, not complete input-space coverage or verifier soundness.
