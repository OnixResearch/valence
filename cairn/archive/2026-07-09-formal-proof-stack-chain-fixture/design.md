# Design: Formal proof stack-chain fixture

## Shape

Add a pure Valence fixture suite for a canonical chain:

```text
Octet formal_proof receipt + proof card + provenance export
  -> Valence stack sidecar and graph report
  -> Cairn readiness input summary
```

The fixture is checked in as static JSON under a Valence testdata or fixture root. The pure validator consumes parsed values and returns a deterministic report with role counts, accepted typed edges, rejected edges, digest/link issues, and non-claim visibility. The CLI shell may expose the report later, but the initial proof surface is a pure core test over frozen inputs.

## Invariants

1. **Property evidence has an Octet-owned receipt.** A Valence graph node may carry `verification_role = property` only when the Octet export links it to a validated `formal_proof` receipt, proof card, and provenance record.
2. **Manual-review remains manual-review.** Imported Trellis references and other manual-review records stay `recorded_only` or boundary evidence unless Octet reruns the proof and emits a new receipt.
3. **Digest roles stay separate.** Receipt hashes, proof-card identities, sidecar byte hashes, binary hashes, and graph report hashes are distinct typed identities even when they use BLAKE3.
4. **Cairn handoff is bounded.** Valence emits a Cairn readiness input summary that states property, recorded-only, boundary, and unknown counts. Cairn decides lifecycle readiness; Valence only preserves stack semantics.
5. **Non-claims are required at every hop.** The chain report fails closed when any hop omits or weakens whole-program, verifier-soundness, semantic-equivalence, release-eligibility, or requirement-satisfaction non-claims.

## Positive fixture

The positive fixture includes one scoped Octet-owned formal-proof receipt, a matching proof card, a matching provenance record, a Valence sidecar, a graph report, and a Cairn readiness input summary. Validation passes only when every hash, typed edge, role, and non-claim matches.

## Negative fixtures

Negative fixtures cover:

- stale proof-card receipt identity;
- imported Trellis manual-review record promoted to `property`;
- missing or weakened non-claim text;
- binary digest mismatch;
- sidecar digest mismatch;
- unsupported edge kind;
- Cairn readiness summary counting `recorded_only` or `boundary` as verified.

## Validation

Focused validation should run:

```bash
nix develop --option builders '' -c cargo test -p valence-core formal_proof_stack_chain
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- validate --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate proposal formal-proof-stack-chain-fixture --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate design formal-proof-stack-chain-fixture --root .
nix run path:/home/brittonr/git/OnixResearch/cairn#cairn -- gate tasks formal-proof-stack-chain-fixture --root .
```

## Risks

- **Fixture drift**: mitigate with stable BLAKE3 identities and negative stale-hash fixtures.
- **Overclaiming release readiness**: keep readiness as Cairn-owned policy; Valence only emits an input summary.
- **Accidental live-repo dependence**: keep fast fixtures frozen and add live sibling probes only to an optional deep lane.
