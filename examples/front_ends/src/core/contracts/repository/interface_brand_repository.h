// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand.h"
#include "front_ends_example_contracts_export.h"
#include "repository/interface_generic_repository.h"
#include "repository/interface_repository.h"
#include "result.h"
#include <QObject>

using namespace FrontEnds;

namespace FrontEnds::Contracts::Repository
{

class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceBrandRepository
    : public virtual FrontEnds::Contracts::Repository::InterfaceGenericRepository<FrontEnds::Entities::Brand>,
      public FrontEnds::Contracts::Repository::InterfaceRepository
{
public:
    virtual ~InterfaceBrandRepository()
    {
    }

    virtual Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace FrontEnds::Contracts::Repository