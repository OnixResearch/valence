use std::path::PathBuf;
use std::process::ExitCode;

use serde::Serialize;
use serde::de::DeserializeOwned;
use valence_core::content_bound_requirements::CONTENT_BOUND_REQUIREMENT_BOUNDARY;
use valence_core::content_bound_requirements::CairnContentBoundRequirementRegistryV1;
use valence_core::content_bound_requirements::RequirementLinkageInputV1;
use valence_core::content_bound_requirements::RequirementValidationIssueV1;
use valence_core::content_bound_requirements::StackTraceValidationEnvelopeV1;
use valence_core::content_bound_requirements::validate_requirement_linkage;
use valence_core::content_bound_requirements::validate_stack_trace_context;

use super::CapabilityRootKind;
use super::cap_atomic_write;
use super::cap_read_bounded_to_string;

const BYTES_PER_KIBIBYTE: u64 = 1_024;
const KIBIBYTES_PER_MEBIBYTE: u64 = 1_024;
const MAX_INPUT_MEBIBYTES: u64 = 16;
const MAX_REGISTRY_MEBIBYTES: u64 = 64;
const MAX_CONTEXT_MEBIBYTES: u64 = 64;
const MAX_INPUT_BYTES: u64 = MAX_INPUT_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_REGISTRY_BYTES: u64 = MAX_REGISTRY_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_CONTEXT_BYTES: u64 = MAX_CONTEXT_MEBIBYTES * KIBIBYTES_PER_MEBIBYTE * BYTES_PER_KIBIBYTE;
const MAX_FIELD_PATH_CHARACTERS: usize = 512;
const USAGE_EXIT_CODE: u8 = 2;
const VALIDATION_EXIT_CODE: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationOptions {
    artifact_root: PathBuf,
    input: PathBuf,
    registry: Option<PathBuf>,
    output_root: Option<PathBuf>,
    out: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct ParseFailureReport {
    valid: bool,
    issue_count: usize,
    issues: Vec<RequirementValidationIssueV1>,
    boundary: String,
}

pub(super) fn run_requirement_reference(args: &[String]) -> ExitCode {
    // r[impl valence.provenance.requirement_registry_validation]
    let options = match parse_validation_options(args, true) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("usage: {message}");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    };
    let input_json = match cap_read_bounded_to_string(
        CapabilityRootKind::InputArtifact,
        &options.artifact_root,
        &options.input,
        MAX_INPUT_BYTES,
    ) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: requirement reference input read failed: {error}");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    };
    let input = match parse_with_safe_path::<RequirementLinkageInputV1>(&input_json, "input") {
        Ok(input) => input,
        Err(issue) => return print_failure(issue),
    };
    let registries = match read_optional_registry(&options) {
        Ok(registries) => registries,
        Err(exit_code) => return exit_code,
    };
    let report = validate_requirement_linkage(&input, &registries);
    write_valid_report(&options, &report, report.valid)
}

pub(super) fn run_stack_context(args: &[String]) -> ExitCode {
    // r[impl valence.provenance.stack_trace_context]
    let options = match parse_validation_options(args, false) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("usage: {message}");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    };
    let input_json = match cap_read_bounded_to_string(
        CapabilityRootKind::InputArtifact,
        &options.artifact_root,
        &options.input,
        MAX_CONTEXT_BYTES,
    ) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: stack context input read failed: {error}");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    };
    let envelope = match parse_with_safe_path::<StackTraceValidationEnvelopeV1>(&input_json, "input") {
        Ok(envelope) => envelope,
        Err(issue) => return print_failure(issue),
    };
    let report = validate_stack_trace_context(&envelope);
    write_valid_report(&options, &report, report.valid)
}

fn parse_validation_options(args: &[String], allow_registry: bool) -> Result<ValidationOptions, &'static str> {
    let Some(command) = args.first() else {
        return Err("valence requirement validation requires the validate subcommand");
    };
    if command != "validate" {
        return Err("valence requirement validation supports only the validate subcommand");
    }
    let mut artifact_root = None;
    let mut input = None;
    let mut registry = None;
    let mut output_root = None;
    let mut out = None;
    let mut index = 1usize;
    while index < args.len() {
        let option = args[index].as_str();
        index = index.saturating_add(1);
        let Some(value) = args.get(index) else {
            return Err("each validation option requires a value");
        };
        match option {
            "--artifact-root" => artifact_root = Some(PathBuf::from(value)),
            "--input" => input = Some(PathBuf::from(value)),
            "--registry" if allow_registry => registry = Some(PathBuf::from(value)),
            "--output-root" => output_root = Some(PathBuf::from(value)),
            "--out" => out = Some(PathBuf::from(value)),
            _ => return Err("validation command contains an unknown option"),
        }
        index = index.saturating_add(1);
    }
    if output_root.is_some() != out.is_some() {
        return Err("--output-root and --out must be supplied together");
    }
    Ok(ValidationOptions {
        artifact_root: artifact_root.ok_or("validation requires --artifact-root <ROOT>")?,
        input: input.ok_or("validation requires --input <FILE>")?,
        registry,
        output_root,
        out,
    })
}

