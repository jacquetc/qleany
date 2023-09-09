#include "controller_registration.h"
#include "car/car_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include "qleany/tools/undo_redo/threaded_undo_redo_system.h"
#include "qleany/tools/undo_redo/undo_redo_scopes.h"
#include <QSharedPointer>

using namespace Simple::Controller;

ControllerRegistration::ControllerRegistration(QObject *parent, InterfaceRepositoryProvider *repositoryProvider)
    : QObject{parent}
{

    auto dispatcher =
        QSharedPointer<Simple::Controller::EventDispatcher>(new Simple::Controller::EventDispatcher(nullptr));

    // Undo Redo System
    Scopes scopes(QStringList() << "car"
                                << "passenger"
                                << "brand"
                                << "client");
    auto *undoRedoSystem = new Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    // error handling
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::errorSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Error in undo redo system: " << error.status() << error.code() << error.message();
                emit dispatcher->error()->errorSent(error);
            });
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::warningSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Warning in undo redo system: " << error.status() << error.code() << error.message();
                emit dispatcher->error()->warningSent(error);
            });

    // PassengerController

    new Passenger::PassengerController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *passengerSignalHolder = repositoryProvider->repository("passenger")->signalHolder();
    emit passengerSignalHolder->removed(QList<int>() << 0);
    connect(passengerSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->passenger(),
            &Simple::Controller::PassengerSignals::removed);
    connect(repositoryProvider->repository("passenger")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->passenger(),
            &Simple::Controller::PassengerSignals::activeStatusChanged);

    // CarController
    new Car::CarController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    connect(repositoryProvider->repository("car")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->car(),
            &Simple::Controller::CarSignals::removed);
    connect(repositoryProvider->repository("car")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->car(),
            &Simple::Controller::CarSignals::activeStatusChanged);
}

ControllerRegistration::~ControllerRegistration()
{
}
