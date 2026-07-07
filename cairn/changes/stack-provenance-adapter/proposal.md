## Why

Mantle should bundle stack provenance as optional external evidence, not own its semantics. Valence is the right home for the stack-specific adapter because it already owns the shared evidence identity model and provenance chain boundary.

This change adds a Valence stack-provenance adapter that can build and verify a cross-repo traceability sidecar from Octet artifacts, Trellis proof-evidence bundles, Cairn receipts, and Mantle release evidence. Mantle remains stack-neutral: it only stores the sidecar bytes and digest, while Valence verifies what those bytes mean.

## What Changes

- Add a stack-provenance adapter surface to Valence.
- Build a canonical Valence provenance sidecar from adapter inputs: Octet provenance/proof artifacts, Trellis proof-evidence bundles, Cairn receipts, and Mantle release manifests.
- Verify a bundled sidecar against a Mantle release evidence directory by matching the sidecar digest, binary hash, source/function/requirement IDs, evidence tiers, and non-claim boundary.
- Keep all cross-repo file IO in the CLI shell; the core verifier operates over parsed in-memory inputs.
- Fail closed for stale hashes, missing parent artifacts, missing requirements, binary mismatches, weakened boundaries, and evidence-tier promotion.

## Impact

- **Valence owns stack semantics**: stack traceability is verified outside Mantle core.
- **Mantle integration is optional**: a Mantle release can include the sidecar as generic external evidence but does not need Valence to verify ordinary release integrity.
- **Octet/Trellis/Cairn inputs remain bounded**: imported Trellis evidence stays manual-review until rerun under Octet-owned proof rails.
- **Non-claims**: the adapter proves identity and linkage only, not behavioral correctness, semantic equivalence, whole-program safety, or that the binary satisfies a requirement.
