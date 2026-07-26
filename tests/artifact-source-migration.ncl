# Positive and negative unified-source migration receipt tests.
let migration = import "../lib/artifact-source-migration-receipt.ncl" in
let receipt = import "../evidence/source/artifact-workspace-migration-v1.ncl" in
let digest_a = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" in
let missing_commit = "1111111111111111111111111111111111111111" in
let result = fun candidate => migration.validate candidate in
let accepted = fun candidate => result candidate |> match { 'Ok => true, 'Error _ => false } in
let rejected_as = fun code candidate =>
  result candidate |> match { 'Ok => false, 'Error error => error.code == code }
in
{
  tests = [
    accepted receipt,
    migration.is_blake3 digest_a,
    migration.is_git_oid receipt.source.revision,
    rejected_as "migration-shape" {},
    rejected_as "migration-header" (receipt & { status | force = "pending" }),
    rejected_as "migration-source" (receipt & { source.revision | force = missing_commit }),
    rejected_as "migration-source" (receipt & { source.packages | force = ["artifact-auth-core"] }),
    rejected_as "migration-source" (receipt & { source.license | force = "AGPL-3.0-or-later" }),
    rejected_as "migration-consumer" (receipt & { consumer.transfer_package_resolved | force = true }),
    rejected_as "migration-consumer" (receipt & { consumer.cargo_nix_aligned | force = false }),
    rejected_as "migration-parity" (receipt & { parity.core_blake3 | force = digest_a }),
    rejected_as "migration-parity" (receipt & { parity.rust_implementation_changed | force = true }),
    rejected_as "migration-validation" (receipt & { validation.post_shell | force = "failed" }),
    rejected_as "migration-history" (receipt & { history.status | force = "active-fallback" }),
    rejected_as "migration-rollback" (receipt & { rollback.automatic | force = true }),
    rejected_as "migration-non-claims" (receipt & { non_claims | force = [] }),
    rejected_as "migration-evidence" (receipt & { evidence | force = ["/tmp/evidence.json"] }),
    rejected_as "migration-overclaim" (receipt & { claims = ["release-ready"] }),
    !migration.is_blake3 "sha256:%{digest_a}",
    !migration.is_git_oid digest_a,
  ] |> std.test.assert_all,
}
