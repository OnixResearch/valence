# Design: Fast and deep test suite partitioning

## Approach

Define named suite lanes:

- fast: unit tests, transcript fixtures, malformed corpus, CLI smoke, and deterministic report normalization;
- deep: stack cross-repo fixture expansion, optional sibling-repo integrations, and capability enforcement probes;
- nightly: full workspace, clippy, Nix checks, and any expensive cross-repo replay.

## Reporting

Each lane emits stable start/end summaries and distinguishes pass, fail, skipped, and unavailable checks. Fast lane failures block ordinary validation. Deep/nightly lanes are opt-in unless CI policy chooses otherwise.

## Boundary

Suite partitioning changes how evidence gates are scheduled; it does not broaden any verifier claim.
