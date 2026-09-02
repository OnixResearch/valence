# Agent Notes

## Repo Layout

```
crates/valence-core/   Pure Evidence IR core (no I/O, no filesystem, no process)
crates/valence-cli/    CLI shell (arg parsing, file reads, stdout/stderr, exit codes)
.cairn/                Native Cairn lifecycle (specs, changes, archive)
```

## Build Environment

- Rust nightly (`nightly-2026-03-21`, pinned in `rust-toolchain.toml`)
- Enter the devshell: `nix develop`
- Build: `cargo build --workspace`
- Test: `cargo test --workspace`
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings`
- Format: `cargo fmt --check`

## Functional Core / Imperative Shell Boundary

`valence-core` is pure deterministic core only. It MUST NOT contain:
filesystem reads, environment reads, process execution, clocks, async,
`println!`, `eprintln!`, or JSON rendering helpers beyond canonical
serialization for hashing.

`valence-cli` owns the imperative shell: argument parsing, file reads,
stdout/stderr, and exit codes. All core logic lives in `valence-core`.

Core purity smoke:

```bash
rg 'std::fs|fs::|std::env|std::process|println!|eprintln!|read_dir|read_to_string|std::time|tokio::|async' crates/valence-core/src
```

Expected result: no matches (except in `#[cfg(test)]` blocks).

## Cairn Lifecycle

This repo uses native Cairn lifecycle (`.cairn/` layout). Legacy OpenSpec is
not used. Run cairn from the cairn source checkout:

```bash
export CAIRN_SOURCE=/home/brittonr/git/cairn
nix run path:$CAIRN_SOURCE#cairn -- init --root .
nix run path:$CAIRN_SOURCE#cairn -- validate --root .
nix run path:$CAIRN_SOURCE#cairn -- change list --root .
nix run path:$CAIRN_SOURCE#cairn -- gate proposal <change> --root .
nix run path:$CAIRN_SOURCE#cairn -- gate design <change> --root .
nix run path:$CAIRN_SOURCE#cairn -- gate tasks <change> --root .
```

Requirements use EARS-style shape: `MUST`/`SHALL`/`SHOULD`/`MAY` normative
strength, `GIVEN`/`WHEN`/`THEN` scenario blocks, stable `r[...]` IDs.

## Non-Claims

Valence supports bounded local evidence replay. It does not claim behavioral
correctness, semantic equivalence, exhaustive Rust coverage, pure-cache
eligibility, cross-machine determinism, cross-toolchain determinism, external
consumer compatibility, mechanized language soundness, or general HIR/THIR/MIR
extraction. Every artifact carries an explicit `non_claims` boundary; the
checker fails closed if the boundary is missing or weakened.

## VCS

This repo uses Jujutsu (`jj`). Use `jj` for all version control operations.
The colocated `.git/` directory is maintained by jj.

## Hashing

BLAKE3 for all content hashing, canonical identity, and evidence artifact
hashes. No SHA-256 for new valence-owned surfaces (existing v1 contract
strings from octet retain their identifiers for compatibility).
