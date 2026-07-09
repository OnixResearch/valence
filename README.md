# valence

The Evidence IR: a canonical, serializable, type-checked intermediate
representation for evidence artifacts with span/provenance threading and
non-claim boundary enforcement.

Valence is the shared evidence identity layer for the local trust stack
(octet, mantle, trellis, cairn). It provides the stable IDs and canonical
forms that thread source -> receipt -> binary across the stack, plus a
checker and a span explainer.

## What it is

Valence is an IR, not a language. It gives the stack canonical-evidence-like
discipline (canonical machine form + human projection + span survival through
the chain) without a compiler, without backends, and without mechanized
type-soundness.

## Modules

### `evidence_ir` (v1)

The Evidence IR v1 spine:

- `EvidenceArtifact`: the unified canonical form for Function Objects,
  evidence index records, and receipts. Carries `schema`, `artifact_kind`,
  `stable_id`, `canonical_hash`, `source_span`, `verification_role`,
  `payload`, and `non_claims`.
- `EvidenceSpan`: extended source span with `source_id`, byte offsets, line/
  column positions, and optional label. The `source_id` is the stable
  identifier that threads through the whole chain.
- `VerificationRole`: `Property` | `RecordedOnly` | `Boundary`. Schema lands
  now; enforcement (counting only `Property` as verified) lands in Phase 1 via
  cairn `release-readiness`.
- `build_evidence_artifact`: construct from typed inputs.
- `check_evidence_artifact`: validate schema + non-claim boundary + hash +
  reject overclaims. Fails closed.
- `explain_span`: resolve a stable source ID to its span from a JSON manifest.

### `artifact_ref` (v1)

Reusable artifact-reference core for stack evidence identity plumbing. It validates
role-specific artifact refs for binaries, sidecars, receipts, source artifacts,
release bundle members, and compatibility refs; profile-scoped BLAKE3 spellings
(raw hex, `b3:`, and `blake3:`); confined already-loaded path/label metadata;
declared hash/size metadata consistency; and public-field adapters that preserve
existing JSON field names. Passing validation proves only artifact ref syntax,
role separation, metadata confinement, and declared hash/size consistency. It
does not prove artifact truth, behavioral correctness, semantic equivalence,
verifier soundness, release eligibility, lifecycle readiness, runtime authority,
or whole-program safety.

### `evidence_model` (v1)

Unison-inspired Evidence IR support models:

- content-addressed evidence definitions: BLAKE3 over typed canonical content
  and dependency identities, with display names, paths, and aliases treated as
  review metadata;
- name-binding history: deterministic alias/name/path bindings to content
  identities, with ambiguous and stale bindings failing closed;
- queryable evidence graph: typed nodes and edges for definitions, artifacts,
  requirements, verifiers, receipts, binaries, sidecars, and aliases, plus pure
  dependency, coverage, binary-traceability, verifier-evidence, and alias
  queries;
- verifier capability labels: deterministic capability sets and fail-closed
  policy checks for denied or unknown effects;
- dependency-closure verifier cache keys: cache reuse is a hit only when all
  identity inputs match and the capability set is pure or explicitly
  replay-safe;
- executable receipt transcripts: in-memory fixtures bind inputs to expected
  reports, issue codes, and hashes without implicit refresh;
- structured refactoring sessions: dry-run-first alias/name/path and
  requirement-binding migrations that preserve content identity and block
  semantic-equivalence claims;
- typed durable storage envelopes: schema/type/codec/content-hash validation
  before persisted evidence values are decoded.

These surfaces are identity, linkage, cache-key, fixture, and decode-compatibility
models only. They do not prove behavioral correctness, semantic equivalence,
verifier soundness, release eligibility, requirement satisfaction, evidence
truth, or whole-program safety.

### `evidence_harness` / `hegel_property` (v1)

Deterministic testing-harness cores for the Evidence IR model:

