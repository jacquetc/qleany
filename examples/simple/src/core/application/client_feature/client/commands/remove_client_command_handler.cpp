// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_client_command_handler.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_client_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Commands;
using namespace Simple::Application::Features::Client::Commands;

RemoveClientCommandHandler::RemoveClientCommandHandler(InterfaceClientRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveClientCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                               const RemoveClientCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveClientCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveClientCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveClientCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveClientCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const RemoveClientCommand &request)
{
    int clientId = request.id;

    Result<Simple::Domain::Client> clientResult = m_repository->get(clientId);

    if (Q_UNLIKELY(clientResult.hasError()))
    {
        qDebug() << "Error getting client from repository:" << clientResult.error().message();
        return Result<int>(clientResult.error());
    }

    // save old entity
    m_oldState = clientResult.value();

    auto deleteResult = m_repository->remove(clientId);

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting client from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit clientRemoved(deleteResult.value());

    qDebug() << "Client removed:" << clientId;

    return Result<int>(clientId);
}

Result<int> RemoveClientCommandHandler::restoreImpl()
{

    // Add the client to the repository
    auto clientResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(clientResult.hasError()))
    {
        return Result<int>(clientResult.error());
    }

    auto clientDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(clientResult.value());

    emit clientCreated(clientDTO);
    qDebug() << "Client added:" << clientDTO.id();

    // Return the UUID of the newly created client as a Result object
    return Result<int>(0);
}

bool RemoveClientCommandHandler::s_mappingRegistered = false;

void RemoveClientCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
}