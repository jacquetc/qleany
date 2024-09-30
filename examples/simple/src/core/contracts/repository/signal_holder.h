// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QList>
#include <QObject>
#include "simple_example_contracts_export.h"

namespace Simple::Contracts::Repository
{

class SIMPLE_EXAMPLE_CONTRACTS_EXPORT SignalHolder : public QObject
{
    Q_OBJECT
  public:
    explicit SignalHolder(QObject *parent = nullptr);

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool active);
};

} // namespace Simple::Contracts::Repository