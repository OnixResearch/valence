## Why

Valence's evidence-codebase index normalizes artifact locators and builds lookup maps for provenance validation. The current `source_hashes` helper duplicates locator/hash identity in a plain map, so duplicate normalized locators can be lost or reduced to whichever value wins collection.

`iddqd` fits this boundary because normalized evidence sources are identity-bearing records: the locator and BLAKE3 digest are part of the value, and duplicate handling should be explicit before provenance links are evaluated.

## What Changes

- Add an iddqd-backed ordered source index for `NormalizedSource` records keyed by normalized locator.
- Preserve the public Evidence IR index and query JSON shapes.
- Report duplicate normalized locators explicitly, with strict/advisory behavior that matches existing build modes.
- Add positive and negative fixtures for unique locators, advisory exact duplicates, conflicting duplicate locators, provenance lookup, missing referenced artifacts, and malformed evidence.

## Impact

- Evidence source lookup becomes deterministic and drift-resistant.
- Provenance hash mismatch diagnostics no longer depend on map overwrite behavior.
- This is a focused Valence adoption that stays inside `valence-core`'s pure in-memory indexing boundary.
