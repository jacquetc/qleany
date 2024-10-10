// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QList>
#include <QObject>
#include "front_ends_example_contracts_export.h"

namespace FrontEnds::Contracts::Repository
{

class FRONT_ENDS_EXAMPLE_CONTRACTS_EXPORT SignalHolder : public QObject
{
    Q_OBJECT
  public:
    explicit SignalHolder(QObject *parent = nullptr);

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool active);
};

} // namespace FrontEnds::Contracts::Repository