## Phase 1: Currentness and persistence

- [x] [serial] I1 Implement pure COB-sequence generation and authorization-profile currentness derivation. r[valence.artifact_auth_operational_receipt.currentness]
- [x] [depends:valence.artifact_auth_operational_receipt.currentness] I2 Build and validate a deterministic RecordedOnly Evidence IR operational receipt. r[valence.artifact_auth_operational_receipt.identity]
- [x] [depends:valence.artifact_auth_operational_receipt.identity] I3 Persist through capability-root artifact I/O and signed Git COB attachment, then materialize and replay. r[valence.artifact_auth_operational_receipt.persistence]
- [x] [depends:valence.artifact_auth_operational_receipt.persistence] I4 Add positive and negative sequence, authorization removal/drift, key, carrier, attachment, malformed, and false-parity tests plus docs. r[valence.artifact_auth_operational_receipt.authority]

## Phase 2: Validation

- [x] [parallel] V1 Run focused Cargo, rustfmt, strict Clippy, Cairn, and purity gates. r[valence.artifact_auth_operational_receipt.persistence]
- [ ] [serial] V2 Run full workspace and Nix gates; sync and archive requirements. r[valence.artifact_auth_operational_receipt.authority]
