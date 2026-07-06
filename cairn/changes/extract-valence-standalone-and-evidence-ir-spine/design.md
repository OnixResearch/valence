## Decision: bounded Evidence IR, not a language

The strategic decision (researched and confirmed) is that valence becomes a
typed Evidence IR — a canonical, serializable, type-checked intermediate
representation — not a general-purpose language. A full language would
abandon the stack's "augment existing languages" thesis, duplicate Verus (which
is already the verification language via trellis), fail to obtain mechanized
soundness merely by inventing syntax, and freeze all other improvements behind a
multi-year build.

The bounded Evidence IR gets canonical-evidence-like discipline (canonical machine
form + human projection + span survival through the chain) without a compiler,
without backends, and without mechanization. It emits canonical evidence plus
a `.spans.json` sidecar; valence's Function Object v1 already supplies a stable
function identity.

## Decision: repo split, not josh

The strategic decision (researched and confirmed) is to split valence-core
into its own repo rather than use josh projection/sync. josh's model is one
central monorepo with filtered views; our stack is five independently-versioned
repos with independent release-evidence bundles. Even with `:rev(...)`/`:pin`
commit pinning, josh adds a filter-trust surface and a two-step pin where
git/flake is one-step. valence-core is ~10 source files; splitting is trivial.

## Architecture

```
crates/valence-core/   Pure Evidence IR core (no I/O, no filesystem, no process)
crates/valence-cli/    CLI shell (arg parsing, file reads, stdout/stderr, exit codes)
cairn/                 Native Cairn lifecycle (specs, changes, archive)
cairn-policy/          Nickel contracts + generated JSON policy
```

### Functional core / imperative shell boundary

`valence-core` is pure deterministic core only. It MUST NOT contain filesystem
reads, environment reads, process execution, clocks, async, `println!`, or
`eprintln!`. `valence-cli` owns the imperative shell: argument parsing, file
reads, stdout/stderr, and exit codes. All core logic lives in `valence-core`.

### Evidence IR v1 data model

`EvidenceArtifact` is the unified canonical form:

- `schema`: always `valence.evidence-ir.v1`
- `artifact_kind`: string identifying the kind
- `stable_id`: the ID that threads through the chain
- `canonical_hash`: BLAKE3 of canonical JSON (excludes `canonical_hash` and
  `source_span` so identity is stable across relocation)
- `source_span`: `Option<EvidenceSpan>` — provenance back to source
- `verification_role`: `Property` | `RecordedOnly` | `Boundary`
- `payload`: kind-specific JSON
- `non_claims`: explicit non-claim boundary strings

`EvidenceSpan` carries source linkage through the stack:

- `source_id`: stable identifier threaded through the chain
- `source_path`, byte offsets, line/column positions, optional label

### Verification role

Schema lands in Phase 0; enforcement (counting only `Property` as verified)
lands in Phase 1 via cairn `release-readiness`. A "0 passed, 0 failed" on
recorded-only artifacts
means "nothing was verified," not "all good."

### Checker

`check_evidence_artifact` validates: schema field, artifact_kind non-empty,
stable_id non-empty, canonical_hash matches recomputed hash, non_claims present
and not weakened, verification_role valid, no overclaim markers. Fails closed
on any issue.

### Explainer

`explain_span` resolves a stable source ID to its span from a JSON manifest
array of `EvidenceSpan` objects.

## Stable v1 contract preservation

The existing `function_object`, `evidence_codebase_index`, and `evidence_index`
modules are copied verbatim. The stable v1 contract strings
(`octet.function-object.v1`, `octet-source-token-v1`,
`octet-evidence-codebase-index/v1`, etc.) remain byte-identical. The canonical
JSON hash construction is unchanged. This preserves existing object corpus
hashes and cross-repo compatibility.

## What is explicitly NOT provided

- Mechanized language soundness (external-evidence layer 1). The stack's guarantees rest
  on "Verus/Z3 checked this program against this spec" and "Slotcar checked
  this core is pure," not on a mechanized proof that the language itself is
  sound. That layer is out of scope for this change and the broader plan.
- A new general-purpose language, a compiler, or numerical backends.
- Whole-program correctness, semantic equivalence, cross-toolchain
  determinism, or exhaustive coverage.
