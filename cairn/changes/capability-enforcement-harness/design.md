# Design: Capability enforcement harness

## Approach

Split capability testing into two lanes:

- pure label-policy tests that evaluate reported capability sets against policy;
- opt-in enforcement tests that run denial probes under an available sandbox backend.

## Enforcement availability

The fast suite checks pure label-policy behavior only. The deep suite may run sandboxed denial probes. If sandbox support is unavailable, the deep report records an unavailable/skipped diagnostic instead of marking enforcement as proven.

## Failure model

Denied effects must be blocked when enforcement is active. Unknown effects and denied labels fail closed in policy evaluation. Missing sandbox support cannot be reported as enforcement success.
