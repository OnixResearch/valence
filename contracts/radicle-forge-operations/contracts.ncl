# Typed static policy for bounded forge metadata migration and CI admission.
# Rust owns canonical bytes, local Git/Radicle access, signing, and shell effects.
let EmptyLength = 0 in
let MinimumPositiveSize = 1 in
let ProfileSchemaValue = "valence.radicle-forge-operations.profile.v1" in
let SourceSchemaValue = "valence.radicle-forge-operations.source-batch.v1" in
let PlanSchemaValue = "valence.radicle-forge-operations.plan.v1" in
let BindingSchemaValue = "valence.radicle-forge-operations.binding.v1" in
let AdmissionSchemaValue = "valence.radicle-forge-operations.ci-admission.v1" in
let IssueCobTypeValue = "xyz.radicle.issue" in
let PatchCobTypeValue = "xyz.radicle.patch" in
let MaxRecordsValue = 128 in
let MaxEventsPerRecordValue = 512 in
let MaxLabelsPerRecordValue = 64 in
let MaxTextBytesValue = 262144 in
let MaxTitleBytesValue = 512 in
let MigrationNonClaimValue = "migration records normalized source metadata and observed native COB state; it does not prove source export completeness, source actor authenticity, original signature or timestamp preservation, or semantic equivalence" in
let ReviewNonClaimValue = "imported review metadata is recorded attribution only and does not create Radicle approval, merge eligibility, delegate authority, or canonical-ref authority" in
let AdmissionNonClaimValue = "CI admission is bounded input for an external guarded compare-and-swap; it is not protocol-enforced mandatory CI, bypass-proof delegate behavior, canonical mutation, CI correctness, or release readiness" in
let Constants = {
  ProfileSchema = ProfileSchemaValue,
  SourceSchema = SourceSchemaValue,
  PlanSchema = PlanSchemaValue,
  BindingSchema = BindingSchemaValue,
  AdmissionSchema = AdmissionSchemaValue,
  IssueCobType = IssueCobTypeValue,
  PatchCobType = PatchCobTypeValue,
  MaxRecords = MaxRecordsValue,
  MaxEventsPerRecord = MaxEventsPerRecordValue,
  MaxLabelsPerRecord = MaxLabelsPerRecordValue,
  MaxTextBytes = MaxTextBytesValue,
  MaxTitleBytes = MaxTitleBytesValue,
  MigrationNonClaim = MigrationNonClaimValue,
  ReviewNonClaim = ReviewNonClaimValue,
  AdmissionNonClaim = AdmissionNonClaimValue,
  SourceKinds = ["issue", "pull_request"],
  IssueStates = ["open", "closed", "solved"],
  PullRequestStates = ["open", "closed", "merged", "draft"],
  ReviewStates = ["approved", "changes_requested", "commented", "dismissed", "pending"],
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
let BoundsContract = {
  max_records | PositiveNumber,
  max_events_per_record | PositiveNumber,
  max_labels_per_record | PositiveNumber,
  max_text_bytes | PositiveNumber,
  max_title_bytes | PositiveNumber,
} in
let CobTypesContract = {
  issue | Exact IssueCobTypeValue,
  pull_request | Exact PatchCobTypeValue,
} in
let ProfileShapeContract = {
  schema | Exact ProfileSchemaValue,
  source_schema | Exact SourceSchemaValue,
  plan_schema | Exact PlanSchemaValue,
  binding_schema | Exact BindingSchemaValue,
  admission_schema | Exact AdmissionSchemaValue,
  source_kinds | Array NonEmptyString,
  issue_states | Array NonEmptyString,
  pull_request_states | Array NonEmptyString,
  review_states | Array NonEmptyString,
  review_handling | Exact "attributed-comment-only",
  cob_types | CobTypesContract,
  bounds | BoundsContract,
  required_non_claims | Array NonEmptyString,
} in
let profile_is_bounded = fun profile =>
  profile.source_kinds == Constants.SourceKinds
  && profile.issue_states == Constants.IssueStates
  && profile.pull_request_states == Constants.PullRequestStates
  && profile.review_states == Constants.ReviewStates
  && unique profile.source_kinds
  && unique profile.issue_states
  && unique profile.pull_request_states
  && unique profile.review_states
  && profile.review_handling == "attributed-comment-only"
  && profile.bounds.max_records == MaxRecordsValue
  && profile.bounds.max_events_per_record == MaxEventsPerRecordValue
  && profile.bounds.max_labels_per_record == MaxLabelsPerRecordValue
  && profile.bounds.max_text_bytes == MaxTextBytesValue
  && profile.bounds.max_title_bytes == MaxTitleBytesValue
  && contains profile.required_non_claims MigrationNonClaimValue
  && contains profile.required_non_claims ReviewNonClaimValue
  && contains profile.required_non_claims AdmissionNonClaimValue
  && unique profile.required_non_claims
in
let BoundedProfileContract = std.contract.from_predicate profile_is_bounded in
{
  constants = Constants,
  Bounds = BoundsContract,
  CobTypes = CobTypesContract,
  ProfileShape = ProfileShapeContract,
  ProfileContract = BoundedProfileContract,
}
