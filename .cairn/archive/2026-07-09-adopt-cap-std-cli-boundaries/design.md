## Design

`valence-cli` will open ambient input and output directories at command entrypoints, convert them into typed wrappers around `cap_std::fs::Dir`, and pass loaded bytes or validated relative locators into `valence-core`.

The first conversion targets are stack-provenance commands that read Octet exports, Mantle manifests, stack sidecars, built binaries, and write sidecar or report outputs. Generic `check` and `explain` file reads can use the same pattern when they operate under operator-declared roots.

`valence-core` remains the authority for Evidence IR, provenance, role, hash, and non-claim validation over in-memory values. It must not depend on `cap-std`, read files, inspect environment state, or render CLI diagnostics.

Tests should cover valid in-root input/output artifacts, traversal, absolute paths, symlink escapes, missing roots, and binary measurement through a declared root. Documentation should state that capability roots bound local CLI filesystem authority only; they do not prove evidence truth, behavioral correctness, release eligibility, or verifier soundness.
