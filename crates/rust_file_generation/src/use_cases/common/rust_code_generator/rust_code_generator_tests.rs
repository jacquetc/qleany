use super::{GenerationReadOps, SnapshotBuilder};
use anyhow::Result;
use common::database::QueryUnitOfWork;
use common::entities::{
    Dto, DtoField, Entity, Feature, Field, FieldType, File, Global, Relationship, RelationshipType,
    UseCase,
};
use common::types::EntityId;
use std::collections::HashMap;

// DummyGenerationReadOps that allows setting return values
struct DummyGenerationReadOps {
    files: HashMap<EntityId, File>,
    globals: HashMap<EntityId, Global>,
    features: HashMap<EntityId, Feature>,
    use_cases: HashMap<EntityId, UseCase>,
    entities: HashMap<EntityId, Entity>,
    dtos: HashMap<EntityId, Dto>,
    dto_fields: HashMap<EntityId, DtoField>,
    fields: HashMap<EntityId, Field>,
    relationships: HashMap<EntityId, Relationship>,
    root_entities: Vec<EntityId>,
}

impl DummyGenerationReadOps {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
            globals: HashMap::new(),
            features: HashMap::new(),
            use_cases: HashMap::new(),
            entities: HashMap::new(),
            dtos: HashMap::new(),
            dto_fields: HashMap::new(),
            fields: HashMap::new(),
            relationships: HashMap::new(),
            root_entities: vec![],
        }
    }
}

// Implement minimal QueryUnitOfWork
impl QueryUnitOfWork for DummyGenerationReadOps {
    fn begin_transaction(&self) -> Result<()> {
        Ok(())
    }
    fn end_transaction(&self) -> Result<()> {
        Ok(())
    }
}

// Implement GenerationReadOps methods (declared via macros on the trait)
impl GenerationReadOps for DummyGenerationReadOps {
    fn get_root_relationship(
        &self,
        _id: &EntityId,
        field: &common::direct_access::root::RootRelationshipField,
    ) -> Result<Vec<EntityId>> {
        match field {
            common::direct_access::root::RootRelationshipField::Entities => {
                Ok(self.root_entities.clone())
            }
            _ => Ok(vec![]),
        }
    }
    fn get_file(&self, id: &EntityId) -> Result<Option<File>> {
        Ok(self.files.get(id).cloned())
    }
    fn get_global(&self, id: &EntityId) -> Result<Option<Global>> {
        Ok(self.globals.get(id).cloned())
    }
    fn get_feature(&self, id: &EntityId) -> Result<Option<Feature>> {
        Ok(self.features.get(id).cloned())
    }
    fn get_feature_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Feature>>> {
        Ok(ids.iter().map(|i| self.features.get(i).cloned()).collect())
    }
    fn get_use_case(&self, id: &EntityId) -> Result<Option<UseCase>> {
        Ok(self.use_cases.get(id).cloned())
    }
    fn get_use_case_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<UseCase>>> {
        Ok(ids.iter().map(|i| self.use_cases.get(i).cloned()).collect())
    }
    fn get_dto(&self, id: &EntityId) -> Result<Option<Dto>> {
        Ok(self.dtos.get(id).cloned())
    }
    fn get_dto_field(&self, id: &EntityId) -> Result<Option<DtoField>> {
        Ok(self.dto_fields.get(id).cloned())
    }
    fn get_dto_field_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<DtoField>>> {
        Ok(ids
            .iter()
            .map(|i| self.dto_fields.get(i).cloned())
            .collect())
    }
    fn get_entity(&self, id: &EntityId) -> Result<Option<Entity>> {
        Ok(self.entities.get(id).cloned())
    }
    fn get_entity_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Entity>>> {
        Ok(ids.iter().map(|i| self.entities.get(i).cloned()).collect())
    }
    fn get_field(&self, id: &EntityId) -> Result<Option<Field>> {
        Ok(self.fields.get(id).cloned())
    }
    fn get_field_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Field>>> {
        Ok(ids.iter().map(|i| self.fields.get(i).cloned()).collect())
    }
    fn get_relationship(&self, id: &EntityId) -> Result<Option<Relationship>> {
        Ok(self.relationships.get(id).cloned())
    }
    fn get_relationship_multi(&self, ids: &[EntityId]) -> Result<Vec<Option<Relationship>>> {
        Ok(ids
            .iter()
            .map(|i| self.relationships.get(i).cloned())
            .collect())
    }
}

