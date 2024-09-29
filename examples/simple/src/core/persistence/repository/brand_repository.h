// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "database/interface_database_table_group.h"
#include "repository/generic_repository.h"
#include "repository/interface_brand_repository.h"
#include "simple_example_persistence_export.h"
#include <QReadWriteLock>

using namespace Simple using namespace Simple::Contracts::Database;
using namespace Simple::Contracts::Repository;

namespace Simple::Persistence::Repository
{

class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT BrandRepository final
    : public Simple::Persistence::Repository::GenericRepository<Simple::Entities::Brand>,
      public Simple::Contracts::Repository::InterfaceBrandRepository
{
  public:
    explicit BrandRepository(InterfaceDatabaseTableGroup<Simple::Entities::Brand> *brandDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids,
                                                                                                  bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Simple::Persistence::Repository