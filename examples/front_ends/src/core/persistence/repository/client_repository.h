// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client.h"
#include "front_ends_example_persistence_export.h"
#include "repository/interface_client_repository.h"
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

class FRONT_ENDS_EXAMPLE_PERSISTENCE_EXPORT ClientRepository final
    : public Qleany::Repository::GenericRepository<FrontEnds::Entities::Client>,
      public FrontEnds::Contracts::Repository::InterfaceClientRepository
{
  public:
    explicit ClientRepository(InterfaceDatabaseTableGroup<FrontEnds::Entities::Client> *clientDatabase,
                              InterfacePassengerRepository *passengerRepository);

    SignalHolder *signalHolder() override;

    Result<FrontEnds::Entities::Client> update(FrontEnds::Entities::Client &&entity) override;
    Result<FrontEnds::Entities::Client> getWithDetails(int entityId) override;

    FrontEnds::Entities::Client::ClientLoader fetchClientLoader() override;

    FrontEnds::Entities::Client::ClientFriendsLoader fetchClientFriendsLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfacePassengerRepository *m_passengerRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace FrontEnds::Persistence::Repository