#pragma once
#include "custom/custom_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Simple::Controller::Custom;

class ForeignCustomController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(CustomController)

  public:
    ForeignCustomController(QObject *parent = nullptr) : QObject(parent)
    {
        s_singletonInstance = CustomController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask runLongOperation() const
    {
        return s_singletonInstance->runLongOperation();
    }

  private:
    Simple::Controller::Custom::CustomController *s_singletonInstance = nullptr;
};
