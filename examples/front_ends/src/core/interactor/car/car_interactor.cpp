// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "car_interactor.h"

#include "car/commands/create_car_command.h"
#include "car/commands/create_car_command_handler.h"
#include "car/commands/remove_car_command.h"
#include "car/commands/remove_car_command_handler.h"
#include "car/commands/update_car_command.h"
#include "car/commands/update_car_command_handler.h"
#include "car/queries/get_all_car_query_handler.h"
#include "car/queries/get_car_query_handler.h"
#include "car/queries/get_car_with_details_query_handler.h"
// #include "car/commands/insert_car_into_xxx_command.h"
// #include "car/commands/update_car_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace FrontEnds::Interactor;
using namespace FrontEnds::Interactor::Car;
using namespace FrontEnds::Application::Features::Car::Commands;
using namespace FrontEnds::Application::Features::Car::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<CarInteractor> CarInteractor::s_instance = nullptr;

CarInteractor::CarInteractor(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                             QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance = this;
}

CarInteractor *CarInteractor::instance()
{
    return s_instance.data();
}

QCoro::Task<CarDTO> CarInteractor::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetCarQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));
        GetCarQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->car()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "car");

    // async wait for result signal
    const std::optional<CarDTO> optional_result =
        co_await qCoro(m_eventDispatcher->car(), &CarSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return CarDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<CarWithDetailsDTO> CarInteractor::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetCarQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));
        GetCarWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->car()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "car");

    // async wait for result signal
    const std::optional<CarWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->car(), &CarSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return CarWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<QList<CarDTO>> CarInteractor::getAll() const
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));
        GetAllCarQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->car()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "car");

    // async wait for result signal
    const std::optional<QList<CarDTO>> optional_result =
        co_await qCoro(m_eventDispatcher->car(), &CarSignals::getAllReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<CarDTO>() << CarDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<CarDTO> CarInteractor::create(const CreateCarDTO &dto)
{
    CreateCarCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));

    auto *handler = new CreateCarCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateCarCommandHandler::carCreated, m_eventDispatcher->car(), &CarSignals::created);

    QObject::connect(handler, &CreateCarCommandHandler::carRemoved, this,
                     [this](int removedId) { emit m_eventDispatcher->car()->removed(QList<int>() << removedId); });

    // Create specialized UndoRedoCommand
    auto command =
        new AlterCommand<CreateCarCommandHandler, CreateCarCommand>(CarInteractor::tr("Create car"), handler, query);

    // push command
    m_undo_redo_system->push(command, "car");

    // async wait for result signal
    const std::optional<CarDTO> optional_result =
        co_await qCoro(handler, &CreateCarCommandHandler::carCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return CarDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<CarDTO> CarInteractor::update(const UpdateCarDTO &dto)
{
    UpdateCarCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));

    auto *handler = new UpdateCarCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateCarCommandHandler::carUpdated, this,
                     [this](CarDTO dto) { emit m_eventDispatcher->car()->updated(dto); });
    QObject::connect(handler, &UpdateCarCommandHandler::carDetailsUpdated, m_eventDispatcher->car(),
                     &CarSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command =
        new AlterCommand<UpdateCarCommandHandler, UpdateCarCommand>(CarInteractor::tr("Update car"), handler, query);

    // push command
    m_undo_redo_system->push(command, "car");

    // async wait for result signal
    const std::optional<CarDTO> optional_result =
        co_await qCoro(handler, &UpdateCarCommandHandler::carUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return CarDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> CarInteractor::remove(int id)
{
    RemoveCarCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceCarRepository *>(m_repositoryProvider->repository("Car"));

    auto *handler = new RemoveCarCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command =
        new AlterCommand<RemoveCarCommandHandler, RemoveCarCommand>(CarInteractor::tr("Remove car"), handler, query);

    // push command
    m_undo_redo_system->push(command, "car");

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateCarDTO CarInteractor::getCreateDTO()
{
    return CreateCarDTO();
}

UpdateCarDTO CarInteractor::getUpdateDTO()
{
    return UpdateCarDTO();
}
