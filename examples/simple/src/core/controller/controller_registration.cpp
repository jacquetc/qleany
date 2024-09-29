// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "controller_registration.h"
#include "brand/brand_controller.h"
#include "car/car_controller.h"
#include "client/client_controller.h"
#include "custom/custom_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include "undo_redo/threaded_undo_redo_system.h"
#include "undo_redo/undo_redo_scopes.h"
#include <QSharedPointer>

using namespace Simple::Controller;

ControllerRegistration::ControllerRegistration(QObject *parent, InterfaceRepositoryProvider *repositoryProvider)
    : QObject{parent}
{

    auto dispatcher = QSharedPointer<EventDispatcher>(new EventDispatcher());

    // Undo Redo System
    Scopes scopes(QStringList() << "car"_L1
                                << "brand"_L1
                                << "passenger"_L1
                                << "client"_L1
                                << "custom"_L1);
    auto *undoRedoSystem = new Simple::Controller::UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    // error handling
    connect(undoRedoSystem, &Simple::Controller::UndoRedo::ThreadedUndoRedoSystem::errorSent, this,
            [dispatcher](Simple::Error error) {
                qDebug() << "Error in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                Q_EMIT dispatcher->error()->errorSent(error);
            });
    connect(undoRedoSystem, &Simple::Controller::UndoRedo::ThreadedUndoRedoSystem::warningSent, this,
            [dispatcher](Simple::Error error) {
                qDebug() << "Warning in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                Q_EMIT dispatcher->error()->warningSent(error);
            });

    // CarController

    new Car::CarController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *carSignalHolder = repositoryProvider->repository("Car")->signalHolder();

    // removal
    connect(carSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, dispatcher->car(),
            &CarSignals::removed);

    // active status
    connect(repositoryProvider->repository("car")->signalHolder(),
            &Simple::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->car(),
            &CarSignals::activeStatusChanged);

    // BrandController

    new Brand::BrandController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *brandSignalHolder = repositoryProvider->repository("Brand")->signalHolder();

    // removal
    connect(brandSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, dispatcher->brand(),
            &BrandSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(brandSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                CarRelationDTO dto(-1, CarRelationDTO::RelationField::Brand, removedIds, -1);
                Q_EMIT dispatcher->car()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("brand")->signalHolder(),
            &Simple::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->brand(),
            &BrandSignals::activeStatusChanged);

    // PassengerController

    new Passenger::PassengerController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *passengerSignalHolder = repositoryProvider->repository("Passenger")->signalHolder();

    // removal
    connect(passengerSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, dispatcher->passenger(),
            &PassengerSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(passengerSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                CarRelationDTO dto(-1, CarRelationDTO::RelationField::Passengers, removedIds, -1);
                Q_EMIT dispatcher->car()->relationRemoved(dto);
            });

    connect(passengerSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                ClientRelationDTO dto(-1, ClientRelationDTO::RelationField::Client, removedIds, -1);
                Q_EMIT dispatcher->client()->relationRemoved(dto);
            });

    connect(passengerSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                ClientRelationDTO dto(-1, ClientRelationDTO::RelationField::ClientFriends, removedIds, -1);
                Q_EMIT dispatcher->client()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("passenger")->signalHolder(),
            &Simple::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->passenger(),
            &PassengerSignals::activeStatusChanged);

    // ClientController

    new Client::ClientController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *clientSignalHolder = repositoryProvider->repository("Client")->signalHolder();

    // removal
    connect(clientSignalHolder, &Simple::Contracts::Repository::SignalHolder::removed, dispatcher->client(),
            &ClientSignals::removed);

    // active status
    connect(repositoryProvider->repository("client")->signalHolder(),
            &Simple::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->client(),
            &ClientSignals::activeStatusChanged);

    // CustomController

    new Custom::CustomController(repositoryProvider, undoRedoSystem, dispatcher);
}

ControllerRegistration::~ControllerRegistration()
{
}