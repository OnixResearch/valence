## Why

`valence function-address validate` currently derives graph rows and a BLAKE3 graph root directly from raw input before acting on the validation result. It then writes requested receipt and graph files before returning the failing status. Reproduced role-promotion and overclaim input exits unsuccessfully but still creates a standalone graph array and a receipt containing that graph root. The graph file carries no validity marker, so a consumer can mistake output from rejected evidence for publication-ready linkage.

The two files are also written sequentially with direct truncating writes. A second-write failure can leave a newly replaced receipt paired with an old or missing graph. Graph identity is computed over a bare row array whose ordering does not include `edge_kind` in the complete sort key and whose root is not bound to the policy identity or validated artifact.

## What Changes

- Introduce a pure publication planner that produces artifacts only from a passing function-address validation report.
- Suppress all receipt, graph, and graph-root publication on invalid input while retaining diagnostics on stdout.
- Replace the bare graph array with a versioned canonical graph report bound to policy identity, validated artifact identity, rows, counts, validity, and non-claims.
- Canonicalize rows by the complete typed edge key and reject ambiguous duplicate edges and empty locators.
- Add an atomic single-artifact publication bundle and make legacy split outputs explicit compatibility surfaces with shared publication identity and honest non-atomicity diagnostics.
- Add failure-injection and overwrite-preservation tests for invalid input and output errors.

## Impact

- **CLI behavior**: invalid validation never creates or replaces requested evidence outputs.
- **Consumers**: Mantle and Cairn receive a versioned valid graph report or canonical bundle rather than an unqualified row array.
- **Compatibility**: split receipt/graph files remain available only as explicit compatibility output and cannot claim cross-file transactional publication.
- **Claims**: successful publication proves validated identity/linkage output coherence only, not Rust semantics, correctness, proof status, or release eligibility.
