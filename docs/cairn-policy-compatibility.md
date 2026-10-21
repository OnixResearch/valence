# Cairn policy compatibility

Valence keeps its lifecycle policy in `cairn-policy/default.ncl`, checked by the typed contracts in `cairn-policy/contracts.ncl`. The generated `cairn-policy/generated/cairn-policy.json` file is an export artifact and must not be edited directly.

The supported parser is the canonical local Cairn checkout selected by `CAIRN_SOURCE` (default `../cairn`). Refresh the generated policy with:

```bash
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- policy export \
  --output cairn-policy/generated/cairn-policy.json
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- policy export --check
nix run path:${CAIRN_SOURCE:-../cairn}#cairn -- validate --root .
```

## Reviewed function-address gate

r[impl valence.evidence_ir.cairn_policy_compatibility.function_address_gate]

Valence selects `optional` mode. An absent function-address receipt is therefore not required for lifecycle validation, while any supplied receipt remains subject to Cairn's function-address checks. The gate accepts only claim scope `function_address.v1.lifecycle_policy_conformance` and the current canonical Valence function-address profile identity:

```text
e5f4f35b3b339fdefcf7a195910c4eb04388dc66de8452891bbceabb60b557a3
```

That identity is the BLAKE3 digest produced by `valence_core::function_address_profile::function_address_policy_identity`. The policy also requires the reviewed sidecar/profile/Kamacite roles and schemas. Placeholder, stale, malformed, empty, or weakened fixture values fail the Nickel contract.

The workspace contract matrix gate is carried explicitly in disabled mode so the complete supported policy field set remains reviewable.

## Artifact substance and authenticated evidence

The gate policy carries Cairn's fail-closed proposal, design, requirement, task,
and dependency-marker substance thresholds. The authenticated-evidence policy
uses the non-promoting `diagnostic` profile, accepts the reviewed corrected
Preserves profile, and binds the archived Valence integrity receipt plus the
stack-provenance and function-address authentication profiles by BLAKE3. Empty
trusted-producer configuration grants no signature authority. These inputs make
the policy schema complete; they do not promote Valence to an authenticated
Onix release profile.

Nickel sources own the two authentication profiles. The JSON files under
`cairn-policy/generated/` are freshness-checked exports and must not be edited
directly. The archived integrity receipt is a byte-for-byte consumer copy of
the Cairn acceptance receipt for Valence revision
`1f4f6fe903a34c22d2cda53cd534ce58bbf84147`.

## Workflow schema catalog

r[impl valence.evidence_ir.cairn_policy_compatibility.workflow_schema]

The supported Cairn parser requires workflow-schema v1 policy content: typed
artifact entries with output paths, path kinds, structural admission classes,
and lifecycle stages, plus schema-level bounds, deterministic state order,
spec-effect and task-link classes, receipt fields, and required non-claims.
Valence declares the full supported profile catalog (`spec-driven`,
`assurance`, `no-spec-delta`, and `outcome-machine`) and the workflow profile
registry in contracted Nickel. The default profile remains `spec-driven`;
declaring the other profiles restores parser compatibility without selecting
them for existing Valence changes.

The registry forbids ambient profile selection sources (`user-global`,
`environment`, `agent-prompt`, `remote-package`). Negative fixtures prove that
a policy missing the workflow profile registry or an artifact missing its
output path fails Nickel contract evaluation before generated JSON is
accepted.

Profile declaration proves parser compatibility only. It does not prove
evidence truth, workflow artifact quality, or release eligibility.

## Current owner-policy compatibility cohort

The reviewed owner source is Cairn revision
`1cacafebbff7b2716a953961a1ebf52bafb352e7` at
`https://github.com/OnixResearch/cairn` (source-review implementation commit
`7e9ff66`). The preparation used its checkout at
`/tmp/cairn-source-review-main-20261020/cairn-policy` (campaign equivalent
`xrepo/cairn-source-review-main-20261020/cairn-policy`). Its observed BLAKE3
file identities are:

| Owner input | BLAKE3 |
| --- | --- |
| `default.ncl` | `c3b5981d71789b04ff3a7567a84a7b1df29b5703dcb3a65e664143713e77a97e` |
| `contracts.ncl` | `bc537dc32ff01fc9004dc032b78ad6fb5855c5c6f0158ecda219d5912b46619c` |
| `LICENSING.md` | `d86b9a6d31606882ae6f2e0fb42493852472f12a60d0e9aeda932c2b9fe50b89` |

These identify the reviewed policy bytes retained across owner publication.
The prepared Basalt cohort's generated outcome-machine graph was a comparison
pattern only; its complete policy was not copied into Valence.

The imported source cohort is deliberately limited to:

- Cairn's bounded `outcome-machine` workflow schema and registry/guidance
  declarations. Its non-skipped design-stage `outcome-machine.json` record is
  required by both design and tasks; the existing proposal/design gates and
  workflow non-claims remain in force.
- Explicit `[task:<id>]` and `[after:<task-id>]` task markers and their typed
  marker kinds, retaining `strict` task-marker enforcement and all existing
  task-link/substance gates.
- The current parser's mandatory `declared` assurance and `exact_marker_bytes`
  anchor fields for every existing Valence traceability profile. Profile IDs,
  requirement/evidence roots, and marker verbs are retained.
- The owner's narrowly admitted `.cairn` relative-path form in Nickel's path
  contract. This does not authorize arbitrary hidden paths or move lifecycle
  content.

The initial preparation off
`b5e558e87f57b0615b50e532846d6865e31042b` observed both
`.cairn/{specs,archive}` and `cairn/{specs,archive}` with no active changes.
That observation is not a claim about the later canonical owner baseline,
whose active `adopt-owned-source-agpl` change must be preserved. This policy
refresh does not select, merge, rename, sync, or archive either lifecycle
tree. Existing Valence `cairn/specs/...` traceability roots and the
`cairn/archive` archive-destination setting remain unchanged; a lifecycle-root
cutover is a separate owner operation.

Valence's function-address contracts still pin its exact optional mode,
profile identity, roles, schemas, and non-claim boundary rather than adopting
the owner's more general gate contract. The authenticated-evidence diagnostic
profile, integrity receipt and authentication-export hashes, empty trusted
producer set, nominal identities, developer-loop configuration, and other
gate/trust settings are unchanged. No Artifact Auth or source-review
implementation/API is part of this migration.

### Policy license and source provenance

Cairn's owner `cairn-policy/LICENSING.md` offers only the named
`default.ncl`, `contracts.ncl`, `lifecycle-store-contracts.ncl`,
`nominal-evidence-identities.ncl`, and generated policy contents under
`AGPL-3.0-or-later OR MIT OR Apache-2.0`, including its explicitly identified
historical copies. This migration retains the AGPL-3.0-or-later option for the
imported policy material; the full text is already present at
`LICENSES/AGPL-3.0-or-later.txt`. Preserve applicable notices and the reviewed
source provenance when redistributing the cohort. The grant does not relicense
the Cairn CLI/exporter/toolchain, other policy inputs, owner documentation, or
third-party material. Valence's prospective owned-source AGPL policy is a
separate owner decision; this workflow-policy migration does not choose or
override that licensing policy or revoke retained historical MPL grants.

### Regeneration and observed compatibility

The separately retained, pre-canonical-baseline
`policy-generation-v2/complete.json` in the native
`align-source-review-auth-cohort` evidence records the earlier export and
freshness check, positive Nickel fixture, supported-parser loading, root
status/list, default-root validation, and native `spec-driven` change creation,
all with exit status zero. That export check also exercised all ten existing
negative fixtures. These observations belong to the earlier prepared subject,
not the later canonical baseline.

For the canonical baseline, guarded Pueue job `1395`
(`valence-policy-canonical-generation-20261020-v1`) exercised the real owner
`policy export` command with exit zero and wrote the repository-owned JSON.
`policy-generation-canonical-v1/complete.json` retains the exact command,
exporter/input identities, and generated-policy BLAKE3
`90a7d6b63788e450adb2e5bea4d380a002a702068567fb7aef5845fb63776935`.
Subsequent receiver-local captures retain supported `policy export --check`,
positive fixture export, and rejection of all eleven existing negative
fixtures. Ten reject in Nickel; the missing-function-address-gate fixture
exports JSON but fails the owner's mandatory-field parser. Parser admission,
explicit-root status/list, and workflow status all exit zero. Preliminary
capture-driver expectation failures are retained separately; they did not
change inputs or weaken validation. No function-address/trust boundary changed.

For reproduction from the Valence root, select the exact reviewed owner source:

```bash
export CAIRN_SOURCE=/tmp/cairn-source-review-main-20261020
nix run path:$CAIRN_SOURCE#cairn -- policy export \
  --source cairn-policy/default.ncl \
  --output cairn-policy/generated/cairn-policy.json
CAIRN_SOURCE="$CAIRN_SOURCE" ./tools/check-cairn-policy-compatibility.rs
nix run path:$CAIRN_SOURCE#cairn -- policy --policy cairn-policy/generated/cairn-policy.json
nix run path:$CAIRN_SOURCE#cairn -- status --root .
nix run path:$CAIRN_SOURCE#cairn -- change list --root .
```

On the campaign host, `CAIRN_SOURCE` is
`/home/brittonr/mantle-cairn-finish/xrepo/cairn-source-review-main-20261020`.
The exercised binary is
`runs/cairn-source-review-native-complete-20261020-v1/cairn` under the campaign
root, run in that owner's Nix shell so its supported `nickel` and
`nickel-export` executables are available. Campaign commands also retain the
process-scoped exact dependency mirrors. The recorded source identities and
commands bind this observation; an ambient binary is not a substitute.

