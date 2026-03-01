#![cfg(test)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::{
    DtoVM, EntityVM, FeatureVM, FieldVM, FileVM, GenerationSnapshot, GlobalVM, UseCaseVM,
    UserInterfaceVM, get_cpp_qt_tera,
};
use common::entities::{Entity, Field, FieldType, File, FileStatus, Global, Relationship, UserInterface};
use indexmap::IndexMap;
use tera::Context;

#[test]
fn render_direct_access_lib_lists_entities() {
    // Build a snapshot with file.entity = Some(0) and two entities in the map
    let file = File {
        id: 1,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        name: "CMakeLists.txt".into(),
        relative_path: "src/common/direct_access/".into(),
        group: "entities".into(),
        template_name: "common_direct_access_cmake".into(),
        generated_code: None,
        status: FileStatus::New,
        feature: None,
        entity: Some(0),
        use_case: None,
        field: None,
    };

    let global = Global {
        id: 50,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        language: "".to_string(),
        application_name: "".to_string(),
        organisation_name: "".to_string(),
        organisation_domain: "".to_string(),
        prefix_path: "".to_string(),
    };

    let e1 = Entity {
        id: 10,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        name: "Feature".into(),
        only_for_heritage: false,
        inherits_from: None,
        single_model: true,

        fields: vec![],
        relationships: vec![],
        undoable: false,
    };
    let e2 = Entity {
        id: 11,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        name: "Field".into(),
        only_for_heritage: false,
        single_model: true,
        inherits_from: None,

        fields: vec![],
        relationships: vec![],
        undoable: false,
    };

    let mut entities = IndexMap::new();
    entities.insert(
        10,
        EntityVM {
            inner: e1,
            relationships: IndexMap::new(),
            forward_relationships: IndexMap::new(),
            backward_relationships: IndexMap::new(),
            snake_name: "feature".into(),
            pascal_name: "Feature".into(),
            pascal_plural_name: "Features".into(),
            camel_name: "feature".into(),
            camel_plural_name: "features".to_string(),
            sql_safe_snake_name: "feature".to_string(),
            fields: vec![],
            normal_fields: vec![],
            owner: None,
            owner_pascal_name: None,
            owner_relationship_field_pascal_name: None,
            owner_relationship_type: None,
        },
    );
    entities.insert(
        11,
        EntityVM {
            inner: e2,
            relationships: IndexMap::new(),
            forward_relationships: IndexMap::new(),
            backward_relationships: IndexMap::new(),
            snake_name: "field".into(),
            pascal_name: "Field".into(),
            pascal_plural_name: "Fields".into(),
            camel_name: "field".into(),
            camel_plural_name: "fields".to_string(),
            sql_safe_snake_name: "field".to_string(),
            fields: vec![],
            normal_fields: vec![],
            owner: None,
            owner_pascal_name: None,
            owner_relationship_field_pascal_name: None,
            owner_relationship_type: None,
        },
    );

    let snapshot = GenerationSnapshot {
        file: FileVM { inner: file },
        global: GlobalVM {
            inner: global,
            application_kebab_name: "".to_string(),
            application_pascal_name: "".to_string(),
            application_snake_name: "".to_string(),
            application_short_name: "".to_string(),
            prefix: "".to_string(),
        },
        ui: UserInterfaceVM {
            inner: UserInterface::default(),
        },
        entities,
        features: IndexMap::new(),
        use_cases: IndexMap::new(),
        dtos: IndexMap::new(),
    };

    let tera = get_cpp_qt_tera();
    let mut context = Context::new();
    context.insert("s", &snapshot);
    let code = tera.render("common_direct_access_cmake", &context).unwrap();

    assert!(code.contains("field"));
}
