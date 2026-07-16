let Contracts = import "contracts.ncl" in
let C = Contracts.constants in
{
  schema = C.ProfileSchema,
  cob_type = C.CobType,
  root_schema = C.RootSchema,
  event_schema = C.EventSchema,
  binding_schema = C.BindingSchema,
  snapshot_schema = C.SnapshotSchema,
  authorization_schema = C.AuthorizationSchema,
  relationships = C.Relationships,
  actor_roles = [
    {
      role = "attacher",
      operations = ["attach", "initialize"],
    },
    {
      role = "maintainer",
      operations = C.Operations,
    },
  ],
  embed_policy = {
    max_bytes = C.MaxEmbeddedArtifactBytes,
    media_types = C.EmbeddedMediaTypes,
    allowed_reference_confidentiality = C.ConfidentialityClasses,
  },
  required_non_claims = [
    C.EvidenceNonClaim,
    C.RetentionNonClaim,
    C.ConfidentialityNonClaim,
  ],
} | Contracts.ProfileShape | Contracts.ProfileContract