The pre-canonical status/list observed `.cairn/specs` and no active change
before the owner CLI created `align-source-review-auth-cohort` with the
explicit `spec-driven` profile. The canonical baseline instead contains
existing active owner changes, including `adopt-owned-source-agpl`; their
source and lifecycle state are preserved. Current-subject status selects the
repository root `.` explicitly and observes native `.cairn/specs` and
`.cairn/changes`; the historical JSON layout label `cairn` is not a migration.
Workflow selection is explicit `spec-driven` with no compatibility fallback.
This does not merge the two lifecycle trees, alter legacy traceability roots,
or imply release readiness.

### Earlier canonical working-copy baseline

The earlier preparation imported the operator-supplied source archive
`valence-canonical-source-resume21.tar`, BLAKE3
`41244c822f93788886bff18c4ed38b242a84bf4081933a05e01ed39c273fab12`.
Its source identifier, observed without JJ auto-snapshot, is
`79d3706d0d634e47cd004a32e009ce1104ed9a25`; the archive includes actual
working files and is not a committed-source or publication claim. The native
change retains the measured 653-file relative-path BLAKE3 input inventory at
`evidence/canonical-owner-baseline/inventory.json`.

The narrow overlay adds the current workflow/marker/anchor compatibility
fields to the canonical policy, preserves its five existing traceability
profiles and trust/function-address settings, observes all four Artifact
source manifests for the complete AGPL source validator, and aligns current
documentation with the owner-selected prospective AGPL boundary. The earlier
prepared Nostr profile and other noncanonical feature changes are not imported.
Full pre-import source and overlay backups remain in the disposable workspace's
ignored `target/`. The original disposable preparation did not mutate canonical
source. The subsequent expressly authorized canonical integration applied only
the guarded overlay, preserving native owner work, historical MPL grants, and
third-party/vendor notices.

Current Cargo/Nix source projections use published immutable Artifact revision
`5f3962d7471d1547e044f57daeda72f56621fd52`. Parent verification compared all
180 transferred source files with the primary acquisition, checked the exact
NAR, and passed the full owner flake check and native validation/gates. The
source validator resolves Cargo's explicit `publish = false` and inherited
`publish.workspace = true` against the workspace's disabled publication policy;
enabling publication in the workspace must reject the inheriting packages.
Receiver-local `cohort-acceptance-resume24/complete.json` now records generated
Cargo/Nix locks, the exact two-package resolved consumer graph, full-source
admission and negative fixtures, package formatting, strict workspace Clippy,
347 passing workspace lib/tests, and the actual CLI build, each with exit zero.
That capture belongs to the broader prepared working-copy subject. It remains
historical evidence, not a byte binding or reused test result for the later
isolated main-line publication. Owner-source acceptance alone is not consumer acceptance.

The prepared remote subject also contained
`cairn-policy/fixtures/invalid-outcome-machine-admission.ncl`; it is retained
alongside the ten canonical negative fixtures. It substitutes the forbidden
`arbitrary_execution` structural-admission value in the outcome-machine graph.
Its exact pre-import source is preserved in the disposable remote-source
backup; the current-subject Nickel execution rejects it with exit one.

### Receiver-local signed-fixture acceptance

The native change's `evidence/receiver-policy-acceptance-v1/complete.json`
retains all commands, exits, twelve exact fixture/request/signature inputs,
five receipts, and binary identities. The fresh run-directory copy of the
actual passing-build Valence CLI has BLAKE3
`3ae24800e4b75a87bf688c7cd478304082de4b980a30efc5ec9d34c9915a7798`.
The Cairn owner CLI has BLAKE3
`104939af5a4ff5bff0e62851110c1c388b15f7974535fb167d05b2266029b45c`.
Neither binary was rebuilt or replaced during the proof.

| Actual receiver case | Exit | Outcome | Distinct issue boundary |
| --- | --- | --- | --- |
| Test-only approval | 0 | `accepted_manual_review` | Exactly one authorized test key; no issues |
| Signed `needs_revision` | 1 | `insufficient` | `approval_threshold_unsatisfied`; no approved keys |
| Revoked key | 1 | `invalid` | `auth.key_not_current`, authentication and distinct-key threshold failure |
| Substituted source | 1 | `invalid` | `review_subject_root_or_policy_mismatch` |
| Excluded author | 1 | `invalid` | `review_author_excluded` |