- transcript fixture loading/evaluation and explicit refresh planning;
- malformed-input corpus execution over pure validation cores;
- normalized golden report identity and semantic diffs;
- requirement marker coverage rows for `r[impl ...]` and `r[verify ...]`;
- capability-label policy checks and opt-in denied-effect probe reporting;
- fast/deep/nightly suite lane definitions and stable status summaries;
- Hegel RS property receipt DTOs, validation, and fast/deep generated-input lane definitions.

See `docs/testing-harness.md` for lane commands and fixture-refresh guidance.
Harness and Hegel reports prove deterministic fixture behavior, marker
traceability, suite scheduling, or bounded generated-input checks only; they do
not prove correctness, verifier soundness, semantic equivalence, requirement
satisfaction, sandbox effectiveness, release eligibility, or whole-program
safety.

### `function_object` (v1, extracted from octet)

Function Object v1 DTOs, canonical hash payloads, caveat summaries, and
pure-cache eligibility from source facts. The stable review/provenance
identity for selected Rust functions.

### `evidence_codebase_index` (v1, extracted from octet)

Builds, queries, renders, and boundary-checks evidence-codebase indexes from
in-memory artifacts.

Kani receipts use dedicated contract and stub Evidence IR kinds with required
non-claim labels; see `docs/kani-evidence-boundaries.md`.

### `evidence_index` (v1, extracted from octet)

Renders and checks the current-head Valence evidence page from typed inputs.

### `stack_provenance_adapter` (v1)

Builds and verifies the stack-specific sidecar that links verified Octet
provenance exports to Mantle release bundles. Mantle treats the sidecar as
opaque external evidence; Valence owns the stack semantics: schema, claim scope,
BLAKE3 digest binding, binary hash binding, parent artifact identity, evidence
tier preservation, and non-claim enforcement.

## CLI

```bash
valence check <FILE>                        Validate a canonical Evidence IR artifact
valence explain --id <ID> --manifest <FILE> Resolve a stable source ID to its span
valence stack-provenance build --octet-export <FILE> --binary-hash <B3> --output <FILE>
valence stack-provenance verify --manifest <FILE> --sidecar <FILE>
valence stack-provenance verify-built-binary --binary <PATH> --octet-export <FILE> --sidecar <FILE> --manifest <FILE> [--json]
valence stack-provenance graph-report --chain <FILE> [--format json|markdown]
valence version                             Print version
```

`check` reads a JSON artifact, validates schema + hash + non-claim boundary,
rejects overclaims, prints visible `non_claim=` lines, and exits 0 (valid) /
1 (invalid) / 2 (I/O error).

`explain` reads a spans manifest, resolves the source ID, prints the span,
and exits 0 (found) / 1 (not found) / 2 (I/O error).

`stack-provenance build` reads an Octet Valence provenance export that has
already passed Octet artifact verification, binds it to a release binary hash,
and writes a canonical Valence sidecar. `stack-provenance verify` checks a
Mantle manifest external-evidence record with role `stack-provenance-trace`
against that sidecar and reports stale digests, missing roles, weakened
boundaries, and tier-promotion attempts. `stack-provenance verify-built-binary`
measures the binary bytes with BLAKE3 and accepts only when that digest matches
the sidecar, every provenance link, the Octet export-derived sidecar content,
and Mantle release binary identity metadata. `stack-provenance graph-report`
checks a canonical stack chain that links requirement, span, receipt, proof,
sidecar, release-bundle, and Cairn gate nodes with typed edges and emits either
machine JSON or operator Markdown from the same report DTO. Success proves
measured-byte identity, canonical IDs, typed linkage, role policy, and non-claim
conformance only; it is not a correctness, semantic-equivalence, verifier
soundness, requirement-satisfaction, or release-eligibility proof.

Nickel contracts in `contracts/stack-provenance/` provide a preflight gate for
static metadata shape and bounded-claim invariants. They check schema strings,
role names, claim scope, BLAKE3 digest syntax, parent linkage, required
non-claims, report shape, and evidence-tier promotion failures before Rust byte
measurement is treated as eligible to run. Nickel does not read or measure the
binary; Rust remains the authority for measured-byte identity and digest
comparison.

