// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "front_ends_example_contracts_export.h"
#include "repository/interface_generic_repository.h"
#include "repository/interface_repository.h"
#include "result.h"
#include <QObject>

using namespace FrontEnds;

namespace FrontEnds::Contracts::Repository
{

class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceCarRepository
    : public virtual FrontEnds::Contracts::Repository::InterfaceGenericRepository<FrontEnds::Entities::Car>,
      public FrontEnds::Contracts::Repository::InterfaceRepository
{
public:
    virtual ~InterfaceCarRepository()
    {
    }

    virtual Result<FrontEnds::Entities::Car> update(FrontEnds::Entities::Car &&entity) override = 0;
    virtual Result<FrontEnds::Entities::Car> getWithDetails(int entityId) = 0;

    virtual FrontEnds::Entities::Car::BrandLoader fetchBrandLoader() = 0;

    virtual FrontEnds::Entities::Car::PassengersLoader fetchPassengersLoader() = 0;

    virtual Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace FrontEnds::Contracts::Repository