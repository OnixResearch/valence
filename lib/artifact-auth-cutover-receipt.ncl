# Pure deterministic validator for Valence's artifact-auth source cutover.
let _digest_hex_chars = 64 in
let _git_oid_hex_chars = 40 in
let _package_count = 2 in
let _blake3_regex = "^[0-9a-f]{%{std.to_string _digest_hex_chars}}$" in
let _git_oid_regex = "^[0-9a-f]{%{std.to_string _git_oid_hex_chars}}$" in
let _rid_regex = "^rad:z[1-9A-HJ-NP-Za-km-z]+$" in
let _expected = {
  rid = "rad:z4JGYYW7WsesXUq7MXVdx16Fawu2f",
  https_git = "https://git.onix.computer/z4JGYYW7WsesXUq7MXVdx16Fawu2f.git",
  commit = "799459346d5416fbd7b9f55840a7371441b55afa",
  source_archive_blake3 = "246a7cad91e7e8a158e22da21f3bff3e61aa0431a58936b5a739178bc62064c7",
  publication_revision = "b4343c1a1217de67726b259b5909657d720f09fd",
  publication_blake3 = "ac9bc126ea66c4ea9debcc320f88df667b8c5559a06eea97437f4e195d815fc2",
  cargo_manifest_blake3 = "45ff39f643cec55c40991acf67657cfc3835b60014201c804ab62c8391542fd0",
  cargo_lock_blake3 = "855618624f1fadb4c5eca0e8c948a68aac8c56c2e1aaab265cb3c277c08e297a",
  flake_blake3 = "d4a38c66262fb8e0e5c598fe45158f6c39fcee446abd51f3dd307c03049e757f",
  flake_lock_blake3 = "02fea0fd364148495be2b39d59522431c378a0acb6303d9c223f9900175ea27a",
  nar_hash = "sha256-nEgz2FtVuDesX95yyxidp0vhjxL4INB6Ve8rkpLyJk0=",
  packages = ["artifact-auth-core", "artifact-auth-ed25519"],
}
in
let _expected_lock_source = "git+%{_expected.https_git}?rev=%{_expected.commit}#%{_expected.commit}" in
let _required_root_fields = [
  "schema_version",
  "receipt_type",
  "status",
  "observed_date",
  "source",
  "publication",
  "cargo",
  "nix",
  "validation",
  "fallback",
  "rollback",
  "non_claims",
  "evidence",
]
in
let _required_non_claims = [
  "artifact-auth-source-correctness",
  "whole-valence-behavior-correctness",
  "forge-availability",
  "automatic-fallback",
  "whole-stack-github-independence",
  "release-readiness",
]
in
let _has_fields = fun fields value =>
  std.is_record value && fields |> std.array.all (fun field => std.record.has_field field value)
in
let _is_blake3 = fun value => std.is_string value && std.string.is_match _blake3_regex value in
let _is_git_oid = fun value => std.is_string value && std.string.is_match _git_oid_regex value in
let _is_rid = fun value => std.is_string value && std.string.is_match _rid_regex value in
let _error = fun error_code error_message =>
  {
    code = error_code,
    message = error_message,
  }
in
let _first_error = fun errors =>
  errors
  |> std.array.filter_map (fun error => if error == null then 'None else 'Some error)
  |> match { [] => 'Ok, [error, ..] => 'Error error }
