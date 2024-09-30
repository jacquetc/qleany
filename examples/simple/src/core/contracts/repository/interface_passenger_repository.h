// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "passenger.h"
#include "repository/interface_generic_repository.h"
#include "repository/interface_repository.h"
#include "result.h"
#include "simple_example_contracts_export.h"
#include <QObject>

using namespace Simple;

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfacePassengerRepository
    : public virtual Simple::Contracts::Repository::InterfaceGenericRepository<Simple::Entities::Passenger>,
      public Simple::Contracts::Repository::InterfaceRepository
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