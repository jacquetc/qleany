// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "custom/custom_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace FrontEnds::Controller::Custom;

class ForeignCustomController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(CustomController)

public:
    ForeignCustomController(QObject *parent = nullptr)
        : QObject(parent)
    {
        s_controllerInstance = CustomController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask getCurrentTime() const
    {
        return s_controllerInstance->getCurrentTime();
    }

    Q_INVOKABLE QCoro::QmlTask writeRandomThings(WriteRandomThingsDTO dto)
    {
        return s_controllerInstance->writeRandomThings(dto);
    }

    Q_INVOKABLE WriteRandomThingsDTO getWriteRandomThingsDTO()
    {
        return s_controllerInstance->getWriteRandomThingsDTO();
    }

    Q_INVOKABLE QCoro::QmlTask runLongOperation()
    {
        return s_controllerInstance->runLongOperation();
    }

    Q_INVOKABLE QCoro::QmlTask closeSystem()
    {
        return s_controllerInstance->closeSystem();
    }

private:
    CustomController *s_controllerInstance = nullptr;
};