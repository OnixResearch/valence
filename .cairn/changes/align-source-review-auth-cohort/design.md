# Design: Receiver-local source-review cohort

## Context

Valence verifies closed Cairn source-review attachments independently of a Cairn verdict. The current policy parser also requires workflow schema declarations and exact marker anchors that the older Valence-generated JSON lacks. Source selection and generated policy freshness are separate admission boundaries.

## Decisions

### Preserve complete source admission

Cargo and Nix must identify one immutable Artifact owner revision, a matching NAR hash, reviewed authentication entry-point bytes, compatible owner licensing, and an exact source workspace/release-profile package set. The Valence consumer graph remains exactly `artifact-auth-core` and `artifact-auth-ed25519`; other source workspace packages grant no consumer or release authority. An inconsistent owner profile is a blocker, not an excuse to inspect only imported sub-crates.

### Regenerate through the actual owner exporter

Use the verified Cairn CLI corresponding to public owner revision `1cacafebbff7b2716a953961a1ebf52bafb352e7` and its supported Nickel/exporter runtime. The scoped `default.ncl` and `contracts.ncl` preserve function-address gates, empty producer trust, nominal identities, strict task markers, legacy traceability roots, and the `cairn/archive` destination. The CLI selects the actual native root; this change does not migrate either lifecycle tree.

### Keep authentication and disposition distinct

The pure core consumes loaded attachment and operator-policy bytes; the shell confines reads and receipt creation to explicit capability roots. Full-key identities, purpose, generations, currentness, author exclusion, threshold, exact subject/claim/context linkage, and non-claims remain mandatory. An authenticated `needs_revision` disposition is attribution, never an approval. Malformed, tampered, revoked, and author-excluded inputs fail closed. Output ordering and canonical identities remain owned by the existing deterministic verifier.

## Rejected Alternatives

- Accepting Cargo-only identity or a sub-crate profile would bypass complete source admission.
- Copying another consumer's complete policy would import its unrelated authority settings.
- Using a Cairn verdict instead of checking signatures would not establish independent receiver acceptance.
- Promoting test-only keys or an accepted manual-review receipt into release authority would cross the explicit non-claim boundary.

## Validation

Retain actual owner export/check output, positive/negative compatibility fixtures, parser/root status/list/workflow and proposal/design/tasks gates, workspace lib/tests, strict workspace Clippy, package formatting, and actual Valence CLI receipts over the existing external-signer fixtures. Require accepted manual review only for the authorized test-only approval; require invalid or insufficient receipts for revoked, substituted-source, author-excluded, and negative-disposition inputs. Retain exact commands, exits, input identities, and limitations.

## Risks and Claim Boundaries

Both `.cairn` and `cairn` trees exist. Explicit root/status evidence prevents inventing a root cutover. Immutable owner profile drift blocks source acceptance until the owner publishes a coherent revision. A passing compatibility result proves shape/freshness/parser compatibility only. Test-key authentication is not actual human review and proves neither evidence truth, build correctness, deployment safety, source correctness, reproducibility, witness independence, nor release eligibility.

## Rollback

Restore an explicitly reviewed predecessor cohort in both manifests and regenerate both locks with their owners, then repeat receiver-local admission and behavior checks. Never select an automatic fallback or rewrite historical receipts as current evidence.
