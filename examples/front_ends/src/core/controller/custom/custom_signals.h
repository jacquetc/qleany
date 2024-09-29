// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "front_ends_example_controller_export.h"

#include "custom/get_current_time_reply_dto.h"

#include <QObject>

namespace FrontEnds::Controller
{

using namespace FrontEnds::Contracts::DTO::Custom;

class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT CustomSignals : public QObject
{
    Q_OBJECT
public:
    explicit CustomSignals(QObject *parent = nullptr)
        : QObject{parent}
    {
    }

Q_SIGNALS:
    void writeRandomThingsChanged();
    void runLongOperationChanged();
    void closeSystemChanged();
    void getCurrentTimeReplied(GetCurrentTimeReplyDTO dto);
};
} // namespace FrontEnds::Controller