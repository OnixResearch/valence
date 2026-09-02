## Design

1. Add a pure `proof_carrying_ir_lineage` profile over typed in-memory rows. Rows include source artifact ref, IR artifact ref, node or region id, proof-obligation id, transform step id, dialect id, sidecar artifact ref, verification role, and non-claims.
2. Bind identity with Valence-owned BLAKE3 canonical hashes and existing artifact-ref roles. External IR hashes may be preserved as metadata but cannot replace Valence identity.
3. Treat dialects as opaque named vocabularies. Valence validates naming, version, payload hash, and preservation metadata; it does not interpret dialect semantics.
4. Treat semantic diffs as recorded evidence unless a future profile-specific validator promotes a narrow property role. Unknown or ignored proofs remain recorded-only.
5. Keep parsing of tMIR text/binary/JSON/MessagePack, proof checking, solver calls, filesystem reads, and CLI presentation outside `valence-core`.
6. Add fixtures for valid lineage, unknown dialect roundtrip metadata, sidecar binding, stale sidecar rejection, cross-artifact substitution, role-promotion rejection, and semantic-equivalence overclaim rejection.

## Non-Goals

- No tMIR crate dependency, parser, serializer, semantic checker, Lean checker, solver adapter, or codegen bridge.
- No claim that Valence proves source-to-IR correctness, transform correctness, proof obligation discharge, compiler correctness, or semantic equivalence.
- No replacement for Kamacite source envelopes or Octet proof rails.