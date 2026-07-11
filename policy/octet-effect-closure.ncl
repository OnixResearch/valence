let contracts = import "octet-effect-closure-contracts.ncl" in
let reviewed_call_summaries = import "octet-effect-closure-call-summaries.ncl" in
{
  schema_version = "octet-effect-closure-workspace-policy/v1",
  profile_id = "deterministic-core",
  function_address_baseline = "evidence/octet/deterministic-core/baseline/function-addresses.json",
  module_roles = [
    {
      path = "crates/valence-core/src/shared_identity.rs",
      role = "core",
    },
    {
      path = "crates/valence-core/src/shared_identity/canonical_hash_profile.rs",
      role = "core",
    },
  ],
  call_summaries = reviewed_call_summaries,
  non_claims = [
    "This policy supports conservative source-token closure over two selected Valence production files only; broader valence-core remains unadmitted.",
    "Reviewed call summaries do not verify dependency implementations, complete Rust name resolution, MIR closure, runtime reachability, semantic determinism, whole-program correctness, or release eligibility.",
  ],
} | contracts.WorkspacePolicy
