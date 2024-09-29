// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "SIMPLE_EXAMPLE_PERSISTENCE_EXPORT"
#include "signal_holder.h"

namespace Simple::Contracts::Repository
{
class simple_example_persistence_export.h InterfaceRepository
{

  public:
    virtual ~InterfaceRepository() = default;

    virtual SignalHolder *signalHolder() = 0;
};
} // namespace Simple::Contracts::Repository