// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include "database/database_context.h"
#include "database/database_table_group.h"

#include "repository/dummy_basic_entity_repository.h"
#include "repository/dummy_entity_with_foreign_repository.h"
#include "repository/dummy_other_entity_repository.h"
#include "repository/generic_repository.h"
#include "repository/repository_provider.h"

using namespace DatabaseTest;
using namespace DatabaseTest::Persistence::Database;
using namespace DatabaseTest::Persistence;
using namespace DatabaseTest::Persistence::Repository;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{
    QSharedPointer<DatabaseContext> context(new DatabaseContext());

    // database tables:

    auto *dummyBasicEntityDatabaseTableGroup =
        new DatabaseTableGroup<DatabaseTest::Entities::DummyBasicEntity>(context);
    auto *dummyEntityWithForeignDatabaseTableGroup =
        new DatabaseTableGroup<DatabaseTest::Entities::DummyEntityWithForeign>(context);
    auto *dummyOtherEntityDatabaseTableGroup =
        new DatabaseTableGroup<DatabaseTest::Entities::DummyOtherEntity>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n"_L1 + error.code() + "\n"_L1 + error.message() + "\n"_L1 + error.data();
    }

    // repositories:

    DummyOtherEntityRepository *dummyOtherEntityRepository =
        new DummyOtherEntityRepository(dummyOtherEntityDatabaseTableGroup);
    DummyBasicEntityRepository *dummyBasicEntityRepository =
        new DummyBasicEntityRepository(dummyBasicEntityDatabaseTableGroup);
    DummyEntityWithForeignRepository *dummyEntityWithForeignRepository =
        new DummyEntityWithForeignRepository(dummyEntityWithForeignDatabaseTableGroup, dummyOtherEntityRepository);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("dummyBasicEntity", dummyBasicEntityRepository);
    RepositoryProvider::instance()->registerRepository("dummyEntityWithForeign", dummyEntityWithForeignRepository);
    RepositoryProvider::instance()->registerRepository("dummyOtherEntity", dummyOtherEntityRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}