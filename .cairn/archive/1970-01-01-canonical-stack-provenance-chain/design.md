# Design: Canonical stack provenance chain

## Functional core / imperative shell boundary

`valence-core` owns pure deterministic canonicalization, edge validation,
non-claim policy checks, graph analysis, and report DTO construction. It accepts
in-memory artifacts and returns reports. `valence-cli` owns file reads, binary
measurement, stdout/stderr, and writing JSON or Markdown reports.

## Canonical identity

Every stack provenance node uses a stable ID derived from deterministic content,
not path or wall-clock metadata. Canonical identity inputs include schema,
artifact kind, producer role, verification role, relevant span or requirement
IDs, parent artifact IDs, and canonical payload bytes. BLAKE3 is the default
content hash for Valence-owned identities.

## Typed edge vocabulary

The chain validator recognizes a policy-owned edge vocabulary:

- `proves_local_property`;
- `checks_source_shape`;
- `exports_provenance`;
- `bundled_into_release`;
- `satisfies_gate`;
- `manual_review_input`.

Each edge records producer role, consumer role, source artifact, target artifact,
claim scope, and required non-claims. Unsupported edge kinds fail closed.

## Non-claim policy

Valence treats non-claims as enforceable policy, not prose-only guidance. A
chain fails when an artifact or edge implies a stronger claim than its producer
role and verification role support. Examples include imported Trellis evidence
promoted to downstream formal proof, Mantle external sidecar metadata presented
as semantic verification, and Cairn gate receipts presented as source-code
correctness proof.

## Graph report

The CLI report has two projections over the same pure DTO:

- JSON: deterministic machine report with artifacts, edges, trust boundaries,
  manual-review nodes, unresolved links, diagnostics, and report hash.
- Markdown: compact operator summary with the same counts and bounded claims.

Markdown rendering remains a shell concern over core report data.

## Fixture strategy

Positive fixtures cover a complete Octet export to Valence sidecar to Mantle
bundle to Cairn gate chain. Negative fixtures cover missing parent, digest
mismatch, stale policy, unsupported edge kind, manual-review tier promotion,
weakened non-claims, and span/source content drift.

## What is explicitly not provided

- No source-code behavioral correctness proof.
- No release eligibility decision; Cairn owns lifecycle gates.
- No Mantle semantic interpretation; Mantle treats the sidecar as opaque
  external evidence.
- No verifier soundness proof.
