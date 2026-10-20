// SPDX-License-Identifier: MPL-2.0
//! Independent source-review authentication over explicit loaded bytes.
//!
//! Acceptance records manual review only; it is never Evidence IR `Property`, a
//! build witness, a release signature, or a release-eligibility decision.

use std::collections::BTreeSet;

use artifact_auth_core::ArtifactRef;
use artifact_auth_core::ArtifactStatement;
use artifact_auth_core::AuthenticationPolicy;
use artifact_auth_core::AuthenticationScope;
use serde::Deserialize;
use serde::Serialize;

use crate::shared_identity::canonical_json_bytes;

pub const SUBJECT_SCHEMA: &str = "cairn.source-review-subject.v1";
pub const CLAIM_SCHEMA: &str = "cairn.source-review-claim.v1";
pub const ATTACHMENT_SCHEMA: &str = "cairn.source-review-attachment.v1";
pub const POLICY_SCHEMA: &str = "cairn.source-review-verification-policy.v1";
pub const ACCEPTANCE_SCHEMA: &str = "valence.source-review-acceptance.v1";
pub const REVIEW_DOMAIN: &str = "cairn.source-review.v1";
pub const REVIEW_PURPOSE: &str = "source-review-approval";
pub const MAX_INPUT_BYTES: usize = 1024 * 1024;
pub const MAX_ROWS: usize = 64;
pub const MAX_TEXT_BYTES: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReviewSubject {
    pub schema: String,
    pub source_archive_blake3: String,
    pub source_revision: Option<String>,
    pub source_tree_blake3: Option<String>,
    pub snapshot_profile: String,
    pub snapshot_profile_version: u32,
    pub projection: String,
}

