// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_interactor_export.h"

#include "custom/get_current_time_reply_dto.h"

#include <QObject>

namespace FrontEnds::Interactor
{

using namespace FrontEnds::Contracts::DTO::Custom;

class FRONT_ENDS_EXAMPLE_INTERACTOR_EXPORT CustomSignals : public QObject
{
    Q_OBJECT
  public:
    explicit CustomSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void writeRandomThingsChanged();
    void runLongOperationChanged();
    void closeSystemChanged();
    void getCurrentTimeReplied(GetCurrentTimeReplyDTO dto);
};
} // namespace FrontEnds::Interactor