## Context

Valence already has a pure core and imperative CLI shell. The license split follows this boundary and keeps the canonical identity layer broadly consumable.

## Decisions

### MPL Evidence IR core

`valence-core` explicitly declares MPL-2.0. The workspace default remains AGPL so CLI and pilot packages continue to inherit AGPL-3.0-or-later.

### AGPL executable surfaces

`valence-cli` and repository-owned executable/research pilot packages remain AGPL. A pilot that is purely reusable may receive a later explicit review rather than silently inheriting the core license.

### Identity stability

Package license metadata and license documents are distribution artifacts, not canonical Evidence IR payload fields. Existing evidence IDs do not change unless their canonical bytes change under their own versioned schema.

### Non-claims

The split does not relicense dependencies, revoke earlier grants, prove behavioral correctness, or weaken Valence's existing evidence non-claims.
