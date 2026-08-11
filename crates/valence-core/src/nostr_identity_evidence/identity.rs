use super::BasaltAuthenticationReceipt;
use super::BasaltCompositionReceipt;
use super::DigestRole;
use super::NostrIdentityEvidenceArtifact;
use super::TypedDigest;

const PUBLIC_FIELD_DOMAIN: &str = "basalt.nostr-public-field.v1";
const AUTHENTICATION_RECEIPT_DOMAIN: &str = "basalt.nostr-authentication-receipt.v1";
const COMPOSITION_RECEIPT_DOMAIN: &str = "basalt.nostr-composition-receipt.v1";
const VALENCE_ARTIFACT_DOMAIN: &str = "valence.nostr-identity-evidence.v1";

/// Recompute one Basalt public-field identity.
#[must_use]
pub fn public_field_identity(field: &str, value: &str) -> TypedDigest {
    TypedDigest::new(
        DigestRole::BasaltBlake3,
        tagged_blake3(PUBLIC_FIELD_DOMAIN, &[field.as_bytes(), value.as_bytes()]),
    )
}

/// Recompute the declared Basalt authentication receipt identity.
#[must_use]
pub fn authentication_receipt_identity(receipt: &BasaltAuthenticationReceipt) -> TypedDigest {
    let mut material = receipt.clone();
    material.receipt_identity.hex.clear();
    let bytes = serde_json::to_vec(&material).expect("crate-owned receipt serialization");
    TypedDigest::new(DigestRole::BasaltBlake3, tagged_blake3(AUTHENTICATION_RECEIPT_DOMAIN, &[bytes.as_slice()]))
}

/// Recompute the declared Basalt composition receipt identity.
#[must_use]
pub fn composition_receipt_identity(receipt: &BasaltCompositionReceipt) -> TypedDigest {
    let mut material = receipt.clone();
    material.receipt_identity.hex.clear();
    let bytes = serde_json::to_vec(&material).expect("crate-owned receipt serialization");
    TypedDigest::new(DigestRole::BasaltBlake3, tagged_blake3(COMPOSITION_RECEIPT_DOMAIN, &[bytes.as_slice()]))
}

pub(crate) fn artifact_identity(artifact: &NostrIdentityEvidenceArtifact) -> TypedDigest {
    let mut material = artifact.clone();
    material.artifact_identity.hex.clear();
    let bytes = serde_json::to_vec(&material).expect("crate-owned artifact serialization");
    TypedDigest::new(DigestRole::ValenceBlake3, tagged_blake3(VALENCE_ARTIFACT_DOMAIN, &[bytes.as_slice()]))
}

fn tagged_blake3(domain: &str, fields: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    update_framed(&mut hasher, domain.as_bytes());
    for field in fields {
        update_framed(&mut hasher, field);
    }
    hasher.finalize().to_hex().to_string()
}

fn update_framed(hasher: &mut blake3::Hasher, field: &[u8]) {
    let length = u64::try_from(field.len()).expect("bounded field length fits u64");
    hasher.update(&length.to_be_bytes());
    hasher.update(field);
}