in
let _validate = fun receipt =>
  if !_has_fields _required_root_fields receipt then
    'Error (_error "cutover-shape" "cutover evidence is incomplete")
  else
    _first_error [
      if receipt.schema_version == 1
      && receipt.receipt_type == "valence.artifact-auth-radicle-cutover.v1"
      && receipt.status == "accepted"
      && receipt.observed_date == "2026-07-25" then
        null
      else
        _error "cutover-header" "cutover identity or status drifted",
      if _has_fields ["rid", "https_git", "reviewed_commit", "source_archive_blake3"] receipt.source
      && _is_rid receipt.source.rid
      && receipt.source.rid == _expected.rid
      && receipt.source.https_git == _expected.https_git
      && _is_git_oid receipt.source.reviewed_commit
      && receipt.source.reviewed_commit == _expected.commit
      && _is_blake3 receipt.source.source_archive_blake3
      && receipt.source.source_archive_blake3 == _expected.source_archive_blake3 then
        null
      else
        _error "cutover-source" "RID, HTTPS URL, commit, or source archive identity drifted",
      if receipt.publication.revision == _expected.publication_revision
      && receipt.publication.receipt_type == "artifact-auth.radicle-publication.v1"
      && _is_blake3 receipt.publication.receipt_blake3
      && receipt.publication.receipt_blake3 == _expected.publication_blake3
      && receipt.publication.status == "accepted" then
        null
      else
        _error "cutover-publication" "accepted publication linkage drifted",
      if _is_blake3 receipt.cargo.manifest_blake3
      && receipt.cargo.manifest_blake3 == _expected.cargo_manifest_blake3
      && _is_blake3 receipt.cargo.lock_blake3
      && receipt.cargo.lock_blake3 == _expected.cargo_lock_blake3
      && receipt.cargo.lock_source == _expected_lock_source
      && std.array.length receipt.cargo.packages == _package_count
      && receipt.cargo.packages == _expected.packages
      && receipt.cargo.package_version == "0.1.0"
      && !receipt.cargo.crate_graph_changed then
        null
      else
        _error "cutover-cargo" "Cargo source, packages, lock identity, or graph drifted",
      if _is_blake3 receipt.nix.flake_blake3
      && receipt.nix.flake_blake3 == _expected.flake_blake3
      && _is_blake3 receipt.nix.lock_blake3
      && receipt.nix.lock_blake3 == _expected.flake_lock_blake3
      && receipt.nix.locked_url == _expected.https_git
      && receipt.nix.original_url == _expected.https_git
      && receipt.nix.locked_rev == _expected.commit
      && receipt.nix.original_rev == _expected.commit
      && receipt.nix.nar_hash == _expected.nar_hash
      && receipt.nix.nar_hash_preserved then
        null
      else
        _error "cutover-nix" "Nix source, revision, narHash, or lock identity drifted",
      if receipt.validation.baseline_revision == "70b36b91ec596915ae1a29aa3bbb53f0cba1ae48"
      && receipt.validation.baseline_workspace == "passed"
      && receipt.validation.post_cutover_workspace == "passed"
      && receipt.validation.focused_positive_mapping == "passed"
      && receipt.validation.focused_negative_mapping == "passed"
      && receipt.validation.frozen_source_no_std == "passed"
      && receipt.validation.formatting == "passed"
      && receipt.validation.clippy == "passed"
      && receipt.validation.source_agreement_positive == "passed"
      && receipt.validation.source_agreement_negative == "passed"
      && !receipt.validation.rust_implementation_changed then
        null
      else
        _error "cutover-validation" "behavioral or source-agreement evidence is incomplete",
      if !receipt.fallback.manifest_github_source
      && !receipt.fallback.cargo_lock_github_source
      && !receipt.fallback.nix_input_github_source
      && !receipt.fallback.nix_lock_github_source
      && !receipt.fallback.automatic_fallback
      && receipt.fallback.outage_result == "visible-resolution-failure" then
        null
      else
        _error "cutover-fallback" "an executable GitHub or automatic fallback remains",
      if !receipt.rollback.automatic
      && receipt.rollback.requires_owning_tool_lock_regeneration
      && receipt.rollback.requires_source_identity_review then
        null
      else
        _error "cutover-rollback" "rollback bypasses lock regeneration or identity review",
      if _required_non_claims |> std.array.all (fun claim => std.array.elem claim receipt.non_claims) then
        null
      else
        _error "cutover-non-claims" "required non-claims are missing",
      if receipt.evidence |> std.array.all (fun path => std.is_string path && !(std.string.is_match "^/" path)) then
        null
      else
        _error "cutover-evidence" "evidence paths must remain relative",
      if !(std.record.has_field "claims" receipt)
      && !(std.record.has_field "credentials" receipt)
      && !(std.record.has_field "fallback_url" receipt) then
        null
      else
        _error "cutover-overclaim" "receipt contains authority, credentials, or fallback",
    ]
in
{
  expected = _expected,
  expected_lock_source = _expected_lock_source,
  required_non_claims = _required_non_claims,
  is_blake3 = _is_blake3,
  is_git_oid = _is_git_oid,
  is_rid = _is_rid,
  validate = _validate,
}
