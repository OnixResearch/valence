let Contracts = import "../contracts.ncl" in
std.record.remove "workflow_profile_policy" (import "../default.ncl") | Contracts.Policy