#[test]
fn for_file_returns_err_when_file_missing() {
    let uow = DummyGenerationReadOps::new();
    let res = SnapshotBuilder::for_file(&uow, 1, &Vec::new());
    assert!(res.is_err());
}

#[test]
fn for_file_feature_without_use_cases_errors() {
    let mut uow = DummyGenerationReadOps::new();
    let file = File {
        id: 1,
        name: "f".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "feature_lib".into(),
        feature: Some(10),
        entity: None,
        use_case: None,
    };
    uow.files.insert(1, file);
    let feature = Feature {
        id: 10,
        name: "Feat".into(),
        use_cases: vec![],
    };
    uow.features.insert(10, feature);
    let res = SnapshotBuilder::for_file(&uow, 1, &Vec::new());
    assert!(res.is_err());
}

#[test]
fn for_file_happy_path_feature_with_use_case_and_dtos() {
    let mut uow = DummyGenerationReadOps::new();
    // File bound to feature
    let file = File {
        id: 1,
        name: "f".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "feature_lib".into(),
        feature: Some(10),
        entity: None,
        use_case: None,
    };
    uow.files.insert(1, file);
    // Feature with use case 100
    let uc = UseCase {
        id: 100,
        name: "UC".into(),
        validator: false,
        entities: vec![300],
        undoable: false,
        read_only: false,
        long_operation: false,
        dto_in: Some(200),
        dto_out: Some(201),
    };
    let feature = Feature {
        id: 10,
        name: "Feat".into(),
        use_cases: vec![100],
    };
    uow.features.insert(10, feature);
    uow.use_cases.insert(100, uc.clone());
    // Entity and fields
    let ent = Entity {
        id: 300,
        name: "User".into(),
        only_for_heritage: false,
        inherits_from: None,
        allow_direct_access: true,
        fields: vec![400],
        relationships: vec![],
    };
    let field = Field {
        id: 400,
        name: "name".into(),
        field_type: FieldType::String,
        entity: Some(300),
        relationship: RelationshipType::OneToOne,
        required: false,
        single_model: true,
        strong: true,
        list_model: false,
        list_model_displayed_field: None,
        enum_name: None,
        enum_values: None,
    };
    uow.entities.insert(300, ent);
    uow.fields.insert(400, field);
    // DTOs and fields
    let dto_in = Dto {
        id: 200,
        name: "In".into(),
        fields: vec![500],
    };
    let dto_out = Dto {
        id: 201,
        name: "Out".into(),
        fields: vec![501],
    };
    uow.dtos.insert(200, dto_in);
    uow.dtos.insert(201, dto_out);
    let df_in = DtoField {
        id: 500,
        name: "a".into(),
        field_type: common::entities::DtoFieldType::String,
        is_nullable: false,
        is_list: false,
        enum_name: None,
        enum_values: None,
    };
    let df_out = DtoField {
        id: 501,
        name: "b".into(),
        field_type: common::entities::DtoFieldType::Integer,
        is_nullable: true,
        is_list: false,
        enum_name: None,
        enum_values: None,
    };
    uow.dto_fields.insert(500, df_in);
    uow.dto_fields.insert(501, df_out);

    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1, &Vec::new()).expect("snapshot");
    assert!(snap.features.contains_key(&10));
    assert!(snap.use_cases.contains_key(&100));
    assert!(snap.entities.contains_key(&300));
    assert!(snap.dtos.contains_key(&200) && snap.dtos.contains_key(&201));
}

