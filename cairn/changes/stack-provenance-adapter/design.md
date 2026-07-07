## Design: stack provenance adapter

### Ownership

Valence owns the stack-specific interpretation of provenance chains. Mantle only bundles optional external evidence. The dependency direction is:

```text
Valence adapter reads Mantle release evidence as data
Mantle never links to Valence adapter code
```

The adapter may parse Octet, Trellis, Cairn, and Mantle artifacts, but those parsers live behind Valence-owned adapter boundaries.

### Adapter artifact

The adapter emits a canonical sidecar with schema `valence.stack-provenance-adapter.v1`. The sidecar contains:

- the validated `valence.provenance-chain.v1` chain;
- source artifact identities used to build the chain;
- evidence-tier and non-claim summaries;
- Mantle release bundle identity and bundled sidecar digest when verifying a release;
- diagnostics for missing, stale, manual-review-only, or overclaimed links.

The existing `ProvenanceChain` remains the inner identity/linkage model. The adapter sidecar records enough surrounding evidence to audit how the chain was assembled.

### Build flow

Example operator flow:

```bash
valence stack-provenance build \
  --octet-artifacts artifacts/octet \
  --trellis-bundle docs/proof-evidence-bundle.json \
  --cairn-receipts target/cairn-receipts \
  --mantle-release target/release-evidence/<id> \
  --out stack-provenance.json
```

The CLI shell reads files and computes BLAKE3 identities. The core build function receives normalized records and returns a deterministic sidecar plus diagnostics.

### Verify flow

Example stack gate:

```bash
valence stack-provenance verify \
  --mantle-release target/release-evidence/<id> \
  --role stack-provenance-trace
```

Verification checks:

- the Mantle bundle has an external evidence entry with the requested role;
- the bundled sidecar bytes match the Mantle-recorded BLAKE3 digest;
- the sidecar schema and non-claim boundary match Valence constants;
- each binary hash in the provenance chain matches a bundled Mantle binary or an explicitly scoped release artifact;
- source/function/requirement IDs are non-empty and duplicate-safe;
- parent artifact identities exist when the sidecar claims derived evidence;
- evidence tiers are not promoted, especially `manual_review_required` to `formal_proof`;
- imported Trellis proof bundles remain manual-review unless accompanied by an Octet-owned rerun receipt.

### Functional core and imperative shell

Core functions accept normalized structs and bytes/digests. CLI code owns:

- filesystem reads;
- walking artifact directories;
- rendering JSON diagnostics;
- exit codes.

### Non-claims

The adapter verifies traceability metadata. It does not prove behavioral correctness, semantic equivalence, business-domain validity, whole-program safety, verifier soundness, or release eligibility by itself.
