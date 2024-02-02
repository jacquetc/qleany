// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "interactor_export.h"

#include "custom/get_current_time_reply_dto.h"

#include <QObject>

namespace Simple::Interactor
{

using namespace Simple::Contracts::DTO::Custom;

class SIMPLEEXAMPLE_INTERACTOR_EXPORT CustomSignals : public QObject
{
    Q_OBJECT
  public:
    explicit CustomSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void writeRandomThingsChanged();
    void runLongOperationChanged();
    void closeSystemChanged();
    void getCurrentTimeReplied(GetCurrentTimeReplyDTO dto);
};
} // namespace Simple::Interactor