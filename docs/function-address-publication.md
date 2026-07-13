# Function-address publication

`valence function-address validate` plans publication only from a passing typed
validation result. Invalid input is diagnostic-only: requested bundle, receipt,
and graph destinations are neither created nor replaced.

The preferred output is `--bundle`, a single
`valence.function-address-publication.v1` artifact containing the validated
Evidence IR receipt and a versioned graph report. The graph binds the current
policy identity, validated artifact identity, complete typed edge rows, counts,
`valid: true`, required non-claims, and a BLAKE3 self-hash-excluded graph
identity. Rows are sorted by `(from, to, edge_kind)`; duplicate edges and empty
locators fail closed. Bundle replacement stages, writes, flushes, and renames a
sibling file through the declared output capability so a pre-commit failure
preserves the prior bundle.

`--receipt` and `--graph` are compatibility outputs. Both carry the same
publication, policy, artifact, and passing-verdict identities. Valence stages
both before replacing either and atomically replaces each file individually,
but does not claim portable crash atomicity across the two renames. Consumers
such as Mantle and Cairn should bind the canonical bundle rather than a bare row
array. Failed validation never emits publication-shaped readiness input.

Passing publication proves validated identity and linkage coherence only. It
does not prove Rust semantics, behavioral correctness, semantic equivalence,
proof status, verifier soundness, requirement satisfaction, or release
eligibility.
