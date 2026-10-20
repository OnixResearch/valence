// SPDX-License-Identifier: AGPL-3.0-or-later
//! Exercise real capability-root receipt creation and failure boundaries.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Root(PathBuf);

impl Root {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "valence-source-review-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).expect("create isolated test capability root");
        fs::write(path.join("attachment.json"), b"{}").unwrap();
        fs::write(path.join("policy.json"), b"{}").unwrap();
        Self(path)
    }

    fn run(&self, attachment: &str, receipt: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_valence"))
            .args(["source-review", "verify", "--artifact-root"])
            .arg(&self.0)
            .args(["--attachment", attachment, "--policy", "policy.json", "--output-root"])
            .arg(&self.0)
            .args(["--receipt", receipt])
            .output()
            .expect("run actual review CLI")
    }
}

impl Drop for Root {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove isolated test root");
    }
}

#[test]
fn invalid_evidence_publishes_invalid_receipt_without_overwriting_existing_files() {
    let root = Root::new();
    let result = root.run("attachment.json", "receipt.json");
    assert_eq!(result.status.code(), Some(1));
    let receipt: serde_json::Value = serde_json::from_slice(&fs::read(root.0.join("receipt.json")).unwrap()).unwrap();
    assert_eq!(receipt["schema"], "valence.source-review-acceptance.v1");
    assert_eq!(receipt["outcome"], "invalid");
    assert_eq!(receipt["verification_role"], "manual_review");
    assert_eq!(receipt["attachment_artifact_blake3"], blake3::hash(b"{}").to_hex().as_str());
    fs::write(root.0.join("receipt.json"), b"existing receipt sentinel").unwrap();
    let result = root.run("attachment.json", "receipt.json");
    assert_eq!(result.status.code(), Some(2));
    assert_eq!(fs::read(root.0.join("receipt.json")).unwrap(), b"existing receipt sentinel");
}

#[test]
fn traversal_and_byte_bound_fail_before_output_creation() {
    let root = Root::new();
    for locator in ["../attachment.json", "/etc/passwd"] {
        let result = root.run(locator, "receipt.json");
        assert_eq!(result.status.code(), Some(2));
        assert!(!root.0.join("receipt.json").exists());
    }
    fs::write(root.0.join("oversized.json"), vec![b' '; valence_core::source_review::MAX_INPUT_BYTES + 1]).unwrap();
    let result = root.run("oversized.json", "receipt.json");
    assert_eq!(result.status.code(), Some(2));
    assert!(!root.0.join("receipt.json").exists());
    let result = root.run("attachment.json", "../escaped.json");
    assert_eq!(result.status.code(), Some(2));
    assert!(!root.0.join("receipt.json").exists());
}
