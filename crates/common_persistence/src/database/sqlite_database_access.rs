use crate::database::DatabaseError;

pub struct SqliteDatabaseAccess<T>
where
    T: common_entities::EntityTrait,
{
    _entity: std::marker::PhantomData<T>,
}

impl<T> SqliteDatabaseAccess<T>
where
    T: common_entities::EntityTrait,
{
    pub fn new(db_connection: SqliteDbConnection) -> SqliteDatabaseAccess<T> {
        SqliteDatabaseAccess {
            _entity: std::marker::PhantomData,
            db_connection
        }
    }
    
    pub fn create(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }
    pub fn get(&self, ids: &[i64]) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }

    pub fn update(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }

    pub fn remove(&self, ids: &[i64]) -> Result<(), DatabaseError> {
        todo!()
    }
}
