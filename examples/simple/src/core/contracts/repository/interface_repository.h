// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "signal_holder.h"
#include "simple_example_contracts_export.h"

namespace Simple::Contracts::Repository
{
class SIMPLE_EXAMPLE_CONTRACTS_EXPORT InterfaceRepository
{

  public:
    virtual ~InterfaceRepository() = default;

    virtual SignalHolder *signalHolder() = 0;
};
} // namespace Simple::Contracts::Repository