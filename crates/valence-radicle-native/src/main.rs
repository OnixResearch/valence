//! Process-isolated native Radicle issue/patch mutation shell.

mod forge_ops;
mod model;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr as _;

use radicle::node::device::Device;
use radicle_crypto::PublicKey;
use radicle_crypto::ssh::agent::Agent;
use serde::Serialize;

use crate::forge_ops::execute_native_plan;
use crate::forge_ops::open_radicle_repository;
use crate::model::MigrationPlanningReport;
use crate::model::validate_plan;

// r[impl valence.radicle_forge_ops.native_cobs]
// r[impl valence.radicle_forge_ops.shell]
// r[impl valence.radicle_forge_ops.boundaries]

const EXIT_INVALID: u8 = 1;
const USAGE: &str = "usage:\n\
valence-radicle-native import --repository PATH --plan FILE --output-root DIR --output FILE [--execute]\n\
\nWithout --execute, the command validates and prints a no-write preview.\n";
const PREVIEW_SCHEMA: &str = "valence.radicle-forge-operations.native-preview.v1";

#[derive(Debug, Serialize)]
struct Preview<'a> {
    schema: &'static str,
    executable: bool,
    repository_rid: &'a str,
    importer_did: &'a str,
    plan_blake3: &'a str,
    record_count: usize,
    effects: &'static str,
}

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(EXIT_INVALID)
        }
    }
}

fn run(args: Vec<String>) -> Result<ExitCode, String> {
    let (command, rest) = args.split_first().ok_or_else(|| USAGE.to_owned())?;
    if command != "import" {
        return Err(format!("unknown command {command:?}\n{USAGE}"));
    }
    let flags = FlagSet::parse(rest)?;
    flags.require_known(&["repository", "plan", "output-root", "output"], &["execute"])?;
    let planning = serde_json::from_slice::<MigrationPlanningReport>(&read(flags.value("plan")?)?)
        .map_err(|error| format!("invalid migration planning report JSON: {error}"))?;
    if !planning.valid || !planning.issues.is_empty() {
        return Err("migration planning report is not valid".to_owned());
    }
    let plan = planning.plan.as_ref().ok_or_else(|| "valid migration planning report lacks a plan".to_owned())?;
    validate_plan(plan)?;
    if !flags.switch("execute") {
        let preview = Preview {
            schema: PREVIEW_SCHEMA,
            executable: true,
            repository_rid: &plan.repository_rid,
            importer_did: &plan.importer_did,
            plan_blake3: &plan.plan_blake3,
            record_count: plan.records.len(),
            effects: "no-write-preview",
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&preview).map_err(|error| format!("cannot serialize preview: {error}"))?
        );
        return Ok(ExitCode::SUCCESS);
    }

    let repository = open_radicle_repository(Path::new(flags.value("repository")?), &plan.repository_rid)
        .map_err(|error| error.to_string())?;
    let actor_text = plan
        .importer_did
        .strip_prefix("did:key:")
        .ok_or_else(|| "planned importer DID is malformed".to_owned())?;
    let actor = PublicKey::from_str(actor_text).map_err(|error| format!("invalid planned importer key: {error}"))?;
    let signer = Agent::connect().map_err(|error| format!("cannot connect to SSH agent: {error}"))?.signer(actor);
    if !signer.is_ready().map_err(|error| format!("cannot query SSH agent: {error}"))? {
        return Err("planned importer key is not loaded in the SSH agent".to_owned());
    }
    let device = Device::from(signer);
    let observed = execute_native_plan(&repository, &device, plan).map_err(|error| error.to_string())?;
    let output = serde_json::to_vec_pretty(&observed)
        .map_err(|error| format!("cannot serialize native observations: {error}"))?;
    let path = safe_output_path(flags.value("output-root")?, flags.value("output")?)?;
    write_new(&path, &output)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&observed)
            .map_err(|error| format!("cannot serialize native observations: {error}"))?
    );
    Ok(ExitCode::SUCCESS)
}

fn read(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("cannot read {path:?}: {error}"))
}

