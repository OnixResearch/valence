# Pure validator for Valence's unified Artifact source migration receipt.
let digest_hex_chars = 64 in
let git_oid_hex_chars = 40 in
let expected_consumer_package_count = 2 in
let expected_source_package_count = 3 in
let expected_core_test_count = 3 in
let expected_shell_test_count = 4 in
let blake3_regex = "^[0-9a-f]{%{std.to_string digest_hex_chars}}$" in
let git_oid_regex = "^[0-9a-f]{%{std.to_string git_oid_hex_chars}}$" in
let expected = {
  repository = "ssh://git@github.com/OnixResearch/onix-artifact.git",
  profile_repository = "https://github.com/OnixResearch/onix-artifact",
  revision = "e41340bec587b6d049b5cc518ec7db925dde84be",
  nar_hash = "sha256-2cM912L2YXVnVX9LquwhAPKyjPP/z/oFQRe7Qq9bHHE=",
  source_archive_blake3 = "8fc65f3d7968d76af419f08ac972955514c8256f8aa5ed90815d8bdcced28113",
  license = "MIT OR Apache-2.0",
  source_packages = ["artifact-auth-core", "artifact-auth-ed25519", "artifact-transfer-core"],
  consumer_packages = ["artifact-auth-core", "artifact-auth-ed25519"],
  core_blake3 = "839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21",
  ed25519_blake3 = "7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633",
  historical_repository = "https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git",
  historical_revision = "799459346d5416fbd7b9f55840a7371441b55afa",
  historical_receipt_blake3 = "ef8d13a0cb8d560a99b79f91f9471350db774e8c3bf753d52fed7382f9bd85f7",
  baseline_revision = "4daa5030f2b0ace54c4c08b3a30ebd8cd0c57621",
}
in
let expected_lock_source = "git+%{expected.repository}?rev=%{expected.revision}#%{expected.revision}" in
let required_root_fields = [
  "schema_version",
  "receipt_type",
  "status",
  "observed_date",
  "source",
  "consumer",
  "parity",
  "validation",
  "history",
  "rollback",
  "non_claims",
  "evidence",
]
in
let required_non_claims = [
  "artifact-source-correctness",
  "whole-valence-behavior-correctness",
  "forge-availability",
  "automatic-fallback",
  "whole-stack-github-independence",
  "authorization",
  "release-readiness",
]
in
let has_fields = fun fields value =>
  std.is_record value && fields |> std.array.all (fun field => std.record.has_field field value)
in
let is_blake3 = fun value => std.is_string value && std.string.is_match blake3_regex value in
let is_git_oid = fun value => std.is_string value && std.string.is_match git_oid_regex value in
let error = fun error_code error_message => { code = error_code, message = error_message } in
let first_error = fun errors =>
  errors
  |> std.array.filter_map (fun candidate => if candidate == null then 'None else 'Some candidate)
  |> match { [] => 'Ok, [candidate, ..] => 'Error candidate }
in
let validate = fun receipt =>
  if !has_fields required_root_fields receipt then
    'Error (error "migration-shape" "migration evidence is incomplete")
  else
    first_error [
      if receipt.schema_version == 1
      && receipt.receipt_type == "valence.artifact-source-migration.v1"
      && receipt.status == "accepted"
      && receipt.observed_date == "2026-07-26" then null
      else error "migration-header" "migration identity or status drifted",
      if receipt.source.repository == expected.repository
      && receipt.source.profile_repository == expected.profile_repository
      && is_git_oid receipt.source.revision
      && receipt.source.revision == expected.revision
      && receipt.source.nar_hash == expected.nar_hash
      && is_blake3 receipt.source.source_archive_blake3
      && receipt.source.source_archive_blake3 == expected.source_archive_blake3
      && receipt.source.license == expected.license
      && std.array.length receipt.source.packages == expected_source_package_count
      && receipt.source.packages == expected.source_packages then null
      else error "migration-source" "unified source identity, package set, or license drifted",
      if receipt.consumer.cargo_lock_source == expected_lock_source
      && std.array.length receipt.consumer.packages == expected_consumer_package_count
      && receipt.consumer.packages == expected.consumer_packages
      && !receipt.consumer.transfer_package_resolved
      && !receipt.consumer.binding_package_resolved
      && receipt.consumer.cargo_nix_aligned then null
      else error "migration-consumer" "Valence consumer graph or Cargo/Nix agreement drifted",
      if is_blake3 receipt.parity.core_blake3
      && receipt.parity.core_blake3 == expected.core_blake3
      && is_blake3 receipt.parity.ed25519_blake3
      && receipt.parity.ed25519_blake3 == expected.ed25519_blake3
      && receipt.parity.directory_comparison == "identical"
      && !receipt.parity.rust_implementation_changed then null
      else error "migration-parity" "authentication source parity or implementation boundary drifted",
      if receipt.validation.baseline_revision == expected.baseline_revision
      && receipt.validation.baseline_core_test_count == expected_core_test_count
      && receipt.validation.baseline_shell_test_count == expected_shell_test_count
      && receipt.validation.historical_receipt == "passed"
      && receipt.validation.post_core == "passed"
      && receipt.validation.post_shell == "passed"
      && receipt.validation.no_std == "passed"
      && receipt.validation.workspace == "passed"
      && receipt.validation.formatting == "passed"
      && receipt.validation.clippy == "passed"
      && receipt.validation.source_admission_positive == "passed"
      && receipt.validation.source_admission_negative == "passed" then null
      else error "migration-validation" "positive or negative validation evidence is incomplete",
      if receipt.history.repository == expected.historical_repository
      && receipt.history.revision == expected.historical_revision
      && is_blake3 receipt.history.receipt_blake3
      && receipt.history.receipt_blake3 == expected.historical_receipt_blake3
      && receipt.history.status == "historical-only" then null
      else error "migration-history" "historical source or receipt boundary drifted",
      if !receipt.rollback.automatic
      && receipt.rollback.requires_owning_tool_lock_regeneration
      && receipt.rollback.requires_source_identity_review then null
      else error "migration-rollback" "rollback bypasses lock regeneration or source review",
      if required_non_claims |> std.array.all (fun claim => std.array.elem claim receipt.non_claims) then null
      else error "migration-non-claims" "required non-claims are missing",
      if receipt.evidence |> std.array.all (fun path => std.is_string path && !(std.string.is_match "^/" path)) then null
      else error "migration-evidence" "evidence paths must remain relative",
      if !(std.record.has_field "claims" receipt)
      && !(std.record.has_field "credentials" receipt)
      && !(std.record.has_field "authority" receipt) then null
      else error "migration-overclaim" "receipt contains authority, credentials, or claims",
    ]
in
let exported_expected = expected in
let exported_lock_source = expected_lock_source in
let exported_is_blake3 = is_blake3 in
let exported_is_git_oid = is_git_oid in
let exported_required_non_claims = required_non_claims in
let exported_validate = validate in
{
  expected = exported_expected,
  expected_lock_source = exported_lock_source,
  is_blake3 = exported_is_blake3,
  is_git_oid = exported_is_git_oid,
  required_non_claims = exported_required_non_claims,
  validate = exported_validate,
}
