use anyhow::Result;
use common::database::db_context::DbContext;
use common::event::EventHub;
use handling_manifest::{LoadDto, SaveDto};
use std::sync::Arc;

mod init;
use init::initialize_app;

fn export_to_mermaid_helper(manifest_path: &str) -> Result<String> {
    // Arrange
    let db_context = DbContext::new()?;
    let event_hub = Arc::new(EventHub::new());
    let load_dto = LoadDto {
        manifest_path: manifest_path.to_string(),
    };

    initialize_app(&db_context, &event_hub)?;
    handling_manifest::handling_manifest_controller::load(&db_context, &event_hub, &load_dto)?;
    let mermaid_output = handling_manifest::handling_manifest_controller::export_to_mermaid(
        &db_context,
        &event_hub,
    )?;

    Ok(mermaid_output.mermaid_diagram)
}

#[test]
fn test_1_entity() -> Result<()> {
    let mermaid_diagram = export_to_mermaid_helper(
        "../../crates/handling_manifest/tests/export_to_mermaid_test_manifests/qleany_1_entity.yaml",
    )?;

    let expected_diagram = String::from(
        r#"erDiagram
Root {
    EntityId id
    First first
}
Root ||--|| First : owns

First {
    EntityId id
    string test_string "optional"
    bool test_bool
}
"#,
    );

    assert_eq!(mermaid_diagram, expected_diagram);

    Ok(())
}

#[test]
fn test_2_entities() -> Result<()> {
    let mermaid_diagram = export_to_mermaid_helper(
        "../../crates/handling_manifest/tests/export_to_mermaid_test_manifests/qleany_2_entities.yaml",
    )?;

    let expected_diagram = String::from(
        r#"erDiagram
Root {
    EntityId id
    First first "optional"
}
Root ||--o| First : owns

First {
    EntityId id
    string test_string "optional"
    Second second "optional"
}
First ||--o| Second : owns

Second {
    EntityId id
    string test_string
    bool test_bool
}
"#,
    );

    assert_eq!(mermaid_diagram, expected_diagram);

    Ok(())
}

#[test]
fn test_full() -> Result<()> {
    let mermaid_diagram = export_to_mermaid_helper(
        "../../crates/handling_manifest/tests/export_to_mermaid_test_manifests/qleany_full.yaml",
    )?;

    let expected_diagram = String::from(
        r#"erDiagram
Root {
    EntityId id
    First first
}
Root ||--|| First : owns

First {
    EntityId id
    string test_string "optional"
    Second second "optional"
}
First ||--o| Second : owns

Second {
    EntityId id
    string test_string
    bool test_bool
    Third thirds "optional"
}
Second ||--o{ Third : owns

Third {
    EntityId id
    string test_string
    Fourth fourths "optional"
    Fifth fifths
}
Third ||--o{ Fourth : owns
Third ||--o{ Fifth : "owns ordered"

Fourth {
    EntityId id
    string test_string
    Second second_ref "optional"
}
Fourth ||--o| Second : refs

Fifth {
    EntityId id
    string test_string "optional"
    Third third_refs "optional"
}
Fifth }o..o{ Third : refs
"#,
    );

    assert_eq!(mermaid_diagram, expected_diagram);

    Ok(())
}
