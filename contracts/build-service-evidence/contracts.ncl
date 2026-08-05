# Nickel contracts for the Valence build-service evidence profile.
#
# These contracts validate metadata shape and static invariants only. Rust
# validates canonical identity, domain-separated BLAKE3 linkage, and record
# graph resolution. Passing these contracts does not prove build correctness,
# hermeticity, isolation, authorization freshness, resource sufficiency,
# accounting fairness, external status truth, or release eligibility.

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let Blake3Hex = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value
) in

let NullableBlake3Hex = std.contract.from_predicate (fun value =>
  value == null || (std.is_string value && std.string.is_match "^[0-9a-f]{64}$" value)
) in

let NullableText = std.contract.from_predicate (fun value =>
  value == null || (std.is_string value && std.string.length value > 0 && std.string.length value <= 4096)
) in

let Rfc3339Utc = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$" value
) in

let BoundedText = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.length value <= 4096
  && !std.string.is_match "(?i)bearer " value
  && !std.string.is_match "(?i)begin (openssh )?private key" value
  && !std.string.is_match "(?i)oidc_token" value
  && !std.string.is_match "(?i)ucan_token" value
  && !std.string.is_match "(?i)compatibility_ticket" value
  && !std.string.is_match "(?i)private_claim" value
  && !std.string.is_match "(?i)replay_key" value
  && !std.string.is_match "(?i)provider_credential" value
  && !std.string.is_match "(?i)secret=" value
  && !std.string.is_match "(?i)raw_log" value
  && !std.string.is_match "(?i)value_derived_hash" value
) in

let NonClaims = std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value > 0
  && std.array.all
    (fun claim =>
      std.is_string claim
      && !std.string.is_match "(?i)proves evidence truth" claim
      && !std.string.is_match "(?i)proves behavioral correctness" claim
      && !std.string.is_match "(?i)proves semantic equivalence" claim
      && !std.string.is_match "(?i)proves verifier soundness" claim
      && !std.string.is_match "(?i)proves release eligibility" claim
      && !std.string.is_match "(?i)proves lifecycle readiness" claim
      && !std.string.is_match "(?i)proves whole-program safety" claim
    )
    value
  && std.array.any
    (fun claim =>
      std.is_string claim
      && std.string.is_match "build-service linkage proves canonical identity and bounded observation linkage only" claim
    )
    value
) in

let Role = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "verified" || value == "linked" || value == "signed" || value == "recorded_only" || value == "external")
) in

let ExternalStatusRole = std.contract.from_predicate (fun value =>
  std.is_string value && (value == "recorded_only" || value == "external")
) in

let ObservationClass = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "build" || value == "retry" || value == "reuse" || value == "usage" || value == "external-status")
) in

let TransitionState = std.contract.from_predicate (fun value =>
  std.is_string value
  && (value == "queued" || value == "scheduled" || value == "running" || value == "succeeded" || value == "failed" || value == "cancelled")
) in

let Profile = {
  profile_id | std.contract.from_predicate (fun value => value == "valence.build-service-evidence.v1"),
  profile_version | std.contract.from_predicate (fun value => value == 1),
  canonicalization | std.contract.from_predicate (fun value => value == "valence-canonical-json-v1"),
  producer_policy | Blake3Hex,
  claimed_observation_classes | Array ObservationClass | NonEmptyArray,
  non_claims | NonClaims,
} in

