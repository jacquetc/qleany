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
        s_singletonInstance = CarController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_singletonInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask create(const CreateCarDTO &dto)
    {
        return s_singletonInstance->create(dto);
    }

    Q_INVOKABLE CreateCarDTO getCreateDTO()
    {
        return s_singletonInstance->getCreateDTO();
    }

    Q_INVOKABLE UpdateCarDTO getUpdateDTO()
    {
        return s_singletonInstance->getUpdateDTO();
    }

  private:
    Simple::Controller::Car::CarController *s_singletonInstance = nullptr;
};
