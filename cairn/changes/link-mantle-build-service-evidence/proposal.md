# Change: Link Mantle build-service evidence

## Why

Mantle, Basalt, Lattice, and OnixOS will produce related observations for one remote build. Without a canonical profile, attempt state, authority decisions, resource selection, retries, usage, reuse, and GitHub Check Runs can become disconnected records with inconsistent identity.

Valence should define the stable linkage and non-claim boundary. It must not promote operational observations into behavioral correctness or release eligibility.

## What Changes

- Add a versioned build-service evidence profile over the existing Evidence IR.
- Define canonical identities and links for service request, durable attempt, state transition, authority decision, machine class, resource decision, resource observation, retry edge, usage entry, result, reuse edge, orchestration job, and external status.
- Define producer and consumer linkage for authorized result reuse.
- Define verification roles for signed, checked, linked, recorded-only, and external facts.
- Use canonical encoding and BLAKE3 for stack-owned profile and record identities.
- Exclude bearer tokens, private claims, secret values, provider credentials, raw logs, and value-derived hashes from canonical artifacts.
- Add Rust and Nickel conformance helpers plus positive and negative fixtures.
- Preserve explicit non-claims for build correctness, hermeticity, isolation, authorization freshness, resource sufficiency, billing fairness, and release eligibility.

## Non-Goals

- Scheduling, execution, store, cache, accounting, OOM retry, or Check Run behavior.
- OIDC or UCAN verification.
- Replacing Mantle's durable state or Lattice's workflow state.
- Embedding raw logs or credentials in Evidence IR.
- Deciding Cairn release readiness.

## Dependencies

- Existing Valence canonical identity, span, verification-role, external-evidence, and non-claim mechanisms.
- Consumer coordination with Mantle, Lattice, Basalt, OnixOS, and Cairn.

## Impact

- **Affected specs:** `evidence-ir`
- **Affected code:** Rust Evidence IR profile types, canonical encoding, Nickel contract, validation, fixtures, documentation, and consumer examples
