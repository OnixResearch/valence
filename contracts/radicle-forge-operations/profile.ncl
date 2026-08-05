# r[impl valence.radicle_forge_ops.profile]
# r[verify valence.radicle_forge_ops.profile]
let Contracts = import "contracts.ncl" in
let C = Contracts.constants in
{
  schema = C.ProfileSchema,
  source_schema = C.SourceSchema,
  plan_schema = C.PlanSchema,
  binding_schema = C.BindingSchema,
  admission_schema = C.AdmissionSchema,
  source_kinds = C.SourceKinds,
  issue_states = C.IssueStates,
  pull_request_states = C.PullRequestStates,
  review_states = C.ReviewStates,
  review_handling = "attributed-comment-only",
  cob_types = {
    issue = C.IssueCobType,
    pull_request = C.PatchCobType,
  },
  bounds = {
    max_records = C.MaxRecords,
    max_events_per_record = C.MaxEventsPerRecord,
    max_labels_per_record = C.MaxLabelsPerRecord,
    max_text_bytes = C.MaxTextBytes,
    max_title_bytes = C.MaxTitleBytes,
  },
  required_non_claims = [
    C.MigrationNonClaim,
    C.ReviewNonClaim,
    C.AdmissionNonClaim,
  ],
} | Contracts.ProfileShape | Contracts.ProfileContract
