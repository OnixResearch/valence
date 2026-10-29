## Context

The official `octet-valence-provenance-export/v1` export of the actual neutral effects component (`db83d1d091b1dc4dfe92a2dd0a9c9b300fcef8a94a6fdb39d6855f374b49b657`) contains `policy_check` records. Official artifact verification succeeds, while the current Valence adapter denies sidecar construction with `unknown_evidence_tier`. Neither output is to be relabeled or edited.

## Decisions

### Decision: Recognize the producer tier at its owning boundary

**Choice:** Add a typed `PolicyCheck` variant preserving the existing `policy_check` wire string. Admit it for provenance records only; retain the pre-existing formal-proof/manual-review proof-card vocabulary and unknown-tier denial.

**Rationale:** A source's evidence tier and Valence's verification role are different facts. Recognition makes the existing static producer vocabulary interoperable without turning recorded identity/linkage into verified behavior. No new generic registry or alternative adapter is needed. The Nickel preflight remains a static metadata-shape/non-promotion check; Rust continues to own typed tier admission.

### Decision: Exercise the unchanged actual export

**Choice:** Keep deterministic positive/negative tests adjacent to the existing tier tests and execute the real CLI against the official Octet export. Retain the original failed handoff and exact actual command/result in change evidence.

**Rationale:** A modeled accepted string alone does not establish actual producer/consumer compatibility.

## Risks / Trade-offs

- Adding a public enum variant requires checking all Rust matches; existing JSON field names and strings remain unchanged.
- A known provenance tier must not accidentally become an accepted proof-card tier. A negative regression preserves this boundary.
- Recognition is not proof promotion. Preserve recorded-only projections, required non-claims, existing manual-review restrictions, and unrelated frozen corpus identities.
- Dependency publication and downstream pinning occur only after repository checks; Molten's runtime execution is separate evidence.
