use crate::entities::EntityId;
use redb::{ReadableTable, Error};


pub fn delete_from_backward_junction_table(junction_table: &mut redb::Table<'_, u64, Vec<u64>>, id: &EntityId) -> Result<(), Error> {
    let mut relationship_iter = junction_table.iter()?;
    let mut junctions_to_modify: Vec<(EntityId, Vec<EntityId>)> = vec![];
    while let Some(Ok((left_id, right_entities))) = relationship_iter.next() {
        let left_id = left_id.value();
        let right_entities = right_entities.value();
        let entities_left: Vec<EntityId> = right_entities.clone().into_iter().filter(|entity_id| *entity_id != *id).collect();
        
        if entities_left.len() == right_entities.len() {
            continue;
        }
        junctions_to_modify.push((left_id, entities_left));
    }
    for (left_id, entities) in junctions_to_modify {
        junction_table.insert(left_id, entities)?;
    }

    Ok(())
}