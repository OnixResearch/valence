## ADDED Requirements

### Requirement: Provenance link connects source to binary
r[valence.provenance] The system MUST provide a `ProvenanceLink` type connecting `source_id`, `function_object_id`, `requirement_ids`, `artifact_hash`, and `binary_hash` so the chain threads source to binary.

#### Scenario: Link carries all five identifiers
r[valence.provenance.link_fields]
- GIVEN a constructed ProvenanceLink
- WHEN its fields are inspected
- THEN it carries source_id, function_object_id, requirement_ids, artifact_hash, and binary_hash

### Requirement: Provenance chain resolves in three directions
r[valence.provenance.resolution] The system MUST provide `resolve` (source ID to link), `resolve_requirement` (requirement ID to function objects), and `trace_binary` (binary hash to source functions).

#### Scenario: Resolve source ID
r[valence.provenance.resolution.source]
- GIVEN a chain with a link carrying source_id "eq:bs_001"
- WHEN `resolve("eq:bs_001")` is called
- THEN the matching link is returned

#### Scenario: Resolve requirement to function objects
r[valence.provenance.resolution.requirement]
- GIVEN a chain with a link carrying requirement_id "r[cairn.pricing]"
- WHEN `resolve_requirement("r[cairn.pricing]")` is called
- THEN the matching link(s) are returned

#### Scenario: Trace binary to source functions
r[valence.provenance.resolution.binary]
- GIVEN a chain with links carrying binary_hash "b3:binary_001"
- WHEN `trace_binary("b3:binary_001")` is called
- THEN all matching links are returned

### Requirement: Provenance chain checker fails closed
r[valence.provenance.checker] The system MUST provide `check_provenance_chain` that fails closed on schema mismatch, weakened boundary, empty source_id, empty function_object_id, and duplicate source_ids.

#### Scenario: Check passes valid chain
r[valence.provenance.checker.valid]
- GIVEN a valid ProvenanceChain
- WHEN checked
- THEN the report is valid with zero issues

#### Scenario: Check fails closed for duplicate source_id
r[valence.provenance.checker.duplicate]
- GIVEN a chain with two links sharing the same source_id
- WHEN checked
- THEN the report is invalid with `duplicate_source_id`

### Requirement: Provenance non-claim boundary
r[valence.provenance.non_claim] The system MUST carry a non-claim boundary stating that provenance proves identity and linkage only, not behavioral correctness or that the binary satisfies the requirement.

#### Scenario: Boundary present in chain
r[valence.provenance.non_claim.present]
- GIVEN a ProvenanceChain
- WHEN inspected
- THEN it carries a boundary field stating "proves identity and linkage only"
