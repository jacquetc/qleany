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
        "entity" | "Entity" => FieldType::Entity,
        "enum" => FieldType::Enum,
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
        "enum" => DtoFieldType::Enum,
        _ => DtoFieldType::String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::entities::{Cardinality, Entity, Field, FieldType, RelationshipType};

    #[test]
    fn test_get_forward_relationships() {
        // Arrange
        // Create two entities: Parent and Child
        let parent_entity = Entity {
            id: 1,
            name: "Parent".to_string(),
            only_for_heritage: false,
            parent: None,
            allow_direct_access: true,
            fields: vec![1, 2], // Field IDs
            relationships: vec![],
        };

        // Create fields for the parent entity
        let fields = vec![
            // A non-list field (OneToOne relationship)
            Field {
                id: 1,
                name: "single_child".to_string(),
                field_type: FieldType::Entity,
                entity: Some(2), // Points to Child entity
                is_nullable: false,
                is_primary_key: false,
                is_list: false,
                single: true,
                strong: true,
                ordered: false,
                list_model: false,
                list_model_displayed_field: None,
                enum_name: None,
                enum_values: None,
            },
            // A list field (should be OneToMany, not ManyToMany)
            Field {
                id: 2,
                name: "multiple_children".to_string(),
                field_type: FieldType::Entity,
                entity: Some(2), // Points to Child entity
                is_nullable: false,
                is_primary_key: false,
                is_list: true,
                single: false,
                strong: true,
                ordered: true,
                list_model: true,
                list_model_displayed_field: None,
                enum_name: None,
                enum_values: None,
            },
        ];

        // Act
        let relationships = get_forward_relationships(&parent_entity, &fields);

        // Assert
        assert_eq!(relationships.len(), 2, "Should have 2 relationships");

        // Check the first relationship (OneToOne)
        let one_to_one = relationships
            .iter()
            .find(|r| r.field_name == "single_child")
            .unwrap();
        assert_eq!(
            one_to_one.relationship_type,
            RelationshipType::OneToOne,
            "First relationship should be OneToOne"
        );
        assert_eq!(
            one_to_one.cardinality,
            Cardinality::One,
            "First relationship cardinality should be One"
        );

        // Check the second relationship
        let many_relationship = relationships
            .iter()
            .find(|r| r.field_name == "multiple_children")
            .unwrap();

        assert_eq!(
            many_relationship.cardinality,
            Cardinality::OneOrMore,
            "List field cardinality should be OneOrMore"
        );
    }
}
