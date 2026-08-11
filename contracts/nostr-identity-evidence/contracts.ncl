let Digest = { role | String, hex | std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) } in
let RequiredNonClaims = [
  "Valence records producer identity and linkage only; it does not verify Nostr signatures",
  "RecordedOnly Nostr evidence does not count as Property evidence or authority",
  "relay, DNS, signer, clock, replay, policy publication, and authorization remain external",
  "BLAKE3 artifact identity does not authenticate a producer or establish release eligibility",
] in
let ProfileShape = {
  schema | std.contract.from_predicate (fun value => value == "valence.nostr-identity-profile.v1"),
  evidence_schema | std.contract.from_predicate (fun value => value == "valence.nostr-identity-evidence.v1"),
  ucan_source_revision | std.contract.from_predicate (fun value => value == "277ab768eb532ac0c715a3bbff2a7dc37a702f11"),
  ucan_credential_revision | std.contract.from_predicate (fun value => value == "c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026"),
  basalt_source_revision | std.contract.from_predicate (fun value => value == "bb8fb5f4a763f532df96b6e65407163ac668834e"),
  basalt_schema_identity | Digest,
  basalt_implementation_identity | Digest,
  basalt_policy_identity | Digest,
  verification_role | std.contract.from_predicate (fun value => value == "recorded_only"),
  protocol_digest_role | std.contract.from_predicate (fun value => value == "nostr_event_sha256"),
  valence_digest_role | std.contract.from_predicate (fun value => value == "valence_blake3"),
  required_non_claims | Array String,
} in
let ProfileInvariants = std.contract.from_predicate (fun profile =>
  profile.basalt_schema_identity.role == "basalt_blake3"
  && profile.basalt_schema_identity.hex == "45824d99ae765e885f085ef2c8c8294343415af8e5489c35509c3e255476e258"
  && profile.basalt_implementation_identity.role == "basalt_blake3"
  && profile.basalt_implementation_identity.hex == "ced0ffcfe006a48ffd8f2b856ad0da9641a03a24e6ebda3877163c398252897c"
  && profile.basalt_policy_identity.role == "basalt_blake3"
  && profile.basalt_policy_identity.hex == "9f5df806b5fac55a35a63f58c86e4218de1414141c20c999cbd140ea3c0556a2"
  && profile.required_non_claims == RequiredNonClaims
) in
{
  Profile = std.contract.Sequence [ProfileShape, ProfileInvariants],
  required_non_claims = RequiredNonClaims,
}
