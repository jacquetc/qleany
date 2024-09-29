// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "car.h"
#include "database/interface_database_table_group.h"
#include "front_ends_example_persistence_export.h"
#include "repository/generic_repository.h"
#include "repository/interface_brand_repository.h"
#include "repository/interface_car_repository.h"
#include "repository/interface_passenger_repository.h"
#include <QReadWriteLock>

using namespace FrontEnds using namespace FrontEnds::Contracts::Database;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Persistence::Repository
{

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT CarRepository final : public FrontEnds::Persistence::Repository::GenericRepository<FrontEnds::Entities::Car>,
                                                                  public FrontEnds::Contracts::Repository::InterfaceCarRepository
{
public:
    explicit CarRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Car> *carDatabase,
                           InterfaceBrandRepository *brandRepository,
                           InterfacePassengerRepository *passengerRepository);

    SignalHolder *signalHolder() override;

    Result<FrontEnds::Entities::Car> update(FrontEnds::Entities::Car &&entity) override;
    Result<FrontEnds::Entities::Car> getWithDetails(int entityId) override;

    FrontEnds::Entities::Car::BrandLoader fetchBrandLoader() override;

    FrontEnds::Entities::Car::PassengersLoader fetchPassengersLoader() override;

    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

private:
    InterfaceBrandRepository *m_brandRepository;
    InterfacePassengerRepository *m_passengerRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace FrontEnds::Repository