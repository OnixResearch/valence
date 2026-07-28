# Proposal: Encode digest roles in Rust types

## Why

`SharedDigest` stores its digest role as a runtime enum. Binary, sidecar, artifact, receipt, policy, and graph digests therefore share one Rust type even when an API expects one exact role.

The stack-provenance adapter already uses distinct wrapper types for selected digest roles. Valence must make that compile-time separation part of the shared identity core.

## What Changes

- Add an owned generic digest type with a private validated BLAKE3 value and a role marker.
- Add borrowed role-specific digest views for allocation-free adapters.
- Define role aliases for binary, sidecar, artifact, receipt, policy, graph, and source identity domains.
- Separate untrusted wire data from admitted typed core values.
- Replace runtime role checks in shared identity and stack-provenance core APIs with typed parameters where the role is statically known.
- Preserve heterogeneous wire and report collections through a closed role-tagged adapter enum.
- Preserve current JSON fields and accepted digest spellings.
- Add compile-pass, compile-fail, parser, serialization, and tamper evidence.

## Impact

- **Core**: `shared_identity`, artifact-reference, Evidence IR, and stack-provenance adapter APIs.
- **CLI**: Parsing and projection adapters only. File and process authority remain unchanged.
- **Compatibility**: Existing wire schemas and canonical identity material remain stable unless a fixture proves an intentional versioned change.
- **Consumers**: Exact source users receive compatibility aliases or migration guidance for changed Rust APIs.

## Non-goals

- Do not treat a matching digest as proof that evidence is true.
- Do not combine verification-role and digest-role authority.
- Do not change BLAKE3 or reinterpret external algorithm fields.
- Do not remove role tags from serialized heterogeneous evidence.
- Do not add filesystem or measurement effects to `valence-core`.
