let Contracts = import "../../contracts.ncl" in
let P = import "../../profile.ncl" in
({
  schema = P.schema,
  evidence_schema = P.evidence_schema,
  ucan_source_revision = P.ucan_source_revision,
  ucan_credential_revision = P.ucan_credential_revision,
  basalt_source_revision = P.basalt_source_revision,
  basalt_schema_identity = P.basalt_schema_identity,
  basalt_implementation_identity = P.basalt_implementation_identity,
  basalt_policy_identity = P.basalt_policy_identity,
  verification_role = "property",
  protocol_digest_role = P.protocol_digest_role,
  valence_digest_role = P.valence_digest_role,
  required_non_claims = P.required_non_claims,
}) | Contracts.Profile
