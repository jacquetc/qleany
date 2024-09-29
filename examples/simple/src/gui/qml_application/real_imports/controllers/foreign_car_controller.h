// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "car/car_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Simple::Controller::Car;

class ForeignCarController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(CarController)

  public:
    ForeignCarController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = CarController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask getWithDetails(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask getAll() const
    {
        return s_controllerInstance->getAll();
    }

    Q_INVOKABLE CreateCarDTO getCreateDTO()
    {
        return s_controllerInstance->getCreateDTO();
    }

    Q_INVOKABLE UpdateCarDTO getUpdateDTO()
    {
        return s_controllerInstance->getUpdateDTO();
    }

    Q_INVOKABLE QCoro::QmlTask create(const CreateCarDTO &dto)
    {
        return s_controllerInstance->create(dto);
    }

    Q_INVOKABLE QCoro::QmlTask update(const UpdateCarDTO &dto)
    {
        return s_controllerInstance->update(dto);
    }

    Q_INVOKABLE QCoro::QmlTask remove(int id)
    {
        return s_controllerInstance->remove(id);
    }

  private:
    CarController *s_controllerInstance = nullptr;
};