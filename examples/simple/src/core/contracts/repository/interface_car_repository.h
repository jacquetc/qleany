// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "simple_example_contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfaceCarRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Simple::Entities::Car>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceCarRepository()
    {
    }

    virtual Result<Simple::Entities::Car> update(Simple::Entities::Car &&entity) = 0;
    virtual Result<Simple::Entities::Car> getWithDetails(int entityId) = 0;

    virtual Simple::Entities::Car::BrandLoader fetchBrandLoader() = 0;

    virtual Simple::Entities::Car::PassengersLoader fetchPassengersLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace Simple::Contracts::Repository