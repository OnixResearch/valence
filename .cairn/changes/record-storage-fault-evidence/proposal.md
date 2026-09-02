# Proposal: Record storage-fault evidence

## Why

ChaosControl, Molten, Octet, and Cairn need stable links for storage-fault campaign inputs and results. Raw receipts can mix candidate, kernel, filesystem, device, schedule, and oracle identities or omit essential non-claims.

Valence can preserve these identities and boundaries. It cannot establish that a fault occurred, that recovery was correct, or that storage is safe.

## What Changes

- Add a versioned storage-fault evidence profile to `valence-core`.
- Bind candidate, workload, kernel, filesystem, mount, device, fault, schedule, oracle, and result identities.
- Preserve producer receipts as `RecordedOnly` evidence.
- Preserve assumptions and non-claims as `Boundary` evidence.
- Link related single-node, consensus, Octet, and Cairn artifacts without merging their authority.
- Reject stale, mixed-cohort, malformed, secret-bearing, or overclaiming artifacts.
- Add capability-rooted CLI validation and deterministic reports.

## Impact

Valence gains a common identity and linkage vocabulary for bounded storage-fault evidence. Producers and lifecycle consumers retain their existing authority.

## Non-goals

- Do not execute fault campaigns or read live devices.
- Do not decide storage repair, consensus repair, or release readiness.
- Do not promote raw campaign output to `Property` evidence.
- Do not prove arbitrary hardware, filesystems, databases, or clusters.
