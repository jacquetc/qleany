// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_client_command_handler.h"
#include "client/validators/update_client_command_validator.h"
#include "qleany/tools/automapper/automapper.h"
#include "repository/interface_client_repository.h"

using namespace Qleany;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Commands;
using namespace Simple::Contracts::CQRS::Client::Validators;
using namespace Simple::Application::Features::Client::Commands;

UpdateClientCommandHandler::UpdateClientCommandHandler(InterfaceClientRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientDTO> UpdateClientCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const UpdateClientCommand &request)
{
    Result<ClientDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateClientCommand:" << ex.what();
    }
    return result;
}

Result<ClientDTO> UpdateClientCommandHandler::restore()
{
    Result<ClientDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateClientCommand restore:" << ex.what();
    }
    return result;
}

Result<ClientDTO> UpdateClientCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const UpdateClientCommand &request)
{
    qDebug() << "UpdateClientCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateClientCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<ClientDTO>(validatorResult.error());
    }

    // map
    auto client = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateClientDTO, Simple::Domain::Client>(request.req);

    // set update timestamp only on first pass
    if (m_newState.isEmpty())
    {
        client.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_newState.isEmpty())
    {
        Result<Simple::Domain::Client> saveResult = m_repository->get(request.req.id());

        if (Q_UNLIKELY(saveResult.hasError()))
        {
            qDebug() << "Error getting client from repository:" << saveResult.error().message();
            return Result<ClientDTO>(saveResult.error());
        }

        // map
        m_newState = Result<ClientDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(saveResult.value()));
    }

    // do
    auto clientResult = m_repository->update(std::move(client));

    if (clientResult.hasError())
    {
        return Result<ClientDTO>(clientResult.error());
    }

    // map
    auto clientDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(clientResult.value());

    emit clientUpdated(clientDto);

    qDebug() << "UpdateClientCommandHandler::handleImpl done";

    return Result<ClientDTO>(clientDto);
}

Result<ClientDTO> UpdateClientCommandHandler::restoreImpl()
{
    qDebug() << "UpdateClientCommandHandler::restoreImpl called with id" << m_newState.value().uuid();

    // map
    auto client = Qleany::Tools::AutoMapper::AutoMapper::map<ClientDTO, Simple::Domain::Client>(m_newState.value());

    // do
    auto clientResult = m_repository->update(std::move(client));

    if (Q_UNLIKELY(clientResult.hasError()))
    {
        return Result<ClientDTO>(clientResult.error());
    }

    // map
    auto clientDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(clientResult.value());

    emit clientUpdated(clientDto);

    qDebug() << "UpdateClientCommandHandler::restoreImpl done";

    return Result<ClientDTO>(clientDto);
}

bool UpdateClientCommandHandler::s_mappingRegistered = false;

void UpdateClientCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Client::UpdateClientDTO,
                                                           Simple::Domain::Client>();
}