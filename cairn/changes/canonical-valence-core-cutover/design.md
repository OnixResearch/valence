## Context

`valence/crates/valence-core` already includes Evidence IR, provenance, stack-provenance, harness, and Hegel-property modules that are absent from the older Octet-hosted seam. Keeping both surfaces without an explicit owner contract weakens cross-repo coupling: consumers can bind to Octet for Valence semantics, while Valence cannot fail drift before release.

## Design

### Canonical module manifest

Add a Valence-owned manifest for public modules, schema strings, fixture roots, and compatibility state. The manifest is Nickel-authored for review and exported to deterministic runtime input when Rust checks need it.

Each row names:

- module or schema family;
- canonical Valence path;
- any legacy Octet path;
- compatibility disposition: `canonical`, `legacy-compatible`, `migration-exception`, or `retired`;
- required non-claim text;
- positive and negative fixture references.

### Drift gate

A pure core check compares manifest rows and supplied file/module digests. The CLI shell loads the manifest and file bytes, computes BLAKE3 digests, then delegates to the pure checker. Drift is allowed only when the row has a bounded migration exception with an owner and expiry condition.

### Consumer guidance

Docs and examples should point consumers at `github:OnixResearch/valence` or a local Valence path for `valence-core`. Octet docs should describe Octet's role as producer/orchestrator and Valence's role as semantic owner.

## Alternatives

### Leave Octet-hosted `valence-core` as an indefinite fork

Rejected. It keeps integration convenient but makes provenance semantics dependent on whichever crate a consumer happened to select.

### Move all Octet evidence code immediately

Rejected for this change. The cutover should first lock ownership, compatibility, and drift diagnostics; implementation migration can proceed in bounded slices.

## Risks

- **Consumer breakage**: mitigate with a compatibility manifest and explicit migration exceptions.
- **False equivalence claims**: require non-claims that this proves ownership and byte/schema compatibility only.
- **Octet release churn**: make Octet consume Valence as an adapter boundary before retiring its local seam.
