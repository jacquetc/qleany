#pragma once

#include "common/error.h"
#include "front_ends_example_controller_export.h"
#include <QObject>

namespace FrontEnds::Controller
{

class FRONT_ENDS_EXAMPLE_CONTROLLER_EXPORT ErrorSignals : public QObject
{
    Q_OBJECT
public:
    explicit ErrorSignals(QObject *parent = nullptr)
        : QObject{parent}
    {
    }

Q_SIGNALS:
    void warningSent(const FrontEnds::Error &error);
    void errorSent(const FrontEnds::Error &error);
};
} // namespace FrontEnds::Controller