let Contracts = import "../../contracts.ncl" in
let base = import "../base.ncl" in
{
  profile = base.profile & { claimed_observation_classes | force = ["build"] },
  service_requests = base.service_requests,
  attempts = base.attempts,
  transitions = base.transitions,
  authority_decisions = base.authority_decisions,
  machine_classes = base.machine_classes,
  resource_decisions = base.resource_decisions,
  resource_observations = base.resource_observations,
  retry_edges = [],
  usage_records = [],
  build_results = base.build_results,
  reuse_edges = [],
  orchestration_jobs = base.orchestration_jobs,
  external_statuses = [],
} | Contracts.Bundle
