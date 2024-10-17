use crate::RepositoryFactoryTrait;


pub fn register() ->Box<dyn RepositoryFactoryTrait> {
    println!("Registering common_direct_access");

    let db_context = SqliteDbContext::new();


}