All negative cases retain zero approved keys. Every receipt retains the exact
operator-policy source subject, claim root, and seven non-claims, including
the witness-independence and release-eligibility exclusions. The test-only
public-key BLAKE3 is
`6384c589c5d965c3c278571ac2bd27d672999ea131e8a5f78b37a5a9cce3c7b6`.
The retained producer driver documents genuine OpenSSL Ed25519 signatures
over raw Cairn signing-request bytes. Independent retained-byte verification
recomputes receipt, subject, and claim-root canonical identities and exact
copied input hashes; it does not rerun product tests.

These fixtures concern `test-only-operator-smoke`, not the actual Valence
source. Successful authentication is not actual human review, review quality,
a property proof, build/reproducibility proof, an independent witness, or
release authorization. Test keys never enter repository producer trust.

### Native requirement sync and readiness

`evidence/native-readiness-v1/` retains proposal/design/tasks gates and the
inspected native sync plan. The authorized `--execute` result and post-sync
validation are in `evidence/native-sync-v1/`: only three existing Artifact
adoption requirements and five added source-review requirements were merged,
into exactly `.cairn/specs/artifact-auth-adoption/spec.md` and
`.cairn/specs/source-review-acceptance/spec.md`. All commands exited zero.
Before/after inventories prove other measured product, policy, and lifecycle
files unchanged by sync. No legacy tree, traceability root, producer trust,
function-address setting, or archive policy was migrated or weakened.

The first archive preview correctly blocked on the not-yet-evidenced lifecycle
task. Its checkbox was updated only after actual sync, validation, and readiness
capture; final native evidence is retained separately under
`evidence/native-final-v1/`: all three gates report `PASS`, validation exits
zero, sync is idempotent, and the archive dry-run is unblocked and non-mutating.
The separate generic readiness receipt is valid but classified `unavailable`
with `observation.unavailable` at `completion`: no authoritative completion
contract, results, and subject-hash observation was supplied. An unblocked
archive plan is not a passing generic completion observation. Neither fixture
results nor checkboxes are promoted into that missing authority. Terminal
archive and publication remain owner operations and were not executed.

### Isolated main-line publication subject

Publication is prepared in a separate Jujutsu workspace based on public main
`b5e5558e87f57b0615b50e532846d6865e31042b`. Only the Artifact dependency and
source-admission updates, contracted workflow policy and generated export,
this guide, and the native `align-source-review-auth-cohort` artifacts are
transferred. Existing main-line schema-identity and Nostr material is preserved;
unrelated working-copy Radicle, licensing, and feature changes are excluded.
The initial isolated input archive has BLAKE3
`cbc83c8f0619f2151b33822180ec7816b45c1ab834175aa53375fd7202719667`.

Fresh owner-tool lock generation, the exact two-package Artifact consumer
graph, and source-pin positive/negative admission are captured in
`evidence/isolated-cohort-v28/`. The resulting Cargo and Nix locks have BLAKE3
`bc82162c66a5ebe81057fe2aa27d998d54964792a64860bc04b372ba18c1aa25`
and `3f25d95045ab294391b3d3040d15c17d2cc0f2b6b81a3a94075af1f057cd8e58`.
`evidence/isolated-cohort-v29/` records 359 passing workspace lib/tests, strict
workspace Clippy, affected-package formatting, and the actual Valence CLI build.
The full-workspace formatting command failed on an unchanged line wrap in
`pilots/schema-identity/src/lib.rs`; its original failure is retained, that
unrelated file is not reformatted, and full-workspace formatting is not claimed.

`evidence/isolated-receiver-v30/` exercises the freshly built CLI, BLAKE3
`ecfcaa539c3d3e4d624a570719d99fd33ade72870ee2af12de7cc1a11fc3932c`,
against the same externally signed test-only inputs. Approval succeeds;
needs-revision, revoked-key, substituted-source, and excluded-author cases
retain their distinct rejection outcomes. Policy freshness, supported-parser
admission, all eleven negative fixtures, native-root discovery, and explicit
workflow selection are recorded again for this isolated subject. These are
actual receiver observations, not real human review or release authorization.

## Compatibility checks

r[verify valence.evidence_ir.cairn_policy_compatibility.final_validation]

`CAIRN_SOURCE=../cairn ./tools/check-cairn-policy-compatibility.rs` runs the positive Nickel fixture, all `invalid-*.ncl` negative fixtures through Cairn's exporter, byte-for-byte generated-policy freshness, and default-path lifecycle validation. Negative fixtures can fail Nickel evaluation or the owner's semantic parser after successful Nickel export; a missing required gate is not necessarily a Nickel type error. Direct drift in generated JSON fails the export check.

Passing these checks proves only that Valence's Nickel configuration has the supported field/value shape, the checked-in JSON is fresh, and the pinned Cairn parser can evaluate the lifecycle tree. It does not prove evidence truth, Rust semantic correctness, verifier soundness, requirement satisfaction, or release eligibility.
