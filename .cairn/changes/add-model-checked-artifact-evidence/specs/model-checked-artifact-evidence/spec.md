# Model-Checked Artifact Evidence Specification

## Purpose

Define a bounded role vocabulary and a BLAKE3-bound evidence sidecar for model-checked artifacts, with closed verdict classes and attached non-claims, without transferring engine, model, or implementation correctness into Valence.

## ADDED Requirements

### Requirement: Model-checked evidence has explicit roles

r[valence.model_check_evidence.roles]

Valence MUST provide a bounded role vocabulary and evidence spans for the spec, invariant, checker, bounds, verdict, and witness-trace layers of a model-checked artifact, with an independent identity per layer.

#### Scenario: Complete role set is admitted
- GIVEN every layer of a model-checked artifact carries a distinct admitted role
- WHEN role admission runs
- THEN Valence MUST produce one deterministic identity binding per role.

#### Scenario: A layer is role-confused
- GIVEN a layer is assigned a role that contradicts its content
- WHEN role admission runs
- THEN Valence MUST fail closed.

### Requirement: The sidecar is BLAKE3-bound and validatable

r[valence.model_check_evidence.sidecar]

Valence MUST define a versioned evidence sidecar that binds model hash, invariant hashes, checker identity and version, declared bounds, verdict class, witness-trace hash where present, and non-claims, and MUST validate the sidecar against the admitted artifact identities.

#### Scenario: Sidecar matches its artifacts
- GIVEN every sidecar identity matches the supplied artifacts
- WHEN sidecar validation runs
- THEN Valence MUST admit the sidecar.

#### Scenario: Sidecar is stale or tampered
- GIVEN the checker, model, bound, or trace identity bound in the sidecar does not match the supplied artifacts
- WHEN sidecar validation runs
- THEN Valence MUST fail closed and MUST report the mismatched layer.

### Requirement: Verdict classes are closed

r[valence.model_check_evidence.verdict]

Valence MUST admit only the `verified-within-bounds`, `violation-with-witness`, and `unsupported-checker` verdict classes, and MUST require a witness-trace hash on the violation class.

#### Scenario: Valid verdict class is admitted
- GIVEN a sidecar uses an admitted verdict class with the required trace binding
- WHEN verdict admission runs
- THEN Valence MUST accept the class.

#### Scenario: Stale or invalid verdict class appears
- GIVEN a sidecar uses a non-admitted class or omits a required witness-trace hash
- WHEN verdict admission runs
- THEN Valence MUST reject the sidecar.

### Requirement: The bounded non-claim stays attached

r[valence.model_check_evidence.nonclaim]

Every model-checked evidence record MUST carry the non-claim that its verdict does not prove implementation correctness, model validity, engine correctness, or general temporal satisfiability.

#### Scenario: Non-claim is present
- GIVEN a record includes the bounded non-claim
- WHEN record validation runs
- THEN Valence MUST accept the record with the non-claim attached.

#### Scenario: Non-claim is missing
- GIVEN a record omits the bounded non-claim
- WHEN record validation runs
- THEN Valence MUST fail closed.

### Requirement: Suite has positive and negative fixtures

r[valence.model_check_evidence.fixtures]

The model-checked evidence suite MUST include positive fixtures for valid sidecars and role admission, and negative fixtures for stale checker identity, tampered traces, wrong roles, invalid verdict classes, and missing non-claims.

#### Scenario: Suite runs
- GIVEN all fixtures are committed with their declared identities
- WHEN the validation harness executes
- THEN positive fixtures MUST pass and each negative fixture MUST fail at its declared boundary.

### Requirement: Evidence changes run full validation

r[valence.model_check_evidence.validation]

Changes to role vocabulary, sidecar schema, verdict classes, validation, or fixtures MUST run focused core tests, clippy with warnings denied, lifecycle validation, Cairn gates, and relevant flake checks.

#### Scenario: Change is validated
- GIVEN an evidence change passes all focused checks
- WHEN lifecycle validation is assembled
- THEN it MUST include both positive and negative results and MUST identify the exact schema version and affected roles.
