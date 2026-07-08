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
    stack-provenance build --octet-export <FILE> --binary-hash <B3> --output <FILE>
    stack-provenance verify --manifest <FILE> --sidecar <FILE>
    stack-provenance verify-built-binary --binary <PATH> --octet-export <FILE> --sidecar <FILE> --manifest <FILE> [--json]
    stack-provenance graph-report --chain <FILE> [--format json|markdown]
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
        "stack-provenance" => run_stack_provenance(&args[2..]),
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
        print_non_claims(&report.non_claims);
        ExitCode::SUCCESS
    } else {
        println!("valid: false");
        println!("issue_count={}", report.issue_codes.len());
        for code in &report.issue_codes {
            println!("issue={code}");
        }
        print_non_claims(&report.non_claims);
        ExitCode::from(1)
    }
}

fn run_stack_provenance(args: &[String]) -> ExitCode {
    let Some(command) = args.first() else {
        eprintln!("usage: valence stack-provenance <build|verify> ...");
        return ExitCode::from(2);
    };
    match command.as_str() {
        "build" => run_stack_provenance_build(&args[1..]),
        "verify" => run_stack_provenance_verify(&args[1..]),
        "verify-built-binary" => run_stack_provenance_verify_built_binary(&args[1..]),
        "graph-report" => run_stack_provenance_graph_report(&args[1..]),
        other => {
            eprintln!("unknown stack-provenance command: {other}");
            ExitCode::from(2)
        }
    }
}

fn run_stack_provenance_build(args: &[String]) -> ExitCode {
    let mut octet_export_path: Option<PathBuf> = None;
    let mut binary_hash: Option<String> = None;
    let mut output_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--octet-export" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --octet-export requires a value");
                    return ExitCode::from(2);
                }
                octet_export_path = Some(PathBuf::from(&args[i]));
            }
            "--binary-hash" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --binary-hash requires a value");
                    return ExitCode::from(2);
                }
                binary_hash = Some(args[i].clone());
            }
            "--output" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --output requires a value");
                    return ExitCode::from(2);
                }
                output_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(octet_export_path) = octet_export_path else {
        eprintln!("error: --octet-export is required");
        return ExitCode::from(2);
    };
    let Some(binary_hash) = binary_hash else {
        eprintln!("error: --binary-hash is required");
        return ExitCode::from(2);
    };
    let Some(output_path) = output_path else {
        eprintln!("error: --output is required");
        return ExitCode::from(2);
    };

    let octet_export_json = match fs::read_to_string(&octet_export_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", octet_export_path.display());
            return ExitCode::from(2);
        }
    };
    let sidecar_json = match valence_core::stack_provenance_adapter::build_sidecar_json_from_octet_export_json(
        &octet_export_json,
        &binary_hash,
    ) {
        Ok(sidecar_json) => sidecar_json,
        Err(report) => {
            print_stack_provenance_report(&report);
            return ExitCode::from(1);
        }
    };
    if let Err(error) = fs::write(&output_path, sidecar_json) {
        eprintln!("error: cannot write {}: {error}", output_path.display());
        return ExitCode::from(2);
    }
    println!("valid: true");
    println!("wrote: {}", output_path.display());
    ExitCode::SUCCESS
}

