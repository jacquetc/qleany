// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "database/interface_database_table_group.h"
#include "front_ends_example_persistence_export.h"
#include "repository/generic_repository.h"
#include "repository/interface_brand_repository.h"
#include <QReadWriteLock>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::Database;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Persistence::Repository
{

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT BrandRepository final : public FrontEnds::Persistence::Repository::GenericRepository<FrontEnds::Entities::Brand>,
                                                                    public FrontEnds::Contracts::Repository::InterfaceBrandRepository
{
public:
    explicit BrandRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Brand> *brandDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace FrontEnds::Repository