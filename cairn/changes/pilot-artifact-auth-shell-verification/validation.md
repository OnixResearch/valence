# Validation evidence

## Baseline

Before core changes, `nix develop -c cargo test -p valence-core artifact_auth` and `nix develop -c cargo test -p valence-radicle` pass. The core baseline covers protocol/standalone mapping and drift; the shell baseline covers real Radicle COB round trips, protocol signatures, authorization, repository confinement, convergence, and failure paths.

The immutable source remains `ssh://git@github.com/OnixResearch/artifact-auth.git` revision `799459346d5416fbd7b9f55840a7371441b55afa`. Completion requires a separate signer operation over exact standalone canonical bytes, standalone verification over independently reconstructed bytes, adversarial evidence, and unchanged authority flags. Entry-signature/boolean reuse, key export, fixture-only verification, or authority promotion are false completion.

## Portfolio-search registry

| Family | State | Evidence |
| --- | --- | --- |
| Radicle entry-signature reuse | Falsified | Protocol framing covers a different preimage. |
| Actor private-key export | Rejected | Transfers SSH-agent/key authority. |
| Fixture-only standalone key | Blocked | Does not exercise `valence-radicle`. |
| Dedicated signature through existing signer | Selected | Distinct operation, exact bytes, no key export. |

Audit risks are actor/raw-key identity conversion, signer substitution, statement reconstruction, malformed signatures, unknown currentness, protocol-rejection signing, carrier drift, disjoint false parity, secret leakage, and authority promotion. The search budget is four mechanism families, five targeted source surfaces, one advisory review attempt, and two deterministic validation rounds.

## Implemented boundary and focused evidence

`valence-core::map_valence_artifact_auth_statement` is the single pure mapping used by preparation and evaluation. The comparator now records protocol and standalone issue families and blocks disjoint dual rejection. The `valence-radicle` shell pins the standalone verifier, requires protocol verification, actor/signer and full-key equality, currentness, and positive reviewed profile/authorization policy before invoking the existing signer on exact standalone canonical bytes.

The carrier exposes only statement/key/signature BLAKE3 refs, public actor key, signature bytes, and lowercase signature hex. The shell independently reconstructs the statement, recomputes every carrier identity, verifies with `artifact-auth-ed25519`, and reports the external signing-authorization observation without claiming it is part of the standalone signature preimage.

A real `MemorySigner` positive test passes with protocol and standalone decisions accepted while legacy authority and rollback remain true and standalone authority remains false. Adversarial evidence passes for protocol-rejection signing, unauthorized actor, signer mismatch, Radicle-preimage signature reuse, changed entry preimage, wrong full key, malformed signature length, carrier substitution, unknown currentness, protocol-pass/standalone-fail drift, protocol-fail/standalone-pass drift, and decode/signature masking. Focused and full-workspace rustfmt, tests, and strict all-target Clippy pass in a repository-local target directory. Native Cairn validation and gates pass.

A secondary VibeThinker audit returned `fetch failed`; it supplied no evidence and deterministic source/tests remain authoritative. The first Nix attempt reached compilation but omitted the new shell module because Git-flake source capture cannot see an uncommitted Jujutsu-only file. The implementation commit was created before the final Nix rerun; no check was suppressed.

Full `nix flake check -L` passes on `x86_64-linux`, including the exact two-package standalone source assertion, Valence binaries, focused tests, and Nix test derivation. The package set remains one Cargo/Nix source and immutable revision. Native Cairn validation and all seven task gates pass. These results establish exact signing and verification over supplied local observations only; they do not prove SSH-agent trust, authorization freshness, repository correctness, Evidence IR semantics, lifecycle readiness, or release eligibility.
