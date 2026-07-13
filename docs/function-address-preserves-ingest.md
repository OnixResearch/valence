# Function-address Preserves ingest

Valence accepts an already-loaded Kamacite canonical Preserves receipt envelope,
checks its exact BLAKE3 identity, schema, digest domains, source-span-backed
function-address profile, dependency identity kind, recorded-only role, and
non-claims, then emits bounded graph rows. JSON projections are compatibility
metadata only and must match their declared canonical projection identity.

The CLI owns receipt-envelope file reads and JSON transport parsing; the core
receives canonical Preserves bytes and typed values. Passing validation proves
canonical receipt identity and bounded linkage only, not Rust semantics,
dependency completeness, correctness, proof status, or release eligibility.
