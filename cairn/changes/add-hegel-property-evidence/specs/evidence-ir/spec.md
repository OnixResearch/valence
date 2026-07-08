# Evidence IR Specification

## Purpose

Extends `evidence-ir` with bounded Hegel property evidence for pure Valence cores.

## Requirements

### Requirement: Hegel property receipts
r[valence.evidence_ir.hegel_property_receipts] Valence MUST model Hegel property-test runs as bounded generated-input evidence receipts with stable identity, runner settings, generator identity, status, diagnostics, and explicit non-claims.

#### Scenario: Receipt carries runner and property identity
r[valence.evidence_ir.hegel_property_receipts.schema]
- GIVEN a Hegel property run targets a Valence pure-core invariant
- WHEN Valence records the receipt
- THEN the receipt MUST include schema version, property ID, linked requirement IDs, checked module or function family, Hegel crate/backend identity, runner settings, case budget, phases, seed or derandomized mode, generator identity, status, issue codes, and non-claims.

#### Scenario: Passing receipt preserves bounded evidence
r[valence.evidence_ir.hegel_property_receipts.valid]
- GIVEN a Hegel property run passes for its configured generated cases and explicit examples
- WHEN Valence validates the receipt
- THEN validation MUST preserve the pass status as bounded generated-testing evidence without promoting it to formal proof.

#### Scenario: Broken receipt fails closed
r[valence.evidence_ir.hegel_property_receipts.fail_closed]
- GIVEN a Hegel receipt omits required identity fields, uses malformed BLAKE3 hashes, records ambiguous settings, or references a stale generator identity
- WHEN Valence validates the receipt
- THEN validation MUST fail closed with deterministic diagnostics naming the rejected field.

#### Scenario: Non-claim boundary is mandatory
r[valence.evidence_ir.hegel_property_receipts.non_claims]
- GIVEN a Hegel receipt omits the generated-testing non-claim boundary or claims exhaustive correctness, semantic equivalence, verifier soundness, requirement satisfaction, release eligibility, or formal proof
- WHEN Valence validates the receipt
- THEN validation MUST reject the receipt with a deterministic boundary or overclaim diagnostic.

### Requirement: Hegel property suites for pure cores
r[valence.evidence_ir.hegel_property_suites] Valence SHOULD provide Hegel-backed property suites for selected pure Evidence IR cores whose invariants benefit from generated-input coverage.

#### Scenario: Canonical identity laws are generated
r[valence.evidence_ir.hegel_property_suites.identity]
- GIVEN generated EvidenceArtifact, Function Object, or related canonical identity inputs that differ only in display metadata or supported field ordering
- WHEN Hegel property suites run
- THEN Valence SHOULD report identical canonical identities and BLAKE3 hashes for semantically equivalent canonical content.

#### Scenario: Graph and cache laws are generated
r[valence.evidence_ir.hegel_property_suites.graph]
- GIVEN generated evidence graph inputs, alias bindings, capability sets, and cache-key fields
- WHEN Hegel property suites run
- THEN Valence SHOULD preserve deterministic graph identity, deterministic query results, deterministic alias resolution, and cache hits only when every identity field matches.

#### Scenario: Generated negative inputs fail closed
r[valence.evidence_ir.hegel_property_suites.fail_closed]
- GIVEN generated duplicate identities, dangling graph edges, stale aliases, weakened non-claim boundaries, unsupported capabilities, or cache-key mismatches
- WHEN Hegel property suites run
- THEN Valence MUST return typed errors or deterministic diagnostics rather than panic, silently accept invalid input, or emit unstable output.

#### Scenario: Evaluation core stays pure
r[valence.evidence_ir.hegel_property_suites.pure_core]
- GIVEN a Hegel test needs to evaluate a Valence invariant
- WHEN the property invokes Valence logic
- THEN the evaluated core MUST operate on in-memory values and MUST NOT read files, inspect environment, spawn processes, access clocks, perform network I/O, or print.

### Requirement: Hegel suite lanes
r[valence.evidence_ir.hegel_suite_lanes] Valence MUST partition Hegel property coverage into explicit fast and deep lanes with named budgets, deterministic or replayable settings, and bounded claims.

#### Scenario: Fast lane is ordinary-test safe
r[valence.evidence_ir.hegel_suite_lanes.fast]
- GIVEN maintainers run ordinary local validation from the Nix devshell
- WHEN the fast Hegel lane runs
- THEN it MUST use small named case budgets, deterministic or replayable settings, and no live sibling repositories or network access.

#### Scenario: Deep lane is opt-in
r[valence.evidence_ir.hegel_suite_lanes.deep]
- GIVEN larger generated case budgets, stateful Hegel models, or shrink-heavy properties are useful
- WHEN maintainers run the deep lane
- THEN the lane MUST be opt-in and report pass, fail, skipped, or unavailable status without becoming a hidden prerequisite for starter validation.

#### Scenario: Lane docs preserve non-claims
r[valence.evidence_ir.hegel_suite_lanes.docs]
- GIVEN documentation or suite reports describe Hegel coverage
- WHEN the supported claim is stated
- THEN the output MUST say Hegel checks bounded generated inputs only and does not prove correctness, semantic equivalence, verifier soundness, requirement satisfaction, release eligibility, or whole-program safety.
