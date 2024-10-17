use entity_enums::EntitySchema;

pub mod entity_enums;
pub mod root;


pub trait EntityTrait {
    fn schema() -> EntitySchema<'static>;
}
