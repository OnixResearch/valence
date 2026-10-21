# Tasks

## 1. Coherent source admission

- [x] [serial] [task:source] Align Cargo/Nix and owning-tool locks with one immutable coherent Artifact source profile, exact source bytes, and the two-package consumer graph; retain the full-source validator and historical evidence. r[valence.artifact_auth_adoption.source] r[valence.artifact_auth_adoption.lock_agreement]

## 2. Repository policy compatibility

- [x] [serial] [task:policy] Regenerate policy through the supported owner exporter and record positive/negative fixtures, freshness, parser admission, root selection, and workflow state without weakening trust or changing traceability roots. r[valence.source_review.policy_compatibility]

## 3. Receiver behavior and shell proof

- [x] [serial] [task:behavior] [after:source] [after:policy] Run final workspace lib/tests, strict workspace Clippy, and appropriate package formatting after source edits. r[valence.source_review.independent_acceptance] r[valence.source_review.cli_boundary]
- [x] [serial] [task:smoke] [after:behavior] Exercise the actual Valence CLI using retained externally signed test-only approval and needs-revision fixtures; reject revoked, substituted-source, and author-excluded inputs and retain exact receipts and commands. r[valence.source_review.independent_acceptance] r[valence.source_review.negative_disposition]

## 4. Native lifecycle evidence

- [x] [serial] [task:lifecycle] [after:smoke] Retain bounded source/command evidence, run required proposal/design/tasks gates, sync only accepted requirements, and inspect archive readiness without claiming publication or release authority. r[valence.source_review.policy_compatibility] r[valence.source_review.cohort_evidence]
