// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger.h"

#include "persistence_export.h"
#include "repository/interface_passenger_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Simple::Persistence::Repository
{

class SIMPLEEXAMPLE_PERSISTENCE_EXPORT PassengerRepository final
    : public Qleany::Repository::GenericRepository<Simple::Domain::Passenger>,
      public Simple::Contracts::Repository::InterfacePassengerRepository
{
  public:
    explicit PassengerRepository(InterfaceDatabaseTableGroup<Simple::Domain::Passenger> *passengerDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Simple::Persistence::Repository