// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "database/interface_database_table_group.h"
#include "repository/generic_repository.h"
#include "repository/interface_brand_repository.h"
#include "repository/interface_car_repository.h"
#include "repository/interface_passenger_repository.h"
#include "simple_example_persistence_export.h"
#include <QReadWriteLock>

using namespace Simple;
using namespace Simple::Contracts::Database;
using namespace Simple::Contracts::Repository;

namespace Simple::Persistence::Repository
{

class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT CarRepository final
    : public Simple::Persistence::Repository::GenericRepository<Simple::Entities::Car>,
      public Simple::Contracts::Repository::InterfaceCarRepository
{
  public:
    explicit CarRepository(InterfaceDatabaseTableGroup<Simple::Entities::Car> *carDatabase,
                           InterfaceBrandRepository *brandRepository,
                           InterfacePassengerRepository *passengerRepository);

    SignalHolder *signalHolder() override;

    Result<Simple::Entities::Car> update(Simple::Entities::Car &&entity) override;
    Result<Simple::Entities::Car> getWithDetails(int entityId) override;

    Simple::Entities::Car::BrandLoader fetchBrandLoader() override;

    Simple::Entities::Car::PassengersLoader fetchPassengersLoader() override;

    Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids,
                                                                                                  bool active) override;

  private:
    InterfaceBrandRepository *m_brandRepository;
    InterfacePassengerRepository *m_passengerRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Simple::Persistence::Repository