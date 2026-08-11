//! r[impl valence.nostr_identity_evidence.shell_boundary]
use std::path::PathBuf;
use std::process::ExitCode;

use super::CapabilityRootKind;
use super::cap_read_to_string;
use super::cap_write;

const USAGE_ERROR: u8 = 2;
const VALIDATION_ERROR: u8 = 1;

pub(super) fn run(args: &[String]) -> ExitCode {
    let Some(command) = args.first() else {
        return usage("usage: valence nostr-identity validate ...");
    };
    match command.as_str() {
        "validate" => run_validate(&args[1..]),
        other => usage(&format!("unknown nostr-identity command: {other}")),
    }
}

fn run_validate(args: &[String]) -> ExitCode {
    let options = match parse_options(args) {
        Ok(options) => options,
        Err(error) => return usage(&error),
    };
    let input = match cap_read_to_string(CapabilityRootKind::InputArtifact, &options.artifact_root, &options.input) {
        Ok(input) => input,
        Err(error) => return io_error(&format!("cannot read Nostr evidence input: {error}")),
    };
    let parsed = match valence_core::nostr_identity_evidence::parse_nostr_identity_evidence(&input) {
        Ok(parsed) => parsed,
        Err(error) => return io_error(&format!("cannot parse Nostr evidence input: {error}")),
    };
    let report = valence_core::nostr_identity_evidence::validate_nostr_identity_evidence(&parsed);
    let report_json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(error) => return io_error(&format!("cannot serialize Nostr evidence report: {error}")),
    };
    println!("{report_json}");
    if !report.valid {
        return ExitCode::from(VALIDATION_ERROR);
    }
    if let (Some(output_root), Some(output)) = (&options.output_root, &options.output) {
        let artifact = report.artifact.as_ref().expect("valid report has artifact");
        let bytes = match serde_json::to_vec_pretty(artifact) {
            Ok(bytes) => bytes,
            Err(error) => return io_error(&format!("cannot serialize Nostr evidence artifact: {error}")),
        };
        if let Err(error) = cap_write(CapabilityRootKind::Output, output_root, output, &bytes) {
            return io_error(&format!("cannot write Nostr evidence artifact: {error}"));
        }
    }
    ExitCode::SUCCESS
}

#[derive(Debug)]
struct Options {
    artifact_root: PathBuf,
    input: PathBuf,
    output_root: Option<PathBuf>,
    output: Option<PathBuf>,
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut artifact_root = None;
    let mut input = None;
    let mut output_root = None;
    let mut output = None;
    let mut index = 0;
    while index < args.len() {
        let option = &args[index];
        index += 1;
        if index >= args.len() {
            return Err(format!("{option} requires a value"));
        }
        let value = PathBuf::from(&args[index]);
        index += 1;
        match option.as_str() {
            "--artifact-root" => artifact_root = Some(value),
            "--input" => input = Some(value),
            "--output-root" => output_root = Some(value),
            "--out" => output = Some(value),
            other => return Err(format!("unknown nostr-identity option: {other}")),
        }
    }
    if output_root.is_some() != output.is_some() {
        return Err("--output-root and --out must be supplied together".to_owned());
    }
    Ok(Options {
        artifact_root: artifact_root.ok_or("--artifact-root is required")?,
        input: input.ok_or("--input is required")?,
        output_root,
        output,
    })
}

fn usage(message: &str) -> ExitCode {
    eprintln!("{message}");
    eprintln!(
        "usage: valence nostr-identity validate --artifact-root <ROOT> --input <FILE> [--output-root <ROOT> --out <FILE>]"
    );
    ExitCode::from(USAGE_ERROR)
}

fn io_error(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::from(USAGE_ERROR)
}
