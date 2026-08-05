use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr as _;

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use radicle_crypto::PublicKey;
use radicle_crypto::ssh::agent::Agent;
use valence_core::cob_evidence::BindingReceipt;
use valence_core::cob_evidence::CobAuthorizationProfile;
use valence_core::cob_evidence::CobEvidenceProfile;
use valence_core::cob_evidence::CobSnapshot;
use valence_core::cob_evidence::CobTargetRef;
use valence_core::radicle_forge_ops::CiAdmissionInput;
use valence_core::radicle_forge_ops::CiAdmissionReceipt;
use valence_core::radicle_forge_ops::ForgeOperationsProfile;
use valence_core::radicle_forge_ops::MigrationBindingReceipt;
use valence_core::radicle_forge_ops::MigrationPlanningReport;
use valence_core::radicle_forge_ops::ObservedRecordBinding;
use valence_core::radicle_forge_ops::admit_ci_revision;
use valence_core::radicle_forge_ops::build_binding_receipt as build_forge_binding_receipt;
use valence_core::radicle_forge_ops::parse_profile as parse_forge_profile;
use valence_core::radicle_forge_ops::parse_source_batch;
use valence_core::radicle_forge_ops::plan_migration;
use valence_core::radicle_forge_ops::validate_profile as validate_forge_profile;
use valence_core::radicle_forge_ops::verify_binding_receipt as verify_forge_binding_receipt;
use valence_core::radicle_forge_ops::verify_ci_admission_receipt;
use valence_radicle::GitCobStore;
use valence_radicle::build_authorization_for_actor;
use valence_radicle::execute_mutation;
use valence_radicle::materialize_snapshot;
use valence_radicle::parse_authorization;
use valence_radicle::parse_mutation_request;
use valence_radicle::parse_profile;
use valence_radicle::prepare_mutation;
use valence_radicle::verify_saved_binding;
use valence_radicle::verify_saved_snapshot;

// r[impl valence.radicle_forge_ops.shell]
// r[impl valence.radicle_forge_ops.boundaries]

const EXIT_INVALID: u8 = 1;
const EXIT_IO: u8 = 2;
const USAGE: &str = "\
valence-radicle mutate --repository PATH --profile FILE --authorization FILE --artifact-root DIR --request FILE --output-root DIR --output FILE [--execute]\n\
valence-radicle snapshot --repository PATH --profile FILE --authorization FILE --artifact-root DIR --target FILE --output-root DIR --output FILE\n\
valence-radicle authorization --profile FILE --actor KEY --role ROLE --output-root DIR --output FILE\n\
valence-radicle verify-binding --profile FILE --receipt FILE\n\
valence-radicle verify-snapshot --profile FILE --authorization FILE --snapshot FILE\n\
valence-radicle forge-plan --profile FILE --source FILE --output-root DIR --output FILE\n\
valence-radicle forge-bind --profile FILE --plan FILE --observations FILE --output-root DIR --output FILE\n\
valence-radicle verify-forge-binding --profile FILE --receipt FILE\n\
valence-radicle ci-admission --input FILE --output-root DIR --output FILE\n\
valence-radicle verify-ci-admission --receipt FILE\n\
\nMutations are preview-only unless --execute is present. Execution signs through the running SSH agent using the planned actor key.\n";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(EXIT_IO)
        }
    }
}

