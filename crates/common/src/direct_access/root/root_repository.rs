use crate::{
    database::transactions::Transaction,
    direct_access::repository_factory,
    entities::Root,
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
    types::EntityId,
};
use redb::Error;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootRelationshipField {
    Global,
    Entities,
    Features,
    Files,
}

impl Display for RootRelationshipField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait RootTable {
    fn create(&mut self, root: &Root) -> Result<Root, Error>;
    fn create_multi(&mut self, roots: &[Root]) -> Result<Vec<Root>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Root>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error>;
    fn update(&mut self, root: &Root) -> Result<Root, Error>;
    fn update_multi(&mut self, roots: &[Root]) -> Result<Vec<Root>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;

    fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;

    fn set_relationship_multi(
        &mut self,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
    fn set_relationship(
        &mut self,
        id: &EntityId,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
}

pub trait RootTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Root>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error>;
    fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error>;
    fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
}

pub struct RootRepository<'a> {
    redb_table: Box<dyn RootTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> RootRepository<'a> {
    pub fn new(redb_table: Box<dyn RootTable + 'a>, transaction: &'a Transaction) -> Self {
        RootRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(&mut self, event_hub: &Arc<EventHub>, root: &Root) -> Result<Root, Error> {
        let new = self.redb_table.create(root)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        roots: &[Root],
    ) -> Result<Vec<Root>, Error> {
        let new_roots = self.redb_table.create_multi(roots)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Created)),
            ids: new_roots.iter().map(|root| root.id.clone()).collect(),
            data: None,
        });

        Ok(new_roots)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(&mut self, event_hub: &Arc<EventHub>, root: &Root) -> Result<Root, Error> {
        let updated_root = self.redb_table.update(root)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
            ids: vec![updated_root.id.clone()],
            data: None,
        });
        Ok(updated_root)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        roots: &[Root],
    ) -> Result<Vec<Root>, Error> {
        let updated_roots = self.redb_table.update_multi(roots)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
            ids: updated_roots.iter().map(|root| root.id.clone()).collect(),
            data: None,
        });

        Ok(updated_roots)
    }

    pub fn delete(&mut self, event_hub: &Arc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let root = match self.redb_table.get(id)? {
            Some(root) => root,
            None => return Ok(()),
        };
        // get all strong forward relationship fields
        let global = root.global.clone();
        let entities = root.entities.clone();
        let features = root.features.clone();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_global_repository(self.transaction)
            .delete(event_hub, &global)?;
        repository_factory::write::create_entity_repository(self.transaction)
            .delete_multi(event_hub, &entities)?;
        repository_factory::write::create_feature_repository(self.transaction)
            .delete_multi(event_hub, &features)?;

        // delete root
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Removed)),
            ids: vec![id.clone()],
            data: None,
        });

        Ok(())
    }

    pub fn delete_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        ids: &[EntityId],
    ) -> Result<(), Error> {
        let roots = self.redb_table.get_multi(ids)?;

        if roots.is_empty() || roots.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        // get all strong forward relationship fields
        let globals: Vec<EntityId> = roots
            .iter()
            .filter_map(|root| root.as_ref().map(|root| root.global.clone()))
            .collect();

        let mut entities: Vec<EntityId> = roots
            .iter()
            .flat_map(|root| root.as_ref().map(|root| root.entities.clone()))
            .flatten()
            .collect();
        // remove duplicates
        entities.sort();
        entities.dedup();

        let mut features: Vec<EntityId> = roots
            .iter()
            .flat_map(|root| root.as_ref().map(|root| root.features.clone()))
            .flatten()
            .collect();
        // remove duplicates
        features.sort();
        features.dedup();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_global_repository(self.transaction)
            .delete_multi(event_hub, &globals)?;
        repository_factory::write::create_entity_repository(self.transaction)
            .delete_multi(event_hub, &entities)?;
        repository_factory::write::create_feature_repository(self.transaction)
            .delete_multi(event_hub, &features)?;

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }

    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }

    pub fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }

    pub fn set_relationship_multi(
        &mut self,
        event_hub: &Arc<EventHub>,
        field: &RootRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table
            .set_relationship_multi(field, relationships.clone())?;

        for relationship in relationships {
            let (left_id, right_ids) = relationship;
            event_hub.send_event(Event {
                origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
                ids: vec![left_id],
                data: Some(format!(
                    "{}:{}",
                    field,
                    right_ids
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )),
            });
        }

        Ok(())
    }

    pub fn set_relationship(
        &mut self,
        event_hub: &Arc<EventHub>,
        id: &EntityId,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table.set_relationship(id, field, right_ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Root(EntityEvent::Updated)),
            ids: vec![id.clone()],
            data: Some(format!(
                "{}:{}",
                field,
                right_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )),
        });
        Ok(())
    }
}

pub struct RootRepositoryRO<'a> {
    redb_table: Box<dyn RootTableRO + 'a>,
}

impl<'a> RootRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn RootTableRO + 'a>) -> Self {
        RootRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Root>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Root>>, Error> {
        self.redb_table.get_multi(ids)
    }
    pub fn get_relationship(
        &self,
        id: &EntityId,
        field: &RootRelationshipField,
    ) -> Result<Vec<EntityId>, Error> {
        self.redb_table.get_relationship(id, field)
    }

    pub fn get_relationships_from_right_ids(
        &self,
        field: &RootRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table
            .get_relationships_from_right_ids(field, right_ids)
    }
}
