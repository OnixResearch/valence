## Context

The current stack-provenance adapter proves identity and linkage across normalized metadata:

```text
Octet verified provenance export
  -> Valence stack sidecar
  -> Mantle external-evidence role/digest metadata
```

That path is necessary but not sufficient for the operator question: "did this source/provenance thread reach this built binary?" The missing step is a verifier path that measures the built artifact bytes in the same run and rejects any mismatch between those bytes, the sidecar's `binary_hash`, the provenance links, and the Mantle release manifest's binary identity.

## Decisions

### 1. Valence owns the built-binary semantic verifier

**Choice:** Implement the built-binary thread verifier in Valence core with a thin CLI shell.

**Rationale:** Mantle must remain generic and should not learn Valence/Octet/Trellis/Cairn semantics. Octet should export verified provenance facts for its own artifacts. Valence is the correct boundary for stack-specific semantic verification because it can interpret the sidecar, Octet export, and Mantle external-evidence metadata without making Mantle stack-aware.

### 2. The pure core accepts bytes and parsed DTOs

**Choice:** Keep the core verifier pure by accepting binary bytes, parsed sidecar/export/manifest DTOs, and returning a deterministic report. The CLI reads files, parses JSON, computes process exit codes, and prints diagnostics.

**Rationale:** This preserves the functional-core / imperative-shell boundary. The core can hash the supplied bytes with BLAKE3 and compare identities without performing filesystem or process I/O.

### 3. The accepted claim remains identity and linkage only

**Choice:** The successful report may say the measured binary bytes are linked to the sidecar and release metadata. It must not say the binary satisfies requirements, is behaviorally correct, is semantically equivalent to source, is whole-program safe, or is release-eligible by itself.

**Rationale:** The prior bounded provenance model is intentionally narrow. The built-binary check closes a missing identity edge; it does not turn evidence linkage into correctness proof.

### 4. The E2E check must include stale-byte and stale-metadata negatives

**Choice:** Add positive and negative tests in the same test surface. At minimum, failures must cover changed binary bytes, mismatched sidecar binary hash, missing Mantle binary identity, stale sidecar digest, and tier-promotion or correctness overclaim attempts.

**Rationale:** The highest-risk failure mode is accidental acceptance of the wrong binary or overbroad wording. Negative tests make the verifier fail closed at each boundary.

## Risks / Trade-offs

- Mantle manifest binary identity field names may evolve. The Valence manifest view should parse only the stable fields it needs and fail closed when the binary identity is absent.
- Building a real binary in every unit test may be too expensive. The implementation should keep pure unit tests over byte fixtures and add one focused integration path that uses the smallest practical build artifact.
- The report will be stronger than today's sidecar digest check but still bounded to byte identity and linkage. Documentation and diagnostics must keep that non-claim visible.
