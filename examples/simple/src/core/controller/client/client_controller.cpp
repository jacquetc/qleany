// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "client_controller.h"

#include "client/commands/create_client_command.h"
#include "client/commands/create_client_command_handler.h"
#include "client/commands/remove_client_command.h"
#include "client/commands/remove_client_command_handler.h"
#include "client/commands/update_client_command.h"
#include "client/commands/update_client_command_handler.h"
#include "client/queries/get_all_client_query_handler.h"
#include "client/queries/get_client_query_handler.h"
#include "client/queries/get_client_with_details_query_handler.h"
// #include "client/commands/insert_client_into_xxx_command.h"
// #include "client/commands/update_client_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Simple::Controller;
using namespace Simple::Controller::Client;
using namespace Simple::Application::Features::Client::Commands;
using namespace Simple::Application::Features::Client::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QScopedPointer<ClientController> ClientController::s_instance = QScopedPointer<ClientController>(nullptr);

ClientController::ClientController(InterfaceRepositoryProvider *repositoryProvider,
                                   ThreadedUndoRedoSystem *undo_redo_system,
                                   QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance.reset(this);
}

ClientController *ClientController::instance()
{
    return s_instance.data();
}

QCoro::Task<ClientDTO> ClientController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetClientQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));
        GetClientQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->client()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "client");

    // async wait for result signal
    const std::optional<ClientDTO> optional_result =
        co_await qCoro(m_eventDispatcher->client(), &ClientSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return ClientDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ClientWithDetailsDTO> ClientController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetClientQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));
        GetClientWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->client()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "client");

    // async wait for result signal
    const std::optional<ClientWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->client(), &ClientSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return ClientWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<QList<ClientDTO>> ClientController::getAll() const
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));
        GetAllClientQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->client()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "client");

    // async wait for result signal
    const std::optional<QList<ClientDTO>> optional_result =
        co_await qCoro(m_eventDispatcher->client(), &ClientSignals::getAllReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<ClientDTO>() << ClientDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ClientDTO> ClientController::create(const CreateClientDTO &dto)
{
    CreateClientCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));

    auto *handler = new CreateClientCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateClientCommandHandler::clientCreated, m_eventDispatcher->client(),
                     &ClientSignals::created);

    QObject::connect(handler, &CreateClientCommandHandler::clientRemoved, this,
                     [this](int removedId) { emit m_eventDispatcher->client()->removed(QList<int>() << removedId); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateClientCommandHandler, CreateClientCommand>(
        ClientController::tr("Create client"), handler, query);

    // push command
    m_undo_redo_system->push(command, "client");

    // async wait for result signal
    const std::optional<ClientDTO> optional_result =
        co_await qCoro(handler, &CreateClientCommandHandler::clientCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return ClientDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ClientDTO> ClientController::update(const UpdateClientDTO &dto)
{
    UpdateClientCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));

    auto *handler = new UpdateClientCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateClientCommandHandler::clientUpdated, this,
                     [this](ClientDTO dto) { emit m_eventDispatcher->client()->updated(dto); });
    QObject::connect(handler, &UpdateClientCommandHandler::clientDetailsUpdated, m_eventDispatcher->client(),
                     &ClientSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateClientCommandHandler, UpdateClientCommand>(
        ClientController::tr("Update client"), handler, query);

    // push command
    m_undo_redo_system->push(command, "client");

    // async wait for result signal
    const std::optional<ClientDTO> optional_result =
        co_await qCoro(handler, &UpdateClientCommandHandler::clientUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return ClientDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> ClientController::remove(int id)
{
    RemoveClientCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceClientRepository *>(m_repositoryProvider->repository("Client"));

    auto *handler = new RemoveClientCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveClientCommandHandler, RemoveClientCommand>(
        ClientController::tr("Remove client"), handler, query);

    // push command
    m_undo_redo_system->push(command, "client");

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateClientDTO ClientController::getCreateDTO()
{
    return CreateClientDTO();
}

UpdateClientDTO ClientController::getUpdateDTO()
{
    return UpdateClientDTO();
}
