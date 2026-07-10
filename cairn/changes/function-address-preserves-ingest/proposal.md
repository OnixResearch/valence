## Why

Valence validates evidence identity, roles, spans, graph linkage, and non-claims. If Kamacite makes Preserves the authoritative function-address receipt form, Valence should validate direct Preserves receipts instead of relying on JSON-only projections or smoke adapter shapes.

## What Changes

- Add a function-address ingest path for already-loaded Kamacite Preserves receipts.
- Verify canonical Preserves receipt hash, schema version, digest domains, spans, dependency identity kind, roles, and non-claims before building Valence Evidence IR rows.
- Keep JSON projection ingest as compatibility only and require binding to the canonical Preserves receipt hash.
- Add positive and negative fixtures for Preserves ingest and projection drift.

## Impact

Valence can sit directly behind Kamacite's canonical receipt boundary while preserving its recorded-only evidence and non-claim enforcement.
