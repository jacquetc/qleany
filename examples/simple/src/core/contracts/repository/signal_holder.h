// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "SIMPLE_EXAMPLE_PERSISTENCE_EXPORT"
#include <QList>
#include <QObject>

namespace Simple::Contracts::Repository
{

class simple_example_persistence_export.h SignalHolder : public QObject
{
    Q_OBJECT
  public:
    explicit SignalHolder(QObject *parent = nullptr);

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool active);
};

} // namespace Simple::Contracts::Repository