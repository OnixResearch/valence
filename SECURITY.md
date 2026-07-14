# Security policy

## Supported revisions

Security fixes are made against the current default branch. Reviewed release
tags may receive fixes when their release notes explicitly say so; no other
long-term-support promise is implied.

## Report a vulnerability

Use GitHub private vulnerability reporting for this repository:

<https://github.com/OnixResearch/valence/security/advisories/new>

Do not include vulnerability details in a public issue, discussion, commit, or
pull request. Include the affected revision, reachable surface, reproduction
steps, expected impact, and any known mitigations in the private report. Avoid
including unrelated credentials, personal data, or production evidence.

Repository administrators must keep private vulnerability reporting enabled
before treating the repository as publicly supported. If the private reporting
interface is unavailable, a public issue may report only that the private
channel is unavailable; it must not contain vulnerability details.

## Scope

A report may concern the Rust core, CLI shell, canonicalization or hashing,
Evidence IR validation, capability-root handling, package boundaries, or build
and release tooling. A passing Valence validator is not a security proof: each
result remains limited to its declared identity, linkage, role, and non-claim
boundary.
