# Tasks

## 1. Baseline and profile contract

- [x] [serial] 1.1 Run existing Evidence IR, canonical identity, role, sidecar, and stack-provenance tests before core changes. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 1.2 Define the versioned build-service profile and canonicalization contract. r[valence.evidence_ir.build_service_profile] r[valence.evidence_ir.build_service_identity]
- [x] [serial] 1.3 Define record and edge kinds for requests, attempts, transitions, authority, classes, decisions, observations, retries, usage, results, reuse, orchestration, and external status. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 1.4 Define required non-claims and privacy exclusions. r[valence.evidence_ir.build_service_privacy] r[valence.evidence_ir.build_service_non_claims]

## 2. Build the functional core

- [x] [serial] 2.1 Add Rust types and pure constructors for profile records and links. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 2.2 Add canonical ordering, explicit units, presence markers, integer widths, timestamp representation, and BLAKE3 domain separation. r[valence.evidence_ir.build_service_identity]
- [x] [serial] 2.3 Add pure path-based validation for kind, identity, version, role, required-link, privacy, and non-claim rules. r[valence.evidence_ir.build_service_roles] r[valence.evidence_ir.build_service_privacy]
- [x] [serial] 2.4 Add positive tests for each record kind and negative tests for malformed, duplicate, dangling, wrong-kind, noncanonical, unsupported-version, and overclaimed input. r[valence.evidence_ir.build_service_profile] r[valence.evidence_ir.build_service_identity]
- [x] [serial] 2.5 Add property tests for deterministic canonicalization and identity stability. r[valence.evidence_ir.build_service_identity]

## 3. Add resource, retry, and accounting linkage

- [x] [serial] 3.1 Link declared requirements, eligible sets, selected classes, policies, observations, and ordered reason codes. r[valence.evidence_ir.resource_retry_links]
- [x] [serial] 3.2 Require positive OOM evidence and increasing eligible class for OOM escalation edges. r[valence.evidence_ir.resource_retry_links]
- [x] [serial] 3.3 Link reservations and reconciliations by attempt and public unit schedule. r[valence.evidence_ir.usage_links]
- [x] [serial] 3.4 Add positive selection, OOM, and accounting fixtures plus negative missing-evidence, class, unit, duplicate-charge, and overflow fixtures. r[valence.evidence_ir.resource_retry_links] r[valence.evidence_ir.usage_links]

## 4. Add result reuse and external status linkage

- [x] [serial] 4.1 Add producer-to-consumer reuse edges with separate sharing, authorization, identity, platform, output, signature, and CAS observations. r[valence.evidence_ir.result_reuse_edges]
- [x] [serial] 4.2 Reject shared-object-only authority and producer-consumer identity confusion. r[valence.evidence_ir.result_reuse_edges]
- [x] [serial] 4.3 Link Lattice orchestration jobs and GitHub Check Runs without promoting external status. r[valence.evidence_ir.external_status_links]
- [x] [serial] 4.4 Add positive same-project, cross-project, and Check Run fixtures plus negative scope, signature, platform, CAS, dangling-status, and release-claim fixtures. r[valence.evidence_ir.result_reuse_edges] r[valence.evidence_ir.external_status_links]

## 5. Add privacy and language surfaces

- [x] [serial] 5.1 Reject raw OIDC, UCAN, ticket, private-key, provider-credential, replay-key, raw-log, and value-derived-hash fields from canonical artifacts. r[valence.evidence_ir.build_service_privacy]
- [x] [serial] 5.2 Add a typed Nickel contract equivalent to the Rust validation surface. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 5.3 Add consumer examples for Mantle, Lattice, Basalt, OnixOS, and Cairn. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 5.4 Document verification roles and claim boundaries field by field. r[valence.evidence_ir.build_service_roles] r[valence.evidence_ir.build_service_non_claims]

## 6. Validate

- [x] [serial] 6.1 Run `cargo fmt --check`. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 6.2 Run focused positive, negative, and property tests. r[valence.evidence_ir.build_service_identity]
- [x] [serial] 6.3 Run `cargo test --workspace`. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 6.4 Run `cargo clippy --workspace --all-targets -- -D warnings`. r[valence.evidence_ir.build_service_roles]
- [x] [serial] 6.5 Run the stack-provenance Nickel contract check. r[valence.evidence_ir.build_service_profile]
- [x] [serial] 6.6 Run Cairn validation, requirement coverage, design gate, and tasks gate. r[valence.evidence_ir.build_service_non_claims]
