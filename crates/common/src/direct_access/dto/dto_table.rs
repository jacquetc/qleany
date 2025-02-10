use crate::database::db_helpers;
use crate::database::Bincode;
use crate::entities::Dto;
use crate::entities::EntityId;
use redb::{Error, ReadTransaction, ReadableTable, TableDefinition, WriteTransaction};

use super::dto_repository::DtoRelationshipField;
use super::dto_repository::DtoTable;
use super::dto_repository::DtoTableRO;

const DTO_TABLE: TableDefinition<EntityId, Bincode<Dto>> = TableDefinition::new("dto");
const COUNTER_TABLE: TableDefinition<String, EntityId> = TableDefinition::new("__counter");
// forward relationships
const DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_field_from_dto_fields_junction");
// backward relationships
const DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_from_use_case_dto_in_junction");
const DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE: TableDefinition<EntityId, Vec<EntityId>> =
    TableDefinition::new("dto_from_use_case_dto_out_junction");

pub struct DtoRedbTable<'a> {
    transaction: &'a WriteTransaction,
}

impl<'a> DtoRedbTable<'a> {
    pub fn new(transaction: &'a WriteTransaction) -> Self {
        DtoRedbTable { transaction }
    }
}

impl<'a> DtoTable for DtoRedbTable<'a> {
    fn create(&mut self, dto: &Dto) -> Result<Dto, Error> {
        let dtos = self.create_multi(&[dto.clone()])?;
        Ok(dtos.into_iter().next().unwrap())
    }

    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        let dtos = self.get_multi(&[id.clone()])?;
        Ok(dtos.into_iter().next().unwrap())
    }

    fn update(&mut self, dto: &Dto) -> Result<Dto, Error> {
        let dtos = self.update_multi(&[dto.clone()])?;
        Ok(dtos.into_iter().next().unwrap())
    }

    fn delete(&mut self, id: &EntityId) -> Result<(), Error> {
        self.delete_multi(&[id.clone()])
    }

    fn create_multi(&mut self, dtos: &[Dto]) -> Result<Vec<Dto>, Error> {
        let mut created_dtos = Vec::new();
        let mut counter_table = self.transaction.open_table(COUNTER_TABLE)?;
        let mut counter = if let Some(counter) = counter_table.get(&"dto".to_string())? {
            counter.value()
        } else {
            1
        };

        let mut dto_table = self.transaction.open_table(DTO_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;

        for dto in dtos {
            // if the id is default, create a new id
            let new_dto = if dto.id == EntityId::default() {
                Dto {
                    id: counter,
                    ..dto.clone()
                }
            } else {
                // ensure that the id is not already in use
                if dto_table.get(&dto.id)?.is_some() {
                    panic!("Dto id already in use while creating it: {:?}", dto.id);
                }
                dto.clone()
            };
            Dto {
                id: counter,
                ..dto.clone()
            };
            dto_table.insert(new_dto.id, new_dto.clone())?;
            field_junction_table.insert(new_dto.id, new_dto.fields.clone())?;
            created_dtos.push(new_dto);

            if dto.id == EntityId::default() {
                counter += 1;
            }
        }

        counter_table.insert("dto".to_string(), counter)?;

        Ok(created_dtos)
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error> {
        let mut dtos = Vec::new();
        let dto_table = self.transaction.open_table(DTO_TABLE)?;
        let field_junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;

        for id in ids {
            let dto = if let Some(guard) = dto_table.get(id)? {
                let mut dto = guard.value().clone();

                // get fields from junction table
                let fields = field_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                dto.fields = fields;
                Some(dto)
            } else {
                None
            };
            dtos.push(dto);
        }
        Ok(dtos)
    }

    fn update_multi(&mut self, dtos: &[Dto]) -> Result<Vec<Dto>, Error> {
        let mut updated_dtos = Vec::new();
        let mut dto_table = self.transaction.open_table(DTO_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;

        for dto in dtos {
            dto_table.insert(dto.id, dto)?;
            field_junction_table.insert(dto.id, dto.fields.clone())?;
            updated_dtos.push(dto.clone());
        }

        Ok(updated_dtos)
    }

    fn delete_multi(&mut self, ids: &[EntityId]) -> Result<(), Error> {
        let mut dto_table = self.transaction.open_table(DTO_TABLE)?;
        let mut field_junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;
        let mut dto_in_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_IN_JUNCTION_TABLE)?;
        let mut dto_out_junction_table = self
            .transaction
            .open_table(DTO_FROM_USE_CASE_DTO_OUT_JUNCTION_TABLE)?;

        for id in ids {
            dto_table.remove(id)?;
            field_junction_table.remove(id)?;
            db_helpers::delete_from_backward_junction_table(&mut dto_in_junction_table, id)?;
            db_helpers::delete_from_backward_junction_table(&mut dto_out_junction_table, id)?;
        }

        Ok(())
    }

    fn get_relationships_of(
        &self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<Vec<(EntityId, Vec<EntityId>)>, Error> {
        let junction_table_definition = match field {
            DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
        };
        let junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut relationships = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            if right_ids
                .iter()
                .any(|entity_id| right_entities.contains(entity_id))
            {
                relationships.push((left_id, right_entities));
            }
        }
        Ok(relationships)
    }

    fn delete_all_relationships_with(
        &mut self,
        field: &DtoRelationshipField,
        right_ids: &[EntityId],
    ) -> Result<(), Error> {
        // delete from junction table
        let junction_table_definition = match field {
            DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        let mut relationship_iter = junction_table.iter()?;
        let mut junctions_to_modify: Vec<(EntityId, Vec<EntityId>)> = vec![];
        while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
            let left_id = left_id.value();
            let right_entities = right_entities.value();
            let entities_left: Vec<EntityId> = right_entities
                .clone()
                .into_iter()
                .filter(|entity_id| !right_ids.contains(entity_id))
                .collect();

            if entities_left.len() == right_entities.len() {
                continue;
            }
            junctions_to_modify.push((left_id, entities_left));
        }
        for (left_id, entities) in junctions_to_modify {
            junction_table.insert(left_id, entities)?;
        }

        Ok(())
    }

    fn set_relationships(
        &mut self,
        field: &DtoRelationshipField,
        relationships: Vec<(EntityId, Vec<EntityId>)>,
    ) -> Result<(), Error> {
        let junction_table_definition = match field {
            DtoRelationshipField::Fields => DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE,
        };
        let mut junction_table = self.transaction.open_table(junction_table_definition)?;
        for (left_id, entities) in relationships {
            junction_table.insert(left_id, entities)?;
        }
        Ok(())
    }
}

pub struct DtoRedbTableRO<'a> {
    transaction: &'a ReadTransaction,
}

impl<'a> DtoRedbTableRO<'a> {
    pub fn new(transaction: &'a ReadTransaction) -> Self {
        DtoRedbTableRO { transaction }
    }
}

impl<'a> DtoTableRO for DtoRedbTableRO<'a> {
    fn get(&self, id: &EntityId) -> Result<Option<Dto>, Error> {
        let dtos = self.get_multi(&[id.clone()])?;
        Ok(dtos.into_iter().next().unwrap())
    }

    fn get_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Dto>>, Error> {
        let mut dtos = Vec::new();
        let dto_table = self.transaction.open_table(DTO_TABLE)?;
        let field_junction_table = self
            .transaction
            .open_table(DTO_FIELD_FROM_DTO_FIELDS_JUNCTION_TABLE)?;

        for id in ids {
            let dto = if let Some(guard) = dto_table.get(id)? {
                let mut dto = guard.value().clone();

                // get fields from junction table
                let fields = field_junction_table
                    .get(id)?
                    .map(|guard| guard.value().clone())
                    .unwrap_or_default();

                dto.fields = fields;
                Some(dto)
            } else {
                None
            };
            dtos.push(dto);
        }
        Ok(dtos)
    }
}
