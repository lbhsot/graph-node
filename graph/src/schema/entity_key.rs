use std::fmt;

use anyhow::Error;

use crate::components::store::{LoadRelatedRequest, StoreError};
use crate::data::store::Value;
use crate::data::value::Word;
use crate::data_source::CausalityRegion;
use crate::schema::EntityType;
use crate::util::intern;

/// Key by which an individual entity in the store can be accessed. Stores
/// only the entity type and id. The deployment must be known from context.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityKey {
    /// Name of the entity type.
    pub entity_type: EntityType,

    /// ID of the individual entity.
    pub entity_id: Word,

    /// This is the causality region of the data source that created the entity.
    ///
    /// In the case of an entity lookup, this is the causality region of the data source that is
    /// doing the lookup. So if the entity exists but was created on a different causality region,
    /// the lookup will return empty.
    pub causality_region: CausalityRegion,

    _force_use_of_new: (),
}

impl EntityKey {
    pub fn unknown_attribute(&self, err: intern::Error) -> StoreError {
        StoreError::UnknownAttribute(self.entity_type.to_string(), err.not_interned())
    }
}

impl EntityKey {
    pub(in crate::schema) fn new(
        entity_type: EntityType,
        entity_id: Word,
        causality_region: CausalityRegion,
    ) -> Self {
        Self {
            entity_type,
            entity_id,
            causality_region,
            _force_use_of_new: (),
        }
    }

    pub fn from(id: &String, load_related_request: &LoadRelatedRequest) -> Self {
        let clone = load_related_request.clone();
        Self::new(
            clone.entity_type,
            Word::from(id.as_str()),
            clone.causality_region,
        )
    }

    pub fn id_value(&self) -> Result<Value, Error> {
        self.entity_type.id_value(self.entity_id.clone())
    }
}

impl std::fmt::Debug for EntityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EntityKey({}[{}], cr={})",
            self.entity_type, self.entity_id, self.causality_region
        )
    }
}