## Design

Valence treats the function-address Evidence IR profile as a versioned policy contract. The policy identity is the BLAKE3 digest of deterministic profile material: profile name, version, accepted digest domains, accepted roles, required non-claims, graph row contract, and validation boundary text. CLI and library outputs expose that identity.

### Decisions

1. **Profile material is canonical.** The hash is derived from deterministic policy/profile bytes, not from ad hoc fixture JSON.
2. **BLAKE3 is the default.** The profile identity uses lowercase BLAKE3 hex with explicit domain metadata.
3. **Receipts carry policy identity.** Valence function-address receipts include the policy/profile identity they were validated against.
4. **Stale identity fails closed.** Mantle/Cairn-facing projections fail validation if the embedded policy identity does not match the current profile version.
5. **Version bumps are explicit.** Changing the validation contract updates the profile version or policy hash and corresponding fixtures.

### Validation shape

Positive fixtures assert the current profile policy identity matches generated outputs. Negative fixtures cover placeholder hashes, stale hashes, missing identity, malformed BLAKE3, and policy identity/domain substitution.
