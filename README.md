# Valence

Valence is the canonical evidence identity and validation layer for the Onix
trust stack. It defines an **Evidence IR**: a serializable, type-checked format
that gives evidence artifacts stable BLAKE3 identities, preserves source spans
and provenance, and keeps verification claims within explicit boundaries.

In plain language, Valence lets tools agree that they are referring to the same
source, receipt, sidecar, or binary—and checks that those references have not
changed shape, identity, role, or explicit claim boundary.

Valence is an intermediate representation, not a programming language. It has
no compiler or code-generation backends and does not claim mechanized language
soundness.

> [!IMPORTANT]
> Passing Valence validation establishes only the syntax, canonical identity,
> typed linkage, role, digest-domain, provenance, or non-claim facts checked by
> the selected profile. It does not establish that the evidence is true or that
> the subject is correct, safe, verified, or ready for release.

## Why Valence exists

Evidence moves through several tools before it reaches a release or lifecycle
decision:

```text
source → evidence artifact → receipt or sidecar → release bundle → lifecycle gate
          └──────────── stable identities and typed links ────────────┘
```

Without a shared identity layer, each handoff can reinterpret hashes, roles,
spans, or claims. Valence supplies canonical forms and fail-closed validators so
those handoffs remain explicit and reviewable.

| Project | Relationship to Valence |
|---|---|
| [Octet](https://github.com/OnixResearch/octet) | Produces source-derived evidence and retains source scanning, proof-rail orchestration, and release ownership for its surfaces. |
| Valence | Owns Evidence IR identity, linkage, role vocabulary, provenance, and non-claim validation. |
| [Trellis](https://github.com/OnixResearch/trellis) | Supplies verified-logic building blocks and proof-artifact identity references. |
| [Mantle](https://github.com/OnixResearch/mantle) | Records bounded Valence evidence and requirement identities in release bundles. |
| [Cairn](https://github.com/OnixResearch/cairn) | Consumes saved Valence reports as bounded lifecycle-policy input. |

## Quick start

With Nix flakes enabled:

```bash
# Show the CLI surface.
nix run .#valence -- --help

# Enter the development environment and test the workspace.
nix develop -c cargo test --workspace
```

The CLI uses explicit capability roots. A file locator must be relative to its
role-specific root:

```bash
nix run .#valence -- check \
  --artifact-root /srv/evidence \
  artifact.json
```

Absolute locators, parent traversal, omitted roots, and root/role mismatches
fail before the file is opened. See [CLI capability roots](docs/cli-capability-roots.md).

## Core model

An Evidence IR v1 `EvidenceArtifact` carries:

| Field | Purpose |
|---|---|
| `schema` and `artifact_kind` | Select the versioned envelope and payload meaning. |
| `stable_id` and `canonical_hash` | Bind canonical typed content to a stable BLAKE3 identity. |
| `source_span` | Preserve source identity, byte range, and line/column coordinates. |
| `verification_role` | Distinguish verified properties from recorded or boundary evidence. |
| `payload` | Carry kind-specific data inside the hash boundary. |
| `non_claims` | State what the artifact explicitly does not establish. |

Verification roles are intentionally separate:

| Role | Meaning |
|---|---|
| `Property` | Eligible to count as verified under the applicable Valence/Cairn profile. |
| `RecordedOnly` | Recorded identity or linkage evidence; it does not count as verification. |
| `Boundary` | A claim or authority boundary; it does not count as verification. |

The control-plane envelope is closed: unknown or duplicate control fields fail
before hash and claim-policy checks. The kind-specific payload remains open but
fully hash-bound. See the [artifact envelope](docs/evidence-artifact-envelope.md)
and [span integrity](docs/evidence-span-integrity.md) specifications.

## What validation means

| Valence can establish | Valence does not establish |
|---|---|
| Canonical content identity and accepted BLAKE3 spelling | Evidence truth or requirement satisfaction |
| Schema, envelope, and declared metadata consistency | Behavioral correctness or semantic equivalence |
| Typed links, digest roles, spans, and provenance continuity | Verifier soundness, runtime authority, or release eligibility |
| Verification-role separation and bounded role summaries | Exhaustive Rust coverage or general HIR/THIR/MIR extraction |
| Required non-claims and rejection of selected overclaims | Whole-program safety, pure-cache eligibility, cross-machine/toolchain determinism, or external compatibility |

Producers still own source analysis and artifact creation. Shells still own file
access, process execution, byte measurement, publication, and operator policy.
Downstream consumers must apply their own freshness, authorization, lifecycle,
and release rules.

## Components

| Component | Purpose |
|---|---|
| [`valence-core`](crates/valence-core/README.md) | MPL-2.0 functional core for deterministic validation over already-loaded values. Its README lists every public module. |
| `valence` | AGPL CLI shell for capability-confined reads, writes, measurements, validation, and reports. |
| [`valence-radicle`](crates/valence-radicle/README.md) | AGPL preview-first adapter for recorded-only evidence links in signed local Radicle Collaborative Objects. |
| `contracts/` and `cairn-policy/` | Nickel-authored schemas and lifecycle compatibility inputs with positive and negative fixtures. |
| `docs/`, `fixtures/`, and `evidence/` | Profile specifications, frozen producer inputs, migration guidance, and retained evidence. |

The reusable core covers five broad areas:

- Evidence IR envelopes, artifact references, canonical identity, and role registries.
- Stack provenance, Preserves bridges, evidence chains, and source-span linkage.
- Function-address, Wasm component, ast-grep, Kani, Trellis, and formal-proof profiles.
- Evidence models, indexes, capability plans, deterministic harnesses, and Hegel properties.
- Radicle COB reduction and standalone artifact-auth mapping over already verified observations.

## Use `valence-core`

Packages are distributed from reviewed Git revisions and are intentionally not
published to crates.io. Pin an exact public revision:

```toml
[dependencies.valence-core]
git = "https://github.com/OnixResearch/valence"
rev = "<reviewed-commit>"
package = "valence-core"
```

Rust imports the package as `valence_core`.

The core has no filesystem, environment, process, clock, network, or terminal
responsibilities. Passing one of its validators grants only the bounded claims
documented for that validator; it does not transfer producer, verifier,
runtime, lifecycle, or release authority.

## CLI

```text
valence check --artifact-root <ROOT> <FILE>
valence explain --manifest-root <ROOT> --id <ID> --manifest <FILE>
valence stack-provenance build --artifact-root <ROOT> --octet-export <FILE> --binary-hash <B3> --output-root <ROOT> --output <FILE>
valence stack-provenance verify --manifest-root <ROOT> --manifest <FILE> --sidecar-root <ROOT> --sidecar <FILE>
valence stack-provenance verify-built-binary --binary-root <ROOT> --binary <FILE> --artifact-root <ROOT> --octet-export <FILE> --sidecar-root <ROOT> --sidecar <FILE> --manifest-root <ROOT> --manifest <FILE> [--json]
valence stack-provenance graph-report --artifact-root <ROOT> --chain <FILE> [--format json|markdown]
valence function-address validate --artifact-root <ROOT> --input <FILE> [--output-root <ROOT> [--bundle <FILE>] [--receipt <FILE>] [--graph <FILE>]]
valence function-address preserves --artifact-root <ROOT> --input <FILE>
valence formal-proof aeneas-chain validate --artifact-root <ROOT> --input <FILE>
valence evidence-chain validate --artifact-root <ROOT> --input <FILE>
valence wasm-component validate --artifact-root <ROOT> --input <FILE>
valence version
```

Successful validation exits with status `0`; invalid evidence or a missing span
uses `1`; usage and I/O failures use `2`. Run `nix run .#valence -- --help` for
the authoritative command summary.

### Radicle sidecars

`valence-radicle` records optional collaboration state without changing the
Evidence IR or Cairn sources of truth:

```text
valence-radicle authorization ...
valence-radicle mutate ...                  # preview; no writes
valence-radicle mutate ... --execute        # sign and update one local COB ref
valence-radicle snapshot ...
valence-radicle verify-binding ...
valence-radicle verify-snapshot ...
```

It does not run a Radicle node, fetch peers, manage private keys, mutate Cairn
lifecycle files, or promote recorded evidence into proof. See
[Radicle COB evidence sidecars](docs/radicle-cob-evidence.md).

## Documentation by task

- **Identity and envelopes:** [artifact envelope](docs/evidence-artifact-envelope.md), [span integrity](docs/evidence-span-integrity.md), and [Preserves migration](docs/preserves-integrity-migration.md).
- **Evidence profiles:** [generic chains](docs/evidence-chain-profile.md), [function addresses](docs/function-address-evidence-profile.md), [Preserves ingest](docs/function-address-preserves-ingest.md), [Wasm components](docs/wasm-component-evidence.md), [ast-grep spans](docs/ast-grep-evidence-spans.md), [Kani boundaries](docs/kani-evidence-boundaries.md), and [FSM receipts](docs/fsm-default-exhaustive.md).
- **Proof-oriented adapters:** [Aeneas/Lean chains](docs/aeneas-translation-chain.md), [Trellis evidence](docs/trellis-proof-evidence.md), and [proof-carrying IR lineage](docs/proof-carrying-ir-lineage.md).
- **Operations and policy:** [CLI roots](docs/cli-capability-roots.md), [function-address publication](docs/function-address-publication.md), [Radicle sidecars](docs/radicle-cob-evidence.md), and [Cairn compatibility](docs/cairn-policy-compatibility.md).
- **Testing and pilots:** [testing harness](docs/testing-harness.md), [strict Rust admission](docs/octet-deterministic-core.md), and the rejected [build-graph pilot](docs/build-graph-pilot.md).

## Development and validation

The repository pins Rust nightly through `rust-toolchain.toml`. Common checks:

```bash
nix develop
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

# Full Rust, Nickel, fixture, and Nix validation.
nix flake check -L
```

Focused fast, deep, Hegel, and nightly lanes are documented in the
[testing harness guide](docs/testing-harness.md). Nickel contracts under
`contracts/` provide static preflight checks; Rust remains responsible for
runtime parsing, capability-confined I/O, and measured-byte identity.

### artifact-auth source transport

Cargo and Nix resolve `artifact-auth-core` and `artifact-auth-ed25519` from the unified [`OnixResearch/onix-artifact`](https://github.com/OnixResearch/onix-artifact) workspace. Both tools pin revision `e41340bec587b6d049b5cc518ec7db925dde84be`. The typed [migration receipt](evidence/source/artifact-workspace-migration-v1.ncl) binds the source, package graph, byte parity, rollback, and non-claims. The prior [Radicle receipt](evidence/radicle/artifact-auth-cutover-v1.ncl) is historical evidence only. Valence does not select a second source if the admitted revision is unavailable.

## Project status and compatibility

Valence is under active development. Explicitly documented schema and identity
versions are compatibility surfaces; other APIs may evolve.

`valence-core`, `valence-cli`, and `valence-radicle` set `publish = false` and
are distributed from reviewed Git revisions. Any future registry release or
broader compatibility promise requires a separate review.

## License

| Surface | License |
|---|---|
| `crates/valence-core/**` | MPL-2.0 |
| CLI, Radicle adapter, executable pilots, and repository documentation | AGPL-3.0-or-later |
| Dependencies and upstream-derived material | Their stated upstream terms |

Complete texts are in [`LICENSES/`](LICENSES/), with the package map and
boundary details in [the licensing guide](docs/licensing.md). Licensing metadata
is outside canonical Evidence IR identity unless a versioned schema explicitly
includes it.

## References

### Onix stack

- [OnixResearch/octet](https://github.com/OnixResearch/octet) — source of the extracted core modules and producer of selected Valence evidence surfaces.
- [OnixResearch/cairn](https://github.com/OnixResearch/cairn) — lifecycle engine that consumes Valence evidence as bounded policy input.
- [OnixResearch/trellis](https://github.com/OnixResearch/trellis) — verified-logic building blocks and proof-artifact identity references.
- [OnixResearch/mantle](https://github.com/OnixResearch/mantle) — build system that records bounded Valence and requirement identities in release bundles.

### Identity and data-model influences

- [Unison: The Big Idea](https://www.unison-lang.org/docs/the-big-idea/) — design reference for content-addressed definitions, names as metadata, durable typed values, effect labels, and structured refactoring. Valence does not adopt the Unison language or runtime.
- [oxidecomputer/iddqd](https://github.com/oxidecomputer/iddqd) — reference for derived-key maps, duplicate-key rejection, borrowed lookup, and property-tested unsafe boundaries.
- [oxidecomputer/newtype-uuid](https://github.com/oxidecomputer/newtype-uuid) — reference for typed external UUID wrappers; Valence-owned canonical identity remains BLAKE3.
- [oxidecomputer/daft](https://github.com/oxidecomputer/daft) — reference for semantic diffs used as review evidence, not semantic-equivalence proof.

### Attestation and proof influences

- [oxidecomputer/rats-corim](https://github.com/oxidecomputer/rats-corim), [oxidecomputer/vm-attest](https://github.com/oxidecomputer/vm-attest), [oxidecomputer/dice-util](https://github.com/oxidecomputer/dice-util), and [oxidecomputer/pki-playground](https://github.com/oxidecomputer/pki-playground) — bounded attestation vocabulary and certificate-chain references whose claims do not transfer to Valence.
- [joweeba/tMIR](https://github.com/joweeba/tMIR) — proof-carrying IR lineage and proof-sidecar reference; Valence remains an evidence IR rather than a compiler IR or proof checker.
- [AeneasVerif/aeneas](https://github.com/AeneasVerif/aeneas) — Rust/Charon/LLBC/proof-assistant translation-chain reference; Valence validates bounded identities, links, roles, and non-claims.
- [AeneasVerif/eurydice](https://github.com/AeneasVerif/eurydice) — deferred Rust-to-C lineage reference. Valence requires a concrete Octet producer schema before it adds a profile.
- [AeneasVerif/scylla](https://github.com/AeneasVerif/scylla) — deferred experimental C-to-Rust lineage reference. Later Rust evidence does not establish source-C equivalence.

### Evaluated but not adopted

- [d-tietjen/build-graph](https://github.com/d-tietjen/build-graph) — rejected advisory-navigation pilot at reviewed revision `68a12dcac9b2475e27b6062f974b5f37321d3c4f`; see the [pilot report](docs/build-graph-pilot.md).
