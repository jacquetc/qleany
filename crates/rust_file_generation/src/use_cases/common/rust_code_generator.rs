use anyhow::Result;
use common::entities::{Dto, DtoField, Entity, Feature, Field, File, UseCase};
use common::types::EntityId;
use include_dir::{Dir, include_dir};
use std::collections::HashMap;
use std::sync::OnceLock;
use tera::{Context, Tera};

static RUST_TEMPLATES_DIR: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/src/use_cases/common/templates");

static RUST_TERA: OnceLock<Tera> = OnceLock::new();

fn get_rust_tera() -> &'static Tera {
    RUST_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        load_templates_from_dir(&mut tera, &RUST_TEMPLATES_DIR);
        tera
    })
}

fn load_templates_from_dir(tera: &mut Tera, dir: &Dir) {
    for file in dir.files() {
        if let Some(Some(file_stem)) = file.path().file_stem().map(|s| s.to_str()) {
            // remove last ".*" from the file stem
            let file_stem = file_stem
                .rsplit_once('.')
                .map_or(file_stem, |(stem, _)| stem);

            // Add the template to Tera
            // The file contents are expected to be UTF-8 encoded
            let content = file.contents_utf8().expect("Invalid UTF-8 in template");
            tera.add_raw_template(file_stem, content)
                .expect("Failed to add template");
        }
    }
    tera.build_inheritance_chains()
        .expect("Failed to build inheritance");
}

pub(crate) fn generate_code(
    file: File,
    entities: HashMap<EntityId, Entity>,
    fields: HashMap<EntityId, Field>,
    features: HashMap<EntityId, Feature>,
    use_cases: HashMap<EntityId, UseCase>,
    dtos: HashMap<EntityId, Dto>,
    dto_fields: HashMap<EntityId, DtoField>,
) -> Result<String> {
    let tera = get_rust_tera();
    let mut context = Context::new();

    let code = match file.template_name.as_str() {
        "root_cargo" => tera.render("root_cargo", &context)?,
        "common_cargo" => tera.render("common_cargo", &context)?,
        _ => {
            return Err(anyhow::anyhow!(
                "Unknown template name: {}",
                file.template_name
            ));
        }
    };

    Ok(code)
}
