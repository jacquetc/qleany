pub struct SqliteDatabaseAccess<T> 
    where T: common_entities::EntityTrait {
    _entity: std::marker::PhantomData<T>,


}

impl<T> SqliteDatabaseAccess<T> 
    where T: common_entities::EntityTrait {
    pub fn create(entities: &[T]) -> Result<Vec<T>, crate::database::DatabaseError> {
        todo!()
    }
}