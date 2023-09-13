#pragma once

#include "controller_export.h"
#include "qleany/common/error.h"
#include <QObject>

namespace Simple::Controller
{

class SIMPLEEXAMPLE_CONTROLLER_EXPORT ErrorSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ErrorSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void warningSent(const Qleany::Error &error);
    void errorSent(const Qleany::Error &error);
};
} // namespace Simple::Controller