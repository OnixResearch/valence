# Build-service evidence contracts

Nickel contracts for the versioned Valence build-service evidence profile
(`valence.build-service-evidence.v1`). The profile links service requests,
durable attempts, transitions, authority decisions, machine classes, resource
decisions, observations, retries, usage, results, reuse, orchestration jobs,
and external statuses for one remote build.

## Layout

- `contracts.ncl` — typed contracts for the profile bundle and each record
  kind, plus the reuse-usability cross-field rule.
- `fixtures/base.ncl` — raw unannotated bundle shared by the fixtures.
- `fixtures/pass/` — shape-valid bundles (complete and minimal).
- `fixtures/fail/` — negative cases for unsupported versions, role overclaim,
  forbidden bearer/secret/raw-log material, missing units, missing non-claims,
  overclaim wording, malformed digests, and usable-reuse edges without
  authority facts.

## Claim boundary

Nickel validates metadata shape and static invariants only. Rust validates
canonical identity, domain-separated BLAKE3 linkage, and record graph
resolution (`valence_core::build_service`).

Passing these contracts does not prove build correctness, hermeticity,
sandboxing, host isolation, identity beyond checked claims, authorization
freshness, resource sufficiency, accounting fairness, external status truth,
or release eligibility.

Run the deterministic contract gate with:

```bash
nix build .#checks.x86_64-linux.build-service-nickel-contracts --no-link
```
