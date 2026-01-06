pub mod write {

    use crate::direct_access::file::file_repository::FileRepository;
    use crate::direct_access::file::file_table::FileRedbTable;
    use crate::{
        database::transactions::Transaction,
        direct_access::{
            dto::{dto_repository::DtoRepository, dto_table::DtoRedbTable},
            dto_field::{
                dto_field_repository::DtoFieldRepository, dto_field_table::DtoFieldRedbTable,
            },
            entity::{entity_repository::EntityRepository, entity_table::EntityRedbTable},
            feature::{feature_repository::FeatureRepository, feature_table::FeatureRedbTable},
            field::{field_repository::FieldRepository, field_table::FieldRedbTable},
            global::{global_repository::GlobalRepository, global_table::GlobalRedbTable},
            relationship::{
                relationship_repository::RelationshipRepository,
                relationship_table::RelationshipRedbTable,
            },
            root::{root_repository::RootRepository, root_table::RootRedbTable},
            use_case::{use_case_repository::UseCaseRepository, use_case_table::UseCaseRedbTable},
        },
    };

    pub fn create_root_repository(transaction: &Transaction) -> RootRepository {
        let root_table = RootRedbTable::new(transaction.get_write_transaction());
        RootRepository::new(Box::new(root_table), transaction)
    }

    pub fn create_entity_repository(transaction: &Transaction) -> EntityRepository {
        let entity_table = EntityRedbTable::new(transaction.get_write_transaction());
        EntityRepository::new(Box::new(entity_table), transaction)
    }

    pub fn create_use_case_repository(transaction: &Transaction) -> UseCaseRepository {
        let use_case_table = UseCaseRedbTable::new(transaction.get_write_transaction());
        UseCaseRepository::new(Box::new(use_case_table), transaction)
    }

    pub fn create_feature_repository(transaction: &Transaction) -> FeatureRepository {
        let table = FeatureRedbTable::new(transaction.get_write_transaction());
        FeatureRepository::new(Box::new(table), transaction)
    }

    pub fn create_global_repository(transaction: &Transaction) -> GlobalRepository {
        let global_table = GlobalRedbTable::new(transaction.get_write_transaction());
        GlobalRepository::new(Box::new(global_table), transaction)
    }

    pub fn create_dto_repository(transaction: &Transaction) -> DtoRepository {
        let table = DtoRedbTable::new(transaction.get_write_transaction());
        DtoRepository::new(Box::new(table), transaction)
    }

    pub fn create_dto_field_repository(transaction: &Transaction) -> DtoFieldRepository {
        let table = DtoFieldRedbTable::new(transaction.get_write_transaction());
        DtoFieldRepository::new(Box::new(table), transaction)
    }

    pub fn create_field_repository(transaction: &Transaction) -> FieldRepository {
        let table = FieldRedbTable::new(transaction.get_write_transaction());
        FieldRepository::new(Box::new(table), transaction)
    }

    pub fn create_relationship_repository(transaction: &Transaction) -> RelationshipRepository {
        let table = RelationshipRedbTable::new(transaction.get_write_transaction());
        RelationshipRepository::new(Box::new(table), transaction)
    }

    pub fn create_file_repository(transaction: &Transaction) -> FileRepository {
        let table = FileRedbTable::new(transaction.get_write_transaction());
        FileRepository::new(Box::new(table), transaction)
    }
}

pub mod read {
    use crate::direct_access::file::file_repository::FileRepositoryRO;
    use crate::direct_access::file::file_table::FileRedbTableRO;
    use crate::{
        database::transactions::Transaction,
        direct_access::{
            dto::{dto_repository::DtoRepositoryRO, dto_table::DtoRedbTableRO},
            dto_field::{
                dto_field_repository::DtoFieldRepositoryRO, dto_field_table::DtoFieldRedbTableRO,
            },
            entity::{entity_repository::EntityRepositoryRO, entity_table::EntityRedbTableRO},
            feature::{feature_repository::FeatureRepositoryRO, feature_table::FeatureRedbTableRO},
            field::{field_repository::FieldRepositoryRO, field_table::FieldRedbTableRO},
            global::{global_repository::GlobalRepositoryRO, global_table::GlobalRedbTableRO},
            relationship::{
                relationship_repository::RelationshipRepositoryRO,
                relationship_table::RelationshipRedbTableRO,
            },
            root::{root_repository::RootRepositoryRO, root_table::RootRedbTableRO},
            use_case::{
                use_case_repository::UseCaseRepositoryRO, use_case_table::UseCaseRedbTableRO,
            },
        },
    };

    pub fn create_root_repository(transaction: &Transaction) -> RootRepositoryRO {
        let root_table = RootRedbTableRO::new(transaction.get_read_transaction());
        RootRepositoryRO::new(Box::new(root_table))
    }

    pub fn create_entity_repository(transaction: &Transaction) -> EntityRepositoryRO {
        let entity_table = EntityRedbTableRO::new(transaction.get_read_transaction());
        EntityRepositoryRO::new(Box::new(entity_table))
    }

    pub fn create_use_case_repository(transaction: &Transaction) -> UseCaseRepositoryRO {
        let use_case_table = UseCaseRedbTableRO::new(transaction.get_read_transaction());
        UseCaseRepositoryRO::new(Box::new(use_case_table))
    }

    pub fn create_feature_repository(transaction: &Transaction) -> FeatureRepositoryRO {
        let table = FeatureRedbTableRO::new(transaction.get_read_transaction());
        FeatureRepositoryRO::new(Box::new(table))
    }

    pub fn create_global_repository(transaction: &Transaction) -> GlobalRepositoryRO {
        let global_table = GlobalRedbTableRO::new(transaction.get_read_transaction());
        GlobalRepositoryRO::new(Box::new(global_table))
    }

    pub fn create_dto_repository(transaction: &Transaction) -> DtoRepositoryRO {
        let table = DtoRedbTableRO::new(transaction.get_read_transaction());
        DtoRepositoryRO::new(Box::new(table))
    }

    pub fn create_dto_field_repository(transaction: &Transaction) -> DtoFieldRepositoryRO {
        let table = DtoFieldRedbTableRO::new(transaction.get_read_transaction());
        DtoFieldRepositoryRO::new(Box::new(table))
    }

    pub fn create_field_repository(transaction: &Transaction) -> FieldRepositoryRO {
        let table = FieldRedbTableRO::new(transaction.get_read_transaction());
        FieldRepositoryRO::new(Box::new(table))
    }

    pub fn create_relationship_repository(transaction: &Transaction) -> RelationshipRepositoryRO {
        let table = RelationshipRedbTableRO::new(transaction.get_read_transaction());
        RelationshipRepositoryRO::new(Box::new(table))
    }

    pub fn create_file_repository(transaction: &Transaction) -> FileRepositoryRO {
        let table = FileRedbTableRO::new(transaction.get_read_transaction());
        FileRepositoryRO::new(Box::new(table))
    }
}
