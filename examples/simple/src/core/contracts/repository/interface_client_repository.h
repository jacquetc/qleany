// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client.h"
#include "simple_example_contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfaceClientRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Simple::Entities::Client>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceClientRepository()
    {
    }

    virtual Result<Simple::Entities::Client> update(Simple::Entities::Client &&entity) override = 0;
    virtual Result<Simple::Entities::Client> getWithDetails(int entityId) = 0;

    virtual Simple::Entities::Client::ClientLoader fetchClientLoader() = 0;

    virtual Simple::Entities::Client::ClientFriendsLoader fetchClientFriendsLoader() = 0;

    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) = 0;
};
} // namespace Simple::Contracts::Repository