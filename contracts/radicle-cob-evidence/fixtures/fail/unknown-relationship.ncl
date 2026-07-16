let Contracts = import "../../contracts.ncl" in
let Profile = import "../../profile.ncl" in
{
  schema = Profile.schema,
  cob_type = Profile.cob_type,
  root_schema = Profile.root_schema,
  event_schema = Profile.event_schema,
  binding_schema = Profile.binding_schema,
  snapshot_schema = Profile.snapshot_schema,
  authorization_schema = Profile.authorization_schema,
  relationships = ["context_for", "unbounded_claim"],
  actor_roles = Profile.actor_roles,
  embed_policy = Profile.embed_policy,
  required_non_claims = Profile.required_non_claims,
} | Contracts.ProfileShape | Contracts.ProfileContract
