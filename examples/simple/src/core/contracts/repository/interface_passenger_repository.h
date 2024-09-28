// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger.h"
#include "simple_example_contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfacePassengerRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Simple::Entities::Passenger>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfacePassengerRepository()
    {
    }

    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> remove(QList<int> ids) = 0;
    virtual Result<QHash<Simple::Entities::Entities::EntityEnum, QList<int>>> changeActiveStatusInCascade(
        QList<int> ids, bool active) = 0;
};
} // namespace Simple::Contracts::Repository