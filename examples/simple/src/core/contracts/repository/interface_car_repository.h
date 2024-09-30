// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "repository/interface_generic_repository.h"
#include "repository/interface_repository.h"
#include "result.h"
#include "simple_example_contracts_export.h"
#include <QObject>

using namespace Simple;

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfaceCarRepository
    : public virtual Simple::Contracts::Repository::InterfaceGenericRepository<Simple::Entities::Car>,
      public Simple::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceCarRepository()
    {
    }

    virtual Result<Simple::Entities::Car> update(Simple::Entities::Car &&entity) override = 0;
    virtual Result<Simple::Entities::Car> getWithDetails(int entityId) = 0;

    virtual Simple::Entities::Car::BrandLoader fetchBrandLoader() = 0;

    virtual Simple::Entities::Car::PassengersLoader fetchPassengersLoader() = 0;

    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) = 0;
};
} // namespace Simple::Contracts::Repository