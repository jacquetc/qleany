use common::database::db_context::DbContext;
use anyhow::Result;
use crate::{units_of_work::LoadUnitOfWorkFactory, use_cases::load_uc::LoadUseCase, LoadDto};


pub fn load(db_context: &DbContext, dto: &LoadDto) -> Result<()> {
    let uow_context = LoadUnitOfWorkFactory::new(&db_context);
    let mut laod_uc = LoadUseCase::new(Box::new(uow_context));
    laod_uc.execute(dto)
}


// test
#[cfg(test)]
mod tests {
    use super::*;
    use common::database::db_context::DbContext;
    

    #[test]
    fn test_load_yaml() {
        let db_context = DbContext::new().unwrap();
        let load_dto = LoadDto {
            manifest_path: "../../qleany.yaml".to_string(),
        };
        load(&db_context, &load_dto).unwrap();
    }
}