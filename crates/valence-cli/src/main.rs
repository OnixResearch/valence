//! CLI shell for the Valence Evidence IR.
//!
//! This is the imperative shell: argument parsing, file reads, stdout/stderr,
//! and exit codes. All core logic lives in `valence_core`.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const VERSION: &str = "0.1.0";
const USAGE: &str = "\
valence — Evidence IR CLI

USAGE:
    valence <COMMAND> [OPTIONS]

COMMANDS:
    check <FILE>                        Validate a canonical Evidence IR artifact
    explain --id <ID> --manifest <FILE> Resolve a stable source ID to its span
    version                             Print version

EXIT CODES:
    0   success
    1   validation failure or not found
    2   usage error or I/O failure
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprint!("{USAGE}");
        return ExitCode::from(2);
    }

    match args[1].as_str() {
        "version" | "--version" | "-V" => {
            println!("valence {VERSION}");
            ExitCode::SUCCESS
        }
        "check" => run_check(&args[2..]),
        "explain" => run_explain(&args[2..]),
        "help" | "--help" | "-h" => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        command => {
            eprintln!("unknown command: {command}");
            eprint!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

fn run_check(args: &[String]) -> ExitCode {
    if args.len() != 1 {
        eprintln!("usage: valence check <FILE>");
        return ExitCode::from(2);
    }

    let path = PathBuf::from(&args[0]);
    let json = match fs::read_to_string(&path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", path.display());
            return ExitCode::from(2);
        }
    };

    let report = valence_core::evidence_ir::check_evidence_artifact(&json);
    if report.valid {
        println!("valid: true");
        println!("issue_count=0");
        ExitCode::SUCCESS
    } else {
        println!("valid: false");
        println!("issue_count={}", report.issue_codes.len());
        for code in &report.issue_codes {
            println!("issue={code}");
        }
        ExitCode::from(1)
    }
}

fn run_explain(args: &[String]) -> ExitCode {
    let mut source_id: Option<String> = None;
    let mut manifest_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--id" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --id requires a value");
                    return ExitCode::from(2);
                }
                source_id = Some(args[i].clone());
            }
            "--manifest" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --manifest requires a value");
                    return ExitCode::from(2);
                }
                manifest_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(source_id) = source_id else {
        eprintln!("error: --id is required");
        return ExitCode::from(2);
    };
    let Some(manifest_path) = manifest_path else {
        eprintln!("error: --manifest is required");
        return ExitCode::from(2);
    };

    let manifest_json = match fs::read_to_string(&manifest_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", manifest_path.display());
            return ExitCode::from(2);
        }
    };

    match valence_core::evidence_ir::explain_span(&manifest_json, &source_id) {
        Some(span) => {
            println!("source_id: {}", span.source_id);
            println!("source_path: {}", span.source_path);
            println!("start_byte: {}", span.start_byte);
            println!("end_byte: {}", span.end_byte);
            println!("start_line: {}", span.start_line);
            println!("start_column: {}", span.start_column);
            println!("end_line: {}", span.end_line);
            println!("end_column: {}", span.end_column);
            if let Some(label) = &span.label {
                println!("label: {label}");
            }
            ExitCode::SUCCESS
        }
        None => {
            println!("not_found: {source_id}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_command_prints_version() {
        assert!(VERSION.contains("0.1."));
    }

    #[test]
    fn usage_documentation_is_stable() {
        assert!(USAGE.contains("check <FILE>"));
        assert!(USAGE.contains("explain --id <ID> --manifest <FILE>"));
        assert!(USAGE.contains("version"));
        assert!(USAGE.contains("EXIT CODES"));
    }
}
