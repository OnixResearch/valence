//! Closed process-boundary DTOs for the native Radicle mutation shell.
//!
//! This crate intentionally does not depend on `valence-core`: upstream Radicle
//! enables `serde_json/preserve_order`, while Valence canonical identities require
//! the default sorted-map backend. JSON is the explicit boundary between them.

use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

pub const PLAN_SCHEMA: &str = "valence.radicle-forge-operations.plan.v1";
pub const ISSUE_COB_TYPE: &str = "xyz.radicle.issue";
pub const PATCH_COB_TYPE: &str = "xyz.radicle.patch";
pub const GIT_OID_HEX_LENGTH: usize = 40;
pub const ATTRIBUTION_HEADER: &str = "<!-- valence-radicle-forge-import:v1 -->";
pub const BLAKE3_HEX_LENGTH: usize = 64;
pub const MIGRATION_NON_CLAIM: &str = "migration records normalized source metadata and observed native COB state; it does not prove source export completeness, source actor authenticity, original signature or timestamp preservation, or semantic equivalence";
pub const REVIEW_NON_CLAIM: &str = "imported review metadata is recorded attribution only and does not create Radicle approval, merge eligibility, delegate authority, or canonical-ref authority";
pub const ADMISSION_NON_CLAIM: &str = "CI admission is bounded input for an external guarded compare-and-swap; it is not protocol-enforced mandatory CI, bypass-proof delegate behavior, canonical mutation, CI correctness, or release readiness";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationPlanningReport {
    pub schema: String,
    pub valid: bool,
    pub issues: Vec<ForgeIssue>,
    pub plan: Option<MigrationPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgeIssue {
    pub code: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationPlan {
    pub schema: String,
    pub profile_blake3: String,
    pub source_batch_blake3: String,
    pub plan_blake3: String,
    pub source_system: String,
    pub repository_url: String,
    pub repository_rid: String,
    pub importer_did: String,
    pub records: Vec<PlannedRecord>,
    pub required_non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedRecord {
    pub source_key: String,
    pub source_record_blake3: String,
    pub target_type: String,
    pub decision: MigrationDecision,
    pub operations: Vec<PlannedOperation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationDecision {
    Create,
    VerifyExisting { target_object_id: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum PlannedOperation {
    CreateIssue {
        title: String,
        description: String,
        labels: Vec<String>,
        state: IssueSourceState,
    },
    CreatePatch {
        title: String,
        description: String,
        labels: Vec<String>,
        lifecycle: PatchLifecycle,
        base_oid: String,
        head_oid: String,
        target_ref: String,
    },
    AppendComment {
        event_id: String,
        body: String,
    },
    VerifyExisting {
        target_object_id: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueSourceState {
    Open,
    Closed,
    Solved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchLifecycle {
    Open,
    Draft,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservedRecordBinding {
    pub source_key: String,
    pub source_record_blake3: String,
    pub target_type: String,
    pub target_object_id: String,
    pub entry_oids: Vec<String>,
    pub observed_state_blake3: String,
}

pub fn validate_plan(plan: &MigrationPlan) -> Result<(), String> {
    if plan.schema != PLAN_SCHEMA {
        return Err("plan schema is unsupported".to_owned());
    }
    if !blake3_digest(&plan.profile_blake3)
        || !blake3_digest(&plan.source_batch_blake3)
        || !blake3_digest(&plan.plan_blake3)
    {
        return Err("plan identity contains an invalid BLAKE3 digest".to_owned());
    }
    if plan.source_system.trim().is_empty()
        || !plan.repository_url.starts_with("https://")
        || !plan.repository_rid.starts_with("rad:z")
        || !plan.importer_did.starts_with("did:key:z")
    {
        return Err("plan source, repository, or importer identity is invalid".to_owned());
    }
    let required = [MIGRATION_NON_CLAIM, REVIEW_NON_CLAIM, ADMISSION_NON_CLAIM];
    if plan.required_non_claims.iter().map(String::as_str).collect::<Vec<_>>() != required {
        return Err("plan non-claim boundary is invalid".to_owned());
    }
    if plan.records.is_empty() {
        return Err("plan has no records".to_owned());
    }
    let mut previous = None;
    let mut source_keys = BTreeSet::new();
    for record in &plan.records {
        if record.source_key.trim().is_empty()
            || !source_keys.insert(record.source_key.as_str())
            || !blake3_digest(&record.source_record_blake3)
        {
            return Err("plan record identity is invalid or duplicated".to_owned());
        }
        if previous.is_some_and(|value: &str| value >= record.source_key.as_str()) {
            return Err("plan records are not strictly source-key ordered".to_owned());
        }
        previous = Some(record.source_key.as_str());
        validate_record(record)?;
    }
    let expected = expected_plan_blake3(plan)?;
    if plan.plan_blake3 != expected {
        return Err("plan BLAKE3 does not bind its payload".to_owned());
    }
    Ok(())
}

fn validate_record(record: &PlannedRecord) -> Result<(), String> {
    match (&record.decision, record.operations.as_slice()) {
        (MigrationDecision::Create, [PlannedOperation::CreateIssue { .. }, comments @ ..])
            if record.target_type == ISSUE_COB_TYPE && comments_are_valid(comments) =>
        {
            Ok(())
        }
        (
            MigrationDecision::Create,
            [
                PlannedOperation::CreatePatch {
                    base_oid,
                    head_oid,
                    target_ref,
                    ..
                },
                comments @ ..,
            ],
        ) if record.target_type == PATCH_COB_TYPE
            && git_oid(base_oid)
            && git_oid(head_oid)
            && target_ref == "refs/heads/main"
            && comments_are_valid(comments) =>
        {
            Ok(())
        }
        (
            MigrationDecision::VerifyExisting { target_object_id },
            [
                PlannedOperation::VerifyExisting {
                    target_object_id: operation_target,
                },
            ],
        ) if (record.target_type == ISSUE_COB_TYPE || record.target_type == PATCH_COB_TYPE)
            && target_object_id == operation_target
            && git_oid(target_object_id) =>
        {
            Ok(())
        }
        _ => Err(format!("plan record {} has an invalid decision/operation shape", record.source_key)),
    }
}

fn comments_are_valid(operations: &[PlannedOperation]) -> bool {
    let mut event_ids = BTreeSet::new();
    operations.iter().all(|operation| {
        if let PlannedOperation::AppendComment { event_id, body } = operation {
            !event_id.trim().is_empty() && !body.trim().is_empty() && event_ids.insert(event_id.as_str())
        } else {
            false
        }
    })
}

pub fn expected_plan_blake3(plan: &MigrationPlan) -> Result<String, String> {
    canonical_blake3(&(
        PLAN_SCHEMA,
        &plan.profile_blake3,
        &plan.source_batch_blake3,
        &plan.source_system,
        &plan.repository_url,
        &plan.repository_rid,
        &plan.importer_did,
        &plan.records,
        &plan.required_non_claims,
    ))
}

fn canonical_blake3<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_vec(value)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| format!("cannot serialize plan identity payload: {error}"))
}

fn blake3_digest(value: &str) -> bool {
    lower_hex(value, BLAKE3_HEX_LENGTH)
}

fn git_oid(value: &str) -> bool {
    lower_hex(value, GIT_OID_HEX_LENGTH)
}

fn lower_hex(value: &str, expected_length: usize) -> bool {
    value.len() == expected_length && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_fixture_plan_is_valid() {
        let report = serde_json::from_slice::<MigrationPlanningReport>(include_bytes!(
            "../../../fixtures/radicle-forge-operations/migration-plan.json"
        ))
        .expect("fixture report");
        assert!(report.valid);
        assert!(report.issues.is_empty());
        assert_eq!(validate_plan(report.plan.as_ref().expect("fixture plan")), Ok(()));
    }

    #[test]
    fn mutated_plan_identity_fails_closed() {
        let report = serde_json::from_slice::<MigrationPlanningReport>(include_bytes!(
            "../../../fixtures/radicle-forge-operations/migration-plan.json"
        ))
        .expect("fixture report");
        let mut plan = report.plan.expect("fixture plan");
        plan.records[0].source_key.push_str(":mutated");
        assert_eq!(validate_plan(&plan), Err("plan BLAKE3 does not bind its payload".to_owned()));
    }
}
