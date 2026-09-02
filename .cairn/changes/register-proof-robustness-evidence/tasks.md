# Tasks

## Phase 1: Producer contract and roles

- [ ] [serial] Pin the accepted Octet proof-robustness schema, producer revision, fixture, and BLAKE3 identities. r[valence.proof_robustness_evidence.schema]
- [ ] [serial] Register profile, proof parent, child run, observation, audit, proof-use, classification, snapshot, and receipt layers. r[valence.proof_robustness_evidence.layers]
- [ ] [serial] Define the closed mapping from robustness layers to `RecordedOnly` and `Boundary` while preserving independent parent `Property` evidence. r[valence.proof_robustness_evidence.roles]

## Phase 2: Core and shell

- [ ] [serial] Implement pure validation for schema identity, layer links, parent proof identity, child completeness, terminal snapshot freshness, bounds, roles, and non-claims. r[valence.proof_robustness_evidence.validation]
- [ ] [serial] Add capability-relative CLI intake that remeasures producer artifacts before core validation. r[valence.proof_robustness_evidence.shell]
- [ ] [serial] Add stack-provenance projection without changing the producer classification or existing proof role. r[valence.proof_robustness_evidence.roles] r[valence.proof_robustness_evidence.boundary]

## Phase 3: Positive and negative evidence

- [ ] [parallel] Add complete stable and unstable producer fixtures with correct recorded and boundary roles. r[valence.proof_robustness_evidence.fixtures]
- [ ] [parallel] Add stale-parent, swapped-child, wrong-role, missing-boundary, live-state-promotion, unsupported-coverage-promotion, excessive, and overclaim fixtures. r[valence.proof_robustness_evidence.fixtures]

## Phase 4: Documentation and closeout

- [ ] [serial] Document producer ownership, layer identities, role mapping, terminal-snapshot rule, and non-claims. r[valence.proof_robustness_evidence.boundary]
- [ ] [serial] Run focused core tests, CLI tests, core-purity smoke, schema freshness, formatting, Clippy with warnings denied, lifecycle validation, Cairn gates, and relevant Nix checks. r[valence.proof_robustness_evidence.fixtures]
- [ ] [serial] Run proposal, design, and tasks gates before sync and archive. r[valence.proof_robustness_evidence.schema]
