#pragma once

#include "interactor_export.h"
#include <QObject>
#include <qleany/common/error.h>

namespace Simple::Interactor
{

class SIMPLEEXAMPLE_INTERACTOR_EXPORT ErrorSignals : public QObject
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
} // namespace Simple::Interactor