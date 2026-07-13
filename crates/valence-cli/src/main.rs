//! CLI shell for the Valence Evidence IR.
//!
//! This is the imperative shell: argument parsing, file reads, stdout/stderr,
//! and exit codes. All core logic lives in `valence_core`.

use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use serde::Serialize;

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
    function-address validate --input <FILE> [--receipt <FILE>] [--graph <FILE>]
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
        "function-address" => run_function_address(&args[2..]),
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
    let json = match cap_read_to_string(CapabilityRootKind::InputArtifact, &path) {
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

#[derive(Serialize)]
struct FunctionAddressCliOutput {
    valid: bool,
    issue_count: usize,
    issues: Vec<valence_core::function_address_profile::FunctionAddressEvidenceIssue>,
    policy_identity: valence_core::function_address_profile::FunctionAddressDigest,
    artifact: Option<valence_core::evidence_ir::EvidenceArtifact>,
    graph_rows: Vec<valence_core::function_address_profile::FunctionAddressGraphRow>,
    graph_root: String,
    non_claims: Vec<String>,
}

fn run_function_address(args: &[String]) -> ExitCode {
    let Some(command) = args.first() else {
        eprintln!("usage: valence function-address validate --input <FILE> [--receipt <FILE>] [--graph <FILE>]");
        return ExitCode::from(2);
    };
    match command.as_str() {
        "validate" => run_function_address_validate(&args[1..]),
        other => {
            eprintln!("unknown function-address command: {other}");
            ExitCode::from(2)
        }
    }
}

fn run_function_address_validate(args: &[String]) -> ExitCode {
    let mut input_path: Option<PathBuf> = None;
    let mut receipt_path: Option<PathBuf> = None;
    let mut graph_path: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --input requires a value");
                    return ExitCode::from(2);
                }
                input_path = Some(PathBuf::from(&args[i]));
            }
            "--receipt" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --receipt requires a value");
                    return ExitCode::from(2);
                }
                receipt_path = Some(PathBuf::from(&args[i]));
            }
            "--graph" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: --graph requires a value");
                    return ExitCode::from(2);
                }
                graph_path = Some(PathBuf::from(&args[i]));
            }
            other => {
                eprintln!("unknown function-address option: {other}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }
    let Some(input_path) = input_path else {
        eprintln!("error: --input is required");
        return ExitCode::from(2);
    };
    let input_json = match cap_read_to_string(CapabilityRootKind::InputArtifact, &input_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", input_path.display());
            return ExitCode::from(2);
        }
    };
    let input: valence_core::function_address_profile::FunctionAddressEvidenceInput =
        match serde_json::from_str(&input_json) {
            Ok(input) => input,
            Err(error) => {
                eprintln!("error: cannot parse function-address input: {error}");
                return ExitCode::from(2);
            }
        };
    let report = valence_core::function_address_profile::validate_function_address_evidence(&input);
    let graph_rows = valence_core::function_address_profile::function_address_graph_rows(&input);
    let graph_bytes = match serde_json::to_vec(&graph_rows) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("error: cannot serialize function-address graph rows: {error}");
            return ExitCode::from(2);
        }
    };
    let graph_root = valence_core::shared_identity::blake3_hex(&graph_bytes);
    let output = FunctionAddressCliOutput {
        valid: report.valid,
        issue_count: report.issues.len(),
        issues: report.issues,
        policy_identity: valence_core::function_address_profile::function_address_policy_identity(),
        artifact: report.artifact,
        graph_rows,
        graph_root,
        non_claims: input.non_claims,
    };
    let output_json = match serde_json::to_string_pretty(&output) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot serialize function-address report: {error}");
            return ExitCode::from(2);
        }
    };
    if let Some(path) = receipt_path
        && let Err(error) = cap_write(CapabilityRootKind::Output, &path, output_json.as_bytes())
    {
        eprintln!("error: cannot write {}: {error}", path.display());
        return ExitCode::from(2);
    }
    if let Some(path) = graph_path
        && let Err(error) = cap_write(CapabilityRootKind::Output, &path, &graph_bytes)
    {
        eprintln!("error: cannot write {}: {error}", path.display());
        return ExitCode::from(2);
    }
    println!("{output_json}");
    if output.valid {
        ExitCode::SUCCESS
    } else {
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

    let octet_export_json = match cap_read_to_string(CapabilityRootKind::InputArtifact, &octet_export_path) {
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
    if let Err(error) = cap_write(CapabilityRootKind::Output, &output_path, sidecar_json.as_bytes()) {
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
    let manifest_json = match cap_read_to_string(CapabilityRootKind::Manifest, &manifest_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", manifest_path.display());
            return ExitCode::from(2);
        }
    };
    let sidecar_json = match cap_read_to_string(CapabilityRootKind::Sidecar, &sidecar_path) {
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
    let chain_json = match cap_read_to_string(CapabilityRootKind::InputArtifact, &chain_path) {
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

    let binary_bytes = match cap_read(CapabilityRootKind::Binary, &binary_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", binary_path.display());
            return ExitCode::from(2);
        }
    };
    let octet_export_json = match cap_read_to_string(CapabilityRootKind::InputArtifact, &octet_export_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", octet_export_path.display());
            return ExitCode::from(2);
        }
    };
    let sidecar_json = match cap_read_to_string(CapabilityRootKind::Sidecar, &sidecar_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", sidecar_path.display());
            return ExitCode::from(2);
        }
    };
    let manifest_json = match cap_read_to_string(CapabilityRootKind::Manifest, &manifest_path) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CapabilityRootKind {
    InputArtifact,
    Manifest,
    Sidecar,
    Binary,
    Output,
}

impl CapabilityRootKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::InputArtifact => "input artifact",
            Self::Manifest => "manifest",
            Self::Sidecar => "sidecar",
            Self::Binary => "binary",
            Self::Output => "output",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityPath {
    root: PathBuf,
    relative: PathBuf,
}

fn cap_read_to_string(kind: CapabilityRootKind, path: &Path) -> Result<String, String> {
    let (root, relative) = open_capability_root(kind, path)?;
    ensure_existing_path_confined(&root, &relative)?;
    root.read_to_string(&relative).map_err(|error| format!("{} read failed: {error}", kind.as_str()))
}

fn cap_read(kind: CapabilityRootKind, path: &Path) -> Result<Vec<u8>, String> {
    let (root, relative) = open_capability_root(kind, path)?;
    ensure_existing_path_confined(&root, &relative)?;
    root.read(&relative).map_err(|error| format!("{} read failed: {error}", kind.as_str()))
}

fn cap_write(kind: CapabilityRootKind, path: &Path, bytes: &[u8]) -> Result<(), String> {
    let (root, relative) = open_capability_root(kind, path)?;
    ensure_output_parent_confined(&root, &relative)?;
    root.write(&relative, bytes).map_err(|error| format!("{} write failed: {error}", kind.as_str()))
}

fn open_capability_root(kind: CapabilityRootKind, path: &Path) -> Result<(Dir, PathBuf), String> {
    let capability_path = split_operator_path(path)?;
    let root = Dir::open_ambient_dir(&capability_path.root, ambient_authority()).map_err(|error| {
        format!("{} root {} could not be opened: {error}", kind.as_str(), capability_path.root.display())
    })?;
    Ok((root, capability_path.relative))
}

fn split_operator_path(path: &Path) -> Result<CapabilityPath, String> {
    if path.as_os_str().is_empty() {
        return Err("empty path is not a valid capability locator".to_owned());
    }
    let file_name = path.file_name().ok_or_else(|| "path must name a file inside a capability root".to_owned())?;
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let root = if parent.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        parent.to_path_buf()
    };
    let relative = PathBuf::from(file_name);
    validate_capability_relative_path(&relative)?;
    Ok(CapabilityPath { root, relative })
}

