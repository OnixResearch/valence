## Why

The smoke path needed custom code to construct `FunctionAddressEvidenceInput`, validate it, emit a Valence sidecar, and render graph rows. Valence owns the Evidence IR profile, so that validation/projection should be a first-class CLI surface.

## What Changes

- Add a Valence CLI command for function-address Evidence IR validation.
- Accept Kamacite projection and Octet/Kamacite receipt identities as inputs without parsing Rust.
- Emit a Valence receipt/sidecar and graph summary suitable for Mantle and Cairn.
- Add positive and negative fixtures for roles, digest domains, non-claims, graph rows, and receipt linkage.

## Impact

Downstream Mantle and Cairn can consume Valence-owned function-address evidence artifacts instead of bespoke adapter output.
