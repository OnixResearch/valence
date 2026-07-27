# Content-Bound Requirement References

Valence can validate a requirement reference against a supplied Cairn registry.
The reference binds the requirement to one repository, revision, specification,
policy, registry row, registry, and registry receipt.

## Cairn contract

Valence consumes `cairn.content-bound-requirement-registry.v1` from Cairn commit
`d953fe11ab620f3a42bdad1db51bc7672dd29824`.

The frozen test registry is
`fixtures/content-bound-requirements/cairn-registry.json`. Cairn generated this
file from the adjacent `cairn-root/` fixture. The Valence core does not run
Cairn, inspect a repository, or contact a service.

Valence checks the closed Cairn schema and all self-bound registry hashes. It
also checks duplicate rows, duplicate specification paths, the nested receipt,
and required Cairn non-claims.

## Validate requirement linkage

Use a root-relative input and an optional root-relative registry:

```bash
valence requirement-reference validate \
  --artifact-root fixtures/content-bound-requirements \
  --input requirement-input.json \
  --registry cairn-registry.json
```

A strict input uses profile `strict_content_bound`. It must contain one or more
`requirement_refs`. Each reference must match one supplied registry row.

Each digest carries an explicit domain. Domains include `specification`,
`requirement_body`, `policy`, `requirement_row`, `requirement_registry`, and
`requirement_registry_receipt`. Valence rejects a digest in the wrong domain,
even if its BLAKE3 spelling is valid.

Use `--output-root ROOT --out FILE` to save a valid report. Valence does not
write a report when validation fails.

## Validate a stack context

A stack validation envelope contains the context, supplied registries, and
expected link identities:

```bash
valence stack-context validate \
  --artifact-root evidence \
  --input stack-context.json
```

The context binds these values:

- A canonical selected repository revision set.
- Sorted content-bound requirement references.
- One evidence graph identity.
- Optional release, lifecycle, and runtime identities.
- The link roles that the selected profile requires.
- Required non-claims.

Input order does not change the selected revision-set or context identity.
Duplicate repositories fail. A present optional link must match an expected
identity. Valence does not ignore an invalid optional link.

## Legacy compatibility

Profile `legacy_recorded_only` keeps existing `r[...]` strings readable. These
strings retain their original spelling and a `RecordedOnly` verification role.
They do not gain repository, revision, specification, policy, row, or registry
facts.

A strict profile fails when an input contains only legacy strings. Existing
`ProvenanceLink.requirement_ids` serialization and hashes do not change.

## Rollout order

1. Cairn publishes a reviewed content-bound registry.
2. Valence validates typed references against the supplied registry.
3. Octet can emit typed producer references in a separate reviewed change.
4. Mantle can bind validated references to release evidence.
5. OnixOS can carry the bounded context into lifecycle and runtime receipts.

Each consumer must apply its own freshness, authorization, lifecycle, and
release policy.

## Non-claims

Passing validation proves typed identity, supplied-registry membership, and
selected-stack linkage only. It does not independently prove registry
freshness, revision authenticity, requirement satisfaction, evidence truth,
runtime authority, lifecycle readiness, or release eligibility.
