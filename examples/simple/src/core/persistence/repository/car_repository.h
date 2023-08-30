// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"

#include "repository/interface_brand_repository.h"

#include "repository/interface_passenger_repository.h"

#include "qleany/contracts/database/interface_database_table_group.h"
#include "persistence_export.h"
#include "qleany/repository/generic_repository.h"
#include "repository/interface_car_repository.h"

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Simple::Persistence::Repository
{

class SIMPLEEXAMPLE_PERSISTENCE_EXPORT CarRepository final
    : public Qleany::Repository::GenericRepository<Simple::Domain::Car>,
      public Simple::Contracts::Repository::InterfaceCarRepository
{
  public:
    explicit CarRepository(InterfaceDatabaseTableGroup<Simple::Domain::Car> *carDatabase,
                           InterfaceBrandRepository *brandRepository,
                           InterfacePassengerRepository *passengerRepository);

    SignalHolder *signalHolder() override;

    Result<Simple::Domain::Car> update(Simple::Domain::Car &&entity) override;
    Result<Simple::Domain::Car> getWithDetails(int entityId) override;

    Simple::Domain::Car::BrandLoader fetchBrandLoader() override;

    Simple::Domain::Car::PassengersLoader fetchPassengersLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceBrandRepository *m_brandRepository;

    InterfacePassengerRepository *m_passengerRepository;

    QScopedPointer<SignalHolder> m_signalHolder;
};

} // namespace Simple::Persistence::Repository
