// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_persistence_export.h"
#include "passenger.h"
#include "repository/interface_passenger_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace FrontEnds::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace FrontEnds::Persistence::Repository
{

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT PassengerRepository final
    : public Qleany::Repository::GenericRepository<FrontEnds::Entities::Passenger>,
      public FrontEnds::Contracts::Repository::InterfacePassengerRepository
{
  public:
    explicit PassengerRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Passenger> *passengerDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace FrontEnds::Persistence::Repository