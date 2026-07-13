## Phase 1: Validated publication core

- [x] [serial] Define a pure publication result that yields diagnostics or publishable receipt/graph material, never both. r[valence.evidence_ir.function_address_publication.validity_gate]
- [x] [serial] Generate graph rows and roots only from the successful validation branch. r[valence.evidence_ir.function_address_publication.validity_gate]
- [x] [serial] Validate non-empty locators and duplicate edges, then canonicalize rows by the complete typed edge key. r[valence.evidence_ir.function_address_publication.graph_validation]
- [x] [serial] Define a versioned graph report bound to policy identity, validated artifact identity, counts, rows, validity, non-claims, and BLAKE3 self-hash-excluded identity. r[valence.evidence_ir.function_address_publication.canonical_graph]

## Phase 2: Shell publication

- [x] [serial] Prevent invalid inputs from creating, truncating, or replacing receipt, graph, bundle, or graph-root outputs. r[valence.evidence_ir.function_address_publication.invalid_no_write]
- [x] [serial] Add atomic same-directory publication for the canonical combined bundle through a declared output capability. r[valence.evidence_ir.function_address_publication.atomic_bundle]
- [x] [serial] Stage and individually atomically replace explicit compatibility outputs while binding both to one publication identity and avoiding cross-file atomicity claims. r[valence.evidence_ir.function_address_publication.compatibility_outputs]
- [x] [parallel] Add positive tests for stable order-independent graph identity, valid bundle replacement, and matching compatibility publication identities. r[valence.evidence_ir.function_address_publication.fixtures.positive]
- [x] [parallel] Add negative and failure-injection tests for invalid-input no-write, preserved existing files, duplicate/empty graph fields, destination failures, and interrupted compatibility staging. r[valence.evidence_ir.function_address_publication.fixtures.negative]

## Phase 3: Consumer migration and evidence

- [x] [serial] Document canonical bundle use, versioned graph reports, diagnostic-only failures, legacy split-output limits, and bounded claims for Mantle/Cairn consumers. r[valence.evidence_ir.function_address_publication.docs]
- [x] [serial] Run focused core/CLI/failure-injection tests, workspace tests, and Cairn validation/gates. r[valence.evidence_ir.function_address_publication.final_validation]
