# Review: policy-check provenance admission

## Scope and findings

- The typed parser recognizes only the exact new provenance spelling `policy_check`; unknown source strings are still preserved and rejected.
- `as_str` preserves the producer spelling. Existing frozen Preserves corpus checks passed, so this does not silently re-identify historical supported metadata.
- The shared evidence-tier enum does not widen proof-card acceptance: the explicit proof-card guard rejects `PolicyCheck` with the existing unsupported-tier diagnostic.
- Formal-proof and manual-review restrictions, artifact/link/binary validation, required non-claims, capability-rooted CLI I/O and pure-core boundaries remain unchanged.
- The public enum gains a variant. Full workspace compilation and Clippy passed; downstream pinned consumers must recompile. No JSON schema/field/tier aliases or compatibility adapter were added.
- The actual official Octet exports, not manually authored acceptance envelopes, produced valid CLI sidecars for both actual component identities.

## Verification reviewed

23 initial focused cases and the final restored-lock cohort's 369 workspace cases, workspace Clippy, changed-source pinned formatting, full Nix flake checks and both actual CLI builds passed. The full formatter retains one pre-existing unchanged pilot wrapping failure; it is not reported as clean, and neither that pilot nor Cargo.lock is included. Exact commands, attempts, results and artifact identities are recorded in [verification evidence](evidence/2026-10-28-verification.md).

## Disposition

The bounded compatibility fix is ready for native requirement synchronization and archive checks. Publication does not establish downstream Molten runtime completion; the consumer must pin and separately operate the published implementation. No blocking code finding remains in this scope, and no unrelated operator/workspace changes were included.
