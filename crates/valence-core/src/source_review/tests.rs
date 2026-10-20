// SPDX-License-Identifier: MPL-2.0
//! Deterministic test-only keys. These fixtures are not real-world approvals.

use artifact_auth_core::KeyCurrentness;
use artifact_auth_core::TrustedKeyObservation;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;

use super::*;

fn hash(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn fixture() -> (SourceReviewAttachment, SourceReviewVerificationPolicy) {
    let subject = SourceReviewSubject {
        schema: SUBJECT_SCHEMA.to_owned(),
        source_archive_blake3: hash("test-only source archive"),
        source_revision: Some("test-only-revision".to_owned()),
        source_tree_blake3: Some(hash("test-only tree")),
        snapshot_profile: "mantle-release-source".to_owned(),
        snapshot_profile_version: 1,
        projection: "source".to_owned(),
    };
    let root = ClaimRoot {
        claim_identity: hash("test-only claim DAG"),
        subject_root: hash("test-only subject root"),
        revision: 3,
    };
    let context = reference("test-only-currentness.v1", &hash("test-only explicit currentness"));
    let keys = [SigningKey::from_bytes(&[17; 32]), SigningKey::from_bytes(&[29; 32])];
    let policy = SourceReviewVerificationPolicy {
        schema: POLICY_SCHEMA.to_owned(),
        subject: subject.clone(),
        claim_root: root.clone(),
        review_policy_blake3: hash("test-only review policy"),
        authentication_policy: AuthenticationPolicy {
            schema: artifact_auth_core::POLICY_SCHEMA_V1.to_owned(),
            profile_id: REVIEW_DOMAIN.to_owned(),
            threshold: 2,
            trusted_keys: keys
                .iter()
                .enumerate()
                .map(|(index, key)| TrustedKeyObservation {
                    producer_id: format!("test-reviewer-{index}"),
                    key_id: format!("test-key-{index}"),
                    key_identity: artifact_auth_ed25519::public_key_identity(key.verifying_key().as_bytes()),
                    allowed_purposes: vec![REVIEW_PURPOSE.to_owned()],
                    generation: 7,
                    currentness: KeyCurrentness::Current,
                    currentness_ref: context.clone(),
                })
                .collect(),
        },
        verifier_context: context,
        author_key_blake3: vec![hash("test-only author")],
        exclude_authors: true,
        required_approvals: 2,
        non_claims: required_non_claims(),
    };
    let claim = SourceReviewClaim {
        schema: CLAIM_SCHEMA.to_owned(),
        subject,
        claim_root: root,
        review_policy_blake3: policy.review_policy_blake3.clone(),
        disposition: ReviewDisposition::Approved,
        non_claims: required_non_claims(),
    };
    let reviews = keys
        .iter()
        .zip(&policy.authentication_policy.trusted_keys)
        .map(|(key, trusted)| {
            let statement = ArtifactStatement {
                schema: "artifact-auth.statement.v1".to_owned(),
                scope: AuthenticationScope {
                    domain: "cairn.source-review.v1".to_owned(),
                    purpose: "source-review-approval".to_owned(),
                    profile_id: "cairn.source-review.v1".to_owned(),
                    subject: reference("cairn.source-review-claim.v1", &claim_identity(&claim).unwrap()),
                    parents: vec![
                        reference("cairn.source-review-policy.v1", &policy.review_policy_blake3),
                        reference("mantle-release-source.v1", &policy.subject.source_archive_blake3),
                    ],
                    verifier_context: policy.verifier_context.clone(),
                },
                producer_id: trusted.producer_id.clone(),
                key_id: trusted.key_id.clone(),
                key_identity: trusted.key_identity.clone(),
            };
            let signature = key.sign(&artifact_auth_core::canonical_statement_bytes(&statement).unwrap());
            SignedSourceReview {
                claim: claim.clone(),
                statement,
                generation: 7,
                public_key_hex: hex(key.verifying_key().as_bytes()),
                signature_hex: hex(&signature.to_bytes()),
            }
        })
        .collect();
    (
        SourceReviewAttachment {
            schema: ATTACHMENT_SCHEMA.to_owned(),
            reviews,
            non_claims: required_non_claims(),
        },
        policy,
    )
}

fn accept(attachment: &SourceReviewAttachment, policy: &SourceReviewVerificationPolicy) -> SourceReviewAcceptance {
    accept_source_review_bytes(&serde_json::to_vec(attachment).unwrap(), &serde_json::to_vec(policy).unwrap())
}

#[test]
fn real_signatures_accept_only_manual_review_and_preserve_all_linkage() {
    let (attachment, policy) = fixture();
    let receipt = accept(&attachment, &policy);
    assert_eq!(receipt.outcome, SourceReviewOutcome::AcceptedManualReview);
    assert_eq!(receipt.verification_role, SourceReviewRole::ManualReview);
    assert_eq!(receipt.subject.as_ref(), Some(&policy.subject));
    assert_eq!(receipt.claim_root.as_ref(), Some(&policy.claim_root));
    assert_eq!(receipt.review_policy_blake3.as_ref(), Some(&policy.review_policy_blake3));
    let mut expected = policy
        .authentication_policy
        .trusted_keys
        .iter()
        .map(|key| key.key_identity.digest_hex.clone())
        .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(receipt.approved_key_blake3, expected);
    let mut signed = attachment
        .reviews
        .iter()
        .map(|review| artifact_auth_core::statement_identity(&review.statement).unwrap())
        .collect::<Vec<_>>();
    signed.sort();
    assert_eq!(receipt.signed_statement_blake3, signed);
    assert_eq!(receipt.non_claims, required_non_claims());
    assert!(receipt.issues.is_empty());
}

#[test]
fn logical_identity_ignores_json_layout_but_exact_artifact_digest_does_not() {
    let (attachment, policy) = fixture();
    let policy_bytes = serde_json::to_vec(&policy).unwrap();
    let compact = accept_source_review_bytes(&serde_json::to_vec(&attachment).unwrap(), &policy_bytes);
    let pretty = accept_source_review_bytes(&serde_json::to_vec_pretty(&attachment).unwrap(), &policy_bytes);
    assert_eq!(compact.outcome, SourceReviewOutcome::AcceptedManualReview);
    assert_eq!(pretty.outcome, SourceReviewOutcome::AcceptedManualReview);
    assert_eq!(compact.attachment_identity, pretty.attachment_identity);
    assert_ne!(compact.attachment_artifact_blake3, pretty.attachment_artifact_blake3);
    assert_ne!(compact.receipt_blake3, pretty.receipt_blake3);
    let mut reordered = attachment.clone();
    reordered.reviews.reverse();
    assert_eq!(attachment_identity(&attachment), attachment_identity(&reordered));
    let mut value = serde_json::to_value(&compact).unwrap();
    value.as_object_mut().unwrap().remove("receipt_blake3");
    assert_eq!(compact.receipt_blake3, identity(ACCEPTANCE_SCHEMA, &value).unwrap());
}

#[test]
fn valid_needs_revision_is_attributed_but_not_approval() {
    let (mut attachment, policy) = fixture();
    let review = &mut attachment.reviews[1];
    review.claim.disposition = ReviewDisposition::NeedsRevision;
    review.statement.scope = expected_scope(&policy, &claim_identity(&review.claim).unwrap());
    let key = SigningKey::from_bytes(&[29; 32]);
    review.signature_hex =
        hex(&key.sign(&artifact_auth_core::canonical_statement_bytes(&review.statement).unwrap()).to_bytes());
    let receipt = accept(&attachment, &policy);
    assert_eq!(receipt.outcome, SourceReviewOutcome::Insufficient);
    assert_eq!(receipt.approved_key_blake3, vec![attachment.reviews[0].statement.key_identity.digest_hex.clone()]);
    assert_eq!(receipt.signed_statement_blake3.len(), 2);
}

#[test]
fn missing_reviews_and_under_threshold_are_insufficient_not_accepted() {
    let (mut attachment, policy) = fixture();
    attachment.reviews.pop();
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Insufficient);
    attachment.reviews.clear();
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Insufficient);
}

