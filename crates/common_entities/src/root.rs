use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::{entity_enums::{EntityEnum, EntitySchema, FieldInfo, FieldType, RelationshipCardinality, RelationshipDirection, RelationshipInfo, RelationshipStrength, RelationshipType}, EntityTrait};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Root {
    pub id: i64,
    pub entities: Vec<i64>,
    pub features: Vec<i64>,
}



impl EntityTrait for Root {
    fn schema() -> EntitySchema<'static> {
        EntitySchema {
            entity_name: "Root",
            fields: vec![
                FieldInfo {
                    field_name: "id",
                    field_type: FieldType::Integer,
                    is_primary_key: true,
                    has_relationship: false,
                },
                FieldInfo {
                    field_name: "global",
                    field_type: FieldType::Integer,
                    is_primary_key: false,
                    has_relationship: true,
                },
                FieldInfo {
                    field_name: "entities",
                    field_type: FieldType::Integer,
                    is_primary_key: false,
                    has_relationship: true,
                },
                FieldInfo {
                    field_name: "features",
                    field_type: FieldType::Integer,
                    is_primary_key: false,
                    has_relationship: true,
                },
            ],
            relationships: vec![
                RelationshipInfo {
                    left_entity: EntityEnum::Root,
                    left_entity_name: "Root",
                    right_entity: EntityEnum::Global,
                    right_entity_name: "Global",
                    field_name: "global",
                    relationship_type: RelationshipType::OneToOne,
                    relationship_strength: RelationshipStrength::Strong,
                    relationship_direction: RelationshipDirection::Forward,
                    relationship_cardinality: RelationshipCardinality::One,
                },
                RelationshipInfo {
                    left_entity: EntityEnum::Root,
                    left_entity_name: "Root",
                    right_entity: EntityEnum::Entity,
                    right_entity_name: "Entity",
                    field_name: "entities",
                    relationship_type: RelationshipType::OneToMany,
                    relationship_strength: RelationshipStrength::Strong,
                    relationship_direction: RelationshipDirection::Forward,
                    relationship_cardinality: RelationshipCardinality::ManyOrdered,
                },
                RelationshipInfo {
                    left_entity: EntityEnum::Root,
                    left_entity_name: "Root",
                    right_entity: EntityEnum::Feature,
                    right_entity_name: "Feature",
                    field_name: "features",
                    relationship_type: RelationshipType::OneToMany,
                    relationship_strength: RelationshipStrength::Strong,
                    relationship_direction: RelationshipDirection::Forward,
                    relationship_cardinality: RelationshipCardinality::ManyOrdered,
                },
            ],
        }
    }
}