fn run(args: Vec<String>) -> Result<ExitCode, String> {
    let Some((command, tail)) = args.split_first() else {
        eprintln!("{USAGE}");
        return Ok(ExitCode::from(EXIT_INVALID));
    };
    let flags = FlagSet::parse(tail)?;
    match command.as_str() {
        "mutate" => run_mutate(&flags),
        "snapshot" => run_snapshot(&flags),
        "authorization" => run_authorization(&flags),
        "verify-binding" => run_verify_binding(&flags),
        "verify-snapshot" => run_verify_snapshot(&flags),
        "forge-plan" => run_forge_plan(&flags),
        "forge-bind" => run_forge_bind(&flags),
        "verify-forge-binding" => run_verify_forge_binding(&flags),
        "ci-admission" => run_ci_admission(&flags),
        "verify-ci-admission" => run_verify_ci_admission(&flags),
        "help" | "--help" | "-h" => {
            println!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        _ => Err(format!("unknown command '{command}'\n{USAGE}")),
    }
}

fn run_mutate(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(
        &[
            "repository",
            "profile",
            "authorization",
            "artifact-root",
            "request",
            "output-root",
            "output",
        ],
        &["execute"],
    )?;
    let profile = load_profile(flags.value("profile")?)?;
    let authorization = load_authorization(flags.value("authorization")?)?;
    let request = parse_mutation_request(&read(flags.value("request")?)?).map_err(|error| error.to_string())?;
    let store = GitCobStore::open(Path::new(flags.value("repository")?)).map_err(|error| error.to_string())?;
    let artifact_root = open_dir(flags.value("artifact-root")?)?;
    let prepared = prepare_mutation(&store, &profile, &authorization, &artifact_root, request)
        .map_err(|error| error.to_string())?;
    if !flags.switch("execute") {
        let valid = prepared.plan.valid;
        write_json(flags, &prepared.plan)?;
        return Ok(if valid {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(EXIT_INVALID)
        });
    }
    if !prepared.plan.valid {
        write_json(flags, &prepared.plan)?;
        return Ok(ExitCode::from(EXIT_INVALID));
    }
    let actor =
        PublicKey::from_str(&prepared.plan.actor).map_err(|error| format!("invalid planned actor key: {error}"))?;
    let signer = Agent::connect().map_err(|error| format!("cannot connect to SSH agent: {error}"))?.signer(actor);
    if !signer.is_ready().map_err(|error| format!("cannot query SSH agent: {error}"))? {
        return Err("planned actor key is not loaded in the SSH agent".to_owned());
    }
    let result = execute_mutation(&store, &profile, &signer, prepared).map_err(|error| error.to_string())?;
    write_json(flags, &result)?;
    Ok(ExitCode::SUCCESS)
}

fn run_snapshot(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(
        &[
            "repository",
            "profile",
            "authorization",
            "artifact-root",
            "target",
            "output-root",
            "output",
        ],
        &[],
    )?;
    let profile = load_profile(flags.value("profile")?)?;
    let authorization = load_authorization(flags.value("authorization")?)?;
    let target = serde_json::from_slice::<CobTargetRef>(&read(flags.value("target")?)?)
        .map_err(|error| format!("invalid target JSON: {error}"))?;
    let store = GitCobStore::open(Path::new(flags.value("repository")?)).map_err(|error| error.to_string())?;
    let artifact_root = open_dir(flags.value("artifact-root")?)?;
    let materialization = materialize_snapshot(&store, &profile, &authorization, &artifact_root, target)
        .map_err(|error| error.to_string())?;
    let valid = materialization.report.valid && materialization.adapter_issues.is_empty();
    write_json(flags, &materialization)?;
    Ok(if valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_authorization(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "actor", "role", "output-root", "output"], &[])?;
    let profile = load_profile(flags.value("profile")?)?;
    let authorization =
        build_authorization_for_actor(&profile, flags.value("actor")?.to_owned(), flags.value("role")?.to_owned());
    let issues = valence_core::cob_evidence::validate_authorization_profile(&profile, &authorization);
    if !issues.is_empty() {
        return Err(format!(
            "authorization input is invalid: {}",
            issues.into_iter().map(|issue| issue.code).collect::<Vec<_>>().join(",")
        ));
    }
    write_json(flags, &authorization)?;
    Ok(ExitCode::SUCCESS)
}

fn run_verify_binding(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "receipt"], &[])?;
    let profile = load_profile(flags.value("profile")?)?;
    let receipt = serde_json::from_slice::<BindingReceipt>(&read(flags.value("receipt")?)?)
        .map_err(|error| format!("invalid binding receipt JSON: {error}"))?;
    let issues = verify_saved_binding(&profile, &receipt);
    print_verification(&issues)?;
    Ok(if issues.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_verify_snapshot(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "authorization", "snapshot"], &[])?;
    let profile = load_profile(flags.value("profile")?)?;
    let authorization = load_authorization(flags.value("authorization")?)?;
    let snapshot = serde_json::from_slice::<CobSnapshot>(&read(flags.value("snapshot")?)?)
        .map_err(|error| format!("invalid snapshot JSON: {error}"))?;
    let issues = verify_saved_snapshot(&profile, &authorization, &snapshot);
    print_verification(&issues)?;
    Ok(if issues.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_forge_plan(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "source", "output-root", "output"], &[])?;
    let profile = load_forge_profile(flags.value("profile")?)?;
    let source = parse_source_batch(&read(flags.value("source")?)?)
        .map_err(|error| format!("invalid normalized forge source JSON: {error}"))?;
    let report = plan_migration(&profile, &source);
    let valid = report.valid;
    write_json(flags, &report)?;
    Ok(if valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_forge_bind(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "plan", "observations", "output-root", "output"], &[])?;
    let profile = load_forge_profile(flags.value("profile")?)?;
    let planning = serde_json::from_slice::<MigrationPlanningReport>(&read(flags.value("plan")?)?)
        .map_err(|error| format!("invalid forge migration planning report JSON: {error}"))?;
    if !planning.valid || !planning.issues.is_empty() {
        return Err("forge migration planning report is not valid".to_owned());
    }
    let plan = planning.plan.as_ref().ok_or_else(|| "valid forge planning report lacks a plan".to_owned())?;
    let observed = serde_json::from_slice::<Vec<ObservedRecordBinding>>(&read(flags.value("observations")?)?)
        .map_err(|error| format!("invalid native forge observations JSON: {error}"))?;
    let receipt = build_forge_binding_receipt(&profile, plan, observed)
        .map_err(|issues| format!("native forge binding is invalid: {}", issue_codes(&issues)))?;
    write_json(flags, &receipt)?;
    Ok(ExitCode::SUCCESS)
}

