## Context

Valence owns canonical Evidence IR identity, typed linkage, verification roles, span/provenance threading, and non-claim enforcement. Its reusable core is deliberately pure and validates already-loaded values; filesystem, process, publication, and operator behavior stay in shells. Existing adapter modules preserve external evidence semantics without inheriting producer claims.

Radicle COBs are signed Git-backed change DAGs identified by reverse-domain type names and object IDs. They provide local-first replication and causal history, but custom applications must define payload schemas, authorization, deterministic concurrent reduction, version migration, retention behavior, and claim boundaries. COB signatures authenticate entry origin; they do not establish that the author was authorized to attach, supersede, or revoke evidence, and they do not prove the attached artifact is true.

The integration therefore needs two identities and two authorities to remain visible: Radicle/Git OIDs identify protocol objects and entries, while Valence-owned BLAKE3 values identify evidence, attachments, profiles, materialized snapshots, and receipts. Cairn may later consume a saved snapshot as bounded policy input, but neither Valence nor Cairn should depend on live Radicle state during deterministic validation.

## Decisions

### Decision: Keep the integration in the Valence repository

**Choice:** Add a pure `cob_evidence` module to `valence-core` and a separate `crates/valence-radicle/` adapter crate. Do not create a sibling project in this change. r[valence.radicle_cob_evidence.pure_core] r[valence.radicle_cob_evidence.adapter_boundary]

**Rationale:** Attachment schema, digest domains, role preservation, and non-claims are Valence semantics and should version with Valence. A separate adapter crate prevents `radicle-cob`, Git storage, signing, and native dependency changes from entering `valence-core` or the existing `valence` CLI. A sibling project becomes justified only if the adapter later owns a daemon, node lifecycle, key custody, network orchestration, independent releases, or support for multiple Valence compatibility generations.

### Decision: Model attachments as target-keyed sidecar COBs

**Choice:** Use the custom type `org.onixresearch.valence.evidence`. Each root records a typed target reference containing repository RID, target COB type, target object ID, a BLAKE3 target key, schema/profile identity, and required non-claims. The Radicle `related` link points to the target object, while the typed payload repeats enough target information to validate type and repository scope. r[valence.radicle_cob_evidence.sidecar_profile]

**Rationale:** Built-in issue and patch reducers should not be extended with unknown Valence operations. A sidecar can relate to any existing COB type without changing it. Offline peers may concurrently create multiple physical sidecars for one target, so the logical sidecar is the deterministic union of every valid root with the same target key; duplicate physical roots are not silently discarded. r[valence.radicle_cob_evidence.convergence]

### Decision: Keep the attachment event algebra small and append-only

**Choice:** The initial profile supports typed attach, supersede, and revoke events over immutable attachment IDs. Attach events carry the target key, relationship from a closed policy vocabulary, Valence artifact reference, observed target-state identity, optional embedded-byte metadata, actor/signature facts, and non-claims. Supersede and revoke events reference an existing attachment ID and retain the historical bytes. Raw comments, labels, assignments, generic issue state, and bulk lint output remain in their owning systems. r[valence.radicle_cob_evidence.sidecar_profile] r[valence.radicle_cob_evidence.privacy_retention]

**Rationale:** A focused event model limits merge ambiguity and keeps the COB from becoming a second issue tracker. Bulk lint findings remain in producer receipts; only selected durable artifacts or dispositions are attached.

### Decision: Define profile and configuration in Nickel

**Choice:** Nickel contracts own the custom type name, schema/profile versions, relationship vocabulary, actor-operation roles, embedding limits, receipt field declarations, required non-claims, and generated runtime profile. Rust parses the checked-in generated profile and fails closed on stale, malformed, or unsupported policy identity. r[valence.radicle_cob_evidence.sidecar_profile] r[valence.radicle_cob_evidence.authority]

**Rationale:** These are reviewed configuration and policy surfaces rather than hidden code literals. Runtime Nickel execution is unnecessary; deterministic export/check produces the profile consumed by Rust.

### Decision: Preserve protocol and evidence digest domains

**Choice:** Repository RIDs, COB object IDs, COB entry OIDs, embedded Git blob/tree OIDs, and tip OIDs remain typed protocol identifiers. Valence artifacts, target keys, attachment IDs, profile identities, materialized-state hashes, and receipt hashes use BLAKE3. Validators reject a syntactically valid digest supplied in another role. r[valence.radicle_cob_evidence.digest_domains]

**Rationale:** Rehashing a Radicle identifier does not make it a Valence artifact identity, and equal byte strings must not erase their roles. The attachment ID is BLAKE3 over typed canonical attachment material and excludes display-only paths, aliases, timestamps, and the future COB entry OID. r[valence.radicle_cob_evidence.attachment_identity]

### Decision: Bind artifacts to COB entries without circular hashing

**Choice:** Validate and identify the Valence artifact first. Create a COB entry that references that stable ID and canonical hash, optionally embedding the exact artifact bytes. After the entry OID exists, emit a separate BLAKE3 binding receipt over the target, attachment ID, artifact identity, entry OID, optional embed OID, profile identity, and explicit non-claims. r[valence.radicle_cob_evidence.binding_receipt]

**Rationale:** Putting the future entry OID into the artifact that determines the entry creates an impossible identity cycle. A post-write binding receipt proves only that the presented identifiers and bytes were linked by the recorded operation.

### Decision: Treat attachments and snapshots as recorded-only linkage

**Choice:** The attachment event and the resulting COB snapshot use `VerificationRole::RecordedOnly`. The referenced artifact's original role remains visible but is never copied into the attachment's role. Unsupported role promotion, weakened non-claims, or claims that attachment authorizes a waiver, proves target correctness, or establishes release eligibility fail closed. r[valence.radicle_cob_evidence.role_policy]