fn read_optional_registry(
    options: &ValidationOptions,
) -> Result<Vec<CairnContentBoundRequirementRegistryV1>, ExitCode> {
    let Some(registry_path) = options.registry.as_deref() else {
        return Ok(Vec::new());
    };
    let registry_json = match cap_read_bounded_to_string(
        CapabilityRootKind::InputArtifact,
        &options.artifact_root,
        registry_path,
        MAX_REGISTRY_BYTES,
    ) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: requirement registry read failed: {error}");
            return Err(ExitCode::from(USAGE_EXIT_CODE));
        }
    };
    match parse_with_safe_path::<CairnContentBoundRequirementRegistryV1>(&registry_json, "registry") {
        Ok(registry) => Ok(vec![registry]),
        Err(issue) => Err(print_failure(issue)),
    }
}

fn parse_with_safe_path<T: DeserializeOwned>(json: &str, root_field: &str) -> Result<T, RequirementValidationIssueV1> {
    // r[impl valence.provenance.requirement_registry_validation]
    let mut deserializer = serde_json::Deserializer::from_str(json);
    serde_path_to_error::deserialize(&mut deserializer).map_err(|error| {
        let nested = bounded_field_path(&error.path().to_string());
        let field_path = if nested.is_empty() {
            root_field.to_owned()
        } else {
            format!("{root_field}.{nested}")
        };
        RequirementValidationIssueV1 {
            code: "input_parse_failed".to_owned(),
            field_path,
            message: "input does not match the closed typed validation schema".to_owned(),
        }
    })
}

fn bounded_field_path(path: &str) -> String {
    path.chars()
        .filter(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '[' | ']' | '_' | '-'))
        .take(MAX_FIELD_PATH_CHARACTERS)
        .collect()
}

fn print_failure(issue: RequirementValidationIssueV1) -> ExitCode {
    let report = ParseFailureReport {
        valid: false,
        issue_count: 1,
        issues: vec![issue],
        boundary: CONTENT_BOUND_REQUIREMENT_BOUNDARY.to_owned(),
    };
    match serde_json::to_string_pretty(&report) {
        Ok(json) => println!("{json}"),
        Err(_error) => eprintln!("error: validation failure report serialization failed"),
    }
    ExitCode::from(VALIDATION_EXIT_CODE)
}

fn write_valid_report<T: Serialize>(options: &ValidationOptions, report: &T, valid: bool) -> ExitCode {
    let mut bytes = match serde_json::to_vec_pretty(report) {
        Ok(bytes) => bytes,
        Err(_error) => {
            eprintln!("error: validation report serialization failed");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    };
    bytes.push(b'\n');
    if valid {
        match write_optional_report(options, &bytes) {
            Ok(()) => {}
            Err(error) => {
                eprintln!("error: validation report write failed: {error}");
                return ExitCode::from(USAGE_EXIT_CODE);
            }
        }
    }
    match String::from_utf8(bytes) {
        Ok(json) => print!("{json}"),
        Err(_error) => {
            eprintln!("error: validation report output was not UTF-8");
            return ExitCode::from(USAGE_EXIT_CODE);
        }
    }
    if valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(VALIDATION_EXIT_CODE)
    }
}

fn write_optional_report(options: &ValidationOptions, bytes: &[u8]) -> Result<(), String> {
    match (options.output_root.as_deref(), options.out.as_deref()) {
        (Some(output_root), Some(out)) => cap_atomic_write(CapabilityRootKind::Output, output_root, out, bytes),
        (None, None) => Ok(()),
        _ => Err("output root and locator must be supplied together".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_parser_accepts_complete_and_rejects_partial_output() {
        let options = parse_validation_options(
            &[
                "validate".to_owned(),
                "--artifact-root".to_owned(),
                "/tmp".to_owned(),
                "--input".to_owned(),
                "input.json".to_owned(),
                "--registry".to_owned(),
                "registry.json".to_owned(),
                "--output-root".to_owned(),
                "/tmp".to_owned(),
                "--out".to_owned(),
                "report.json".to_owned(),
            ],
            true,
        )
        .expect("valid options");
        assert_eq!(options.input, PathBuf::from("input.json"));
        assert_eq!(options.registry, Some(PathBuf::from("registry.json")));

        let partial = parse_validation_options(
            &[
                "validate".to_owned(),
                "--artifact-root".to_owned(),
                "/tmp".to_owned(),
                "--input".to_owned(),
                "input.json".to_owned(),
                "--out".to_owned(),
                "report.json".to_owned(),
            ],
            true,
        );
        assert!(partial.is_err());
    }

    #[test]
    fn parse_failure_reports_only_a_bounded_field_path() {
        let issue = parse_with_safe_path::<RequirementLinkageInputV1>(
            r#"{"schema":"bad","profile":{"private":"SECRET"}}"#,
            "input",
        )
        .expect_err("malformed nested profile must fail");
        assert_eq!(issue.code, "input_parse_failed");
        assert!(issue.field_path.starts_with("input.profile"));
        assert!(!issue.message.contains("SECRET"));
        assert!(issue.field_path.len() <= MAX_FIELD_PATH_CHARACTERS + "input.".len());
    }
}
