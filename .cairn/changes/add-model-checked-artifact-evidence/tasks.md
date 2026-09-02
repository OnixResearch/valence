## Tasks

- [ ] [serial] Define the bounded evidence role vocabulary and spans for spec, invariant, checker, bounds, verdict, and witness-trace artifacts. r[valence.model_check_evidence.roles]
- [ ] [serial] Author the versioned Nickel contract for the BLAKE3-bound model-checked artifact sidecar. r[valence.model_check_evidence.sidecar]
- [ ] [serial] Implement pure validation of identity linkage, verdict classes, witness-trace binding, and non-claims, failing closed on any mismatch. r[valence.model_check_evidence.sidecar]
- [ ] [parallel] Enforce the closed verdict classes and require witness-trace identity for the violation class. r[valence.model_check_evidence.verdict]
- [ ] [parallel] Add positive fixtures for valid sidecars and negative fixtures for stale checker identity, tampered traces, wrong roles, invalid verdict classes, and missing non-claims. r[valence.model_check_evidence.fixtures]
- [ ] [serial] Run focused core tests, clippy with warnings denied, lifecycle validation, Cairn gates, and relevant flake checks. r[valence.model_check_evidence.validation]