fn validate_capability_relative_path(path: &Path) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err("capability-relative path is empty".to_owned());
    }
    let mut saw_normal_component = false;
    for component in path.components() {
        match component {
            Component::Normal(component) if !component.is_empty() => saw_normal_component = true,
            Component::CurDir => {}
            Component::ParentDir => return Err("capability-relative path must not contain parent traversal".to_owned()),
            Component::RootDir | Component::Prefix(_) => {
                return Err("capability-relative path must not be absolute".to_owned());
            }
            Component::Normal(_) => return Err("capability-relative path contains an empty component".to_owned()),
        }
    }
    if !saw_normal_component {
        return Err("capability-relative path must name a file".to_owned());
    }
    Ok(())
}

fn ensure_existing_path_confined(root: &Dir, relative: &Path) -> Result<(), String> {
    validate_capability_relative_path(relative)?;
    let canonical = root
        .canonicalize(relative)
        .map_err(|error| format!("capability path is unavailable or escapes root: {error}"))?;
    validate_capability_relative_path(&canonical)
}

fn ensure_output_parent_confined(root: &Dir, relative: &Path) -> Result<(), String> {
    validate_capability_relative_path(relative)?;
    if let Some(parent) = relative.parent()
        && !parent.as_os_str().is_empty()
    {
        ensure_existing_path_confined(root, parent)?;
    }
    Ok(())
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

    let manifest_json = match cap_read_to_string(CapabilityRootKind::Manifest, &manifest_path) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", manifest_path.display());
            return ExitCode::from(2);
        }
    };

    match valence_core::evidence_ir::explain_span(&manifest_json, &source_id) {
        valence_core::evidence_ir::ExplainSpanResult::Found(span) => {
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
        valence_core::evidence_ir::ExplainSpanResult::NotFound => {
            println!("not_found: {source_id}");
            ExitCode::from(1)
        }
        valence_core::evidence_ir::ExplainSpanResult::InvalidManifest(issues) => {
            for issue in issues {
                eprintln!("invalid_manifest: {}:{}", issue.field, issue.code);
            }
            ExitCode::from(2)
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

    #[test]
    fn capability_roots_read_and_write_in_root() {
        // r[verify valence.stack_provenance_adapter.cap_std_cli_boundary.tests.positive]
        let root = temp_capability_dir("positive");
        let input = root.join("input.json");
        let output = root.join("output.json");
        std::fs::write(&input, "{\"valid\":true}").expect("write input fixture");

        let read = cap_read_to_string(CapabilityRootKind::InputArtifact, &input).expect("read through cap root");
        cap_write(CapabilityRootKind::Output, &output, read.as_bytes()).expect("write through cap root");

        assert_eq!(read, "{\"valid\":true}");
        assert_eq!(std::fs::read_to_string(output).expect("read output fixture"), read);
    }

    #[test]
    fn capability_relative_paths_reject_escape_attempts() {
        // r[verify valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
        assert!(validate_capability_relative_path(Path::new("artifact.json")).is_ok());
        assert!(validate_capability_relative_path(Path::new("../artifact.json")).is_err());
        assert!(validate_capability_relative_path(Path::new("nested/../artifact.json")).is_err());
        assert!(validate_capability_relative_path(Path::new("/tmp/artifact.json")).is_err());
        assert!(validate_capability_relative_path(Path::new("")).is_err());
    }

    #[test]
    fn missing_capability_root_fails_before_read() {
        // r[verify valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
        let root = temp_capability_dir("missing-root");
        let missing_root = root.join("absent").join("input.json");

        let error =
            cap_read_to_string(CapabilityRootKind::Manifest, &missing_root).expect_err("missing root should fail");

        assert!(error.contains("root"));
    }

    #[cfg(unix)]
    #[test]
    fn symlink_escape_is_rejected() {
        // r[verify valence.stack_provenance_adapter.cap_std_cli_boundary.tests.negative]
        let root = temp_capability_dir("symlink-root");
        let outside = temp_capability_dir("symlink-outside");
        let outside_file = outside.join("secret.txt");
        let link = root.join("secret-link.txt");
        std::fs::write(&outside_file, "secret").expect("write outside target");
        std::os::unix::fs::symlink(&outside_file, &link).expect("create symlink escape fixture");

        let error =
            cap_read_to_string(CapabilityRootKind::InputArtifact, &link).expect_err("symlink escape should fail");

        assert!(error.contains("escapes root") || error.contains("absolute"));
    }

    fn temp_capability_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("valence-cap-std-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp capability dir");
        dir
    }
}
