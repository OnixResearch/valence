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
  relationships = Profile.relationships,
  actor_roles = Profile.actor_roles,
  embed_policy = {
    max_bytes = 0,
    media_types = Profile.embed_policy.media_types,
    allowed_reference_confidentiality = Profile.embed_policy.allowed_reference_confidentiality,
  },
  required_non_claims = Profile.required_non_claims,
} | Contracts.ProfileShape | Contracts.ProfileContract
