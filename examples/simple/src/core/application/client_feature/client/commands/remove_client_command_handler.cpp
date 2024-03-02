// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_client_command_handler.h"
#include "client/validators/remove_client_command_validator.h"
#include "repository/interface_client_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Commands;
using namespace Simple::Application::Features::Client::Commands;
using namespace Simple::Contracts::CQRS::Client::Validators;

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
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveClientCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveClientCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveClientCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const RemoveClientCommand &request)
{
    int clientId = request.id;

    // Validate the command using the validator
    auto validator = RemoveClientCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(clientId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<Simple::Entities::Client> clientResult = m_repository->get(clientId);

    QLN_RETURN_IF_ERROR(int, clientResult)

    // save old entity
    m_oldState = clientResult.value();

    auto deleteResult = m_repository->removeInCascade(QList<int>() << clientId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove signals
    // emit clientRemoved(deleteResult.value());

    qDebug() << "Client removed:" << clientId;

    return Result<int>(clientId);
}

Result<int> RemoveClientCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveClientCommandHandler::s_mappingRegistered = false;

void RemoveClientCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Entities::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
}