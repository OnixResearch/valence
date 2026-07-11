## Why

Valence describes its `cap-std` shell as using typed capability roots, but the current path splitter derives a fresh ambient root from every operator-supplied path's parent and validates only the final file name. An absolute path therefore grants authority to its own parent instead of being rejected. This contradicts the accepted capability-boundary requirement and means the apparent root is not an independently declared authority boundary.

The flaw is reproducible with the current CLI: an absolute `/tmp/...` artifact path is opened and validated successfully. Path canonicalization after deriving that parent cannot repair the authority escalation because the untrusted path already selected the root.

## What Changes

- Require each CLI filesystem role to receive an explicit capability root before accepting a file locator.
- Interpret artifact, manifest, sidecar, binary, and output locators strictly relative to their declared role root.
- Remove parent-derived ambient roots and keep capability planning in a pure core with opening and I/O in the CLI shell.
- Add positive fixtures for explicit in-root access and negative fixtures for omitted roots, absolute locators, traversal, wrong-role handles, and symlink escapes.
- Document the intentional compatibility break for ambient absolute paths and the bounded filesystem-authority claim.

## Impact

- **CLI behavior**: file-taking commands require explicit role-root authority and reject formerly accepted ambient locators.
- **Architecture**: `valence-core` remains free of filesystem access; the CLI owns `cap-std` handles and I/O.
- **Consumers**: scripts invoking `valence check`, `explain`, `stack-provenance`, or `function-address` must supply declared roots and relative locators.
- **Claims**: passing root confinement proves only that the selected local operation stayed beneath the supplied capability handle; it does not prove evidence truth, sandbox completeness, behavioral correctness, or release eligibility.
