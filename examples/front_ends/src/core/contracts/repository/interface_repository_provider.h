// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_contracts_export.h"
#include "interface_repository.h"
#include <QSharedPointer>

namespace FrontEnds::Contracts::Repository
{
class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceRepositoryProvider
{
public:
    virtual ~InterfaceRepositoryProvider()
    {
    }

    virtual void registerRepository(const char *name, InterfaceRepository *repository) = 0;
    virtual InterfaceRepository *repository(const char *name) = 0;
};
} // namespace FrontEnds::Contracts::Repository