#[test]
fn signature_key_generation_currentness_and_purpose_fail_closed() {
    for case in 0..9 {
        let (mut attachment, mut policy) = fixture();
        match case {
            0 => attachment.reviews[0].signature_hex = "00".repeat(64),
            1 => attachment.reviews[0].public_key_hex = attachment.reviews[1].public_key_hex.clone(),
            2 => attachment.reviews[0].generation -= 1,
            3 => policy.authentication_policy.trusted_keys[0].currentness = KeyCurrentness::Revoked,
            4 => policy.authentication_policy.trusted_keys[0].currentness = KeyCurrentness::Superseded,
            5 => policy.authentication_policy.trusted_keys[0].currentness = KeyCurrentness::Unknown,
            6 => policy.authentication_policy.trusted_keys[0].allowed_purposes = vec!["release-signature".to_owned()],
            7 => policy.authentication_policy.trusted_keys[0].allowed_purposes.push("build-witness".to_owned()),
            _ => {
                policy.authentication_policy.trusted_keys[0].currentness_ref.digest_hex =
                    hash("wrong currentness observation")
            }
        }
        let receipt = accept(&attachment, &policy);
        assert_eq!(receipt.outcome, SourceReviewOutcome::Invalid, "case {case}");
        assert!(receipt.approved_key_blake3.is_empty());
    }
}

