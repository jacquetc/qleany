use common::entities::{
    Cardinality, Direction, DtoFieldType, Entity, Field, FieldType, Order, Relationship,
    RelationshipType, Strength,
};
use common::types::EntityId;
use std::collections::HashMap;

pub fn generate_relationships(
    entities: &Vec<Entity>,
    fields: &Vec<Field>,
) -> HashMap<EntityId, Vec<Relationship>> {
    let mut all_forward_relationships_per_entity: HashMap<u64, Vec<Relationship>> = HashMap::new();

    // Generate all forward relationships per entity
    entities.iter().for_each(|entity| {
        let relationships = get_forward_relationships(entity, fields);
        all_forward_relationships_per_entity.insert(entity.id, relationships);
    });

    // Generate all backward relationships per entity
    let mut all_backward_relationships_per_entity: HashMap<u64, Vec<Relationship>> = HashMap::new();
    entities.iter().for_each(|entity| {
        let relationships =
            get_backward_relationships(&entity.id, &all_forward_relationships_per_entity);
        all_backward_relationships_per_entity.insert(entity.id, relationships);
    });

    // merge forward and backward relationships
    let mut all_relationships_per_entity: HashMap<u64, Vec<Relationship>> = HashMap::new();
    entities.iter().for_each(|entity| {
        let mut relationships = all_forward_relationships_per_entity
            .get(&entity.id)
            .unwrap()
            .clone();
        relationships.extend(
            all_backward_relationships_per_entity
                .get(&entity.id)
                .unwrap()
                .clone(),
        );
        all_relationships_per_entity.insert(entity.id, relationships);
    });

    all_relationships_per_entity
}

fn get_forward_relationships(entity: &Entity, fields: &Vec<Field>) -> Vec<Relationship> {
    entity
        .fields
        .iter()
        // get fields from ids
        .filter_map(|field_id| {
            let field = fields.iter().find(|f| f.id == *field_id)?;
            if field.entity.is_none() {
                return None;
            }
            Some(field)
        })
        // map fields to relationships
        .map(|field| {
            let relationship_type = if field.is_list {
                RelationshipType::ManyToMany
            } else {
                RelationshipType::OneToOne
            };

            let cardinality = if field.is_list {
                if field.is_nullable {
                    Cardinality::ZeroOrMore
                } else {
                    Cardinality::OneOrMore
                }
            } else {
                if field.is_nullable {
                    Cardinality::ZeroOrOne
                } else {
                    Cardinality::One
                }
            };

            Relationship {
                id: 0,
                left_entity: entity.id,
                right_entity: field.entity.unwrap(),
                field_name: field.name.clone(),
                relationship_type,
                strength: if field.strong {
                    Strength::Strong
                } else {
                    Strength::Weak
                },
                direction: Direction::Forward,
                cardinality,
                order: if field.ordered {
                    Some(Order::Ordered)
                } else {
                    Some(Order::Unordered)
                },
            }
        })
        .collect()
}

fn get_backward_relationships(
    entity_id: &EntityId,
    all_forward_relationships_per_entity: &HashMap<EntityId, Vec<Relationship>>,
) -> Vec<Relationship> {
    all_forward_relationships_per_entity
        .values()
        .flatten()
        .filter(|relationship| relationship.right_entity == *entity_id)
        .map(|relationship| Relationship {
            id: 0,
            left_entity: relationship.left_entity,
            right_entity: relationship.right_entity,
            field_name: relationship.field_name.clone(),
            relationship_type: relationship.relationship_type.clone(),
            strength: relationship.strength.clone(),
            direction: Direction::Backward,
            cardinality: relationship.cardinality.clone(),
            order: relationship.order.clone(),
        })
        .collect()
}

pub fn str_to_field_type(s: &str) -> FieldType {
    match s {
        "boolean" | "bool" => FieldType::Boolean,
        "int" | "integer" => FieldType::Integer,
        "uint" | "uinteger" => FieldType::UInteger,
        "float" => FieldType::Float,
        "string" => FieldType::String,
        "uuid" => FieldType::Uuid,
        "datetime" => FieldType::DateTime,
        _ => FieldType::String,
    }
}

pub fn str_to_dto_field_type(s: &str) -> DtoFieldType {
    match s {
        "boolean" | "bool" => DtoFieldType::Boolean,
        "int" | "integer" => DtoFieldType::Integer,
        "uint" | "uinteger" => DtoFieldType::UInteger,
        "float" => DtoFieldType::Float,
        "string" => DtoFieldType::String,
        "uuid" => DtoFieldType::Uuid,
        "datetime" => DtoFieldType::DateTime,
        _ => DtoFieldType::String,
    }
}
