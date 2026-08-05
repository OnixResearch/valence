# Raw build-service bundle for contract fixtures. Not contract-annotated;
# pass fixtures annotate it, fail fixtures merge overrides and then annotate.
let h = fun letter => std.string.join "" (std.array.generate (fun _ => letter) 64) in
let nc = ["build-service linkage proves canonical identity and bounded observation linkage only; it does not prove build correctness, hermeticity, sandboxing, host isolation, identity beyond checked claims, authorization freshness, future resource sufficiency, accounting fairness, external status correctness, or release eligibility"] in
{
  profile = {
    profile_id = "valence.build-service-evidence.v1",
    profile_version = 1,
    canonicalization = "valence-canonical-json-v1",
    producer_policy = h "1",
    claimed_observation_classes = ["build", "retry", "reuse", "usage", "external-status"],
    non_claims = nc,
  },
  service_requests = [
    { request_id = "request-one", concrete_request_hash = h "2", idempotency_key = "idem-one", role = "linked", non_claims = nc },
    { request_id = "request-two", concrete_request_hash = h "3", idempotency_key = "idem-two", role = "linked", non_claims = nc },
  ],
  attempts = [
    { attempt_id = "attempt-one", request_identity = h "4", generation = 1, parent_attempt_identity = null, service_policy = "mantle-service-policy.v1", role = "linked", non_claims = nc },
    { attempt_id = "attempt-two", request_identity = h "4", generation = 2, parent_attempt_identity = h "5", service_policy = "mantle-service-policy.v1", role = "linked", non_claims = nc },
  ],
  transitions = [
    { attempt_identity = h "5", prior_state = "queued", next_state = "running", sequence = 1, observed_at = "2026-08-05T12:00:00Z", producer = "mantle", reason = "scheduled", role = "recorded_only", non_claims = nc },
    { attempt_identity = h "5", prior_state = "running", next_state = "failed", sequence = 2, observed_at = "2026-08-05T12:01:00Z", producer = "mantle", reason = "oom", role = "recorded_only", non_claims = nc },
  ],
  authority_decisions = [
    { decision_id = "decision-one", attempt_identity = h "6", subject_projection = "basalt-public-subject:builder", policy_identity = h "7", operation = "build.execute", decision = "allow", verifier_evidence_ref = "basalt-verification:receipt-1", role = "verified", non_claims = nc },
  ],
  machine_classes = [
    { class_id = "small", platform = "x86_64-linux", resource_class = "class-small", ordinal = 1, role = "linked", non_claims = nc },
    { class_id = "large", platform = "x86_64-linux", resource_class = "class-large", ordinal = 2, role = "linked", non_claims = nc },
  ],
  resource_decisions = [
    { decision_id = "resource-decision-one", attempt_identity = h "6", declared_minima = ["mem>=512MiB"], eligible_class_identities = [h "8", h "9"], selected_class_identity = h "9", policy = "mantle-resource-policy.v1", compatible_observation_identities = [h "a"], reasons = ["oom-escalation"], role = "verified", non_claims = nc },
  ],
  resource_observations = [
    { observation_id = "observation-one", attempt_identity = h "6", measurements = [{ name = "wall-clock", value = 42, unit = "s" }], collector = "mantle-executor", compatibility_scope = "class-comparison", terminal_outcome = "succeeded", role = "recorded_only", non_claims = nc },
  ],
  retry_edges = [
    { predecessor_attempt_identity = h "5", successor_attempt_identity = h "6", trigger = "oom_escalation", trigger_evidence = "oom-kill-event:1", prior_class_identity = h "8", next_class_identity = h "9", limit_state = "memory-limit-exceeded", policy = "mantle-retry-policy.v1", role = "linked", non_claims = nc },
  ],
  usage_records = [
    { usage_id = "usage-reservation", attempt_identity = h "6", usage_kind = "reservation", unit_schedule = "cpu-second:1", window = "2026-08-05T12:00:00Z/2026-08-05T12:05:00Z", scope = "project:onix", outcome = "reserved", role = "recorded_only", non_claims = nc },
  ],
  build_results = [
    { result_id = "result-one", request_identity = h "4", attempt_identity = h "6", platform = "x86_64-linux", policy = "mantle-build-policy.v1", outputs = [{ path = "out/bin/tool", content_hash = h "b" }], producer_signature_ref = "mantle-signer:sig-1", cas_observation = "cas:present", role = "signed", non_claims = nc },
  ],
  reuse_edges = [
    { producer_result_identity = h "c", consumer_request_identity = h "d", sharing_policy = "mantle-sharing-policy.v1", authorization_decision_identity = h "7", request_compatible = true, platform_compatible = true, output_check_ref = "output-check:ok", signature_ref = "mantle-signer:sig-1", cas_observation = "cas:present", usability_outcome = "usable", role = "verified", non_claims = nc },
  ],
  orchestration_jobs = [
    { job_id = "lattice-job-1", workflow_identity = "lattice-workflow:build", matrix_job_identity = "matrix:x86_64-linux", attempt_identity = h "6", role = "linked", non_claims = nc },
  ],
  external_statuses = [
    { status_id = "check-run-1", provider = "github-check-run", job_identity = h "e", attempt_identity = h "6", observed_state = "success", role = "external", non_claims = nc },
  ],
}
