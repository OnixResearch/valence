# Formal proof stack-chain fixture

Octet can already emit scoped `formal_proof` provenance for validated Verus receipts, Valence can verify stack-provenance sidecars and graph reports, and Cairn can count `property` verification roles. The weak spot is the seam between those facts: reviewers need one frozen positive/negative chain fixture that proves the stack preserves a scoped formal-proof claim without promoting recorded-only or manual-review evidence.

## Scope

- Add a checked-in Valence stack-chain fixture that links an Octet `formal_proof` export, proof card, receipt identity, Valence sidecar, graph report, and Cairn readiness input.
- Preserve verification-role semantics across the chain: only `property` records from validated Octet-owned receipts count as verified.
- Add negative fixtures for stale proof-card links, imported Trellis manual-review promotion, missing non-claims, digest drift, binary identity drift, and Cairn readiness overclaims.
- Keep the fixture frozen and hermetic; it must not require live sibling repositories, network access, or rerunning Verus.

## Impact

- **Valence** owns the canonical stack-chain fixture and role-preserving validation semantics.
- **Octet** remains the producer of scoped formal-proof provenance and source proof receipts.
- **Cairn** remains the lifecycle/release-readiness consumer that counts only eligible property evidence.
- **Reviewers** get an executable seam check instead of inferring correctness from separate repo-local checks.

## Non-claims

This change proves fixture identity, role preservation, typed linkage, and non-claim enforcement for the frozen chain only. It does not prove whole-program correctness, verifier soundness, semantic equivalence, release eligibility, live sibling-repo freshness, or requirement satisfaction outside the declared fixture scope.
