## Design

Valence models state-machine exhaustiveness as an Evidence IR receipt family produced by Octet. The receipt is an identity and boundary object, not a source analyzer. Octet remains responsible for rustc facts, lint diagnostics, artifact verification, and export preconditions.

## Receipt model

The state-machine receipt carries:

- schema and receipt kind for Octet state-machine exhaustiveness evidence;
- Octet producer identity and checker identity;
- transition-core source span and stable source identifier;
- state type identity, event type identity, and variant inventory digest;
- coverage summary digest for explicit state/event handling;
- status and deterministic diagnostic codes;
- artifact BLAKE3 identity and parent Octet export identity;
- evidence tier for bounded source-shape checking;
- optional links to separate Verus or Kani receipts for semantic invariants;
- required non-claims.

The coverage summary digest binds the exported matrix facts without requiring Valence to reconstruct Rust HIR or check pattern exhaustiveness. If the summary is absent, malformed, or stale against the exported artifact identity, validation fails closed.

## Validation core

The pure validation core checks parsed receipt values only. It validates required fields, known schema strings, known evidence tier, BLAKE3 digest syntax, source-span presence, parent linkage, non-claim coverage, status/diagnostic consistency, and overclaim text. CLI and adapter shells may read JSON files and render reports, but they must route decisions through the pure validator.

## Graph and proof links

A state-machine receipt may link to formal proof receipts for invariants such as transition preservation or impossible-state exclusion. Those proof receipts keep their own verifier identity and tier. The state-machine lint receipt remains bounded source-shape evidence even when graph-linked to formal proof evidence.

## Validation

Focused validation should run:

```bash
cargo test -p valence-core state_machine_evidence_boundary
cargo fmt --check
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate proposal state-machine-evidence-boundary --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate design state-machine-evidence-boundary --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate tasks state-machine-evidence-boundary --root .
```

## Risks

- **Overclaim through evidence graph links**: require non-claims and preserve proof tiers per linked receipt.
- **Schema drift between Octet and Valence**: bind schema and checker vocabulary through deterministic fixtures and digest checks.
- **False completeness claims**: reports must say Valence validates receipt shape and linkage only.
