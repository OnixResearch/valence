## Why

Unison separates immutable content identities from mutable human names. Valence has stable IDs, paths, aliases, and requirement names that are useful for humans but should not be confused with identity.

Valence needs explicit evidence name-binding history so renames and alias changes remain traceable without changing content identity or breaking provenance.

## What Changes

- Add a model for binding human aliases/names/paths to content-addressed evidence identities.
- Record binding history in receipts or sidecars where it affects operator review.
- Detect ambiguous aliases and stale bindings deterministically.
- Provide rename/path-churn tests proving provenance follows content identity.

## Impact

- **Files**: likely Evidence IR/provenance modules, receipts/docs, tests.
- **Compatibility**: additive metadata; existing IDs remain accepted during migration.
- **Validation**: focused evidence/provenance tests and Cairn gates.