#[test]
fn duplicated_full_key_and_author_only_evidence_cannot_inflate_threshold() {
    let (mut attachment, mut policy) = fixture();
    attachment.reviews[1] = attachment.reviews[0].clone();
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Invalid);
    let (attachment, _) = fixture();
    policy.author_key_blake3 =
        attachment.reviews.iter().map(|review| review.statement.key_identity.digest_hex.clone()).collect();
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Invalid);
    policy.exclude_authors = false;
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::AcceptedManualReview);
}

#[test]
fn changed_subject_root_policy_scope_or_signer_label_is_invalid() {
    for case in 0..10 {
        let (mut attachment, mut policy) = fixture();
        match case {
            0 => policy.subject.source_archive_blake3 = hash("different archive"),
            1 => policy.subject.source_tree_blake3 = Some(hash("different tree")),
            2 => policy.subject.source_revision = Some("different revision".to_owned()),
            3 => policy.claim_root.revision += 1,
            4 => policy.claim_root.claim_identity = hash("different root"),
            5 => policy.review_policy_blake3 = hash("different reviewed policy"),
            6 => attachment.reviews[0].statement.scope.purpose = "build-witness".to_owned(),
            7 => attachment.reviews[0].statement.scope.parents.push(reference("extra-parent.v1", &hash("extra"))),
            8 => attachment.reviews[0].statement.producer_id = "forged reviewer name".to_owned(),
            _ => attachment.reviews[0].claim.disposition = ReviewDisposition::NeedsRevision,
        }
        assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Invalid, "case {case}");
    }
}

#[test]
fn malformed_or_open_json_and_weakened_nonclaims_fail_closed() {
    let (attachment, policy) = fixture();
    let attachment_bytes = serde_json::to_vec(&attachment).unwrap();
    let policy_bytes = serde_json::to_vec(&policy).unwrap();
    for input in [
        b"{\"schema\":\"cairn.source-review-attachment.v1\",\"schema\":\"cairn.source-review-attachment.v1\",\"reviews\":[],\"non_claims\":[]}".to_vec(),
        { let mut value = serde_json::to_value(&attachment).unwrap(); value["verified"] = true.into(); serde_json::to_vec(&value).unwrap() },
        { let mut value = serde_json::to_value(&attachment).unwrap(); value["reviews"][0]["statement"]["verified"] = true.into(); serde_json::to_vec(&value).unwrap() },
        { let mut changed = attachment.clone(); changed.schema = "cairn.source-review-attachment.v2".to_owned(); serde_json::to_vec(&changed).unwrap() },
        { let mut changed = attachment.clone(); changed.non_claims.pop(); serde_json::to_vec(&changed).unwrap() },
        { let mut changed = attachment.clone(); changed.reviews[0].claim.non_claims.push("source-correctness-proven".to_owned()); serde_json::to_vec(&changed).unwrap() },
    ] {
        assert_eq!(accept_source_review_bytes(&input, &policy_bytes).outcome, SourceReviewOutcome::Invalid);
    }
    let duplicate_policy = String::from_utf8(policy_bytes.clone()).unwrap().replacen(
        "{",
        "{\"schema\":\"cairn.source-review-verification-policy.v1\",",
        1,
    );
    assert_eq!(
        accept_source_review_bytes(&attachment_bytes, duplicate_policy.as_bytes()).outcome,
        SourceReviewOutcome::Invalid
    );
    let mut bad_policy = policy.clone();
    bad_policy.required_approvals = 1;
    assert_eq!(accept(&attachment, &bad_policy).outcome, SourceReviewOutcome::Invalid);
    bad_policy = policy.clone();
    bad_policy.authentication_policy.trusted_keys[1] = bad_policy.authentication_policy.trusted_keys[0].clone();
    assert_eq!(accept(&attachment, &bad_policy).outcome, SourceReviewOutcome::Invalid);
}

#[test]
fn explicit_count_and_byte_bounds_reject_oversized_inputs() {
    let (mut attachment, mut policy) = fixture();
    attachment.reviews = vec![attachment.reviews[0].clone(); MAX_ROWS + 1];
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Invalid);
    let (attachment, _) = fixture();
    policy.author_key_blake3 = (0..=MAX_ROWS).map(|index| hash(&format!("test-only author {index}"))).collect();
    assert_eq!(accept(&attachment, &policy).outcome, SourceReviewOutcome::Invalid);
    let oversized = vec![b' '; MAX_INPUT_BYTES + 1];
    assert_eq!(accept_source_review_bytes(&oversized, b"{}").outcome, SourceReviewOutcome::Invalid);
    assert_eq!(accept_source_review_bytes(b"{}", &oversized).outcome, SourceReviewOutcome::Invalid);
}
