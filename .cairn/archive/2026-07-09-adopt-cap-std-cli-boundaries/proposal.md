## Why

Valence core is pure Evidence IR logic, but `valence-cli` reads manifests, sidecars, Octet exports, Mantle release metadata, built binaries, and output paths supplied by operators. Those shell paths should be bounded by explicit filesystem authority instead of ambient `std::fs` calls and path-string checks.

## What Changes

- Adopt `cap-std` in `valence-cli` for stack-provenance and evidence-file shell boundaries.
- Introduce typed capability roots for input artifact, manifest, sidecar, binary, and output directories.
- Keep `valence-core` free of filesystem and capability dependencies.
- Add positive and negative fixtures for valid in-root artifacts, `../`, absolute paths, symlink escapes, and missing roots.

## Impact

- Valence CLI local file authority becomes explicit and root-confined.
- Evidence IR validation and stack-provenance semantics remain pure and deterministic.
- Invalid path states deny before Valence reads unrelated host files or writes sidecars outside declared outputs.
