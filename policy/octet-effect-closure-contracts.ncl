let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in
let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in
let Blake3Identity = NonEmptyString in
let OneOf = fun allowed => std.contract.from_predicate (fun value =>
  std.is_string value && std.array.elem value allowed
) in
let WorkspaceSchema = OneOf ["octet-effect-closure-workspace-policy/v1"] in
let EffectContract = OneOf ["alloc", "panic", "unsafe", "clock", "random", "fs", "net", "spawn", "lock", "ffi", "unknown"] in
let RoleContract = OneOf ["core", "shell", "adapter", "ffi", "proof"] in
let ProviderKindContract = OneOf ["cargo-package", "toolchain", "workspace"] in
let ModuleRoleContract = {
  path | NonEmptyString,
  role | RoleContract,
} in
let ProviderIdentityContract = {
  kind | ProviderKindContract,
  name | NonEmptyString,
  version | NonEmptyString,
  source | NonEmptyString,
  target | NonEmptyString,
  features | Array NonEmptyString,
} in
let EvidenceReferenceContract = {
  kind | NonEmptyString,
  locator | NonEmptyString,
  blake3 | Blake3Identity,
} in
let CallSummaryContract = {
  call_site_id | Blake3Identity,
  review_label | NonEmptyString,
  callee | NonEmptyString,
  provider | ProviderIdentityContract,
  effects | Array EffectContract,
  evidence | EvidenceReferenceContract,
} in
{
  WorkspacePolicy = {
    schema_version | WorkspaceSchema,
    profile_id | OneOf ["deterministic-core"],
    function_address_baseline | NonEmptyString,
    module_roles | NonEmptyArray | Array ModuleRoleContract,
    call_summaries | Array CallSummaryContract,
    non_claims | NonEmptyArray | Array NonEmptyString,
  },
}