let ServiceRequest = {
  request_id | BoundedText,
  concrete_request_hash | Blake3Hex,
  idempotency_key | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let Attempt = {
  attempt_id | BoundedText,
  request_identity | Blake3Hex,
  generation | std.contract.from_predicate (fun value => std.is_number value && value >= 1),
  parent_attempt_identity | NullableBlake3Hex | optional,
  service_policy | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let AttemptTransition = {
  attempt_identity | Blake3Hex,
  prior_state | TransitionState,
  next_state | TransitionState,
  sequence | std.number.Nat,
  observed_at | Rfc3339Utc,
  producer | BoundedText,
  reason | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let AuthorityDecision = {
  decision_id | BoundedText,
  attempt_identity | Blake3Hex,
  subject_projection | BoundedText,
  policy_identity | Blake3Hex,
  operation | BoundedText,
  decision | std.contract.from_predicate (fun value => value == "allow" || value == "deny"),
  verifier_evidence_ref | NullableText | optional,
  role | Role,
  non_claims | NonClaims,
} in

let MachineClass = {
  class_id | BoundedText,
  platform | BoundedText,
  resource_class | BoundedText,
  ordinal | std.number.Nat,
  role | Role,
  non_claims | NonClaims,
} in

let ResourceDecision = {
  decision_id | BoundedText,
  attempt_identity | Blake3Hex,
  declared_minima | Array BoundedText,
  eligible_class_identities | Array Blake3Hex | NonEmptyArray,
  selected_class_identity | Blake3Hex,
  policy | BoundedText,
  compatible_observation_identities | Array Blake3Hex,
  reasons | Array BoundedText | NonEmptyArray,
  role | Role,
  non_claims | NonClaims,
} in

let Measurement = {
  name | BoundedText,
  value | std.number.Nat,
  unit | NonEmptyString,
} in

let ResourceObservation = {
  observation_id | BoundedText,
  attempt_identity | Blake3Hex,
  measurements | Array Measurement | NonEmptyArray,
  collector | BoundedText,
  compatibility_scope | BoundedText,
  terminal_outcome | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let RetryEdge = {
  predecessor_attempt_identity | Blake3Hex,
  successor_attempt_identity | Blake3Hex,
  trigger | std.contract.from_predicate (fun value => value == "oom_escalation"),
  trigger_evidence | BoundedText,
  prior_class_identity | Blake3Hex,
  next_class_identity | Blake3Hex,
  limit_state | BoundedText,
  policy | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let Usage = {
  usage_id | BoundedText,
  attempt_identity | Blake3Hex,
  usage_kind | std.contract.from_predicate (fun value => value == "reservation" || value == "reconciliation"),
  unit_schedule | BoundedText,
  window | BoundedText,
  scope | BoundedText,
  outcome | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let OutputRef = {
  path | BoundedText,
  content_hash | Blake3Hex,
} in

let BuildResult = {
  result_id | BoundedText,
  request_identity | Blake3Hex,
  attempt_identity | Blake3Hex,
  platform | BoundedText,
  policy | BoundedText,
  outputs | Array OutputRef | NonEmptyArray,
  producer_signature_ref | BoundedText,
  cas_observation | BoundedText,
  role | Role,
  non_claims | NonClaims,
} in

let ReuseEdge = {
  producer_result_identity | Blake3Hex,
  consumer_request_identity | Blake3Hex,
  sharing_policy | BoundedText,
  authorization_decision_identity | NullableBlake3Hex | optional,
  request_compatible | Bool,
  platform_compatible | Bool,
  output_check_ref | NullableText | optional,
  signature_ref | NullableText | optional,
  cas_observation | NullableText | optional,
  usability_outcome | std.contract.from_predicate (fun value => value == "usable" || value == "rejected"),
  role | Role,
  non_claims | NonClaims,
} in

let OrchestrationJob = {
  job_id | BoundedText,
  workflow_identity | BoundedText,
  matrix_job_identity | BoundedText,
  attempt_identity | Blake3Hex,
  role | Role,
  non_claims | NonClaims,
} in

let ExternalStatus = {
  status_id | BoundedText,
  provider | BoundedText,
  job_identity | Blake3Hex,
  attempt_identity | Blake3Hex,
  observed_state | BoundedText,
  role | ExternalStatusRole,
  non_claims | NonClaims,
} in

{
  Bundle = {
    profile | Profile,
    service_requests | Array ServiceRequest,
    attempts | Array Attempt,
    transitions | Array AttemptTransition,
    authority_decisions | Array AuthorityDecision,
    machine_classes | Array MachineClass,
    resource_decisions | Array ResourceDecision,
    resource_observations | Array ResourceObservation,
    retry_edges | Array RetryEdge,
    usage_records | Array Usage,
    build_results | Array BuildResult,
    reuse_edges | Array ReuseEdge,
    orchestration_jobs | Array OrchestrationJob,
    external_statuses | Array ExternalStatus,
  },

  # A usable reuse edge must carry authority, signature, CAS, compatibility,
  # and output-check facts. Shared object identity alone never suffices.
  # Apply after Bundle: `bundle | Contracts.Bundle | Contracts.ReuseUsabilityRule`.
  ReuseUsabilityRule = std.contract.from_predicate (fun bundle =>
    std.is_record bundle
    && std.array.all
      (fun edge =>
        edge.usability_outcome == "rejected"
        || (
          std.record.has_field "authorization_decision_identity" edge
          && edge.authorization_decision_identity != null
          && std.record.has_field "signature_ref" edge
          && edge.signature_ref != null
          && std.record.has_field "cas_observation" edge
          && edge.cas_observation != null
          && std.record.has_field "output_check_ref" edge
          && edge.output_check_ref != null
          && edge.request_compatible
          && edge.platform_compatible
        )
      )
      bundle.reuse_edges
  ),
}
