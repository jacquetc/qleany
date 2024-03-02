// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "repository/interface_brand_repository.h"
#include "simple_example_persistence_export.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Simple::Persistence::Repository
{

class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT BrandRepository final
    : public Qleany::Repository::GenericRepository<Simple::Entities::Brand>,
      public Simple::Contracts::Repository::InterfaceBrandRepository
{
  public:
    explicit BrandRepository(InterfaceDatabaseTableGroup<Simple::Entities::Brand> *brandDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Simple::Persistence::Repository