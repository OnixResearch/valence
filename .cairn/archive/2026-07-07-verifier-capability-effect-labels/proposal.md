## Why

Unison makes effects explicit through abilities such as IO, Exception, Remote, and Http. Valence receipts and verifier reports should similarly expose verifier capabilities so operators can distinguish pure deterministic checks from checks that read files, run processes, access clocks, use networks, or rely on secrets.

This improves replay, cache eligibility, trust review, and policy gating.

## What Changes

- Add a typed verifier capability/effect label model.
- Attach capability sets to verifier receipts/reports where verification behavior depends on effects.
- Treat pure capability sets as stronger cache/replay candidates.
- Add policy checks that can deny risky capabilities for selected gates.

## Impact

- **Files**: likely Evidence IR verifier/report DTOs, stack-provenance reports where relevant, docs, tests.
- **Compatibility**: additive fields; old receipts may default to conservative unknown/effectful labels.
- **Validation**: positive/negative capability parsing, policy, and replay tests.
