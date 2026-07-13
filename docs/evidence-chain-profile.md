# Generic evidence-chain profile

Valence validates already-loaded Kamacite evidence-chain envelopes in two
stages. Generic checks bind canonical envelope bytes, profile and policy roots,
typed parent links, compatibility projections, verification roles, and required
non-claims. An explicit registry then dispatches the kind-specific payload
validator; function-address is the first complete profile and Trellis proof
evidence is a separately registered extension.

Successful validation emits deterministic typed graph rows and a BLAKE3 graph
root. Digest domains remain explicit, and recorded-only input is not promoted
to property or formal-proof evidence. Preserves/JSON parsing, file reads,
stdout/stderr, and exit codes remain in `valence-cli`; `valence-core` receives
in-memory envelope values and canonical bytes.

Passing the profile proves bounded canonical identity, linkage, role,
projection, and non-claim conformance only. It does not prove producer
semantics, evidence truth, behavioral correctness, semantic equivalence,
verifier soundness, requirement satisfaction, or release eligibility.
