// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "signal_holder.h"
#include "database_test_contracts_export.h"

namespace DatabaseTest::Contracts::Repository
{
class DATABASE_TEST_CONTRACTS_EXPORT InterfaceRepository
{

  public:
    virtual ~InterfaceRepository() = default;

    virtual SignalHolder *signalHolder() = 0;
};
} // namespace DatabaseTest::Contracts::Repository