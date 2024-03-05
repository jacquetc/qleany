// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client.h"
#include "front_ends_example_contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace FrontEnds::Contracts::Repository
{

class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceClientRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<FrontEnds::Entities::Client>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceClientRepository()
    {
    }

    virtual Result<FrontEnds::Entities::Client> update(FrontEnds::Entities::Client &&entity) = 0;
    virtual Result<FrontEnds::Entities::Client> getWithDetails(int entityId) = 0;

    virtual FrontEnds::Entities::Client::ClientLoader fetchClientLoader() = 0;

    virtual FrontEnds::Entities::Client::ClientFriendsLoader fetchClientFriendsLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace FrontEnds::Contracts::Repository