fn run_verify_forge_binding(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["profile", "receipt"], &[])?;
    let profile = load_forge_profile(flags.value("profile")?)?;
    let receipt = serde_json::from_slice::<MigrationBindingReceipt>(&read(flags.value("receipt")?)?)
        .map_err(|error| format!("invalid forge binding receipt JSON: {error}"))?;
    let issues = verify_forge_binding_receipt(&profile, &receipt);
    let codes = issues.iter().map(|issue| issue.code.clone()).collect::<Vec<_>>();
    print_verification(&codes)?;
    Ok(if issues.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_ci_admission(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["input", "output-root", "output"], &[])?;
    let input = serde_json::from_slice::<CiAdmissionInput>(&read(flags.value("input")?)?)
        .map_err(|error| format!("invalid CI admission input JSON: {error}"))?;
    let report = admit_ci_revision(&input);
    let admitted = report.admitted;
    write_json(flags, &report)?;
    Ok(if admitted {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn run_verify_ci_admission(flags: &FlagSet) -> Result<ExitCode, String> {
    flags.require_known(&["receipt"], &[])?;
    let receipt = serde_json::from_slice::<CiAdmissionReceipt>(&read(flags.value("receipt")?)?)
        .map_err(|error| format!("invalid CI admission receipt JSON: {error}"))?;
    let issues = verify_ci_admission_receipt(&receipt);
    let codes = issues.iter().map(|issue| issue.code.clone()).collect::<Vec<_>>();
    print_verification(&codes)?;
    Ok(if issues.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    })
}

fn load_forge_profile(path: &str) -> Result<ForgeOperationsProfile, String> {
    let profile = parse_forge_profile(&read(path)?).map_err(|error| error.to_string())?;
    let issues = validate_forge_profile(&profile);
    if issues.is_empty() {
        Ok(profile)
    } else {
        Err(format!("invalid forge operations profile: {}", issue_codes(&issues)))
    }
}

fn issue_codes(issues: &[valence_core::radicle_forge_ops::ForgeIssue]) -> String {
    issues.iter().map(|issue| issue.code.as_str()).collect::<Vec<_>>().join(",")
}

fn load_profile(path: &str) -> Result<CobEvidenceProfile, String> {
    let profile = parse_profile(&read(path)?).map_err(|error| error.to_string())?;
    let issues = valence_core::cob_evidence::validate_profile(&profile);
    if issues.is_empty() {
        Ok(profile)
    } else {
        Err(format!(
            "invalid profile: {}",
            issues.into_iter().map(|issue| issue.code).collect::<Vec<_>>().join(",")
        ))
    }
}

fn load_authorization(path: &str) -> Result<CobAuthorizationProfile, String> {
    parse_authorization(&read(path)?).map_err(|error| error.to_string())
}

fn read(path: &str) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|error| format!("cannot read {path}: {error}"))
}

