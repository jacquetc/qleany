// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "front_ends_example_persistence_export.h"
#include "repository/interface_brand_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace FrontEnds::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace FrontEnds::Persistence::Repository
{

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT BrandRepository final : public Qleany::Repository::GenericRepository<FrontEnds::Entities::Brand>,
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