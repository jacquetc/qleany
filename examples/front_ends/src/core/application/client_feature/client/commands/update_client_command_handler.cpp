// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_client_command_handler.h"
#include "client/validators/update_client_command_validator.h"
#include "repository/interface_client_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Client;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Client::Commands;
using namespace FrontEnds::Contracts::CQRS::Client::Validators;
using namespace FrontEnds::Application::Features::Client::Commands;

UpdateClientCommandHandler::UpdateClientCommandHandler(InterfaceClientRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ClientDTO> UpdateClientCommandHandler::handle(QPromise<Result<void>> &progressPromise, const UpdateClientCommand &request)
{
    Result<ClientDTO> result;

    try {
        result = handleImpl(progressPromise, request);
    } catch (const std::exception &ex) {
        result = Result<ClientDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateClientCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<ClientDTO> UpdateClientCommandHandler::restore()
{
    Result<ClientDTO> result;

    try {
        result = restoreImpl();
    } catch (const std::exception &ex) {
        result = Result<ClientDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateClientCommand restore:" << ex.what();
    }
    return result;
}

Result<ClientDTO> UpdateClientCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise, const UpdateClientCommand &request)
{
    qDebug() << "UpdateClientCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateClientCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(ClientDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty()) {
        Result<FrontEnds::Entities::Client> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(ClientDTO, currentResult)

        // map
        m_undoState = Result<ClientDTO>(FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Client, ClientDTO>(currentResult.value()));
    }
    auto updateDto = FrontEnds::Tools::AutoMapper::map<ClientDTO, UpdateClientDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto client = FrontEnds::Tools::AutoMapper::map<UpdateClientDTO, FrontEnds::Entities::Client>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty()) {
        client.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto clientResult = m_repository->update(std::move(client));

    if (clientResult.hasError()) {
        return Result<ClientDTO>(clientResult.error());
    }

    // map
    auto clientDto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Client, ClientDTO>(clientResult.value());

    Q_EMIT clientUpdated(clientDto);

    if (request.req.metaData().areDetailsSet()) {
        Q_EMIT clientDetailsUpdated(clientDto.id());
    }

    qDebug() << "UpdateClientCommandHandler::handleImpl done";

    return Result<ClientDTO>(clientDto);
}

Result<ClientDTO> UpdateClientCommandHandler::restoreImpl()
{
    qDebug() << "UpdateClientCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto client = FrontEnds::Tools::AutoMapper::map<ClientDTO, FrontEnds::Entities::Client>(m_undoState.value());

    // do
    auto clientResult = m_repository->update(std::move(client));

    QLN_RETURN_IF_ERROR(ClientDTO, clientResult)

    // map
    auto clientDto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Client, ClientDTO>(clientResult.value());

    Q_EMIT clientUpdated(clientDto);

    qDebug() << "UpdateClientCommandHandler::restoreImpl done";

    return Result<ClientDTO>(clientDto);
}

bool UpdateClientCommandHandler::s_mappingRegistered = false;

void UpdateClientCommandHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Client, Contracts::DTO::Client::ClientDTO>(true, true);
    FrontEnds::Tools::AutoMapper::registerMapping<Contracts::DTO::Client::UpdateClientDTO, Contracts::DTO::Client::ClientDTO>(true, true);
    FrontEnds::Tools::AutoMapper::registerMapping<Contracts::DTO::Client::UpdateClientDTO, FrontEnds::Entities::Client>();
}