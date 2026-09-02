# Design: Stack provenance cross-repo fixtures

## Approach

The fast suite uses small, frozen JSON fixtures and deterministic byte strings. The fixture core builds or verifies in-memory stack-provenance values; the test shell owns fixture loading.

## Fixture scope

Fixtures cover the minimum identity thread:

- Octet Valence provenance export facts;
- Valence stack-provenance sidecar;
- Mantle external evidence manifest entry;
- measured binary byte hash and sidecar digest linkage.

## Deep integration

Live sibling repository checks are separated into a deep/nightly suite so ordinary tests remain fast and reproducible.

## Failure model

Negative fixtures cover stale binary bytes, mismatched sidecar digest, missing Mantle role, parent artifact mismatch, tier promotion, and weakened non-claim boundaries.
