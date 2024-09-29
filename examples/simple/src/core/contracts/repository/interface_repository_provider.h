// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "SIMPLE_EXAMPLE_PERSISTENCE_EXPORT"
#include "interface_repository.h"
#include <QSharedPointer>

namespace Simple::Contracts::Repository
{
class simple_example_persistence_export.h InterfaceRepositoryProvider
{
  public:
    virtual ~InterfaceRepositoryProvider()
    {
    }

    virtual void registerRepository(const char *name, InterfaceRepository *repository) = 0;
    virtual InterfaceRepository *repository(const char *name) = 0;
};
} // namespace Simple::Contracts::Repository