/// Existing Cairn claim-DAG root, not the source-review subject's logical hash.
/// The signed claim binds both this exact root and the independently matched source subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimRoot {
    pub claim_identity: String,
    pub subject_root: String,
    pub revision: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDisposition {
    Approved,
    NeedsRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReviewClaim {
    pub schema: String,
    pub subject: SourceReviewSubject,
    pub claim_root: ClaimRoot,
    pub review_policy_blake3: String,
    pub disposition: ReviewDisposition,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedSourceReview {
    pub claim: SourceReviewClaim,
    pub statement: ArtifactStatement,
    pub generation: u64,
    pub public_key_hex: String,
    pub signature_hex: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReviewAttachment {
    pub schema: String,
    pub reviews: Vec<SignedSourceReview>,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReviewVerificationPolicy {
    pub schema: String,
    pub subject: SourceReviewSubject,
    pub claim_root: ClaimRoot,
    pub review_policy_blake3: String,
    pub authentication_policy: AuthenticationPolicy,
    pub verifier_context: ArtifactRef,
    pub author_key_blake3: Vec<String>,
    pub exclude_authors: bool,
    pub required_approvals: u16,
    pub non_claims: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceReviewOutcome {
    AcceptedManualReview,
    Insufficient,
    Invalid,
}

/// Separate from Evidence IR verification roles by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceReviewRole {
    ManualReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceReviewAcceptance {
    pub schema: String,
    pub outcome: SourceReviewOutcome,
    pub verification_role: SourceReviewRole,
    pub subject: Option<SourceReviewSubject>,
    pub subject_identity: Option<String>,
    pub claim_root: Option<ClaimRoot>,
    pub claim_root_identity: Option<String>,
    pub review_policy_blake3: Option<String>,
    pub verification_policy_identity: Option<String>,
    pub authentication_policy_identity: Option<String>,
    pub approved_key_blake3: Vec<String>,
    pub signed_statement_blake3: Vec<String>,
    pub claim_identity: Vec<String>,
    pub attachment_identity: Option<String>,
    pub attachment_artifact_blake3: String,
    pub policy_artifact_blake3: String,
    pub receipt_blake3: String,
    pub non_claims: Vec<String>,
    pub issues: Vec<String>,
}

#[must_use]
pub fn required_non_claims() -> Vec<String> {
    let mut claims = [
        "source-review-does-not-prove-source-correctness",
        "source-review-does-not-prove-review-quality",
        "source-review-does-not-prove-build-correctness",
        "source-review-does-not-prove-reproducibility",
        "source-review-does-not-prove-witness-independence",
        "source-review-does-not-prove-deployment-safety",
        "source-review-does-not-prove-release-eligibility",
    ]
    .map(str::to_owned)
    .to_vec();
    claims.sort();
    claims
}

fn identity<T: Serialize>(domain: &str, value: &T) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|_| "canonical_serialization_failed".to_owned())?;
    let bytes = canonical_json_bytes(&value).map_err(|_| "canonical_serialization_failed".to_owned())?;
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    hasher.update(&bytes);
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn subject_identity(subject: &SourceReviewSubject) -> Result<String, String> {
    validate_subject(subject)?;
    identity(SUBJECT_SCHEMA, subject)
}

pub fn claim_identity(claim: &SourceReviewClaim) -> Result<String, String> {
    validate_claim(claim)?;
    identity(CLAIM_SCHEMA, claim)
}

/// Valence's explicit logical claim-root identity, distinct from the root fields.
pub fn claim_root_identity(root: &ClaimRoot) -> Result<String, String> {
    validate_root(root)?;
    identity("cairn.claim-root.v1", root)
}

/// Full verification-policy logical identity, not the reviewed policy digest.
/// Set-like policy vectors are normalized; exact loaded bytes remain separate.
pub fn verification_policy_identity(policy: &SourceReviewVerificationPolicy) -> Result<String, String> {
    validate_policy(policy)?;
    let mut normalized = policy.clone();
    normalized.author_key_blake3.sort();
    normalized
        .authentication_policy
        .trusted_keys
        .sort_by(|left, right| left.key_identity.cmp(&right.key_identity));
    for key in &mut normalized.authentication_policy.trusted_keys {
        key.allowed_purposes.sort();
    }
    identity(POLICY_SCHEMA, &normalized)
}

pub fn attachment_identity(attachment: &SourceReviewAttachment) -> Result<String, String> {
    validate_attachment(attachment)?;
    let mut rows = Vec::with_capacity(attachment.reviews.len());
    for review in &attachment.reviews {
        let statement =
            artifact_auth_core::statement_identity(&review.statement).map_err(|_| "statement_invalid".to_owned())?;
        rows.push((review.statement.key_identity.digest_hex.as_str(), statement, review.clone()));
    }
    rows.sort_by(|left, right| (&left.0, &left.1).cmp(&(&right.0, &right.1)));
    let normalized = SourceReviewAttachment {
        schema: attachment.schema.clone(),
        reviews: rows.into_iter().map(|(_, _, review)| review).collect(),
        non_claims: attachment.non_claims.clone(),
    };
    identity(ATTACHMENT_SCHEMA, &normalized)
}

fn digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_subject(subject: &SourceReviewSubject) -> Result<(), String> {
    if subject.schema != SUBJECT_SCHEMA
        || subject.snapshot_profile != "mantle-release-source"
        || subject.snapshot_profile_version != 1
        || subject.projection != "source"
    {
        return Err("subject_profile_invalid".to_owned());
    }
    if !digest(&subject.source_archive_blake3)
        || subject.source_tree_blake3.as_deref().is_some_and(|value| !digest(value))
    {
        return Err("subject_digest_invalid".to_owned());
    }
    if subject
        .source_revision
        .as_deref()
        .is_some_and(|value| value.is_empty() || value.len() > MAX_TEXT_BYTES || value.chars().any(char::is_control))
    {
        return Err("source_revision_invalid".to_owned());
    }
    Ok(())
}

fn validate_root(root: &ClaimRoot) -> Result<(), String> {
    if !digest(&root.claim_identity) || !digest(&root.subject_root) {
        return Err("claim_root_invalid".to_owned());
    }
    Ok(())
}

fn validate_claim(claim: &SourceReviewClaim) -> Result<(), String> {
    if claim.schema != CLAIM_SCHEMA || claim.non_claims != required_non_claims() {
        return Err("claim_schema_or_non_claims_invalid".to_owned());
    }
    validate_subject(&claim.subject)?;
    validate_root(&claim.claim_root)?;
    if !digest(&claim.review_policy_blake3) {
        return Err("review_policy_digest_invalid".to_owned());
    }
    Ok(())
}

fn validate_policy(policy: &SourceReviewVerificationPolicy) -> Result<(), String> {
    if policy.schema != POLICY_SCHEMA || policy.non_claims != required_non_claims() {
        return Err("policy_schema_or_non_claims_invalid".to_owned());
    }
    validate_subject(&policy.subject)?;
    validate_root(&policy.claim_root)?;
    if !digest(&policy.review_policy_blake3)
        || policy.required_approvals == 0
        || usize::from(policy.required_approvals) > MAX_ROWS
        || policy.required_approvals != policy.authentication_policy.threshold
        || policy.authentication_policy.profile_id != REVIEW_DOMAIN
    {
        return Err("policy_profile_or_threshold_invalid".to_owned());
    }
    let authors = policy.author_key_blake3.iter().collect::<BTreeSet<_>>();
    if authors.len() != policy.author_key_blake3.len()
        || authors.len() > MAX_ROWS
        || authors.iter().any(|value| !digest(value))
    {
        return Err("policy_author_keys_invalid".to_owned());
    }
    if policy.authentication_policy.trusted_keys.is_empty()
        || policy.authentication_policy.trusted_keys.len() > MAX_ROWS
    {
        return Err("policy_trusted_key_count_invalid".to_owned());
    }
    for key in &policy.authentication_policy.trusted_keys {
        if key.allowed_purposes.len() != 1
            || key.allowed_purposes[0] != REVIEW_PURPOSE
            || key.currentness_ref != policy.verifier_context
        {
            return Err("policy_currentness_context_or_review_role_invalid".to_owned());
        }
    }
    artifact_auth_core::policy_identity(&policy.authentication_policy)
        .map_err(|_| "authentication_policy_invalid".to_owned())?;
    // Validate the explicit verifier context even when there are no reviews.
    let statement = ArtifactStatement {
        schema: artifact_auth_core::STATEMENT_SCHEMA_V1.to_owned(),
        scope: expected_scope(policy, &policy.review_policy_blake3),
        producer_id: "profile-validation".to_owned(),
        key_id: "profile-validation".to_owned(),
        key_identity: policy.authentication_policy.trusted_keys[0].key_identity.clone(),
    };
    artifact_auth_core::canonical_statement_bytes(&statement).map_err(|_| "verifier_context_invalid".to_owned())?;
    Ok(())
}

fn validate_attachment(attachment: &SourceReviewAttachment) -> Result<(), String> {
    if attachment.schema != ATTACHMENT_SCHEMA || attachment.non_claims != required_non_claims() {
        return Err("attachment_schema_or_non_claims_invalid".to_owned());
    }
    if attachment.reviews.len() > MAX_ROWS {
        return Err("review_count_exceeded".to_owned());
    }
    let mut keys = BTreeSet::new();
    for review in &attachment.reviews {
        validate_claim(&review.claim)?;
        if !keys.insert(&review.statement.key_identity.digest_hex) {
            return Err("duplicate_review_key".to_owned());
        }
        decode_hex::<32>(&review.public_key_hex)?;
        decode_hex::<64>(&review.signature_hex)?;
    }
    Ok(())
}

fn reference(profile: &str, digest: &str) -> ArtifactRef {
    ArtifactRef {
        profile: profile.to_owned(),
        algorithm: "blake3".to_owned(),
        digest_hex: digest.to_owned(),
    }
}

fn expected_scope(policy: &SourceReviewVerificationPolicy, claim: &str) -> AuthenticationScope {
    let mut parents = vec![
        reference("mantle-release-source.v1", &policy.subject.source_archive_blake3),
        reference("cairn.source-review-policy.v1", &policy.review_policy_blake3),
    ];
    parents.sort();
    AuthenticationScope {
        domain: REVIEW_DOMAIN.to_owned(),
        purpose: REVIEW_PURPOSE.to_owned(),
        profile_id: REVIEW_DOMAIN.to_owned(),
        subject: reference(CLAIM_SCHEMA, claim),
        parents,
        verifier_context: policy.verifier_context.clone(),
    }
}

fn decode_hex<const N: usize>(value: &str) -> Result<[u8; N], String> {
    if value.len() != N * 2 || !value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err("signature_or_key_encoding_invalid".to_owned());
    }
    let mut bytes = [0; N];
    for (output, pair) in bytes.iter_mut().zip(value.as_bytes().chunks_exact(2)) {
        let nibble = |byte| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        *output = (nibble(pair[0]) << 4) | nibble(pair[1]);
    }
    Ok(bytes)
}

fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input_byte_limit_exceeded".to_owned());
    }
    // Deserialize directly into closed DTOs, never via Value (which loses duplicate keys).
    serde_json::from_slice(bytes).map_err(|_| "closed_json_schema_invalid".to_owned())
}

/// Independently authenticate exact attachment and explicit operator-policy bytes.
/// Parse/crypto/linkage errors produce `invalid`; only valid under-threshold
/// evidence produces `insufficient`. No producer verdict is accepted as input.
#[must_use]
pub fn accept_source_review_bytes(attachment_bytes: &[u8], policy_bytes: &[u8]) -> SourceReviewAcceptance {
    let mut receipt = SourceReviewAcceptance {
        schema: ACCEPTANCE_SCHEMA.to_owned(),
        outcome: SourceReviewOutcome::Invalid,
        verification_role: SourceReviewRole::ManualReview,
        subject: None,
        subject_identity: None,
        claim_root: None,
        claim_root_identity: None,
        review_policy_blake3: None,
        verification_policy_identity: None,
        authentication_policy_identity: None,
        approved_key_blake3: Vec::new(),
        signed_statement_blake3: Vec::new(),
        claim_identity: Vec::new(),
        attachment_identity: None,
        attachment_artifact_blake3: blake3::hash(attachment_bytes).to_hex().to_string(),
        policy_artifact_blake3: blake3::hash(policy_bytes).to_hex().to_string(),
        receipt_blake3: String::new(),
        non_claims: required_non_claims(),
        issues: Vec::new(),
    };
    if let Err(issue) = accept_inner(attachment_bytes, policy_bytes, &mut receipt) {
        receipt.outcome = SourceReviewOutcome::Invalid;
        receipt.approved_key_blake3.clear();
        receipt.issues.push(issue);
    }
    receipt.issues.sort();
    receipt.issues.dedup();
    // The receipt's own identity field is excluded, not hashed as an empty value.
    let mut material = serde_json::to_value(&receipt).expect("receipt is JSON serializable");
    material.as_object_mut().expect("receipt is an object").remove("receipt_blake3");
    receipt.receipt_blake3 = identity(ACCEPTANCE_SCHEMA, &material).expect("receipt canonicalizes");
    receipt
}

fn accept_inner(
    attachment_bytes: &[u8],
    policy_bytes: &[u8],
    receipt: &mut SourceReviewAcceptance,
) -> Result<(), String> {
    let policy: SourceReviewVerificationPolicy = parse(policy_bytes)?;
    let attachment: SourceReviewAttachment = parse(attachment_bytes)?;
    validate_policy(&policy)?;
    validate_attachment(&attachment)?;
    receipt.subject = Some(policy.subject.clone());
    receipt.subject_identity = Some(subject_identity(&policy.subject)?);
    receipt.claim_root = Some(policy.claim_root.clone());
    receipt.claim_root_identity = Some(claim_root_identity(&policy.claim_root)?);
    receipt.review_policy_blake3 = Some(policy.review_policy_blake3.clone());
    receipt.verification_policy_identity = Some(verification_policy_identity(&policy)?);
    receipt.authentication_policy_identity = Some(
        artifact_auth_core::policy_identity(&policy.authentication_policy)
            .map_err(|_| "authentication_policy_invalid".to_owned())?,
    );
    receipt.attachment_identity = Some(attachment_identity(&attachment)?);
    let mut approved = BTreeSet::new();
    let mut statements = BTreeSet::new();
    let mut claims = BTreeSet::new();
    // Dispositions have different signed subjects. Evaluate each exact scope with
    // threshold one, retaining the full supplied trust/currentness policy.
    let mut row_policy = policy.authentication_policy.clone();
    row_policy.threshold = 1;
    for review in &attachment.reviews {
        if review.claim.subject != policy.subject
            || review.claim.claim_root != policy.claim_root
            || review.claim.review_policy_blake3 != policy.review_policy_blake3
        {
            return Err("review_subject_root_or_policy_mismatch".to_owned());
        }
        let claim = claim_identity(&review.claim)?;
        let scope = expected_scope(&policy, &claim);
        // Require sorted unique exact parent references, not additional authority.
        if review.statement.scope != scope {
            return Err("review_statement_scope_mismatch".to_owned());
        }
        let key = decode_hex::<32>(&review.public_key_hex)?;
        let signature = decode_hex::<64>(&review.signature_hex)?;
        let actual_key = artifact_auth_ed25519::public_key_identity(&key);
        if review.statement.key_identity != actual_key {
            return Err("review_full_key_mismatch".to_owned());
        }
        if policy.exclude_authors && policy.author_key_blake3.contains(&actual_key.digest_hex) {
            return Err("review_author_excluded".to_owned());
        }
        let decision = artifact_auth_ed25519::verify_and_evaluate(&row_policy, &scope, &[
            artifact_auth_ed25519::Ed25519Evidence {
                statement: &review.statement,
                generation: review.generation,
                public_key_bytes: &key,
                signature_bytes: &signature,
            },
        ]);
        if !decision.passed {
            receipt.issues.extend(decision.issues.into_iter().map(|issue| issue.code));
            return Err("review_authentication_failed".to_owned());
        }
        statements.insert(
            artifact_auth_core::statement_identity(&review.statement).map_err(|_| "statement_invalid".to_owned())?,
        );
        claims.insert(claim);
        if review.claim.disposition == ReviewDisposition::Approved {
            approved.insert(actual_key.digest_hex);
        }
    }
    receipt.approved_key_blake3 = approved.into_iter().collect();
    receipt.signed_statement_blake3 = statements.into_iter().collect();
    receipt.claim_identity = claims.into_iter().collect();
    receipt.outcome = if receipt.approved_key_blake3.len() >= usize::from(policy.required_approvals) {
        SourceReviewOutcome::AcceptedManualReview
    } else {
        receipt.issues.push("approval_threshold_unsatisfied".to_owned());
        SourceReviewOutcome::Insufficient
    };
    Ok(())
}

#[cfg(test)]
#[path = "source_review/tests.rs"]
mod tests;
