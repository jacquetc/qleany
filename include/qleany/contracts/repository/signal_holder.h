#pragma once

#include "qleany/qleany_global.h"
#include <QList>
#include <QObject>

namespace Qleany::Contracts::Repository
{

class QLEANY_EXPORT SignalHolder : public QObject
{
    Q_OBJECT
  public:
    explicit SignalHolder(QObject *parent = nullptr);

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool active);
};

} // namespace Qleany::Contracts::Repository