fn open_dir(path: &str) -> Result<Dir, String> {
    Dir::open_ambient_dir(path, ambient_authority()).map_err(|error| format!("cannot open root {path}: {error}"))
}

fn write_json(flags: &FlagSet, value: &impl serde::Serialize) -> Result<(), String> {
    let root_path = flags.value("output-root")?;
    let locator = PathBuf::from(flags.value("output")?);
    validate_output_locator(&locator)?;
    let root = open_dir(root_path)?;
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    root.write(&locator, &bytes)
        .map_err(|error| format!("cannot write output {}: {error}", locator.display()))?;
    println!("{}", String::from_utf8(bytes).map_err(|error| error.to_string())?);
    Ok(())
}

fn print_verification(issues: &[String]) -> Result<(), String> {
    let value = serde_json::json!({"valid": issues.is_empty(), "issue_codes": issues});
    println!("{}", serde_json::to_string_pretty(&value).map_err(|error| error.to_string())?);
    Ok(())
}

fn validate_output_locator(locator: &Path) -> Result<(), String> {
    if locator.as_os_str().is_empty() || locator.is_absolute() {
        return Err("output locator must be non-empty and relative".to_owned());
    }
    if locator.components().any(|component| !matches!(component, Component::Normal(_))) {
        return Err("output locator must not contain parent, root, or current-directory components".to_owned());
    }
    Ok(())
}

#[derive(Debug)]
struct FlagSet {
    values: BTreeMap<String, String>,
    switches: BTreeSet<String>,
}

impl FlagSet {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut values = BTreeMap::new();
        let mut switches = BTreeSet::new();
        let mut index = 0;
        while index < args.len() {
            let flag = args[index]
                .strip_prefix("--")
                .ok_or_else(|| format!("expected flag, found '{}'", args[index]))?
                .to_owned();
            if flag == "execute" {
                if !switches.insert(flag.clone()) {
                    return Err(format!("duplicate flag '--{flag}'"));
                }
                index += 1;
                continue;
            }
            let value_index = index + 1;
            let value = args.get(value_index).ok_or_else(|| format!("flag '--{flag}' requires a value"))?;
            if value.starts_with("--") {
                return Err(format!("flag '--{flag}' requires a value"));
            }
            if values.insert(flag.clone(), value.clone()).is_some() {
                return Err(format!("duplicate flag '--{flag}'"));
            }
            index += 2;
        }
        Ok(Self { values, switches })
    }

    fn require_known(&self, values: &[&str], switches: &[&str]) -> Result<(), String> {
        let known_values = values.iter().copied().collect::<BTreeSet<_>>();
        let known_switches = switches.iter().copied().collect::<BTreeSet<_>>();
        for name in self.values.keys() {
            if !known_values.contains(name.as_str()) {
                return Err(format!("unknown flag '--{name}'"));
            }
        }
        for name in &self.switches {
            if !known_switches.contains(name.as_str()) {
                return Err(format!("unknown switch '--{name}'"));
            }
        }
        Ok(())
    }

    fn value(&self, name: &str) -> Result<&str, String> {
        self.values.get(name).map(String::as_str).ok_or_else(|| format!("missing required flag '--{name}'"))
    }

    fn switch(&self, name: &str) -> bool {
        self.switches.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_accept_values_and_execute_switch() {
        let flags = FlagSet::parse(&["--repository".to_owned(), "repo".to_owned(), "--execute".to_owned()])
            .expect("valid flags");
        assert_eq!(flags.value("repository").expect("repository"), "repo");
        assert!(flags.switch("execute"));
    }

    #[test]
    fn duplicate_unknown_and_escaping_outputs_fail() {
        assert!(
            FlagSet::parse(&[
                "--profile".to_owned(),
                "one".to_owned(),
                "--profile".to_owned(),
                "two".to_owned()
            ])
            .is_err()
        );
        let flags = FlagSet::parse(&["--unknown".to_owned(), "value".to_owned()]).expect("parse unknown flag shape");
        assert!(flags.require_known(&["profile"], &[]).is_err());
        assert!(validate_output_locator(Path::new("../escape.json")).is_err());
        assert!(validate_output_locator(Path::new("/absolute.json")).is_err());
    }
}
