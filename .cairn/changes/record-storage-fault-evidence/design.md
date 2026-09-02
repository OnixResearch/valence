# Design: Record storage-fault evidence

## Goals

- Preserve exact cohort and result identities.
- Keep producer observations separate from assumptions and lifecycle decisions.
- Reject mixed or weakened claim boundaries.
- Keep validation pure and file access capability rooted.

## Evidence profile

Add a versioned `storage-fault-evidence` profile. The closed envelope includes:

- candidate and source identities;
- producer and workload identities;
- kernel and guest image identities;
- filesystem, mount, and device profile identities;
- fault catalog, selected fault, and schedule identities;
- oracle and expected-property identities;
- result, diagnostics summary, and producer receipt identities;
- required assumptions, limitations, and non-claims.

All Valence-owned canonical identities use BLAKE3. Protocol-required external identities retain typed external roles.

## Role policy

Campaign observations, producer verdicts, and linked test reports use `RecordedOnly`.

Assumptions, storage models, excluded behaviors, and non-claims use `Boundary`.

The initial profile rejects `Property` for raw campaign or producer output. A later property profile needs a separate reviewed contract.

## Pure core

The core validates schema, identity roles, exact linkage, cohort consistency, case identity, result vocabulary, required boundaries, redaction, and duplicate prevention.

It does not read files, clocks, environment, devices, networks, VMMs, databases, or secrets.

## Shell

The CLI reads already-saved artifacts through declared roots and emits one canonical Evidence IR artifact plus a deterministic report.

It does not execute ChaosControl, Octet, Cairn, Molten, or storage operations.

## Privacy

The profile rejects credentials, raw keys, absolute host paths, full environment dumps, raw database contents, and unbounded logs. Bounded diagnostics use stable categories and counts.

## Verification

Positive fixtures cover complete single-node, persistent-consensus, Octet, and Cairn linkage.

Negative fixtures cover tampering, digest-role substitution, mixed cohorts, stale candidates, duplicate cases, missing boundaries, `Property` promotion, secret fields, raw paths, and overclaims.

## Claim Boundary

A passing result establishes only canonical identity, role, linkage, bounded field policy, and required non-claims.
