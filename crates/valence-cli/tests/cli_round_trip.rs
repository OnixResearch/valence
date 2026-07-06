//! CLI round-trip integration test: build a valid artifact, write it to a
//! temp file, run `valence check` on it, and assert it passes. This is the
//! end-to-end smoke for the Evidence IR v1 spine.

use std::fs;
use std::io::Write;
use std::process::Command;

use valence_core::evidence_ir::EvidenceSpan;
use valence_core::evidence_ir::REQUIRED_NON_CLAIM;
use valence_core::evidence_ir::VerificationRole;
use valence_core::evidence_ir::build_evidence_artifact;

fn valence_binary() -> String {
    env!("CARGO_BIN_EXE_valence").to_owned()
}

#[test]
fn check_accepts_library_emitted_artifact() {
    let artifact = build_evidence_artifact(
        "function-object-v1",
        "eq:bs_001",
        Some(EvidenceSpan {
            source_id: "eq:bs_001".to_owned(),
            source_path: "src/pricing.rs".to_owned(),
            start_byte: 142,
            end_byte: 168,
            start_line: 7,
            start_column: 12,
            end_line: 7,
            end_column: 38,
            label: Some("eq:bs".to_owned()),
        }),
        VerificationRole::RecordedOnly,
        serde_json::json!({"function": "black_scholes_call", "body_hash": "b3:abc123"}),
        vec![
            "evidence artifact records identity and linkage only".to_owned(),
            REQUIRED_NON_CLAIM.to_owned(),
        ],
    )
    .expect("build artifact");

    let json = serde_json::to_string_pretty(&artifact).expect("serialize artifact");

    let mut tmp = fs::File::create("/tmp/valence-cli-valid-artifact.json").expect("create temp file");
    tmp.write_all(json.as_bytes()).expect("write temp file");

    let output = Command::new(valence_binary())
        .arg("check")
        .arg("/tmp/valence-cli-valid-artifact.json")
        .output()
        .expect("run valence check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "check should succeed: {stdout}");
    assert!(stdout.contains("valid: true"), "stdout should report valid: {stdout}");
    assert!(stdout.contains("issue_count=0"), "stdout should report zero issues: {stdout}");
}

#[test]
fn check_rejects_overclaim_artifact() {
    let artifact = build_evidence_artifact(
        "test",
        "overclaim_test",
        None,
        VerificationRole::Property,
        serde_json::json!({"claim": "proves semantic equivalence"}),
        vec!["records identity only".to_owned(), REQUIRED_NON_CLAIM.to_owned()],
    )
    .expect("build artifact");

    let json = serde_json::to_string(&artifact).expect("serialize artifact");

    let mut tmp = fs::File::create("/tmp/valence-cli-overclaim-artifact.json").expect("create temp file");
    tmp.write_all(json.as_bytes()).expect("write temp file");

    let output = Command::new(valence_binary())
        .arg("check")
        .arg("/tmp/valence-cli-overclaim-artifact.json")
        .output()
        .expect("run valence check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "check should fail for overclaim: {stdout}");
    assert!(stdout.contains("valid: false"), "stdout should report invalid: {stdout}");
    assert!(stdout.contains("overclaim_detected"), "stdout should report overclaim_detected: {stdout}");
}

#[test]
fn explain_resolves_known_id_from_manifest() {
    let span = EvidenceSpan {
        source_id: "eq:bs_001".to_owned(),
        source_path: "src/pricing.rs".to_owned(),
        start_byte: 142,
        end_byte: 168,
        start_line: 7,
        start_column: 12,
        end_line: 7,
        end_column: 38,
        label: Some("eq:bs".to_owned()),
    };
    let manifest = serde_json::to_string_pretty(&[span]).expect("serialize manifest");

    let mut tmp = fs::File::create("/tmp/valence-cli-spans.json").expect("create temp file");
    tmp.write_all(manifest.as_bytes()).expect("write temp file");

    let output = Command::new(valence_binary())
        .arg("explain")
        .arg("--id")
        .arg("eq:bs_001")
        .arg("--manifest")
        .arg("/tmp/valence-cli-spans.json")
        .output()
        .expect("run valence explain");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "explain should succeed: {stdout}");
    assert!(stdout.contains("source_id: eq:bs_001"), "stdout should report source_id: {stdout}");
    assert!(stdout.contains("source_path: src/pricing.rs"), "stdout should report path: {stdout}");
    assert!(stdout.contains("label: eq:bs"), "stdout should report label: {stdout}");
}

#[test]
fn explain_reports_not_found_for_unknown_id() {
    let span = EvidenceSpan {
        source_id: "eq:bs_001".to_owned(),
        source_path: "src/pricing.rs".to_owned(),
        start_byte: 142,
        end_byte: 168,
        start_line: 7,
        start_column: 12,
        end_line: 7,
        end_column: 38,
        label: None,
    };
    let manifest = serde_json::to_string(&[span]).expect("serialize manifest");

    let mut tmp = fs::File::create("/tmp/valence-cli-spans-notfound.json").expect("create temp file");
    tmp.write_all(manifest.as_bytes()).expect("write temp file");

    let output = Command::new(valence_binary())
        .arg("explain")
        .arg("--id")
        .arg("nonexistent")
        .arg("--manifest")
        .arg("/tmp/valence-cli-spans-notfound.json")
        .output()
        .expect("run valence explain");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!output.status.success(), "explain should fail for unknown id: {stdout}");
    assert!(stdout.contains("not_found: nonexistent"), "stdout should report not_found: {stdout}");
}

#[test]
fn version_command_succeeds() {
    let output = Command::new(valence_binary()).arg("version").output().expect("run valence version");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "version should succeed: {stdout}");
    assert!(stdout.contains("valence"), "stdout should contain 'valence': {stdout}");
}
