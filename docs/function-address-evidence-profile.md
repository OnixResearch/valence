# Function-address Evidence IR profile

Valence validates function-address evidence as recorded-only identity/linkage evidence.

Inputs may come directly from Octet function-address artifacts or from Kamacite portable function-address receipts. Valence checks profile version, declared roles, digest domains, dependency graph rows, source spans, optional Octet producer receipt identity, optional Kamacite supplied-receipt identity, optional Valence projection identity, optional Mantle sidecar/release-binary context, and non-claim boundaries. It does not parse Rust and does not promote function-address evidence to a formal proof.

## Roles

- Octet: Rust source-shape producer.
- Kamacite: portable receipt/interchange owner.
- Valence: Evidence IR identity/linkage and graph-row validator.
- Mantle: release artifact bundler for validated sidecars.
- Cairn: lifecycle/readiness policy gate consumer.

## Policy identity

The function-address Evidence IR profile has deterministic policy material covering its profile version, accepted digest domains, accepted roles, required non-claims, graph edge kinds, and boundary text. Valence computes a BLAKE3 `policy-profile` identity from that material and requires function-address evidence to carry the current identity, so Mantle and Cairn can bind receipts to the reviewed Valence validation contract instead of placeholder hashes.

## Digest spellings

Valence normalizes digest spellings only at explicit stack boundaries. Octet producer inputs use `b3:<hex>`; Kamacite receipt identities and Mantle sidecar digests may arrive as bare lowercase BLAKE3 hex with an explicit typed domain. Valence emits canonical report spellings as `b3:<hex>` and validates the digest domain separately so a syntactically valid hash cannot move between source, function, receipt, sidecar, or binary roles.

## Non-claims

Function-address Evidence IR does not claim Rust semantic correctness, semantic equivalence, whole-program safety, verifier soundness, release eligibility, or runtime behavior.

Valid graph rows are linkage/context only: function dependency edges, Kamacite receipt backing edges, Valence projection edges, and Mantle bundle-local sidecar/release context edges. Cairn remains responsible for lifecycle/readiness policy.
