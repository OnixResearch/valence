# Validation evidence

## Baseline

Before core changes, native Cairn validation and proposal/design/tasks gates passed. `nix develop -c cargo test -p valence-core cob_evidence` passed all eleven current COB-evidence tests. The reviewed standalone profile at artifact-auth revision `799459346d5416fbd7b9f55840a7371441b55afa` records Valence source baseline `8eccac35fcd8aae8ef64b3d1021e7f1a53656180`, shared protocol-verified key/entry/parent/related/profile fields, Radicle/Git/Evidence IR extensions, and retained protocol, authorization, reduction, storage, SSH-agent, and lifecycle authority. Current source review found those same semantic classes in `valence-core::cob_evidence` and `valence-radicle::transport`.

## Exact source admission

Cargo and Nix resolve `ssh://git@github.com/OnixResearch/artifact-auth.git` at full revision `799459346d5416fbd7b9f55840a7371441b55afa`. `Cargo.lock` contains one `artifact-auth-core` package from that revision. `flake.lock` records the same SSH URL/revision and NAR hash `sha256-nEgz2FtVuDesX95yyxidp0vhjxL4INB6Ve8rkpLyJk0=`. Flake evaluation asserts exact Cargo/Nix revisions, lock uniqueness/source, and the standalone `MIT OR Apache-2.0` license. Cargo and Nix tooling generated the locks.

## Adapter and cutover result

`valence-core::artifact_auth` is a pure one-way post-protocol adapter. It BLAKE3-binds exact Radicle entry, parent, related-target, profile-context, and currentness strings while retaining raw Radicle/Git identities as compatibility extensions. The caller must separately supply the full raw-key BLAKE3 identity and a `CryptographicObservation` for the exact standalone statement. Radicle `signature_valid` is never promoted into standalone proof.

Compatibility classifies distinct protocol/standalone preimages, key identity, decision drift, issue taxonomy, non-claims, malformed/lossy OIDs, unobserved related targets, unknown currentness, and unrelated decode/signature failure masking. Legacy authority and rollback remain true. Runtime standalone authority is rejected because no Valence shell currently verifies the exact standalone statement; `standalone_authority_admitted` is fixed false.

## Approach registry

| Family | Result |
| --- | --- |
| Reconstruct Radicle verification in artifact-auth | Rejected: generic Ed25519 verification cannot replace Radicle protocol semantics. |
| Reuse `signature_valid` | Rejected: it authenticates protocol-owned bytes, not the standalone statement. |
| Map raw Git SHA-1 OIDs directly | Rejected: artifact-auth intentionally supports BLAKE3/SHA-256 references, so exact typed OID strings are BLAKE3-bound and retained separately. |
| Pure post-protocol one-way adapter | Selected: it preserves protocol facts and claim boundaries without filesystem, Git, SSH, or authorization effects. |

## Focused verification

Three adapter tests pass. Positive coverage checks exact entry/parent/related/key binding, mandatory non-claims, and retained authority. Negative coverage checks malformed OIDs, missing related targets, full-key drift, false parity, unknown currentness, and unrelated decode/signature failures. `cargo fmt` and strict all-target Clippy pass for `valence-core`.

Repository-local-target workspace tests, strict workspace all-target Clippy, native Cairn validation/gates, and full `nix flake check -L` pass on `x86_64-linux`. The isolated Cargo target avoids cross-toolchain rustdoc metadata from concurrent sibling validation without changing source or suppressing tests.

These checks establish deterministic mapping over supplied observations only. They do not prove Radicle verifier correctness, actor authorization, Evidence IR semantics, COB reduction, Git integrity, SSH-agent behavior, lifecycle readiness, or release eligibility.
