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

### `function_object` (v1, extracted from octet)

Function Object v1 DTOs, canonical hash payloads, caveat summaries, and
pure-cache eligibility from source facts. The stable review/provenance
identity for selected Rust functions.

### `evidence_codebase_index` (v1, extracted from octet)

Builds, queries, renders, and boundary-checks evidence-codebase indexes from
in-memory artifacts.

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
valence version                             Print version
```

`check` reads a JSON artifact, validates schema + hash + non-claim boundary,
rejects overclaims, and exits 0 (valid) / 1 (invalid) / 2 (I/O error).

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
and Mantle release binary identity metadata. Success proves measured-byte
identity and linkage only; it is not a correctness, semantic-equivalence, or
release-eligibility proof.

Nickel contracts in `contracts/stack-provenance/` provide a preflight gate for
static metadata shape and bounded-claim invariants. They check schema strings,
role names, claim scope, BLAKE3 digest syntax, parent linkage, required
non-claims, report shape, and evidence-tier promotion failures before Rust byte
measurement is treated as eligible to run. Nickel does not read or measure the
binary; Rust remains the authority for measured-byte identity and digest
comparison.

## Boundary

Valence supports bounded local evidence replay for selected evidence surfaces.
For stack provenance, Octet exports verified artifact facts, Mantle bundles the
sidecar opaquely as optional external evidence, and Valence performs the
stack-specific semantic verification. It does not claim behavioral correctness,
semantic equivalence, exhaustive Rust coverage, pure-cache eligibility,
cross-machine determinism, cross-toolchain determinism, external consumer
compatibility, mechanized language soundness, requirement satisfaction, or
general HIR/THIR/MIR extraction.

## Quick start

```bash
nix develop                # enter devshell
cargo test --workspace     # run all tests
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
