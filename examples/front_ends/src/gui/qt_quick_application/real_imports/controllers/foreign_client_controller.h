// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "client/client_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace FrontEnds::Controller::Client;

class ForeignClientController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(ClientController)

  public:
    ForeignClientController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = ClientController::instance();
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

    Q_INVOKABLE CreateClientDTO getCreateDTO()
    {
        return s_controllerInstance->getCreateDTO();
    }

    Q_INVOKABLE UpdateClientDTO getUpdateDTO()
    {
        return s_controllerInstance->getUpdateDTO();
    }

    Q_INVOKABLE QCoro::QmlTask create(const CreateClientDTO &dto)
    {
        return s_controllerInstance->create(dto);
    }

    Q_INVOKABLE QCoro::QmlTask update(const UpdateClientDTO &dto)
    {
        return s_controllerInstance->update(dto);
    }

    Q_INVOKABLE QCoro::QmlTask remove(int id)
    {
        return s_controllerInstance->remove(id);
    }

  private:
    ClientController *s_controllerInstance = nullptr;
};