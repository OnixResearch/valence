## Why

Molten's genuine Mantle-produced effects component passes the published Octet checker and its official provenance export is valid. The actual Valence stack-sidecar adapter rejects that unchanged export with `unknown_evidence_tier` because its typed vocabulary omits Octet's `policy_check` provenance tier.

## What Changes

- Recognize `policy_check` for provenance records without rewriting producer evidence.
- Preserve the exact tier and existing identity/linkage-only, recorded-only evidence boundary.
- Keep unknown tiers denied and keep policy checks outside the accepted proof-card tier vocabulary.
- Prove the handoff with the actual official Octet export plus positive and negative regression coverage.

## Impact

- **Files**: `crates/valence-core/src/stack_provenance_adapter.rs`, stack-provenance documentation and native Cairn requirements/evidence.
- **Consumer**: Molten hosted-effects actor and installed-extension adoption, consuming genuine Mantle component bytes.
- **Owner**: Valence maintains tier parsing and sidecar linkage; Octet still owns static checking; Molten owns runtime/authority admission.
- **Testing**: focused and workspace Rust tests, Clippy, formatting, actual CLI sidecar build over the unchanged Octet export, relevant Nickel/Nix checks and strict Cairn validation.
- **Non-claims**: no static evidence promotion, runtime correctness, authority, requirement satisfaction or release eligibility.
