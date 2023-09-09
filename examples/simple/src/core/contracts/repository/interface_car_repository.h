// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "contracts_export.h"
#include "qleany/common/result.h"
#include "qleany/contracts/repository/interface_generic_repository.h"
#include "qleany/contracts/repository/interface_repository.h"
#include <QObject>

using namespace Qleany;

namespace Simple::Contracts::Repository
{

class SIMPLEEXAMPLE_CONTRACTS_EXPORT InterfaceCarRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Simple::Domain::Car>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceCarRepository()
    {
    }

    virtual Result<Simple::Domain::Car> update(Simple::Domain::Car &&entity) = 0;
    virtual Result<Simple::Domain::Car> getWithDetails(int entityId) = 0;

    virtual Simple::Domain::Car::BrandLoader fetchBrandLoader() = 0;

    virtual Simple::Domain::Car::PassengersLoader fetchPassengersLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace Simple::Contracts::Repository