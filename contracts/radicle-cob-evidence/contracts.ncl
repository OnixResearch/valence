# Typed static policy for Valence evidence sidecars on Radicle COBs.
# Rust owns byte measurement, Git/Radicle access, signing, and runtime reduction.
let EmptyLength = 0 in
let MinimumPositiveSize = 1 in
let Blake3HexLength = 64 in
let GitOidHexLength = 40 in
let CobTypeValue = "org.onixresearch.valence.evidence" in
let ProfileSchemaValue = "valence.radicle-cob-evidence.profile.v1" in
let RootSchemaValue = "valence.radicle-cob-evidence.root.v1" in
let EventSchemaValue = "valence.radicle-cob-evidence.event.v1" in
let BindingSchemaValue = "valence.radicle-cob-evidence.binding.v1" in
let SnapshotSchemaValue = "valence.radicle-cob-evidence.snapshot.v1" in
let AuthorizationSchemaValue = "valence.radicle-cob-evidence.authorization.v1" in
let EvidenceNonClaimValue = "attaching evidence records identity and linkage only; it does not prove target correctness, authorize a waiver or disposition, satisfy a requirement, or establish lifecycle or release readiness" in
let RetentionNonClaimValue = "supersession and revocation change accepted state but do not promise deletion of replicated Git or Radicle history" in
let ConfidentialityNonClaimValue = "Radicle selective replication is not an at-rest encryption or secret-storage guarantee" in
let MaxEmbeddedArtifactBytesValue = 65536 in
let Constants = {
  CobType = CobTypeValue,
  ProfileSchema = ProfileSchemaValue,
  RootSchema = RootSchemaValue,
  EventSchema = EventSchemaValue,
  BindingSchema = BindingSchemaValue,
  SnapshotSchema = SnapshotSchemaValue,
  AuthorizationSchema = AuthorizationSchemaValue,
  EvidenceNonClaim = EvidenceNonClaimValue,
  RetentionNonClaim = RetentionNonClaimValue,
  ConfidentialityNonClaim = ConfidentialityNonClaimValue,
  MaxEmbeddedArtifactBytes = MaxEmbeddedArtifactBytesValue,
  Relationships = ["context_for", "disposition_for", "evidence_against", "evidence_for"],
  Operations = ["attach", "initialize", "revoke", "supersede"],
  ConfidentialityClasses = ["public", "restricted"],
  EmbeddedMediaTypes = ["application/json"],
} in
let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > EmptyLength
) in
let PositiveNumber = std.contract.from_predicate (fun value =>
  std.is_number value && value >= MinimumPositiveSize
) in
let Exact = fun expected => std.contract.from_predicate (fun value => value == expected) in
let contains = fun values expected => std.array.any (fun value => value == expected) values in
let unique = fun values =>
  std.array.all (fun value =>
    std.array.length (std.array.filter (fun candidate => candidate == value) values) == MinimumPositiveSize
  ) values
in
let has_required_non_claims = fun value =>
  contains value.required_non_claims EvidenceNonClaimValue
  && contains value.required_non_claims RetentionNonClaimValue
  && contains value.required_non_claims ConfidentialityNonClaimValue
in
let ActorRoleContract = {
  role | NonEmptyString,
  operations | Array NonEmptyString,
} in
let EmbedPolicyContract = {
  max_bytes | PositiveNumber,
  media_types | Array NonEmptyString,
  allowed_reference_confidentiality | Array NonEmptyString,
} in
let ProfileShapeContract = {
  schema | Exact ProfileSchemaValue,
  cob_type | Exact CobTypeValue,
  root_schema | Exact RootSchemaValue,
  event_schema | Exact EventSchemaValue,
  binding_schema | Exact BindingSchemaValue,
  snapshot_schema | Exact SnapshotSchemaValue,
  authorization_schema | Exact AuthorizationSchemaValue,
  relationships | Array NonEmptyString,
  actor_roles | Array ActorRoleContract,
  embed_policy | EmbedPolicyContract,
  required_non_claims | Array NonEmptyString,
} in
let profile_is_bounded = fun profile =>
  profile.relationships == Constants.Relationships
  && unique profile.relationships
  && std.array.length profile.actor_roles > EmptyLength
  && unique (std.array.map (fun role => role.role) profile.actor_roles)
  && std.array.all (fun role =>
    std.array.length role.operations > EmptyLength
    && unique role.operations
    && std.array.all (fun operation => contains Constants.Operations operation) role.operations
  ) profile.actor_roles
  && profile.embed_policy.max_bytes == MaxEmbeddedArtifactBytesValue
  && profile.embed_policy.media_types == Constants.EmbeddedMediaTypes
  && profile.embed_policy.allowed_reference_confidentiality == Constants.ConfidentialityClasses
  && has_required_non_claims profile
in
let BoundedProfileContract = std.contract.from_predicate profile_is_bounded in
{
  constants = Constants,
  ActorRole = ActorRoleContract,
  EmbedPolicy = EmbedPolicyContract,
  ProfileShape = ProfileShapeContract,
  ProfileContract = BoundedProfileContract,
}
