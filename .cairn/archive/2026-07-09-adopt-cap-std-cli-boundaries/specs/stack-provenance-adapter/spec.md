# Stack Provenance Adapter Specification

## Purpose

Adds capability-bounded local filesystem roots for Valence CLI evidence and stack-provenance file boundaries.

## Requirements

### Requirement: Valence CLI filesystem authority is capability-rooted
r[valence.stack_provenance_adapter.cap_std_cli_boundary.root_wrappers] Valence MUST represent CLI input artifact, manifest, sidecar, binary, and output filesystem authority as typed capability roots opened by `valence-cli`.

#### Scenario: Valid CLI artifact path stays under the declared root
r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.positive]
- GIVEN `valence-cli` has opened an operator-declared input or output root
- WHEN stack-provenance or evidence-file commands read a valid relative artifact path, measure a binary, or write a sidecar/report through the typed root
- THEN the operation MUST remain confined to that declared root while preserving existing valid-input behavior.

#### Scenario: Invalid CLI artifact path cannot escape the root
r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, a symlink escape attempt, or an output escape attempt
- WHEN `valence-cli` resolves the path through the typed root
- THEN Valence MUST fail closed before reading, measuring, or writing outside the declared root.

### Requirement: Capability dependency stays outside Valence core
r[valence.stack_provenance_adapter.cap_std_cli_boundary.dependency] Valence MUST add `cap-std` only to `valence-cli` shell/adaptor code that owns filesystem effects.

#### Scenario: Evidence core remains pure
r[valence.stack_provenance_adapter.cap_std_cli_boundary.conversion]
- GIVEN `valence-core` Evidence IR, provenance, role, hash, and non-claim validation logic is reviewed
- WHEN cap-std adoption is complete
- THEN `valence-core` MUST receive loaded bytes, parsed DTOs, or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths directly.

### Requirement: Capability boundary is documented without evidence overclaims
r[valence.stack_provenance_adapter.cap_std_cli_boundary.docs] Valence MUST document that capability roots bound local CLI filesystem authority only and do not prove evidence truth, behavioral correctness, release eligibility, or verifier soundness.

#### Scenario: Closeout evidence includes CLI boundary validation
r[valence.stack_provenance_adapter.cap_std_cli_boundary.validation]
- GIVEN the cap-std CLI-boundary change is ready to archive
- WHEN focused CLI tests, core purity checks, Cairn validation, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible non-claim documentation.
