// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "interface_repository.h"
#include "simple_example_contracts_export.h"
#include <QSharedPointer>

namespace Simple::Contracts::Repository
{
class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfaceRepositoryProvider
{
  public:
    virtual ~InterfaceRepositoryProvider()
    {
    }

    virtual void registerRepository(const char *name, InterfaceRepository *repository) = 0;
    virtual InterfaceRepository *repository(const char *name) = 0;
};
} // namespace Simple::Contracts::Repository