#[test]
fn for_file_various_combinations_generate_expected_items() {
    // Prepare uow with feature, use_case, entities, dtos
    let mut uow = DummyGenerationReadOps::new();

    // Common entities
    let ent_a = Entity {
        id: 1,
        name: "A".into(),
        only_for_heritage: false,
        inherits_from: None,
        allow_direct_access: true,
        fields: vec![],
        relationships: vec![],
    };
    let ent_b = Entity {
        id: 2,
        name: "B".into(),
        only_for_heritage: false,
        inherits_from: None,
        allow_direct_access: true,
        fields: vec![],
        relationships: vec![],
    };
    uow.entities.insert(1, ent_a.clone());
    uow.entities.insert(2, ent_b.clone());
    // Root contains both entities (for entity: Some(0))
    uow.root_entities = vec![1, 2];

    // DTOs for UC
    let dto_in = Dto {
        id: 10,
        name: "In".into(),
        fields: vec![],
    };
    let dto_out = Dto {
        id: 11,
        name: "Out".into(),
        fields: vec![],
    };
    uow.dtos.insert(10, dto_in);
    uow.dtos.insert(11, dto_out);

    // Use case referencing ent_a and ent_b
    let uc = UseCase {
        id: 100,
        name: "UC".into(),
        validator: false,
        entities: vec![1, 2],
        undoable: false,
        read_only: false,
        long_operation: false,
        dto_in: Some(10),
        dto_out: Some(11),
    };
    uow.use_cases.insert(100, uc.clone());

    // Feature with the UC
    let feat = Feature {
        id: 200,
        name: "Feat".into(),
        use_cases: vec![100],
    };
    uow.features.insert(200, feat.clone());

    // 1) File with only feature
    let file_feature_only = File {
        id: 1000,
        name: "f1".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "feature_lib".into(),
        feature: Some(200),
        entity: None,
        use_case: None,
    };
    uow.files.insert(1000, file_feature_only);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1000, &Vec::new()).expect("snapshot");
    assert!(snap.features.contains_key(&200));
    assert!(snap.use_cases.contains_key(&100));
    assert!(snap.entities.contains_key(&1) && snap.entities.contains_key(&2));
    assert!(snap.dtos.contains_key(&10) && snap.dtos.contains_key(&11));

    // 2) File with only use_case
    let file_uc_only = File {
        id: 1001,
        name: "f2".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "feature_use_case".into(),
        feature: None,
        entity: None,
        use_case: Some(100),
    };
    uow.files.insert(1001, file_uc_only);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1001, &Vec::new()).expect("snapshot");
    assert!(snap.features.is_empty());
    assert!(snap.use_cases.contains_key(&100));
    assert!(snap.entities.contains_key(&1) && snap.entities.contains_key(&2));
    assert!(snap.dtos.contains_key(&10) && snap.dtos.contains_key(&11));

    // 3) File with only entity
    let file_ent_only = File {
        id: 1002,
        name: "f3".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "entity_mod".into(),
        feature: None,
        entity: Some(1),
        use_case: None,
    };
    uow.files.insert(1002, file_ent_only);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1002, &Vec::new()).expect("snapshot");
    assert!(snap.features.is_empty());
    assert!(snap.use_cases.is_empty());
    assert!(snap.entities.contains_key(&1));

    // 4) File with entity Some(0) -> loads all entities from root
    let file_all_ent = File {
        id: 1003,
        name: "f4".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "entity_mod".into(),
        feature: None,
        entity: Some(0),
        use_case: None,
    };
    uow.files.insert(1003, file_all_ent);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1003, &Vec::new()).expect("snapshot");
    assert!(snap.entities.contains_key(&1) && snap.entities.contains_key(&2));

    // 5) File with feature + entity: ensure both feature scope (UCs, dtos, uc entities) and explicit entity are included
    let file_feat_ent = File {
        id: 1004,
        name: "f5".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "feature_lib".into(),
        feature: Some(200),
        entity: Some(1),
        use_case: None,
    };
    uow.files.insert(1004, file_feat_ent);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1004, &Vec::new()).expect("snapshot");
    assert!(snap.features.contains_key(&200));
    assert!(snap.use_cases.contains_key(&100));
    // must include entity 1 (explicit) and UC entities
    assert!(snap.entities.contains_key(&1) && snap.entities.contains_key(&2));

    // 6) File with use_case + entity
    let file_uc_ent = File {
        id: 1005,
        name: "f6".into(),
        relative_path: "p".into(),
        group: "g".into(),
        template_name: "entity_use_cases_mod".into(),
        feature: None,
        entity: Some(2),
        use_case: Some(100),
    };
    uow.files.insert(1005, file_uc_ent);
    let (snap, _from_cache) = SnapshotBuilder::for_file(&uow, 1005, &Vec::new()).expect("snapshot");
    assert!(snap.use_cases.contains_key(&100));
    // entities from UC plus explicitly provided entity
    assert!(snap.entities.contains_key(&1) && snap.entities.contains_key(&2));
}
