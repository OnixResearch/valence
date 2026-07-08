## ADDED Requirements

### Requirement: Kani stub facets
r[valence.kani_stub_roles.stub_facets] Valence MUST represent Kani verified contract stubs, plain stubs, and environmental stubs as distinct evidence facets.

#### Scenario: Stub kind is preserved
r[valence.kani_stub_roles.stub_facets.kind]
- GIVEN a Kani artifact declares stub use
- WHEN Valence validates the artifact
- THEN the stub kind MUST be preserved as typed payload data.

#### Scenario: Unknown stub kind fails closed
r[valence.kani_stub_roles.stub_facets.unknown]
- GIVEN a Kani artifact declares an unsupported stub kind
- WHEN Valence validates the artifact
- THEN validation MUST fail with a deterministic unsupported-stub diagnostic.

### Requirement: Verified-stub linkage
r[valence.kani_stub_roles.verified_linkage] Valence MUST accept Kani `stub_verified` property evidence only when the artifact links to a passing contract receipt for the original function and exact source scope.

#### Scenario: Linked verified stub passes
r[valence.kani_stub_roles.verified_linkage.valid]
- GIVEN a caller artifact uses `stub_verified` and links to a passing contract artifact for the original function
- WHEN Valence validates the caller artifact
- THEN the verified-stub facet MAY support scoped property evidence for the caller.

#### Scenario: Missing verified-stub link fails
r[valence.kani_stub_roles.verified_linkage.missing]
- GIVEN a caller artifact claims `stub_verified` but omits the linked contract receipt
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

### Requirement: Stub boundary classification
r[valence.kani_stub_roles.boundary_classification] Valence MUST classify plain Kani stubs and environmental stubs as boundary or recorded-only evidence with explicit assumptions.

#### Scenario: Environmental stub records assumption
r[valence.kani_stub_roles.boundary_classification.environment]
- GIVEN a Kani harness stubs a clock, randomness source, syscall, or FFI boundary
- WHEN Valence validates the artifact
- THEN the artifact MUST record the environmental assumption and MUST NOT treat it as implemented behavior.

#### Scenario: Plain stub remains conditional
r[valence.kani_stub_roles.boundary_classification.plain]
- GIVEN a Kani harness replaces a function with a handwritten plain stub
- WHEN Valence validates the artifact
- THEN the stub MUST be classified as conditional model evidence unless a verified contract link exists.

### Requirement: Kani stub positive fixtures
r[valence.kani_stub_roles.positive_fixtures] Valence MUST include positive fixtures for verified-stub linkage and environmental-stub boundary artifacts.

#### Scenario: Valid stub fixtures pass
r[valence.kani_stub_roles.positive_fixtures.valid]
- GIVEN valid verified-stub and environmental-stub fixtures
- WHEN the fixture suite runs
- THEN validation MUST produce deterministic accepted reports with distinct roles.

### Requirement: Kani stub negative fixtures
r[valence.kani_stub_roles.negative_fixtures] Valence MUST include negative fixtures for missing contract links, stale contract hashes, signature mismatches, plain-stub promotion, and missing assumptions.

#### Scenario: Plain-stub promotion fails
r[valence.kani_stub_roles.negative_fixtures.promotion]
- GIVEN a plain stub artifact is marked as property evidence without a verified contract
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

#### Scenario: Missing assumption fails
r[valence.kani_stub_roles.negative_fixtures.assumption]
- GIVEN an environmental stub artifact omits its assumption boundary
- WHEN Valence validates the artifact
- THEN validation MUST fail closed.

### Requirement: No unverified-stub promotion
r[valence.kani_stub_roles.no_unverified_promotion] Valence MUST reject any attempt to promote unverified Kani stubs into property evidence.

#### Scenario: Promotion rejected in stack provenance
r[valence.kani_stub_roles.no_unverified_promotion.stack]
- GIVEN stack provenance input includes Kani plain-stub evidence marked as a property
- WHEN Valence verifies the sidecar
- THEN verification MUST reject the tier promotion.

### Requirement: Kani stub boundary documentation
r[valence.kani_stub_roles.boundary_docs] Valence documentation MUST explain that verified stubs require linked passing contracts and plain or environmental stubs are conditional assumptions.

#### Scenario: Stub boundary is documented
r[valence.kani_stub_roles.boundary_docs.visible]
- GIVEN an operator reads Valence Kani evidence documentation
- WHEN stub handling is described
- THEN the documentation MUST distinguish verified contract stubs from plain and environmental stubs.
