use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

pub const NOSTR_EVIDENCE_SCHEMA: &str = "valence.nostr-identity-evidence.v1";
pub const UCAN_CREDENTIAL_RECORD_SCHEMA: &str = "valence.ucan-nostr-credential-record.v1";
pub const BASALT_AUTHENTICATION_SCHEMA: &str = "basalt.nostr-authentication-receipt.v1";
pub const BASALT_COMPOSITION_SCHEMA: &str = "basalt.nostr-composition-receipt.v1";
pub const UCAN_SOURCE_REVISION: &str = "277ab768eb532ac0c715a3bbff2a7dc37a702f11";
pub const UCAN_CREDENTIAL_REVISION: &str = "c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026";
pub const UCAN_CREDENTIAL_CONTRACT: &str =
    "ucan-nostr::VerifiedNostrCredential@c9d43a7c11dd2fab4d5cb3d100012fa9a2e5b026";
pub const BASALT_SOURCE_REVISION: &str = "bb8fb5f4a763f532df96b6e65407163ac668834e";
pub const BASALT_SCHEMA_BLAKE3: &str = "45824d99ae765e885f085ef2c8c8294343415af8e5489c35509c3e255476e258";
pub const BASALT_IMPLEMENTATION_BLAKE3: &str = "ced0ffcfe006a48ffd8f2b856ad0da9641a03a24e6ebda3877163c398252897c";
pub const BASALT_POLICY_BLAKE3: &str = "9f5df806b5fac55a35a63f58c86e4218de1414141c20c999cbd140ea3c0556a2";

pub const REQUIRED_NON_CLAIMS: &[&str] = &[
    "Valence records producer identity and linkage only; it does not verify Nostr signatures",
    "RecordedOnly Nostr evidence does not count as Property evidence or authority",
    "relay, DNS, signer, clock, replay, policy publication, and authorization remain external",
    "BLAKE3 artifact identity does not authenticate a producer or establish release eligibility",
];

pub const BASALT_NON_CLAIMS: &[&str] = &[
    "Nostr key control does not prove a human identity or product ownership",
    "subject authentication does not grant a resource or ability",
    "Nostr relays, NIP-05, npub text, and profile metadata are not authority",
    "clocks, replay storage, policy publication, authorization, and release remain caller-owned",
    "BLAKE3 receipt identity does not authenticate the producer or establish trust",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestRole {
    NostrEventSha256,
    UcanAdapterBlake3,
    BasaltBlake3,
    ValenceBlake3,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedDigest {
    pub role: DigestRole,
    pub hex: String,
}

impl TypedDigest {
    pub(crate) fn new(role: DigestRole, hex: impl Into<String>) -> Self {
        Self { role, hex: hex.into() }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedRole {
    RecordedOnly,
    Property,
    Authority,
    Boundary,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UcanNostrCredentialRecord {
    pub schema: String,
    pub source_revision: String,
    pub credential_revision: String,
    pub credential_contract: String,
    pub event_identity: TypedDigest,
    pub public_key: String,
    pub adapter_profile_identity: TypedDigest,
    pub adapter_implementation_identity: TypedDigest,
    pub request_identity: TypedDigest,
    pub audience: String,
    pub purpose: String,
    pub freshness: String,
    pub replay: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationDecision {
    Authenticated,
    Denied,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BasaltAuthenticationReceipt {
    pub schema: String,
    pub ucan_source_revision: String,
    pub credential_contract: String,
    pub producer_schema_identity: TypedDigest,
    pub producer_implementation_identity: TypedDigest,
    pub event_identity: TypedDigest,
    pub public_key: String,
    pub adapter_profile_identity: TypedDigest,
    pub adapter_implementation_identity: TypedDigest,
    pub request_identity: TypedDigest,
    pub audience_identity: TypedDigest,
    pub purpose_identity: TypedDigest,
    pub policy_identity: TypedDigest,
    pub subject_id: Option<String>,
    pub binding_id: Option<String>,
    pub binding_identity: Option<TypedDigest>,
    pub decision: AuthenticationDecision,
    pub reason: String,
    pub verification_role: RecordedRole,
    pub non_claims: Vec<String>,
    pub receipt_identity: TypedDigest,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompositionDecision {
    ComposedAllow,
    Denied,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BasaltCompositionReceipt {
    pub schema: String,
    pub producer_schema_identity: TypedDigest,
    pub producer_implementation_identity: TypedDigest,
    pub authentication_receipt_identity: TypedDigest,
    pub authorization_receipt_identity: TypedDigest,
    pub subject_id: String,
    pub request_identity: TypedDigest,
    pub policy_identity: TypedDigest,
    pub resource_identity: Option<TypedDigest>,
    pub ability_identity: Option<TypedDigest>,
    pub decision: CompositionDecision,
    pub reason: String,
    pub verification_role: RecordedRole,
    pub non_claims: Vec<String>,
    pub receipt_identity: TypedDigest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProducerPins {
    pub ucan_source_revision: String,
    pub ucan_credential_revision: String,
    pub basalt_source_revision: String,
    pub basalt_schema_identity: TypedDigest,
    pub basalt_implementation_identity: TypedDigest,
    pub basalt_policy_identity: TypedDigest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NostrIdentityEvidenceInput {
    pub schema: String,
    pub producer_pins: ProducerPins,
    pub credential: UcanNostrCredentialRecord,
    pub authentication: BasaltAuthenticationReceipt,
    #[serde(default)]
    pub composition: Option<BasaltCompositionReceipt>,
    pub verification_role: RecordedRole,
    #[serde(default)]
    pub public_metadata: BTreeMap<String, String>,
    pub non_claims: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NostrIdentityEvidenceArtifact {
    pub schema: String,
    pub producer_pins: ProducerPins,
    pub event_identity: TypedDigest,
    pub public_key: String,
    pub request_identity: TypedDigest,
    pub audience_identity: TypedDigest,
    pub purpose_identity: TypedDigest,
    pub subject_id: String,
    pub binding_id: String,
    pub binding_identity: TypedDigest,
    pub policy_identity: TypedDigest,
    pub authentication_receipt_identity: TypedDigest,
    pub composition_receipt_identity: Option<TypedDigest>,
    pub authorization_receipt_identity: Option<TypedDigest>,
    pub verification_role: RecordedRole,
    pub public_metadata: BTreeMap<String, String>,
    pub non_claims: Vec<String>,
    pub artifact_identity: TypedDigest,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NostrIdentityEvidenceIssue {
    pub field: &'static str,
    pub code: &'static str,
}

impl NostrIdentityEvidenceIssue {
    pub(crate) const fn new(field: &'static str, code: &'static str) -> Self {
        Self { field, code }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NostrIdentityEvidenceReport {
    pub valid: bool,
    pub issues: Vec<NostrIdentityEvidenceIssue>,
    pub artifact: Option<NostrIdentityEvidenceArtifact>,
    pub property_count: u32,
    pub recorded_only_count: u32,
    pub non_claims: Vec<String>,
}
