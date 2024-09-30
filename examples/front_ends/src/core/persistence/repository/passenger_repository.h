// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "database/interface_database_table_group.h"
#include "front_ends_example_persistence_export.h"
#include "passenger.h"
#include "repository/generic_repository.h"
#include "repository/interface_passenger_repository.h"
#include <QReadWriteLock>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::Database;
using namespace FrontEnds::Contracts::Repository;

namespace FrontEnds::Persistence::Repository
{

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT PassengerRepository final
    : public FrontEnds::Persistence::Repository::GenericRepository<FrontEnds::Entities::Passenger>,
      public FrontEnds::Contracts::Repository::InterfacePassengerRepository
{
public:
    explicit PassengerRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Passenger> *passengerDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) override;
    Result<QHash<FrontEnds::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace FrontEnds::Repository