// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database/interface_database_table_group.h"
#include "database_test_persistence_export.h"
#include "dummy_entity_with_foreign.h"
#include "repository/generic_repository.h"
#include "repository/interface_dummy_entity_with_foreign_repository.h"
#include "repository/interface_dummy_other_entity_repository.h"
#include <QReadWriteLock>

using namespace DatabaseTest;
using namespace DatabaseTest::Contracts::Database;
using namespace DatabaseTest::Contracts::Repository;

namespace DatabaseTest::Persistence::Repository
{

class DATABASE_TEST_PERSISTENCE_EXPORT DummyEntityWithForeignRepository final
    : public DatabaseTest::Persistence::Repository::GenericRepository<DatabaseTest::Entities::DummyEntityWithForeign>,
      public DatabaseTest::Contracts::Repository::InterfaceDummyEntityWithForeignRepository
{
  public:
    explicit DummyEntityWithForeignRepository(
        InterfaceDatabaseTableGroup<DatabaseTest::Entities::DummyEntityWithForeign> *dummyEntityWithForeignDatabase,
        InterfaceDummyOtherEntityRepository *dummyOtherEntityRepository);

    SignalHolder *signalHolder() override;

    Result<DatabaseTest::Entities::DummyEntityWithForeign> update(
        DatabaseTest::Entities::DummyEntityWithForeign &&entity) override;
    Result<DatabaseTest::Entities::DummyEntityWithForeign> getWithDetails(int entityId) override;

    DatabaseTest::Entities::DummyEntityWithForeign::UniqueLoader fetchUniqueLoader() override;

    DatabaseTest::Entities::DummyEntityWithForeign::OrderedListLoader fetchOrderedListLoader() override;

    DatabaseTest::Entities::DummyEntityWithForeign::UnorderedListLoader fetchUnorderedListLoader() override;

    Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) override;

  private:
    InterfaceDummyOtherEntityRepository *m_dummyOtherEntityRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace DatabaseTest::Persistence::Repository