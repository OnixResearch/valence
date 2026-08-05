let Contracts = import "../../contracts.ncl" in
let Profile = import "../../profile.ncl" in
let C = Contracts.constants in
{
  schema = Profile.schema,
  source_schema = Profile.source_schema,
  plan_schema = Profile.plan_schema,
  binding_schema = Profile.binding_schema,
  admission_schema = Profile.admission_schema,
  source_kinds = Profile.source_kinds,
  issue_states = Profile.issue_states,
  pull_request_states = Profile.pull_request_states,
  review_states = Profile.review_states,
  review_handling = Profile.review_handling,
  cob_types = Profile.cob_types,
  bounds = Profile.bounds,
  required_non_claims = [C.MigrationNonClaim, C.ReviewNonClaim],
} | Contracts.ProfileShape | Contracts.ProfileContract
