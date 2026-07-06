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

## CLI

```bash
valence check <FILE>                        Validate a canonical Evidence IR artifact
valence explain --id <ID> --manifest <FILE> Resolve a stable source ID to its span
valence version                             Print version
```

`check` reads a JSON artifact, validates schema + hash + non-claim boundary,
rejects overclaims, and exits 0 (valid) / 1 (invalid) / 2 (I/O error).

`explain` reads a spans manifest, resolves the source ID, prints the span,
and exits 0 (found) / 1 (not found) / 2 (I/O error).

## Boundary

Valence supports bounded local evidence replay for selected evidence surfaces.
It does not claim behavioral correctness, semantic equivalence, exhaustive
Rust coverage, pure-cache eligibility, cross-machine determinism,
cross-toolchain determinism, external consumer compatibility, mechanized
language soundness, or general HIR/THIR/MIR extraction.

## Quick start

```bash
nix develop                # enter devshell
cargo test --workspace     # run all tests
cargo build -p valence-cli # build the CLI
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
