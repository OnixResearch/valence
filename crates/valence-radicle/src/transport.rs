//! `radicle-cob` evaluator that preserves signed transport facts for the pure Valence reducer.

use std::convert::Infallible;

use radicle_cob::Entry;
use radicle_cob::Evaluate;
use radicle_oid::Oid;
use serde::Deserialize;
use serde::Serialize;
use valence_core::cob_evidence::CobEvent;
use valence_core::cob_evidence::ObservedCobEvent;

use crate::store::GitCobStore;

const INVALID_OPERATION_INDEX: u32 = u32::MAX;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransportObservedEvent {
    pub revision_oid: String,
    pub observed: ObservedCobEvent,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PhysicalSidecar {
    pub events: Vec<TransportObservedEvent>,
}

impl Evaluate<GitCobStore> for PhysicalSidecar {
    type Error = Infallible;

    fn init(entry: &Entry, _store: &GitCobStore) -> Result<Self, Self::Error> {
        Ok(Self {
            events: decode_entry(entry),
        })
    }

    fn apply<'a, I: Iterator<Item = (&'a Oid, &'a Entry)>>(
        &mut self,
        entry: &Entry,
        _concurrent: I,
        _store: &GitCobStore,
    ) -> Result<(), Self::Error> {
        self.events.extend(decode_entry(entry));
        Ok(())
    }
}

fn decode_entry(entry: &Entry) -> Vec<TransportObservedEvent> {
    entry
        .contents()
        .iter()
        .enumerate()
        .map(|(index, bytes)| decode_operation(entry, index, bytes))
        .collect()
}

fn decode_operation(entry: &Entry, index: usize, bytes: &[u8]) -> TransportObservedEvent {
    let (event, decode_issue) = match serde_json::from_slice::<CobEvent>(bytes) {
        Ok(event) => (Some(event), None),
        Err(error) => (None, Some(format!("event_json_invalid:{error}"))),
    };
    let operation_index = u32::try_from(index).unwrap_or(INVALID_OPERATION_INDEX);
    TransportObservedEvent {
        revision_oid: entry.revision.to_string(),
        observed: ObservedCobEvent {
            sidecar_object_id: String::new(),
            entry_oid: entry.id.to_string(),
            operation_index,
            parent_oids: entry.parents.iter().map(ToString::to_string).collect(),
            related_oids: entry.related.iter().map(ToString::to_string).collect(),
            signer: entry.author().to_string(),
            signature_valid: entry.valid_signatures(),
            event,
            decode_issue,
        },
    }
}
