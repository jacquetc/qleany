#pragma once

#include "simple_example_interactor_export.h"
#include <QObject>
#include <qleany/common/error.h>

namespace Simple::Interactor
{

class SIMPLE_EXAMPLE_INTERACTOR_EXPORT ErrorSignals : public QObject
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
} // namespace Simple::Interactor