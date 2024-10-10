// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QList>
#include <QObject>
#include "database_test_contracts_export.h"

namespace DatabaseTest::Contracts::Repository
{

class DATABASE_TEST_CONTRACTS_EXPORT SignalHolder : public QObject
{
    Q_OBJECT
  public:
    explicit SignalHolder(QObject *parent = nullptr);

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool active);
};

} // namespace DatabaseTest::Contracts::Repository