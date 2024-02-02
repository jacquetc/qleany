// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "custom/custom_interactor.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Simple::Interactor::Custom;

class ForeignCustomInteractor : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(CustomInteractor)

  public:
    ForeignCustomInteractor(QObject *parent = nullptr) : QObject(parent)
    {
        s_interactorInstance = CustomInteractor::instance();
    }

    Q_INVOKABLE QCoro::QmlTask getCurrentTime() const
    {
        return s_interactorInstance->getCurrentTime();
    }

    Q_INVOKABLE QCoro::QmlTask writeRandomThings(WriteRandomThingsDTO dto)
    {
        return s_interactorInstance->writeRandomThings(dto);
    }

    Q_INVOKABLE QCoro::QmlTask runLongOperation()
    {
        return s_interactorInstance->runLongOperation();
    }

    Q_INVOKABLE QCoro::QmlTask closeSystem()
    {
        return s_interactorInstance->closeSystem();
    }

  private:
    CustomInteractor *s_interactorInstance = nullptr;
};