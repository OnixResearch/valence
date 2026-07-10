## Why

Valence owns canonical Evidence IR, spans, verification roles, non-claim boundaries, and stack evidence linkage. The tMIR repository is a useful reference for proof-carrying IR lineage, typed validation, semantic diffing, dialect round-tripping, and proof-sidecar manifests. Valence should adapt those concepts as an evidence profile without becoming a compiler IR.

Reference source: `https://github.com/joweeba/tMIR`.

## What Changes

- Add a proof-carrying IR lineage Evidence IR profile for recorded IR nodes, proof-obligation references, transform steps, dialect identity, and proof-certificate sidecars.
- Preserve Valence's claim boundary: identity/linkage and role validation only, not IR semantics or codegen correctness.
- Add positive and negative fixtures for valid lineage, unknown dialect preservation, stale proof sidecar, role promotion, semantic-equivalence overclaim, and cross-artifact substitution.
- Document how Kamacite or Octet producers may export this profile without making Valence a tMIR consumer.

## Impact

- **Surfaces**: `valence-core` Evidence IR adapters, formal proof chain docs, stack evidence fixtures.
- **Non-claims**: No tMIR dependency, no verification of tMIR semantics, no compiler correctness or machine-code proof.
- **Validation**: pure adapter tests, fixture corpus, core purity smoke, Valence tests, Cairn gates/validate.