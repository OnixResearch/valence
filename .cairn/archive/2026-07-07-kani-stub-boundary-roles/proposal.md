# Change: Kani stub boundary roles

## Why

Kani has two very different stubbing modes. `stub_verified` can be backed by a machine-checked contract, while a plain `stub` is a trusted or environmental model. Valence must preserve that distinction so downstream gates do not treat trusted mocks as verified behavior.

## What Changes

- Add Kani stub facets for verified contract stubs, plain implementation stubs, and environmental stubs.
- Require verified-stub artifacts to link to a passing contract receipt for the original function and exact source identity.
- Classify plain and environmental stubs as boundary assumptions or recorded-only evidence.
- Reject promotion of unverified stubs into property evidence.

## Impact

- **Files**: Evidence IR payload validators, fixtures, stack-provenance compatibility checks, and docs.
- **Consumers**: Octet can export Kani stub details and Cairn can gate on them without manual log inspection.
- **Non-claims**: Plain stubs remain conditional models; Valence does not certify that they faithfully over-approximate real behavior.
