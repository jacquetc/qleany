// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "signal_holder.h"
#include "front_ends_example_contracts_export.h"

namespace FrontEnds::Contracts::Repository
{
class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT InterfaceRepository
{

  public:
    virtual ~InterfaceRepository() = default;

    virtual SignalHolder *signalHolder() = 0;
};
} // namespace FrontEnds::Contracts::Repository