use super::{
    DtoVM, EntityVM, FeatureVM, FieldVM, FileVM, GenerationSnapshot, GlobalVM, UseCaseVM,
    UserInterfaceVM, get_rust_tera,
};
use common::entities::{Entity, Field, FieldType, File, Global, Relationship, UserInterface};
use common::types::EntityId;
use indexmap::IndexMap;
use tera::Context;

#[test]
fn render_direct_access_lib_lists_entities() {
    // Build a snapshot with file.entity = Some(0) and two entities in the map
    let file = File {
        id: 1,
        name: "lib".into(),
        relative_path: "crates/direct_access/src/lib.rs".into(),
        group: "entities".into(),
        template_name: "direct_access_lib".into(),
        feature: None,
        entity: Some(0),
        use_case: None,
    };

    let global = Global {
        id: 50,
        language: "".to_string(),
        application_name: "".to_string(),
        organisation_name: "".to_string(),
        organisation_domain: "".to_string(),
        prefix_path: "".to_string(),
    };

    let e1 = Entity {
        id: 10,
        name: "Feature".into(),
        only_for_heritage: false,
        inherits_from: None,
        single_model: true,
        allow_direct_access: true,
        fields: vec![],
        relationships: vec![],
        undoable: false,
    };
    let e2 = Entity {
        id: 11,
        name: "Field".into(),
        only_for_heritage: false,
        single_model: true,
        inherits_from: None,
        allow_direct_access: true,
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
            fields: vec![],
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
            fields: vec![],
        },
    );

    let snapshot = GenerationSnapshot {
        file: FileVM { inner: file },
        global: GlobalVM {
            inner: global,
            application_kebab_name: "".to_string(),
        },
        ui: UserInterfaceVM {
            inner: UserInterface::default(),
            application_kebab_name: "".to_string(),
            application_snake_name: "".to_string(),
        },
        entities,
        features: IndexMap::new(),
        use_cases: IndexMap::new(),
        dtos: IndexMap::new(),
    };

    let tera = get_rust_tera();
    let mut context = Context::new();
    context.insert("s", &snapshot);
    let code = tera.render("direct_access_lib", &context).unwrap();

    assert!(code.contains("pub mod feature;"));
    assert!(code.contains("pub mod field;"));
    assert!(code.contains("pub use feature::feature_controller;"));
    assert!(code.contains("pub use field::field_controller;"));
}
