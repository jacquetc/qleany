use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub version: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Organisation {
    pub name: String,
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct Global {
    pub language: String,
    pub application_name: String,
    pub organisation: Organisation,
    pub prefix_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub is_list: Option<bool>,
    pub ordered: Option<bool>,
    pub strong: Option<bool>,
    pub list_model: Option<bool>,
    pub list_model_displayed_field: Option<String>,
    pub is_nullable: Option<bool>,
    pub is_primary_key: Option<bool>,
    pub single: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub parent: Option<String>,
    pub only_for_heritage: Option<bool>,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct DtoField {
    pub name: String,
    pub r#type: String,
    pub is_nullable: Option<bool>,
    pub is_list: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Dto {
    pub name: String,
    pub fields: Vec<DtoField>,
}

#[derive(Serialize, Deserialize)]
pub struct UseCase {
    pub name: String,
    pub validator: bool,
    pub entities: Option<Vec<String>>,
    pub undoable: bool,
    pub dto_in: Option<Dto>,
    pub dto_out: Option<Dto>,
}

#[derive(Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub use_cases: Vec<UseCase>,
}

#[derive(Serialize, Deserialize)]
pub struct Ui {
    pub cli: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub schema: Schema,
    pub global: Global,
    pub entities: Vec<Entity>,
    pub features: Vec<Feature>,
    pub ui: Ui,
}