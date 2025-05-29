use anyhow::Result;
use common::database::db_context::DbContext;
use common::event::EventHub;
use handling_manifest::{LoadDto, SaveDto};
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[test]
fn test_load_yaml() -> Result<()> {
    // Arrange
    let db_context = DbContext::new()?;
    let event_hub = Arc::new(EventHub::new());
    let load_dto = LoadDto {
        manifest_path: "../../qleany.yaml".to_string(),
    };

    // Act
    handling_manifest::handling_manifest_controller::load(&db_context, &event_hub, &load_dto)?;

    // Assert
    // If we got here without errors, the test passed
    Ok(())
}

#[test]
fn test_save_yaml() -> Result<()> {
    // Arrange
    let db_context = DbContext::new()?;
    let event_hub = Arc::new(EventHub::new());
    let original_file_path = "../../qleany.yaml";
    let temp_file_path = "../../temp_manifest.yaml";

    // Load the original manifest
    let load_dto = LoadDto {
        manifest_path: original_file_path.to_string(),
    };
    handling_manifest::handling_manifest_controller::load(&db_context, &event_hub, &load_dto)?;

    // Save to a temporary file
    let save_dto = SaveDto {
        manifest_path: temp_file_path.to_string(),
    };

    // Act
    let result =
        handling_manifest::handling_manifest_controller::save(&db_context, &event_hub, &save_dto);

    if let Err(e) = result {
        // If save failed, we still want to clean up the temp file if it exists
        if Path::new(temp_file_path).exists() {
            fs::remove_file(temp_file_path)?;
        }
        return Err(e);
    }

    // Clean up
    if Path::new(temp_file_path).exists() {
        fs::remove_file(temp_file_path)?;
    }

    Ok(())
}

#[test]
fn test_load_and_save_yaml() -> Result<()> {
    // Arrange
    let db_context = DbContext::new()?;
    let event_hub = Arc::new(EventHub::new());
    let original_file_path = "../../qleany.yaml";
    let temp_file_path = "../../temp_manifest.yaml";

    // Load the original manifest
    let load_dto = LoadDto {
        manifest_path: original_file_path.to_string(),
    };
    handling_manifest::handling_manifest_controller::load(&db_context, &event_hub, &load_dto)?;

    // Save to a temporary file
    let save_dto = SaveDto {
        manifest_path: temp_file_path.to_string(),
    };

    // Act
    let save_result =
        handling_manifest::handling_manifest_controller::save(&db_context, &event_hub, &save_dto);

    if let Err(e) = save_result {
        // If save failed, we still want to clean up the temp file if it exists
        if Path::new(temp_file_path).exists() {
            fs::remove_file(temp_file_path)?;
        }
        return Err(e);
    }

    // Assert
    // Check if the temporary file was created
    if !Path::new(temp_file_path).exists() {
        return Err(anyhow::anyhow!("Temporary file was not created"));
    }
    // Clean up
    // if Path::new(temp_file_path).exists() {
    //     fs::remove_file(temp_file_path)?;
    // }

    Ok(())
}
