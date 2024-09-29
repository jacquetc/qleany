#pragma once

#include "simple_example_controller_export.h"
#include <QObject>
#include <qleany/common/error.h>

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
    void warningSent(const Qleany::Error &error);
    void errorSent(const Qleany::Error &error);
};
} // namespace Simple::Controller