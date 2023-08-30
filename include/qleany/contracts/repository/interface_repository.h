#pragma once
#include "qleany/qleany_global.h"
#include "signal_holder.h"

namespace Qleany::Contracts::Repository
{
class QLEANY_EXPORT InterfaceRepository
{

  public:
    virtual ~InterfaceRepository() = default;

    virtual SignalHolder *signalHolder() = 0;
};
} // namespace Qleany::Contracts::Repository
