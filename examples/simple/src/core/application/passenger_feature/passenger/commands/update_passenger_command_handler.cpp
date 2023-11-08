// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_passenger_command_handler.h"
#include "passenger/validators/update_passenger_command_validator.h"
#include "repository/interface_passenger_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Commands;
using namespace Simple::Contracts::CQRS::Passenger::Validators;
using namespace Simple::Application::Features::Passenger::Commands;

UpdatePassengerCommandHandler::UpdatePassengerCommandHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<PassengerDTO> UpdatePassengerCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                           const UpdatePassengerCommand &request)
{
    Result<PassengerDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdatePassengerCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<PassengerDTO> UpdatePassengerCommandHandler::restore()
{
    Result<PassengerDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdatePassengerCommand restore:" << ex.what();
    }
    return result;
}

Result<PassengerDTO> UpdatePassengerCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                               const UpdatePassengerCommand &request)
{
    qDebug() << "UpdatePassengerCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdatePassengerCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(PassengerDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Simple::Domain::Passenger> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(PassengerDTO, currentResult)

        // map
        m_undoState = Result<PassengerDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<PassengerDTO, UpdatePassengerDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto passenger =
        Qleany::Tools::AutoMapper::AutoMapper::map<UpdatePassengerDTO, Simple::Domain::Passenger>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        passenger.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto passengerResult = m_repository->update(std::move(passenger));

    if (passengerResult.hasError())
    {
        return Result<PassengerDTO>(passengerResult.error());
    }

    // map
    auto passengerDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passengerResult.value());

    emit passengerUpdated(passengerDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit passengerDetailsUpdated(passengerDto.id());
    }

    qDebug() << "UpdatePassengerCommandHandler::handleImpl done";

    return Result<PassengerDTO>(passengerDto);
}

Result<PassengerDTO> UpdatePassengerCommandHandler::restoreImpl()
{
    qDebug() << "UpdatePassengerCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto passenger =
        Qleany::Tools::AutoMapper::AutoMapper::map<PassengerDTO, Simple::Domain::Passenger>(m_undoState.value());

    // do
    auto passengerResult = m_repository->update(std::move(passenger));

    QLN_RETURN_IF_ERROR(PassengerDTO, passengerResult)

    // map
    auto passengerDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passengerResult.value());

    emit passengerUpdated(passengerDto);

    qDebug() << "UpdatePassengerCommandHandler::restoreImpl done";

    return Result<PassengerDTO>(passengerDto);
}

bool UpdatePassengerCommandHandler::s_mappingRegistered = false;

void UpdatePassengerCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Passenger::UpdatePassengerDTO,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Passenger::UpdatePassengerDTO,
                                                           Simple::Domain::Passenger>();
}