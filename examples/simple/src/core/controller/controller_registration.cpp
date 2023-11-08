#include "controller_registration.h"
#include "brand/brand_controller.h"
#include "car/car_controller.h"
#include "client/client_controller.h"
#include "custom/custom_controller.h"
#include "event_dispatcher.h"
#include "passenger/passenger_controller.h"
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>
#include <qleany/tools/undo_redo/undo_redo_scopes.h>

using namespace Simple::Controller;

ControllerRegistration::ControllerRegistration(QObject *parent, InterfaceRepositoryProvider *repositoryProvider)
    : QObject{parent}
{

    auto dispatcher = QSharedPointer<EventDispatcher>(new EventDispatcher(nullptr));

    // Undo Redo System
    Scopes scopes(QStringList() << "car"
                                << "brand"
                                << "passenger"
                                << "client"
                                << "custom");
    auto *undoRedoSystem = new Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    // error handling
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::errorSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Error in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                emit dispatcher->error()->errorSent(error);
            });
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::warningSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Warning in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                emit dispatcher->error()->warningSent(error);
            });

    // CarController

    new Car::CarController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *carSignalHolder = repositoryProvider->repository("Car")->signalHolder();

    // removal
    connect(carSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->car(),
            &CarSignals::removed);

    // active status
    connect(repositoryProvider->repository("car")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->car(),
            &CarSignals::activeStatusChanged);

    // BrandController

    new Brand::BrandController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *brandSignalHolder = repositoryProvider->repository("Brand")->signalHolder();

    // removal
    connect(brandSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->brand(),
            &BrandSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(brandSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                CarRelationDTO dto(-1, CarRelationDTO::RelationField::Brand, removedIds, -1);
                emit dispatcher->car()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("brand")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->brand(),
            &BrandSignals::activeStatusChanged);

    // PassengerController

    new Passenger::PassengerController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *passengerSignalHolder = repositoryProvider->repository("Passenger")->signalHolder();

    // removal
    connect(passengerSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->passenger(),
            &PassengerSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(passengerSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                CarRelationDTO dto(-1, CarRelationDTO::RelationField::Passengers, removedIds, -1);
                emit dispatcher->car()->relationRemoved(dto);
            });

    connect(passengerSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                ClientRelationDTO dto(-1, ClientRelationDTO::RelationField::Client, removedIds, -1);
                emit dispatcher->client()->relationRemoved(dto);
            });

    connect(passengerSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                ClientRelationDTO dto(-1, ClientRelationDTO::RelationField::ClientFriends, removedIds, -1);
                emit dispatcher->client()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("passenger")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->passenger(),
            &PassengerSignals::activeStatusChanged);

    // ClientController

    new Client::ClientController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *clientSignalHolder = repositoryProvider->repository("Client")->signalHolder();

    // removal
    connect(clientSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->client(),
            &ClientSignals::removed);

    // active status
    connect(repositoryProvider->repository("client")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->client(),
            &ClientSignals::activeStatusChanged);

    // CustomController

    new Custom::CustomController(nullptr, repositoryProvider, undoRedoSystem, dispatcher);
}

ControllerRegistration::~ControllerRegistration()
{
}