#pragma once

#include "common/error.h"
#include "simple_example_controller_export.h"
#include <QObject>

namespace Simple::Controller
{

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT ErrorSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ErrorSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  Q_SIGNALS:
    void warningSent(const Simple::Error &error);
    void errorSent(const Simple::Error &error);
};
} // namespace Simple::Controller