#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-cairn-policy-compatibility-script nix shell "github:nix-community/fenix?rev=4e49ef62eedaa5c149d62b63b3f53844e1cc45d7#complete.toolchain" nixpkgs#gcc -c cargo -q -Zscript

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const CAIRN_SOURCE_ENV: &str = "CAIRN_SOURCE";
const DEFAULT_CAIRN_SOURCE: &str = "../cairn";
const POLICY_SOURCE: &str = "cairn-policy/default.ncl";
const POLICY_CONTRACTS: &str = "cairn-policy/contracts.ncl";
const GENERATED_POLICY: &str = "cairn-policy/generated/cairn-policy.json";
const POLICY_FIXTURES: &str = "cairn-policy/fixtures";
const POSITIVE_FIXTURE: &str = "cairn-policy/fixtures/valid-function-address-policy.ncl";
const INVALID_FIXTURE_PREFIX: &str = "invalid-";
const NICKEL_EXTENSION: &str = "ncl";

#[derive(Debug, Clone, PartialEq, Eq)]
struct CheckCommand {
    label: &'static str,
    program: &'static str,
    args: Vec<OsString>,
}

fn compatibility_plan(cairn_source: &Path) -> Vec<CheckCommand> {
    let cairn_app = format!("path:{}#cairn", cairn_source.display());
    vec![
        CheckCommand {
            label: "positive Nickel fixture",
            program: "nix",
            args: strings(&[
                "develop",
                "-c",
                "nickel",
                "export",
                "--format",
                "json",
                POSITIVE_FIXTURE,
            ]),
        },
        CheckCommand {
            label: "negative fixtures and generated-policy freshness",
            program: "nix",
            args: vec![
                "run".into(),
                cairn_app.clone().into(),
                "--".into(),
                "policy".into(),
                "export".into(),
                "--check".into(),
            ],
        },
        CheckCommand {
            label: "supported Cairn default-path lifecycle validation",
            program: "nix",
            args: vec![
                "run".into(),
                cairn_app.into(),
                "--".into(),
                "validate".into(),
                "--root".into(),
                ".".into(),
            ],
        },
    ]
}

fn strings(values: &[&str]) -> Vec<OsString> {
    values.iter().map(OsString::from).collect()
}

fn is_invalid_nickel_fixture(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with(INVALID_FIXTURE_PREFIX))
        && path.extension().and_then(|extension| extension.to_str()) == Some(NICKEL_EXTENSION)
}

fn cairn_source(root: &Path) -> PathBuf {
    env::var_os(CAIRN_SOURCE_ENV).map(PathBuf::from).unwrap_or_else(|| root.join(DEFAULT_CAIRN_SOURCE))
}

fn require_inputs(root: &Path, cairn_source: &Path) -> Result<(), Box<dyn Error>> {
    for relative in [POLICY_SOURCE, POLICY_CONTRACTS, GENERATED_POLICY, POSITIVE_FIXTURE] {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("required compatibility input is missing: {}", path.display()).into());
        }
    }
    if !cairn_source.join("flake.nix").is_file() {
        return Err(
            format!("Cairn source is missing flake.nix: {}; set {CAIRN_SOURCE_ENV}", cairn_source.display()).into()
        );
    }
    let invalid_fixture_count = fs::read_dir(root.join(POLICY_FIXTURES))?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|entry| is_invalid_nickel_fixture(&entry.path()))
        .count();
    if invalid_fixture_count == 0 {
        return Err("policy compatibility requires at least one invalid Nickel fixture".into());
    }
    Ok(())
}

fn run_plan(root: &Path, plan: &[CheckCommand]) -> Result<(), Box<dyn Error>> {
    for check in plan {
        eprintln!("check: {}", check.label);
        let status = Command::new(check.program).args(&check.args).current_dir(root).status()?;
        if !status.success() {
            return Err(format!("compatibility check failed: {}", check.label).into());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = env::current_dir()?;
    let cairn_source = cairn_source(&root);
    require_inputs(&root, &cairn_source)?;
    let plan = compatibility_plan(&cairn_source);
    run_plan(&root, &plan)?;
    println!("Valence Cairn policy compatibility: PASS");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_CHECK_COUNT: usize = 3;

    #[test]
    fn plan_covers_positive_negative_freshness_and_validation() {
        let plan = compatibility_plan(Path::new("/reviewed/cairn"));
        let labels = plan.iter().map(|check| check.label).collect::<Vec<_>>();

        assert_eq!(plan.len(), EXPECTED_CHECK_COUNT);
        assert!(labels.contains(&"positive Nickel fixture"));
        assert!(labels.contains(&"negative fixtures and generated-policy freshness"));
        assert!(labels.contains(&"supported Cairn default-path lifecycle validation"));
    }

    #[test]
    fn invalid_fixture_classifier_rejects_nonmatching_paths() {
        assert!(is_invalid_nickel_fixture(Path::new("invalid-mode.ncl")));
        assert!(!is_invalid_nickel_fixture(Path::new("valid-mode.ncl")));
        assert!(!is_invalid_nickel_fixture(Path::new("invalid-mode.json")));
    }
}
