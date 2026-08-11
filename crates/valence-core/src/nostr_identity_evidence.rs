//! Closed recorded-only linkage for reviewed UCAN and Basalt Nostr artifacts.
//!
//! The pure core accepts already-loaded public producer records. It performs no
//! relay, DNS, signer, clock, replay-store, filesystem, or authorization effect.
//!
//! Traceability:
//! - r[impl valence.nostr_identity_evidence.profile]
//! - r[impl valence.nostr_identity_evidence.producer_linkage]
//! - r[impl valence.nostr_identity_evidence.digest_domains]
//! - r[verify valence.nostr_identity_evidence.role_policy]
//! - r[verify valence.nostr_identity_evidence.privacy]

mod identity;
mod model;
#[cfg(test)]
mod tests;
mod validate;

pub use identity::authentication_receipt_identity;
pub use identity::composition_receipt_identity;
pub use identity::public_field_identity;
pub use model::*;
pub use validate::parse_nostr_identity_evidence;
pub use validate::validate_nostr_identity_evidence;