fn safe_output_path(root: &str, relative: &str) -> Result<PathBuf, String> {
    let root = Path::new(root);
    if !root.is_dir() {
        return Err("output root must be an existing directory".to_owned());
    }
    let relative = Path::new(relative);
    if relative.is_absolute() || relative.components().any(|component| !matches!(component, Component::Normal(_))) {
        return Err("output must be a relative file name without traversal".to_owned());
    }
    let output = root.join(relative);
    let parent = output.parent().ok_or_else(|| "output path has no parent".to_owned())?;
    let canonical_root = root.canonicalize().map_err(|error| format!("cannot resolve output root: {error}"))?;
    let canonical_parent = parent.canonicalize().map_err(|error| format!("cannot resolve output parent: {error}"))?;
    if canonical_parent != canonical_root {
        return Err("output parent must be the selected output root".to_owned());
    }
    Ok(output)
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("cannot create output {path:?}: {error}"))?;
    file.write_all(bytes).map_err(|error| format!("cannot write output {path:?}: {error}"))?;
    file.write_all(b"\n").map_err(|error| format!("cannot terminate output {path:?}: {error}"))
}

struct FlagSet {
    values: std::collections::BTreeMap<String, String>,
    switches: std::collections::BTreeSet<String>,
}

impl FlagSet {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut values = std::collections::BTreeMap::new();
        let mut switches = std::collections::BTreeSet::new();
        let mut index = 0;
        while index < args.len() {
            let argument = &args[index];
            let name = argument.strip_prefix("--").ok_or_else(|| format!("expected flag, found {argument:?}"))?;
            if name == "execute" {
                if !switches.insert(name.to_owned()) {
                    return Err(format!("duplicate flag --{name}"));
                }
                index += 1;
                continue;
            }
            let value_index = index + 1;
            let value = args.get(value_index).ok_or_else(|| format!("missing value for --{name}"))?;
            if value.starts_with("--") || values.insert(name.to_owned(), value.clone()).is_some() {
                return Err(format!("invalid or duplicate value for --{name}"));
            }
            index += 2;
        }
        Ok(Self { values, switches })
    }

    fn require_known(&self, values: &[&str], switches: &[&str]) -> Result<(), String> {
        for name in self.values.keys() {
            if !values.contains(&name.as_str()) {
                return Err(format!("unknown value flag --{name}"));
            }
        }
        for name in &self.switches {
            if !switches.contains(&name.as_str()) {
                return Err(format!("unknown switch --{name}"));
            }
        }
        for name in values {
            if !self.values.contains_key(*name) {
                return Err(format!("missing required flag --{name}"));
            }
        }
        Ok(())
    }

    fn value(&self, name: &str) -> Result<&str, String> {
        self.values.get(name).map(String::as_str).ok_or_else(|| format!("missing required flag --{name}"))
    }

    fn switch(&self, name: &str) -> bool {
        self.switches.contains(name)
    }
}

#[cfg(test)]
mod tests {
    // r[verify valence.radicle_forge_ops.shell]
    // r[verify valence.radicle_forge_ops.validation]

    use radicle::identity::Did;

    use super::*;

    #[test]
    fn flags_accept_preview_and_reject_duplicates() {
        let flags =
            FlagSet::parse(&["--plan", "plan.json", "--execute"].into_iter().map(str::to_owned).collect::<Vec<_>>())
                .expect("flags");
        assert_eq!(flags.value("plan"), Ok("plan.json"));
        assert!(flags.switch("execute"));
        assert!(
            FlagSet::parse(&["--plan", "one", "--plan", "two"].into_iter().map(str::to_owned).collect::<Vec<_>>())
                .is_err()
        );
    }

    #[test]
    fn output_path_rejects_escape_and_existing_file() {
        let root = tempfile::tempdir().expect("output root");
        assert!(safe_output_path(root.path().to_str().expect("path"), "../escape.json").is_err());
        let output = safe_output_path(root.path().to_str().expect("path"), "observations.json").expect("output");
        write_new(&output, b"[]").expect("first write");
        assert!(write_new(&output, b"[]").is_err());
    }

    #[test]
    fn importer_did_rendering_matches_device_identity() {
        let fixture = radicle::test::setup::NodeWithRepo::default();
        assert!(Did::from(*fixture.signer.public_key()).to_string().starts_with("did:key:z"));
    }
}
