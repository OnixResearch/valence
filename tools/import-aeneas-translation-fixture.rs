#!/usr/bin/env -S CARGO_TARGET_DIR=/tmp/valence-aeneas-import-script-target nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const EXPECTED_ARGUMENT_COUNT: usize = 3;
const SELF_TEST_ARGUMENT_COUNT: usize = 2;
const SELF_TEST_FLAG: &str = "--self-test";
const DESTINATION_DIRECTORY: &str = "fixtures/aeneas-translation-chain/artifacts";
const IMPORT_ENTRY_COUNT: usize = 18;

const IMPORT_ENTRIES: &[(&str, &str)] = &[
    ("standards/proof-rails/aeneas-lean/generated/valence-adapter-fixture.json", "producer-adapter.json"),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/provider-profile.json",
        "provider-profile.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/source/src/lib.rs",
        "source.rs",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/charon-options.json",
        "charon-options.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/charon-command.json",
        "charon-command.json",
    ),
    ("standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/llbc.llbc", "llbc.llbc"),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/aeneas-command.json",
        "aeneas-command.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/translation.json",
        "translation.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/Llbc.lean",
        "generated.lean",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/external-models.json",
        "external-models.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/theorem-statement.lean",
        "theorem.lean",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/theorem-adequacy.json",
        "theorem-adequacy.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/trust-closure.json",
        "trust-closure.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/package-lock.json",
        "package-lock.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/aeneas/checker-result.json",
        "checker-result.json",
    ),
    (
        "standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proofs/octet.aeneas_lean_proof_rails.fixture-rail-0.receipt",
        "octet-receipt.json",
    ),
    ("standards/proof-rails/aeneas-lean/fixtures/valid/artifacts/proof-cards.json", "proof-cards.json"),
    ("standards/proof-rails/aeneas-lean/fixtures/valid/verification.json", "provenance.json"),
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportEntry {
    source: PathBuf,
    destination: PathBuf,
}

fn import_plan(octet_root: &Path, valence_root: &Path) -> Result<Vec<ImportEntry>, String> {
    if octet_root.as_os_str().is_empty() || valence_root.as_os_str().is_empty() {
        return Err("source and destination roots must be non-empty".to_owned());
    }
    if octet_root == valence_root {
        return Err("source and destination roots must differ".to_owned());
    }
    let destination_root = valence_root.join(DESTINATION_DIRECTORY);
    let plan = IMPORT_ENTRIES
        .iter()
        .map(|(source, destination)| ImportEntry {
            source: octet_root.join(source),
            destination: destination_root.join(destination),
        })
        .collect::<Vec<_>>();
    if plan.len() != IMPORT_ENTRY_COUNT {
        return Err("import inventory count drifted".to_owned());
    }
    Ok(plan)
}

fn execute_import(plan: &[ImportEntry]) -> Result<(), String> {
    for entry in plan {
        if !entry.source.is_file() {
            return Err(format!("missing source artifact: {}", entry.source.display()));
        }
    }
    for entry in plan {
        let parent = entry
            .destination
            .parent()
            .ok_or_else(|| format!("destination has no parent: {}", entry.destination.display()))?;
        fs::create_dir_all(parent).map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        fs::copy(&entry.source, &entry.destination).map_err(|error| {
            format!("cannot copy {} to {}: {error}", entry.source.display(), entry.destination.display())
        })?;
        println!("{} <- {}", entry.destination.display(), entry.source.display());
    }
    Ok(())
}

fn self_test() -> Result<(), String> {
    let source = Path::new("/tmp/octet-source");
    let destination = Path::new("/tmp/valence-destination");
    let plan = import_plan(source, destination)?;
    assert_eq!(plan.len(), IMPORT_ENTRY_COUNT);
    assert!(plan.iter().all(|entry| entry.source.starts_with(source)));
    assert!(plan.iter().all(|entry| entry.destination.starts_with(destination)));
    assert!(import_plan(source, source).is_err());
    assert!(import_plan(Path::new(""), destination).is_err());
    println!("Aeneas fixture import self-test passed");
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    if args.len() == SELF_TEST_ARGUMENT_COUNT && args[1] == SELF_TEST_FLAG {
        return self_test();
    }
    if args.len() != EXPECTED_ARGUMENT_COUNT {
        return Err("usage: import-aeneas-translation-fixture.rs <octet-root> <valence-root>".to_owned());
    }
    let plan = import_plan(Path::new(&args[1]), Path::new(&args[2]))?;
    execute_import(&plan)
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
