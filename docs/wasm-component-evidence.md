# Wasm component evidence

The Wasm component profile keeps WIT/interface, portable component, plan,
transform, target-specific precompile, Mantle materialization bundle,
configuration, and producer-receipt roles distinct. Valence-owned identities
use exact-byte or canonical DTO BLAKE3. OCI/wkg SHA-256 is external metadata
only. Typed edges preserve stage and bundle-child linkage; matching WIT worlds
never imply matching implementations or semantic equivalence. Receipts default
to recorded-only, and target-specific artifacts require target/CPU context.

The pure Rust validator and Nickel static preflight check shape, linkage, roles,
and non-claims without invoking component tools or runtimes. Passing validation
does not prove behavior, package trust, sandbox effectiveness, authority,
compiler/runtime correctness, native portability, or release eligibility.
