use std::rc::Rc;

use crate::{
    database::transactions::Transaction,
    direct_access::repository_factory,
    entities::{EntityId, Feature},
    event::{DirectAccessEntity, EntityEvent, Event, EventHub, Origin},
};

use redb::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureRelationshipField {
    UseCases,
}

pub trait FeatureTable {
    fn create(&mut self, feature: &Feature) -> Result<Feature, Error>;
    fn create_multi(&mut self, features: &[Feature]) -> Result<Vec<Feature>, Error>;
    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error>;
    fn update(&mut self, feature: &Feature) -> Result<Feature, Error>;
    fn update_multi(&mut self, features: &[Feature]) -> Result<Vec<Feature>, Error>;
    fn delete(&mut self, id: &EntityId) -> Result<(), Error>;
    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error>;
    fn get_relationships_of(
        &self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error>;
    fn delete_all_relationships_with(
        &mut self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error>;
    fn set_relationships(
        &mut self,
        field: &FeatureRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error>;
}

pub trait FeatureTableRO {
    fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error>;
    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error>;
}

pub struct FeatureRepository<'a> {
    redb_table: Box<dyn FeatureTable + 'a>,
    transaction: &'a Transaction,
}

impl<'a> FeatureRepository<'a> {
    pub fn new(redb_table: Box<dyn FeatureTable + 'a>, transaction: &'a Transaction) -> Self {
        FeatureRepository {
            redb_table,
            transaction,
        }
    }

    pub fn create(
        &mut self,
        event_hub: &Rc<EventHub>,
        feature: &Feature,
    ) -> Result<Feature, Error> {
        let new = self.redb_table.create(feature)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Created)),
            ids: vec![new.id.clone()],
            data: None,
        });
        Ok(new)
    }

    pub fn create_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        features: &[Feature],
    ) -> Result<Vec<Feature>, Error> {
        let new_features = self.redb_table.create_multi(features)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Created)),
            ids: new_features
                .iter()
                .map(|feature| feature.id.clone())
                .collect(),
            data: None,
        });

        Ok(new_features)
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error> {
        self.redb_table.get_multi(ids)
    }

    pub fn update(
        &mut self,
        event_hub: &Rc<EventHub>,
        feature: &Feature,
    ) -> Result<Feature, Error> {
        let updated_feature = self.redb_table.update(feature)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Updated)),
            ids: vec![updated_feature.id.clone()],
            data: None,
        });
        Ok(updated_feature)
    }

    pub fn update_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        features: &[Feature],
    ) -> Result<Vec<Feature>, Error> {
        let updated_features = self.redb_table.update_multi(features)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Updated)),
            ids: updated_features
                .iter()
                .map(|feature| feature.id.clone())
                .collect(),
            data: None,
        });

        Ok(updated_features)
    }

    pub fn delete(&mut self, event_hub: &Rc<EventHub>, id: &EntityId) -> Result<(), Error> {
        let feature = match self.redb_table.get(id)? {
            Some(feature) => feature,
            None => return Ok(()),
        };

        // get all strong forward relationship fields
        let use_cases = feature.use_cases.clone();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_use_case_repository(self.transaction)
            .delete_multi(event_hub, &use_cases)?;

        // delete feature
        self.redb_table.delete(id)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Removed)),
            ids: vec![id.clone()],
            data: None,
        });

        Ok(())
    }

    pub fn delete_multi(
        &mut self,
        event_hub: &Rc<EventHub>,
        ids: &[EntityId],
    ) -> Result<(), Error> {
        let features = self.redb_table.get_multi(ids)?;

        if features.is_empty() || features.iter().all(|root| root.is_none()) {
            return Ok(());
        }

        // get all strong forward relationship fields
        let use_cases: Vec<EntityId> = features
            .iter()
            .flat_map(|feature| feature.as_ref().map(|feature| feature.use_cases.clone()))
            .flatten()
            .collect();

        // delete all strong relationships, initiating a cascade delete
        repository_factory::write::create_use_case_repository(self.transaction)
            .delete_multi(event_hub, &use_cases)?;

        self.redb_table.delete_multi(ids)?;
        event_hub.send_event(Event {
            origin: Origin::DirectAccess(DirectAccessEntity::Feature(EntityEvent::Removed)),
            ids: ids.into(),
            data: None,
        });

        Ok(())
    }

    pub fn get_relationships_of(
        &self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        self.redb_table.get_relationships_of(field, right_ids)
    }

    pub fn delete_all_relationships_with(
        &mut self,
        field: &FeatureRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        self.redb_table
            .delete_all_relationships_with(field, right_ids)
    }

    pub fn set_relationships(
        &mut self,
        field: &FeatureRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        self.redb_table.set_relationships(field, relationships)
    }
}

pub struct FeatureRepositoryRO<'a> {
    redb_table: Box<dyn FeatureTableRO + 'a>,
}

impl<'a> FeatureRepositoryRO<'a> {
    pub fn new(redb_table: Box<dyn FeatureTableRO + 'a>) -> Self {
        FeatureRepositoryRO { redb_table }
    }

    pub fn get(&self, id: &EntityId) -> Result<Option<Feature>, Error> {
        self.redb_table.get(id)
    }

    pub fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>, Error> {
        self.redb_table.get_multi(ids)
    }
}