fn run_stack_provenance_verify(args: &[String]) -> ExitCode {
    let mut manifest_path: Option<PathBuf> = None;
    let mut sidecar_path: Option<PathBuf> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--manifest" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --manifest requires a value");
                    return ExitCode::from(2);
                }
                manifest_path = Some(PathBuf::from(&args[i]));
            }
            "--sidecar" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --sidecar requires a value");
                    return ExitCode::from(2);
                }
                sidecar_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(manifest_path) = manifest_path else {
        eprintln!("error: --manifest is required");
        return ExitCode::from(2);
    };
    let Some(sidecar_path) = sidecar_path else {
        eprintln!("error: --sidecar is required");
        return ExitCode::from(2);
    };
    let manifest_json = match fs::read_to_string(&manifest_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", manifest_path.display());
            return ExitCode::from(2);
        }
    };
    let sidecar_json = match fs::read_to_string(&sidecar_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", sidecar_path.display());
            return ExitCode::from(2);
        }
    };
    let report =
        valence_core::stack_provenance_adapter::verify_mantle_external_evidence_sidecar(&manifest_json, &sidecar_json);
    print_stack_provenance_report(&report);
    if report.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn run_stack_provenance_graph_report(args: &[String]) -> ExitCode {
    // r[impl valence.stack_provenance_adapter.graph_report]
    let mut chain_path: Option<PathBuf> = None;
    let mut format = "json".to_owned();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--chain" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --chain requires a value");
                    return ExitCode::from(2);
                }
                chain_path = Some(PathBuf::from(&args[i]));
            }
            "--format" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --format requires a value");
                    return ExitCode::from(2);
                }
                format = args[i].clone();
            }
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(chain_path) = chain_path else {
        eprintln!("error: --chain is required");
        return ExitCode::from(2);
    };
    let chain_json = match fs::read_to_string(&chain_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", chain_path.display());
            return ExitCode::from(2);
        }
    };
    let report = valence_core::stack_provenance_adapter::stack_provenance_graph_report_from_json(&chain_json);
    match format.as_str() {
        "json" => match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("error: cannot serialize graph report: {error}");
                return ExitCode::from(2);
            }
        },
        "markdown" => print!("{}", render_stack_provenance_graph_markdown(&report)),
        other => {
            eprintln!("unsupported graph-report format: {other}");
            return ExitCode::from(2);
        }
    }
    if report.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn run_stack_provenance_verify_built_binary(args: &[String]) -> ExitCode {
    let mut binary_path: Option<PathBuf> = None;
    let mut octet_export_path: Option<PathBuf> = None;
    let mut sidecar_path: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut json_output = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--binary" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --binary requires a value");
                    return ExitCode::from(2);
                }
                binary_path = Some(PathBuf::from(&args[i]));
            }
            "--octet-export" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --octet-export requires a value");
                    return ExitCode::from(2);
                }
                octet_export_path = Some(PathBuf::from(&args[i]));
            }
            "--sidecar" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --sidecar requires a value");
                    return ExitCode::from(2);
                }
                sidecar_path = Some(PathBuf::from(&args[i]));
            }
            "--manifest" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --manifest requires a value");
                    return ExitCode::from(2);
                }
                manifest_path = Some(PathBuf::from(&args[i]));
            }
            "--json" => json_output = true,
            other => {
                eprintln!("unknown option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }

    let Some(binary_path) = binary_path else {
        eprintln!("error: --binary is required");
        return ExitCode::from(2);
    };
    let Some(octet_export_path) = octet_export_path else {
        eprintln!("error: --octet-export is required");
        return ExitCode::from(2);
    };
    let Some(sidecar_path) = sidecar_path else {
        eprintln!("error: --sidecar is required");
        return ExitCode::from(2);
    };
    let Some(manifest_path) = manifest_path else {
        eprintln!("error: --manifest is required");
        return ExitCode::from(2);
    };

    let binary_bytes = match fs::read(&binary_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", binary_path.display());
            return ExitCode::from(2);
        }
    };
    let octet_export_json = match fs::read_to_string(&octet_export_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", octet_export_path.display());
            return ExitCode::from(2);
        }
    };
    let sidecar_json = match fs::read_to_string(&sidecar_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", sidecar_path.display());
            return ExitCode::from(2);
        }
    };
    let manifest_json = match fs::read_to_string(&manifest_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", manifest_path.display());
            return ExitCode::from(2);
        }
    };

    let report = valence_core::stack_provenance_adapter::verify_built_binary_thread_json(
        &binary_bytes,
        &octet_export_json,
        &sidecar_json,
        &manifest_json,
    );
    if json_output {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("error: cannot serialize report: {error}");
                return ExitCode::from(2);
            }
        }
    } else {
        print_built_binary_thread_report(&report);
    }
    if report.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn print_non_claims(non_claims: &[String]) {
    println!("non_claim_count={}", non_claims.len());
    for claim in non_claims {
        println!("non_claim={claim}");
    }
}

fn print_stack_provenance_report(report: &valence_core::stack_provenance_adapter::StackProvenanceReport) {
    println!("valid: {}", report.valid);
    println!("issue_count={}", report.issue_codes.len());
    for code in &report.issue_codes {
        println!("issue={code}");
    }
}

fn render_stack_provenance_graph_markdown(
    report: &valence_core::stack_provenance_adapter::StackProvenanceGraphReport,
) -> String {
    // r[impl valence.stack_provenance_adapter.graph_report]
    let mut output = String::new();
    output.push_str("# Stack provenance graph report\n\n");
    output.push_str(&format!("- valid: {}\n", report.valid));
    output.push_str(&format!("- artifacts: {}\n", report.artifact_count));
    output.push_str(&format!("- edges: {}\n", report.edge_count));
    output.push_str(&format!("- report_hash: {}\n", report.report_hash));
    output.push_str(&format!("- supported_claim: {}\n", report.supported_claim));
    output.push_str("\n## Diagnostics\n");
    if report.diagnostics.is_empty() {
        output.push_str("- none\n");
    } else {
        for diagnostic in &report.diagnostics {
            output.push_str(&format!("- {diagnostic}\n"));
        }
    }
    output.push_str("\n## Trust boundaries\n");
    for boundary in &report.trust_boundaries {
        output.push_str(&format!("- {boundary}\n"));
    }
    output.push_str("\n## Manual-review nodes\n");
    if report.manual_nodes.is_empty() {
        output.push_str("- none\n");
    } else {
        for node in &report.manual_nodes {
            output.push_str(&format!("- {node}\n"));
        }
    }
    output.push_str("\n## Non-claims\n");
    for non_claim in &report.non_claims {
        output.push_str(&format!("- {non_claim}\n"));
    }
    output
}

fn print_built_binary_thread_report(report: &valence_core::stack_provenance_adapter::BuiltBinaryThreadReport) {
    println!("valid: {}", report.valid);
    println!("issue_count={}", report.issue_codes.len());
    println!("binary_hash={}", report.binary_hash);
    println!("sidecar_digest_blake3={}", report.sidecar_digest_blake3);
    println!("preserves_content_hash={}", report.preserves_content_hash);
    for code in &report.issue_codes {
        println!("issue={code}");
    }
    for non_claim in &report.non_claims {
        println!("non_claim={non_claim}");
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
        assert!(USAGE.contains("stack-provenance build"));
        assert!(USAGE.contains("stack-provenance verify"));
        assert!(USAGE.contains("stack-provenance verify-built-binary"));
        assert!(USAGE.contains("version"));
        assert!(USAGE.contains("EXIT CODES"));
    }
}
