// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "interface_repository.h"
#include "database_test_contracts_export.h"
#include <QSharedPointer>

namespace DatabaseTest::Contracts::Repository
{
class DATABASE_TEST_CONTRACTS_EXPORT InterfaceRepositoryProvider
{
  public:
    virtual ~InterfaceRepositoryProvider()
    {
    }

    virtual void registerRepository(const char *name, InterfaceRepository *repository) = 0;
    virtual InterfaceRepository *repository(const char *name) = 0;
};
} // namespace DatabaseTest::Contracts::Repository