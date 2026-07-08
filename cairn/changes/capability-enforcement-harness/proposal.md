# Change: Capability enforcement harness

## Why

Verifier capability labels are currently receipt metadata. To avoid confusing labels with enforcement, Valence should add a harness that separately tests label policy decisions and sandboxed enforcement behavior when an enforcement backend is available.

## What changes

- Add tests that keep label-only policy evaluation distinct from enforcement tests.
- Add opt-in sandbox/enforcement checks for denied filesystem, process, network, clock, remote-builder, and secret effects.
- Report enforcement availability explicitly instead of silently treating missing sandbox support as success.

## Out of scope

- No claim that labels alone enforce effects.
- No mandatory dependency on a host sandbox in the fast suite.
