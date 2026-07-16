## Context

`formal_proof_chain` currently validates a frozen direct proof stack chain centered on one Octet receipt. It intentionally does not execute proof tools. A translation-based provider adds a new linkage problem, not a new execution responsibility: the final theorem must be attached to the exact model generated from the exact LLBC generated from the exact selected Rust source.

The profile must remain provider-aware. A generic list of files and hashes would permit illegal stage ordering, digest-role substitution, and property-role promotion of translation-only artifacts.

## Decisions

### 1. Extend the formal-proof chain instead of creating a second evidence authority

**Choice:** Add a versioned provider profile to the existing formal-proof chain model. The common chain retains Octet receipt, proof card, provenance, role summary, and non-claims. The `aeneas-lean` payload adds closed typed stage and edge records.

**Rationale:** Valence should have one formal-proof linkage authority while preserving provider-specific invariants.

### 2. Use a closed stage vocabulary for the initial profile

**Choice:** The Aeneas profile recognizes these artifact roles:

- checked Rust source selection;
- effective Charon extraction contract and target metadata;
- Charon/rustc extraction observation;
- LLBC artifact;
- Aeneas translation observation;
- measured `translation.json` manifest;
- generated Lean definitions and generated-file inventory;
- external Lean model, assumption, axiom, or opaque-definition artifact;
- hand-maintained theorem source;
- theorem approval, claim-class, adequacy, and trust-closure decision;
- Lean package/toolchain lock;
- Lean checker receipt; and
- Octet proof receipt/proof card/provenance projection.

Each stage has a stable ID, typed artifact role, BLAKE3 content identity where Valence owns identity, producer/tool identity, verification role, status, optional source span, and non-claims.

**Rationale:** Closed roles make missing and swapped stages diagnosable and keep generated definitions distinct from theorem evidence.

### 3. Validate exact typed edges

**Choice:** Edges use a closed vocabulary such as `configured-by`, `extracted-from`, `declared-in-manifest`, `generated-as`, `translated-from`, `modeled-by`, `approved-as`, `trusts`, `theorem-over`, `checked-with`, and `attested-by`. Every required stage has exact incoming/outgoing obligations. Edge endpoints bind typed node identities rather than display paths. Duplicate edges, dangling nodes, skipped required stages, cycles, and role-incompatible edges fail closed.

**Rationale:** Matching hashes in an unordered bag do not establish which extraction policy produced LLBC, which declarations became Lean definitions, or which approved statement was checked.

### 4. Keep translation declaration IDs run-local

**Choice:** Manifest `def_id` values may join one frozen LLBC artifact to one frozen generated declaration inventory only. Canonical chain identity and durable node IDs use typed BLAKE3 identities over source bytes/spans, reviewed provider and extraction profile, generated model, maintained theorem source, and receipt material. A `def_id` cannot identify an artifact across runs or profiles.

**Rationale:** Tool-local numbering is useful linkage evidence but is not stable stack identity.

### 5. Count only the final Octet property receipt

**Choice:** Extraction contracts, LLBC, translation manifests, generated Lean, translation logs, models, adequacy decisions, trust-closure inventories, and package locks are `recorded_only` or `boundary`. A property role is accepted only for the final Octet-owned receipt when Octet artifact verification passed; the extraction contract is reviewed and non-errorful; the theorem ID, approved statement BLAKE3, requirement ID, claim class, adequacy decision, and trust closure match their nodes and proof card; and all required chain links validate. A passing Lean status or adequacy label imported without an Octet receipt remains non-property.

**Rationale:** Valence validates identity and role policy; it does not take proof execution authority from Octet.

### 6. Compute canonical chain identity with role-separated BLAKE3

**Choice:** Canonical material includes profile/schema, sorted typed nodes, sorted typed edges, effective extraction contract, translation-manifest identity, generated-file inventory, receipt/proof-card identities, theorem/property ID, approved statement BLAKE3, requirement ID, claim class, adequacy/trust-closure identities, role summary, and required non-claims. Display paths, prose labels, and run-local `def_id` values remain review/link metadata rather than durable identity inputs. Digest parsing preserves source, extraction profile, LLBC, translation manifest, generated model, external model, theorem, adequacy decision, receipt, sidecar, and chain domains even if bytes happen to share a digest.

**Rationale:** Same digest bytes do not erase semantic role, and path churn must not silently redefine chain identity.

### 7. Keep the core pure and the CLI read-only

**Choice:** Core validation operates only on parsed in-memory values. The CLI opens an operator-declared artifact root, reads one adapter input and local artifacts through confined paths, measures bytes, invokes the core, and renders JSON or human reports. It does not spawn Charon, Aeneas, Lean, Octet, or Cairn and does not fetch sibling fixtures.

**Rationale:** This preserves the functional-core/imperative-shell boundary and Valence's evidence-only authority.

### 8. Freeze the producer fixture

**Choice:** The positive fixture is copied from an Octet adapter export after Octet artifact verification. It records the source Octet revision and exact fixture BLAKE3. Ordinary tests use only checked-in fixture bytes. A separate opt-in stack smoke may compare a live producer output but cannot replace frozen tests.

**Rationale:** Valence tests must be hermetic and must not infer live sibling freshness.

### 9. Reuse generic downstream boundaries

**Choice:** The report emits the existing property/recorded-only/boundary summary and a canonical chain identity. Cairn consumes that summary through generic evidence-chain readiness; Mantle carries the report and source receipt through generic external evidence. No Aeneas-specific Cairn or Mantle command is introduced.

**Rationale:** Provider details belong at execution and evidence-semantics layers, not in generic lifecycle or bundle-local transport.

## Risks / Trade-offs

- The first stage vocabulary may be too Aeneas/Lean-specific for later HOL4, F*, or Coq providers. New providers should add reviewed profiles rather than weakening the closed initial contract.
- Octet receipt evolution can stale the frozen fixture. Fixture refresh must be explicit and retain old compatibility coverage when schemas remain supported.
- Canonical graph identity can appear stronger than it is. Every report must state that identity/linkage validation does not establish translation, theorem soundness, or theorem adequacy.
- Charon `def_id` values can drift between otherwise comparable runs. Valence preserves them only as frozen-run joins and rejects their use as durable identities.
- Octet may strengthen its adequacy policy over time. Valence binds the exact imported decision and policy identity but does not independently decide whether the selected product requirement is sufficient.
- Requiring all intermediate identities increases receipt size, but omitting them would leave the central stale-artifact risk unchecked.
