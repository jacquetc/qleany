#include "passenger_controller.h"
#include "passenger/commands/create_passenger_command.h"
#include "passenger/commands/create_passenger_command_handler.h"
#include "passenger/commands/remove_passenger_command.h"
#include "passenger/commands/remove_passenger_command_handler.h"
#include "passenger/commands/update_passenger_command.h"
#include "passenger/commands/update_passenger_command_handler.h"
#include "passenger/queries/get_all_passenger_query_handler.h"
#include "passenger/queries/get_passenger_query_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Simple::Controller;
using namespace Simple::Controller::Passenger;
using namespace Simple::Application::Features::Passenger::Commands;
using namespace Simple::Application::Features::Passenger::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QScopedPointer<PassengerController> PassengerController::s_instance = QScopedPointer<PassengerController>(nullptr);

PassengerController::PassengerController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                         ThreadedUndoRedoSystem *undo_redo_system,
                                         QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{parent}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance.reset(this);
}

PassengerController *PassengerController::instance()
{
    return s_instance.data();
}

QCoro::Task<PassengerDTO> PassengerController::get(int id)
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([=](QPromise<Result<void>> &progressPromise) {
        GetPassengerQuery query;
        query.id = id;
        auto interface = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));
        GetPassengerQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->passenger()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "passenger");

    // async wait for result signal
    const std::optional<PassengerDTO> optional_result =
        co_await qCoro(m_eventDispatcher->passenger(), &PassengerSignals::getReplied, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return PassengerDTO();
    }

    co_return optional_result.value();
}

// QCoro::Task<PassengerWithDetailsDTO> PassengerController::getWithDetails(int id)
//{
//     auto queryCommand = new QueryCommand("getWithDetails");

//    queryCommand->setQueryFunction([=](QPromise<Result<void>> &progressPromise) {
//        GetPassengerWithDetailsQuery query;
//        query.id = id;
//        auto interface = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));
//        GetPassengerWithDetailsQueryHandler handler(interface);
//        auto result = handler.handle(progressPromise, query);

//        if (result.isSuccess())
//        {
//            emit m_eventDispatcher->passenger()->getWithDetailsReplied(result.value());
//        }
//        return Result<void>(result.error());
//    });

//    m_undo_redo_system->push(queryCommand, "passenger");

//    // async wait for result signal
//    const std::optional<PassengerWithDetailsDTO> optional_result =
//        co_await qCoro(m_eventDispatcher.get(), &PassengerSignals::getWithDetailsReplied,
//        std::chrono::milliseconds(200));

//    if (!optional_result.has_value())
//    {
//        // for now, I insert one invalid item to the list to show that there was an error
//        co_return PassengerWithDetailsDTO();
//    }

//    co_return optional_result.value();
//}

QCoro::Task<QList<PassengerDTO>> PassengerController::getAll()
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));
        GetAllPassengerQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->passenger()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "passenger");

    // async wait for result signal
    const std::optional<QList<PassengerDTO>> optional_result = co_await qCoro(
        m_eventDispatcher->passenger(), &PassengerSignals::getAllReplied, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<PassengerDTO>() << PassengerDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<PassengerDTO> PassengerController::create(const CreatePassengerDTO &dto)
{
    CreatePassengerCommand query;

    query.req = dto;

    auto repository = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("passenger"));

    auto *handler = new CreatePassengerCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreatePassengerCommandHandler::passengerCreated, m_eventDispatcher->passenger(),
                     &PassengerSignals::created);
    QObject::connect(handler, &CreatePassengerCommandHandler::passengerRemoved, this, [this](int removedId) {
        emit m_eventDispatcher->passenger()->removed(QList<int>() << removedId);
    });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreatePassengerCommandHandler, CreatePassengerCommand>(
        PassengerController::tr("Create passenger"), handler, query);

    // push command
    m_undo_redo_system->push(command, "passenger, car");

    // async wait for result signal
    const std::optional<PassengerDTO> optional_result =
        co_await qCoro(handler, &CreatePassengerCommandHandler::passengerCreated, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        co_return PassengerDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<PassengerDTO> PassengerController::update(const UpdatePassengerDTO &dto)
{
    UpdatePassengerCommand query;

    query.req = dto;

    auto repository = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));

    auto *handler = new UpdatePassengerCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdatePassengerCommandHandler::passengerUpdated, this,
                     [this](PassengerDTO dto) { emit m_eventDispatcher->passenger()->updated(dto); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdatePassengerCommandHandler, UpdatePassengerCommand>(
        PassengerController::tr("Update passenger"), handler, query);

    // push command
    m_undo_redo_system->push(command, "passenger");

    // async wait for result signal
    const std::optional<PassengerDTO> optional_result =
        co_await qCoro(handler, &UpdatePassengerCommandHandler::passengerUpdated, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        co_return PassengerDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> PassengerController::remove(int id)
{
    RemovePassengerCommand query;

    query.id = id;

    auto repository = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));

    auto *handler = new RemovePassengerCommandHandler(repository);

    // connect
    QObject::connect(handler, &RemovePassengerCommandHandler::passengerCreated, this,
                     [this](PassengerDTO dto) { emit m_eventDispatcher->passenger()->created(dto); });
    QObject::connect(handler, &RemovePassengerCommandHandler::passengerRemoved, this,
                     [this](int id) { emit m_eventDispatcher->passenger()->removed(QList<int>() << id); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemovePassengerCommandHandler, RemovePassengerCommand>(
        PassengerController::tr("Remove passenger"), handler, query);

    // push command
    m_undo_redo_system->push(command, "passenger");

    // async wait for result signal
    const std::optional<int> optional_result =
        co_await qCoro(handler, &RemovePassengerCommandHandler::passengerRemoved, std::chrono::milliseconds(200));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

QCoro::Task<bool> PassengerController::removeTree(int id)
{
    //    RemovePassengerTreeCommand query;

    //    query.id = id;

    //    auto repository = static_cast<InterfacePassengerRepository *>(m_repositoryProvider->repository("Passenger"));

    //    auto *handler = new RemovePassengerTreeCommandHandler(repository);

    //    // connect
    //    QObject::connect(handler, &RemovePassengerTreeCommandHandler::passengerCreated, this,
    //                     [this](PassengerDTO dto) { emit m_eventDispatcher->passenger()->created(dto); });
    //    // no need to connect to removed signal, because it will be emitted by the repository itself

    //    // Create specialized UndoRedoCommand
    //    auto command = new AlterCommand<RemovePassengerTreeCommandHandler, RemovePassengerTreeCommand>(
    //        PassengerController::tr("Remove passenger tree"), handler, query);

    //    // push command
    //    m_undo_redo_system->push(command, "passenger");

    //    // async wait for result signal
    //    const std::optional<QList<int>> optional_result =
    //        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(200));

    //    if (!optional_result.has_value())
    //    {
    //        co_return false;
    //    }

    co_return true;
}

CreatePassengerDTO PassengerController::getCreateDTO()
{
    return CreatePassengerDTO();
}

UpdatePassengerDTO PassengerController::getUpdateDTO()
{
    return UpdatePassengerDTO();
}
