use anyhow::{Result, anyhow};
use serde_json::Value;

pub const CURRENT_SCHEMA_VERSION: i64 = 3;

/// Migrate a manifest JSON value to the current schema version.
/// Operates on raw `serde_json::Value` before schema validation.
pub fn migrate_to_current(value: &mut Value) -> Result<()> {
    let version = value
        .get("schema")
        .and_then(|s| s.get("version"))
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow!("Missing or invalid schema.version"))?;

    if version > CURRENT_SCHEMA_VERSION {
        return Err(anyhow!(
            "Manifest schema version {} is newer than supported version {}",
            version,
            CURRENT_SCHEMA_VERSION
        ));
    }

    if version <= 2 {
        migrate_v2_to_v3(value);
    }

    Ok(())
}

/// Migrate a v2 manifest to v3: remove `allow_direct_access` from entities, bump version.
fn migrate_v2_to_v3(value: &mut Value) {
    // Remove allow_direct_access from each entity
    if let Some(entities) = value.get_mut("entities").and_then(|e| e.as_array_mut()) {
        for entity in entities.iter_mut() {
            if let Some(obj) = entity.as_object_mut() {
                obj.remove("allow_direct_access");
            }
        }
    }

    // Bump version to 3
    if let Some(schema) = value.get_mut("schema").and_then(|s| s.as_object_mut()) {
        schema.insert("version".to_string(), Value::Number(3.into()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_migrate_v2_strips_allow_direct_access() {
        let mut value = json!({
            "schema": { "version": 2 },
            "global": { "language": "rust", "application_name": "Test", "organisation": { "name": "Test", "domain": "test.com" }, "prefix_path": "" },
            "entities": [
                { "name": "Foo", "allow_direct_access": true, "fields": [] },
                { "name": "Bar", "allow_direct_access": false, "fields": [] }
            ],
            "features": [],
            "ui": {}
        });

        migrate_to_current(&mut value).unwrap();

        // Version bumped to 3
        assert_eq!(value["schema"]["version"], 3);

        // allow_direct_access removed from all entities
        for entity in value["entities"].as_array().unwrap() {
            assert!(entity.get("allow_direct_access").is_none());
        }
    }

    #[test]
    fn test_v3_passes_through() {
        let mut value = json!({
            "schema": { "version": 3 },
            "global": { "language": "rust", "application_name": "Test", "organisation": { "name": "Test", "domain": "test.com" }, "prefix_path": "" },
            "entities": [
                { "name": "Foo", "fields": [] }
            ],
            "features": [],
            "ui": {}
        });

        migrate_to_current(&mut value).unwrap();
        assert_eq!(value["schema"]["version"], 3);
    }

    #[test]
    fn test_future_version_errors() {
        let mut value = json!({
            "schema": { "version": 99 },
            "global": {},
            "entities": [],
            "features": [],
            "ui": {}
        });

        let result = migrate_to_current(&mut value);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("newer than supported"));
    }
}
