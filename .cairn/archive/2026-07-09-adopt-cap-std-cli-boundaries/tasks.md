## Tasks

- [x] [serial] Add `cap-std` only to `valence-cli` shell/adaptor code that owns filesystem effects. r[valence.stack_provenance_adapter.cap_std_cli_boundary.dependency]
- [x] [serial] Introduce typed capability roots for input artifact, manifest, sidecar, binary, and output directories. r[valence.stack_provenance_adapter.cap_std_cli_boundary.root_wrappers]
- [x] [serial] Convert stack-provenance and evidence-file CLI opens to capability-relative operations while keeping `valence-core` pure. r[valence.stack_provenance_adapter.cap_std_cli_boundary.conversion]
- [x] [serial] Add positive fixtures for valid in-root artifacts, binary measurement, sidecar reads, and output writes. r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.positive]
- [x] [serial] Add negative fixtures for traversal, absolute paths, symlink escapes, missing roots, and output escape attempts. r[valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
- [x] [serial] Document the CLI filesystem-authority boundary and evidence/provenance non-claims. r[valence.stack_provenance_adapter.cap_std_cli_boundary.docs]
- [x] [serial] Run focused CLI tests, core purity scan, Cairn validation, and gates. r[valence.stack_provenance_adapter.cap_std_cli_boundary.validation]
