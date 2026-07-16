//! Local Git storage implementation for `radicle-cob` special refs.

use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr as _;

use radicle_cob::ObjectId;
use radicle_cob::TypeName;
use radicle_cob::change;
use radicle_cob::object;
use radicle_cob::object::storage::convert;
use radicle_cob::signatures::ExtendedSignature;
use radicle_crypto::PublicKey;
use radicle_oid::Oid;
use thiserror::Error;

const REF_ROOT: &str = "refs/rad";
const COBS_COMPONENT: &str = "cobs";
const REF_COMPONENT_COUNT: usize = 6;

#[derive(Debug, Error)]
pub enum StoreOpenError {
    #[error("cannot open selected Git repository: {0}")]
    Git(#[from] git2::Error),
}

#[derive(Debug, Error)]
pub enum StoreReferenceError {
    #[error(transparent)]
    Conversion(#[from] convert::Error),
    #[error(transparent)]
    Git(#[from] git2::Error),
    #[error("invalid COB object id in reference '{name}': {detail}")]
    ObjectId { name: String, detail: String },
    #[error("unexpected COB reference shape '{0}'")]
    Shape(String),
}

pub struct GitCobStore {
    raw: git2::Repository,
}

impl GitCobStore {
    pub fn open(path: &Path) -> Result<Self, StoreOpenError> {
        Ok(Self {
            raw: git2::Repository::open(path)?,
        })
    }

    #[must_use]
    pub const fn as_raw(&self) -> &git2::Repository {
        &self.raw
    }

    #[must_use]
    pub fn cob_ref_name(namespace: &PublicKey, type_name: &TypeName, object_id: &ObjectId) -> String {
        format!("{REF_ROOT}/{namespace}/{COBS_COMPONENT}/{type_name}/{object_id}")
    }

    pub fn all_reference_targets(&self) -> Result<BTreeMap<String, String>, git2::Error> {
        let mut refs = BTreeMap::new();
        for reference in self.raw.references()? {
            let reference = reference?;
            let Some(name) = reference.name() else {
                continue;
            };
            let Some(target) = reference.target() else {
                continue;
            };
            refs.insert(name.to_owned(), target.to_string());
        }
        Ok(refs)
    }
}

impl radicle_cob::Store for GitCobStore {}

impl change::Storage for GitCobStore {
    type StoreError = <git2::Repository as change::Storage>::StoreError;
    type LoadError = <git2::Repository as change::Storage>::LoadError;
    type ObjectId = <git2::Repository as change::Storage>::ObjectId;
    type Parent = <git2::Repository as change::Storage>::Parent;
    type Signatures = <git2::Repository as change::Storage>::Signatures;

    fn store<Signer>(
        &self,
        resource: Option<Self::Parent>,
        related: Vec<Self::Parent>,
        signer: &Signer,
        template: change::Template<Self::ObjectId>,
    ) -> Result<change::store::Entry<Self::Parent, Self::ObjectId, Self::Signatures>, Self::StoreError>
    where
        Signer: radicle_crypto::signature::Signer<ExtendedSignature>,
    {
        self.raw.store(resource, related, signer, template)
    }

    fn load(
        &self,
        id: Self::ObjectId,
    ) -> Result<change::store::Entry<Self::Parent, Self::ObjectId, Self::Signatures>, Self::LoadError> {
        self.raw.load(id)
    }

    fn parents_of(&self, id: &Oid) -> Result<Vec<Oid>, Self::LoadError> {
        self.raw.parents_of(id)
    }

    fn manifest_of(&self, id: &Oid) -> Result<radicle_cob::Manifest, Self::LoadError> {
        self.raw.manifest_of(id)
    }
}

impl object::Storage for GitCobStore {
    type ObjectsError = StoreReferenceError;
    type TypesError = StoreReferenceError;
    type UpdateError = git2::Error;
    type RemoveError = git2::Error;
    type Namespace = PublicKey;

    fn objects(&self, type_name: &TypeName, object_id: &ObjectId) -> Result<object::Objects, Self::ObjectsError> {
        let glob = format!("{REF_ROOT}/*/{COBS_COMPONENT}/{type_name}/{object_id}");
        let references = self
            .raw
            .references_glob(&glob)?
            .map(|reference| {
                let reference = reference?;
                object::Reference::try_from(reference).map_err(StoreReferenceError::from)
            })
            .collect::<Result<Vec<_>, StoreReferenceError>>()?;
        Ok(references.into())
    }

    fn types(&self, type_name: &TypeName) -> Result<BTreeMap<ObjectId, object::Objects>, Self::TypesError> {
        let glob = format!("{REF_ROOT}/*/{COBS_COMPONENT}/{type_name}/*");
        let mut objects = BTreeMap::new();
        for reference in self.raw.references_glob(&glob)? {
            let reference = reference?;
            let name = reference.name().ok_or_else(|| StoreReferenceError::Shape("<non-utf8>".to_owned()))?;
            let object_id = object_id_from_ref(name, type_name)?;
            let reference = object::Reference::try_from(reference)?;
            objects
                .entry(object_id)
                .and_modify(|existing: &mut object::Objects| existing.push(reference.clone()))
                .or_insert_with(|| object::Objects::new(reference));
        }
        Ok(objects)
    }

    fn update(
        &self,
        namespace: &Self::Namespace,
        type_name: &TypeName,
        object_id: &ObjectId,
        entry: &change::EntryId,
    ) -> Result<(), Self::UpdateError> {
        let name = Self::cob_ref_name(namespace, type_name, object_id);
        self.raw.reference(&name, (*entry).into(), true, "valence-radicle COB update")?;
        Ok(())
    }

    fn remove(
        &self,
        namespace: &Self::Namespace,
        type_name: &TypeName,
        object_id: &ObjectId,
    ) -> Result<(), Self::RemoveError> {
        let name = Self::cob_ref_name(namespace, type_name, object_id);
        self.raw.find_reference(&name)?.delete()
    }
}

fn object_id_from_ref(name: &str, expected_type: &TypeName) -> Result<ObjectId, StoreReferenceError> {
    let components = name.split('/').collect::<Vec<_>>();
    if components.len() != REF_COMPONENT_COUNT
        || components[0] != "refs"
        || components[1] != "rad"
        || components[3] != COBS_COMPONENT
        || components[4] != expected_type.as_str()
    {
        return Err(StoreReferenceError::Shape(name.to_owned()));
    }
    ObjectId::from_str(components[5]).map_err(|error| StoreReferenceError::ObjectId {
        name: name.to_owned(),
        detail: error.to_string(),
    })
}
