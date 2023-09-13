// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_client_command_handler.h"
#include "client/validators/create_client_command_validator.h"
#include "qleany/tools/automapper/automapper.h"

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Client::Validators;
using namespace Simple::Application::Features::Client::Commands;

CreateClientCommandHandler::CreateClientCommandHandler(InterfaceClientRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientDTO> CreateClientCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const CreateClientCommand &request)
{
    Result<ClientDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateClientCommand:" << ex.what();
    }
    return result;
}

Result<ClientDTO> CreateClientCommandHandler::restore()
{
    Result<ClientDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<ClientDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateClientCommand restore:" << ex.what();
    }
    return result;
}

Result<ClientDTO> CreateClientCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const CreateClientCommand &request)
{
    qDebug() << "CreateClientCommandHandler::handleImpl called";
    Simple::Domain::Client client;
    CreateClientDTO createDTO = request.req;

    if (m_firstPass)
    {
        // Validate the create Client command using the validator
        auto validator = CreateClientCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(ClientDTO, validatorResult);

        // Map the create Client command to a domain Client object and
        // generate a UUID
        client = Qleany::Tools::AutoMapper::AutoMapper::map<CreateClientDTO, Simple::Domain::Client>(createDTO);

        // allow for forcing the uuid
        if (client.uuid().isNull())
        {
            client.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        client.setCreationDate(QDateTime::currentDateTime());
        client.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        client = m_newEntity.value();
    }

    // Add the client to the repository

    m_repository->beginChanges();
    auto clientResult = m_repository->add(std::move(client));

    QLN_RETURN_IF_ERROR_WITH_ACTION(ClientDTO, clientResult, m_repository->cancelChanges();)

    // Get the newly created Client entity
    client = clientResult.value();
    // Save the newly created entity
    m_newEntity = clientResult;

    //  Manage relation to owner

    m_repository->saveChanges();

    m_newEntity = clientResult;

    auto clientDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Client, ClientDTO>(clientResult.value());
    emit clientCreated(clientDTO);

    qDebug() << "Client added:" << clientDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Client as a Result object
    return Result<ClientDTO>(clientDTO);
}

Result<ClientDTO> CreateClientCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newEntity.value().id());

    QLN_RETURN_IF_ERROR(ClientDTO, deleteResult)

    emit clientRemoved(deleteResult.value());

    qDebug() << "Client removed:" << deleteResult.value();

    return Result<ClientDTO>(ClientDTO());
}

bool CreateClientCommandHandler::s_mappingRegistered = false;

void CreateClientCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Client, Contracts::DTO::Client::ClientDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Client::CreateClientDTO,
                                                           Simple::Domain::Client>();
}