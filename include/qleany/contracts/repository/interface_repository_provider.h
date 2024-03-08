#include "interface_repository.h"
#include "qleany/qleany_export.h"
#include <QSharedPointer>

#pragma once
namespace Qleany::Contracts::Repository
{
class QLEANY_EXPORT InterfaceRepositoryProvider
{
  public:
    virtual ~InterfaceRepositoryProvider()
    {
    }

    virtual void registerRepository(const char *name, InterfaceRepository *repository) = 0;
    virtual InterfaceRepository *repository(const char *name) = 0;
};
} // namespace Qleany::Contracts::Repository