**Rationale:** A `Property` artifact can remain a property artifact, but attaching it proves only a link. Radicle signature validity authenticates an actor key and is recorded separately from Valence verification roles.

### Decision: Separate signature validation from authorization

**Choice:** The shell verifies Radicle entry signatures and loads the reviewed actor-role profile. The pure core receives normalized signer facts, profile identity, and operation authorization inputs, then accepts only policy-authorized attach, supersede, or revoke events. Unknown, invalidly signed, or unauthorized events remain visible in rejected-event diagnostics and cannot affect accepted state. r[valence.radicle_cob_evidence.authority]

**Rationale:** Cryptographic authorship is necessary but insufficient. Future delegate, UCAN, or Basalt authorization adapters can produce normalized authorization inputs without changing the evidence identity model.

### Decision: Make concurrent reduction deterministic without timestamp authority

**Choice:** Valid attachments form a set keyed by attachment ID; exact replay is idempotent. Causal supersede/revoke events apply only to known predecessors. Concurrent incompatible supersessions, revocations, or actor-policy outcomes produce explicit conflict rows and do not silently choose a winner by wall clock, traversal order, or hash order. Inputs and diagnostics are canonically sorted before hashing. r[valence.radicle_cob_evidence.convergence]

**Rationale:** Radicle can provide causal traversal, but application semantics must define convergence. Timestamps are display metadata and must not become hidden authority.

### Decision: Support bounded embeds and reference-only attachments

**Choice:** Small artifacts may be embedded when their measured bytes, Git embed OID, declared BLAKE3 hash, and Valence canonical hash agree and the generated embed policy permits their size and media class. Large or sensitive evidence uses a typed artifact reference and verified registry locator. Missing bytes, tampering, undeclared embeds, unsafe locators, or policy-limit violations fail closed. r[valence.radicle_cob_evidence.payload_modes]

**Rationale:** COB history should not absorb every evidence bundle. Reference-only mode keeps replication bounded, while exact embeds permit portable small evidence without weakening content identity.

### Decision: Export saved snapshots for downstream consumption

**Choice:** `valence-radicle` materializes local COB history into a deterministic snapshot artifact and binding receipts. The snapshot records target identity, observed target tips/state identity, all contributing sidecar object IDs and tips, accepted attachments, rejected events, conflicts, actor-policy/profile identity, materialized-state hash, receipt hash, and non-claims. Cairn and other consumers read the saved artifact; they do not query live Radicle refs during a gate. r[valence.radicle_cob_evidence.snapshot_handoff]

**Rationale:** Saved, hash-bound inputs preserve Valence's replay model and Cairn's deterministic core/shell boundary. A later Cairn policy may require freshness or authorized disposition, but a snapshot does not itself waive a lint or establish lifecycle readiness.

### Decision: Keep mutation local, preview-first, and jj-safe

**Choice:** Attach, supersede, and revoke commands produce a deterministic plan by default and update only the selected local Radicle COB refs after explicit execution. They must not update `refs/heads`, tags, the jj working-copy commit, tracked files, or remote/network state. Read/snapshot/verify commands are read-only. r[valence.radicle_cob_evidence.repository_safety]

**Rationale:** Valence uses a colocated jj/Git repository. Direct special-ref access needs dedicated positive and negative tests so ordinary Valence development remains unchanged.

## Functional Core / Imperative Shell

The pure core accepts parsed profile data, normalized target references, measured evidence metadata, normalized signer/authorization facts, and ordered or unordered event sets. It returns validation reports, reduced logical state, conflict rows, binding-receipt material, and snapshot material without reading Git, files, environment, clocks, keys, or network state.

The `valence-radicle` shell opens the selected repository, reads and writes Radicle COB refs, verifies signatures, measures embedded bytes, loads generated profile data, confines artifact paths, signs explicit mutations, and renders or writes receipts. Shell errors become typed inputs or deterministic diagnostics before core decisions are made.

## Risks / Trade-offs

- `radicle-cob` is a pre-1.0 dependency with evolving APIs. Exact pinning and an internal adapter boundary reduce but do not eliminate upgrade cost.
- Custom COBs are invisible to many ordinary forge interfaces and branch-only clones. They remain optional collaboration/evidence transport, not canonical Valence or Cairn source.
- Multiple offline sidecar roots increase discovery and reduction work. Grouping by a BLAKE3 target key preserves convergence without pretending root uniqueness.
- Replicated Git objects are difficult to erase reliably. Revocation and supersession change accepted state but do not promise historical-byte deletion; secret or regulated evidence must not be embedded.
- Radicle private repositories use selective replication and are not an at-rest encryption guarantee. Operators must apply separate confidentiality policy.
- Signer authorization can drift as delegate or capability policy changes. Every snapshot binds the reviewed actor-policy/profile identity and must report stale or unknown policy rather than reinterpret history silently.
- An attached artifact can become stale when the target or source changes. Snapshots expose observed target state and freshness inputs; downstream policy decides whether staleness blocks consumption.
- The adapter adds Git and cryptographic dependencies to workspace checks. Keeping it out of `valence-core` and the existing CLI limits consumer impact but does not remove CI cost.

## Deferred Work

- Radicle node lifecycle, gossip, seeding, peer discovery, remote fetch, and availability guarantees.
- Key custody, signing agents, hardware-backed keys, and interactive authorization UX.
- A daemon, TUI, web forge, or general issue/patch replacement.
- Automatic creation of one COB per raw lint finding.
- Cairn mutation driven directly by live COB state.
- Cross-repository or public rollout before the Valence pilot proves convergence, backup/restore, jj safety, and bounded operational cost.
