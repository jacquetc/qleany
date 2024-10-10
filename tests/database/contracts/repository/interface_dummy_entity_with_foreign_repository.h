// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database_test_contracts_export.h"
#include "dummy_entity_with_foreign.h"
#include "repository/interface_generic_repository.h"
#include "repository/interface_repository.h"
#include "result.h"
#include <QObject>

using namespace DatabaseTest;

namespace DatabaseTest::Contracts::Repository
{

class DATABASE_TEST_CONTRACTS_EXPORT InterfaceDummyEntityWithForeignRepository
    : public virtual DatabaseTest::Contracts::Repository::InterfaceGenericRepository<
          DatabaseTest::Entities::DummyEntityWithForeign>,
      public DatabaseTest::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceDummyEntityWithForeignRepository()
    {
    }

    virtual Result<DatabaseTest::Entities::DummyEntityWithForeign> update(
        DatabaseTest::Entities::DummyEntityWithForeign &&entity) override = 0;
    virtual Result<DatabaseTest::Entities::DummyEntityWithForeign> getWithDetails(int entityId) = 0;

    virtual DatabaseTest::Entities::DummyEntityWithForeign::UniqueLoader fetchUniqueLoader() = 0;

    virtual DatabaseTest::Entities::DummyEntityWithForeign::OrderedListLoader fetchOrderedListLoader() = 0;

    virtual DatabaseTest::Entities::DummyEntityWithForeign::UnorderedListLoader fetchUnorderedListLoader() = 0;

    virtual Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<DatabaseTest::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) = 0;
};
} // namespace DatabaseTest::Contracts::Repository