Preserves corpus fixtures in `contracts/stack-provenance/preserves-corpus/`
provide canonical metadata value identity for the same stack-provenance surfaces.
Valence projects Octet export metadata, sidecars, Mantle manifest views,
built-binary thread input envelopes, and verification reports into deterministic
Preserves text values, then records BLAKE3 `preserves_content_hash` values over
those canonical metadata bytes. These hashes are additive receipt evidence; they
do not replace raw binary BLAKE3 measurements or Mantle sidecar byte digests.

Rust also parses high-risk stack-provenance strings into typed internal boundary
values before making verifier decisions. BLAKE3 values are separated by role
(binary digest, sidecar byte digest, artifact digest, and Preserves metadata
hash), while schema versions, claim scope, Mantle role, verification status, and
evidence tiers are parsed into narrow enums. This is internal hardening only: the
public JSON contracts and CLI output remain string-compatible, and typed values
do not broaden the measured-byte identity-and-linkage-only claim.

## Boundary

Valence supports bounded local evidence replay for selected evidence surfaces.
For stack provenance, Octet exports verified artifact facts, Mantle bundles the
sidecar opaquely as optional external evidence, and Cairn consumes Valence graph
and sidecar reports as lifecycle/release-policy inputs. Valence is the stack
semantics verifier for identity, digest, typed-edge, role, and non-claim policy;
Mantle remains bundle-local and Cairn remains lifecycle-policy-local. Valence
does not claim behavioral correctness, semantic equivalence, exhaustive Rust
coverage, pure-cache eligibility,
cross-machine determinism, cross-toolchain determinism, external consumer
compatibility, mechanized language soundness, requirement satisfaction, or
general HIR/THIR/MIR extraction.

## Quick start

```bash
nix develop                # enter devshell
cargo test --workspace     # run all tests
cargo test stack_provenance_preserves -- --nocapture
cargo test -p valence-core evidence_harness
cargo test -p valence-core hegel_property_laws
cargo build -p valence-cli # build the CLI
nix build .#checks.x86_64-linux.stack-provenance-nickel-contracts --no-link
./target/debug/valence version
```

## License

AGPL-3.0-or-later

## References

- `../octet/` — source of the extracted valence-core modules; Octet retains
  CLI parsing, source scanning, corpus orchestration, and release ownership
  for its own evidence surfaces.
- `../cairn/` — lifecycle engine that will consume `verification_role` in
  Phase 1 and the provenance ID threading in Phase 2.
- `../trellis/` — verified-logic building blocks; `verification_frontier`
  module is the natural core for future property-vocabulary resolution.
- `../mantle/` — build system that will record covered valence/requirement IDs
  in release-evidence bundles in Phase 2.
- unison-lang.org/docs/the-big-idea — design reference for content-addressed
  definitions, names-as-metadata, codebase-as-database tooling, effect labels,
  durable typed values, and structured refactoring. Valence uses these patterns
  as design input only and does not adopt the Unison language or runtime.
- [oxidecomputer/iddqd](https://github.com/oxidecomputer/iddqd) — reference pattern for derived-key ID maps, duplicate-key rejection, borrowed lookup forms, and property-tested unsafe boundaries.
- [oxidecomputer/newtype-uuid](https://github.com/oxidecomputer/newtype-uuid) — reference pattern for typed external UUID wrappers; Valence-owned canonical artifact identity remains BLAKE3.
- [oxidecomputer/daft](https://github.com/oxidecomputer/daft) — reference pattern for semantic diffs of Rust data structures used as review evidence, not semantic-equivalence proof.
- [oxidecomputer/rats-corim](https://github.com/oxidecomputer/rats-corim), [oxidecomputer/vm-attest](https://github.com/oxidecomputer/vm-attest), [oxidecomputer/dice-util](https://github.com/oxidecomputer/dice-util), and [oxidecomputer/pki-playground](https://github.com/oxidecomputer/pki-playground) — attestation vocabulary and certificate-chain references for bounded Evidence IR metadata; Valence does not inherit their attestation claims.
