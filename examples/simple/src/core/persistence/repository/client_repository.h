// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client.h"
#include "repository/interface_client_repository.h"
#include "repository/interface_passenger_repository.h"
#include "simple_example_persistence_export.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Simple::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Simple::Persistence::Repository
{

class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT ClientRepository final
    : public Qleany::Repository::GenericRepository<Simple::Entities::Client>,
      public Simple::Contracts::Repository::InterfaceClientRepository
{
  public:
    explicit ClientRepository(InterfaceDatabaseTableGroup<Simple::Entities::Client> *clientDatabase,
                              InterfacePassengerRepository *passengerRepository);

    SignalHolder *signalHolder() override;

    Result<Simple::Entities::Client> update(Simple::Entities::Client &&entity) override;
    Result<Simple::Entities::Client> getWithDetails(int entityId) override;

    Simple::Entities::Client::ClientLoader fetchClientLoader() override;

    Simple::Entities::Client::ClientFriendsLoader fetchClientFriendsLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfacePassengerRepository *m_passengerRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Simple::Persistence::Repository