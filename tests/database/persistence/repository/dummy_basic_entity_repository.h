// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database/interface_database_table_group.h"
#include "database_test_persistence_export.h"
#include "dummy_basic_entity.h"
#include "repository/generic_repository.h"
#include "repository/interface_dummy_basic_entity_repository.h"
#include <QReadWriteLock>

using namespace DatabaseTest;
using namespace DatabaseTest::Contracts::Database;
using namespace DatabaseTest::Contracts::Repository;

namespace DatabaseTest::Persistence::Repository
{

class DATABASE_TEST_PERSISTENCE_EXPORT DummyBasicEntityRepository final
    : public DatabaseTest::Persistence::Repository::GenericRepository<DatabaseTest::Entities::DummyBasicEntity>,
      public DatabaseTest::Contracts::Repository::InterfaceDummyBasicEntityRepository
{
  public:
    explicit DummyBasicEntityRepository(
        InterfaceDatabaseTableGroup<DatabaseTest::Entities::DummyBasicEntity> *dummyBasicEntityDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace DatabaseTest::Persistence::Repository