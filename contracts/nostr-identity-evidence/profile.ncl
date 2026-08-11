let Contracts = import "contracts.ncl" in
({
  schema = "valence.nostr-identity-profile.v1",
  evidence_schema = "valence.nostr-identity-evidence.v1",
  ucan_source_revision = "277ab768eb532ac0c715a3bbff2a7dc37a702f11",
  ucan_credential_revision = "c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026",
  basalt_source_revision = "bb8fb5f4a763f532df96b6e65407163ac668834e",
  basalt_schema_identity = {
    role = "basalt_blake3",
    hex = "45824d99ae765e885f085ef2c8c8294343415af8e5489c35509c3e255476e258",
  },
  basalt_implementation_identity = {
    role = "basalt_blake3",
    hex = "ced0ffcfe006a48ffd8f2b856ad0da9641a03a24e6ebda3877163c398252897c",
  },
  basalt_policy_identity = {
    role = "basalt_blake3",
    hex = "9f5df806b5fac55a35a63f58c86e4218de1414141c20c999cbd140ea3c0556a2",
  },
  verification_role = "recorded_only",
  protocol_digest_role = "nostr_event_sha256",
  valence_digest_role = "valence_blake3",
  required_non_claims = Contracts.required_non_claims,
}) | Contracts.Profile
