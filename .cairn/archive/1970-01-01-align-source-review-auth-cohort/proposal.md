# Change: Align the source-review authentication cohort and policy owner

## Why

The source-review verifier independently authenticates Cairn attachments, but Valence's prepared Cargo authentication dependencies and stale generated lifecycle policy do not yet describe one current receiver-owned cohort. Current Cairn parsing requires explicit workflow and traceability fields. A source profile that disagrees with its immutable workspace must remain blocked rather than weakening admission.

## What Changes

- Align Cargo and Nix with one reviewed immutable Artifact owner revision and the complete declared source workspace while keeping Valence's consumer graph limited to the two authentication packages.
- Regenerate repository-owned policy from the scoped Nickel inputs through the verified Cairn owner exporter and retain positive/negative fixture evidence.
- Specify independent source-review acceptance, exercise the real Valence CLI against retained external-signer test-only fixtures, and record bounded authentication results.
- Record root selection, native lifecycle gates, workspace behavior, strict Clippy, and formatting after final source edits.

## Impact

Affected surfaces are dependency identity admission, `source-review-acceptance`, lifecycle policy compatibility, and receiver-local evidence. Existing function-address, trust, archive destination, and traceability roots remain unchanged. Historical receipts stay historical rather than being rewritten as current evidence.

## Out of Scope

No real reviewer approval, signing-key operation, release authorization, source correctness claim, trust weakening, lifecycle-root migration, publication, or Mantle change is authorized by this change.
