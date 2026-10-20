// SPDX-License-Identifier: AGPL-3.0-or-later
//! Capability-root shell for independently authenticated source-review receipts.

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

use valence_core::source_review::MAX_INPUT_BYTES;
use valence_core::source_review::SourceReviewOutcome;
use valence_core::source_review::accept_source_review_bytes;

use super::CapabilityRootKind;
use super::cap_read_bounded_to_string;
use super::ensure_output_parent_confined;
use super::open_capability_root;

const USAGE: &str = "usage: valence source-review verify --artifact-root <ROOT> --attachment <FILE> --policy <FILE> --output-root <ROOT> --receipt <FILE>";

pub(super) fn run(args: &[String]) -> ExitCode {
    match execute(args) {
        Ok(outcome) => {
            if outcome == SourceReviewOutcome::AcceptedManualReview {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

fn execute(args: &[String]) -> Result<SourceReviewOutcome, String> {
    if args.first().map(String::as_str) != Some("verify") || args.len() != 11 {
        return Err(USAGE.to_owned());
    }
    let mut options = BTreeMap::new();
    for pair in args[1..].chunks_exact(2) {
        if ![
            "--artifact-root",
            "--attachment",
            "--policy",
            "--output-root",
            "--receipt",
        ]
        .contains(&pair[0].as_str())
            || options.insert(pair[0].as_str(), pair[1].as_str()).is_some()
        {
            return Err(USAGE.to_owned());
        }
    }
    let get = |name| options.get(name).copied().ok_or_else(|| USAGE.to_owned());
    let root = Path::new(get("--artifact-root")?);
    let attachment = cap_read_bounded_to_string(
        CapabilityRootKind::InputArtifact,
        root,
        Path::new(get("--attachment")?),
        MAX_INPUT_BYTES as u64,
    )?;
    let policy = cap_read_bounded_to_string(
        CapabilityRootKind::InputArtifact,
        root,
        Path::new(get("--policy")?),
        MAX_INPUT_BYTES as u64,
    )?;
    let receipt = accept_source_review_bytes(attachment.as_bytes(), policy.as_bytes());
    let bytes =
        serde_json::to_vec_pretty(&receipt).map_err(|error| format!("cannot encode review receipt: {error}"))?;
    let (output_root, relative) = open_capability_root(
        CapabilityRootKind::Output,
        Path::new(get("--output-root")?),
        Path::new(get("--receipt")?),
        true,
    )?;
    ensure_output_parent_confined(&output_root, &relative)?;
    let mut open = cap_std::fs::OpenOptions::new();
    open.write(true).create_new(true);
    let mut file = output_root
        .open_with(&relative, &open)
        .map_err(|error| format!("cannot create new review receipt: {error}"))?;
    file.write_all(&bytes).map_err(|error| format!("cannot write review receipt: {error}"))?;
    file.sync_all().map_err(|error| format!("cannot flush review receipt: {error}"))?;
    println!("{}", String::from_utf8(bytes).map_err(|error| format!("cannot render review receipt: {error}"))?);
    Ok(receipt.outcome)
}
