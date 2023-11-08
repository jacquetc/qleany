// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_passenger_command_handler.h"
#include "passenger/validators/create_passenger_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

#include "car.h"

using namespace Qleany;
using namespace Simple::Domain;
using namespace Simple::Contracts::DTO::Passenger;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Passenger::Validators;
using namespace Simple::Application::Features::Passenger::Commands;

CreatePassengerCommandHandler::CreatePassengerCommandHandler(InterfacePassengerRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<PassengerDTO> CreatePassengerCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                           const CreatePassengerCommand &request)
{
    Result<PassengerDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreatePassengerCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<PassengerDTO> CreatePassengerCommandHandler::restore()
{
    Result<PassengerDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<PassengerDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreatePassengerCommand restore:" << ex.what();
    }
    return result;
}

Result<PassengerDTO> CreatePassengerCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                               const CreatePassengerCommand &request)
{
    qDebug() << "CreatePassengerCommandHandler::handleImpl called";
    Simple::Domain::Passenger passenger;
    CreatePassengerDTO createDTO = request.req;

    QList<Simple::Domain::Passenger> ownerEntityPassengers;

    // Get the entities from owner
    int ownerId = createDTO.carId();
    m_ownerId = ownerId;

    if (m_firstPass)
    {
        // Validate the create Passenger command using the validator
        auto validator = CreatePassengerCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(PassengerDTO, validatorResult);

        // Map the create Passenger command to a domain Passenger object and
        // generate a UUID
        passenger =
            Qleany::Tools::AutoMapper::AutoMapper::map<CreatePassengerDTO, Simple::Domain::Passenger>(createDTO);

        // allow for forcing the uuid
        if (passenger.uuid().isNull())
        {
            passenger.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        passenger.setCreationDate(QDateTime::currentDateTime());
        passenger.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        passenger = m_newEntity.value();
    }

    // Add the passenger to the repository

    m_repository->beginChanges();
    auto passengerResult = m_repository->add(std::move(passenger));

    QLN_RETURN_IF_ERROR_WITH_ACTION(PassengerDTO, passengerResult, m_repository->cancelChanges();)

    // Get the newly created Passenger entity
    passenger = passengerResult.value();
    // Save the newly created entity
    m_newEntity = passengerResult;

    //  Manage relation to owner

    int position = -1;

    if (m_firstPass)
    {

        auto originalOwnerPassengersResult = m_repository->getEntitiesInRelationOf(Car::schema, ownerId, "passengers");
        if (Q_UNLIKELY(originalOwnerPassengersResult.hasError()))
        {
            return Result<PassengerDTO>(originalOwnerPassengersResult.error());
        }
        auto originalOwnerPassengers = originalOwnerPassengersResult.value();

        // save
        m_oldOwnerPassengers = originalOwnerPassengers;

        // Insert to the right position

        position = createDTO.position();
        if (position == -1)
        {
            position = originalOwnerPassengers.count();
        }
        if (position > originalOwnerPassengers.count())
        {
            position = originalOwnerPassengers.count();
        }
        else if (position < 0)
        {
            position = 0;
        }

        m_position = position;

        originalOwnerPassengers.insert(position, passenger);

        m_ownerPassengersNewState = originalOwnerPassengers;
        ownerEntityPassengers = originalOwnerPassengers;
    }
    else
    {
        ownerEntityPassengers = m_ownerPassengersNewState;
        position = m_position;
    }

    // Add the passenger to the owner entity
    Result<QList<Simple::Domain::Passenger>> updateResult =
        m_repository->updateEntitiesInRelationOf(Car::schema, ownerId, "passengers", ownerEntityPassengers);

    QLN_RETURN_IF_ERROR_WITH_ACTION(PassengerDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = passengerResult;

    auto passengerDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Simple::Domain::Passenger, PassengerDTO>(passengerResult.value());
    emit passengerCreated(passengerDTO);

    // send an insertion signal
    emit relationWithOwnerInserted(passenger.id(), ownerId, position);

    qDebug() << "Passenger added:" << passengerDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Passenger as a Result object
    return Result<PassengerDTO>(passengerDTO);
}

Result<PassengerDTO> CreatePassengerCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(PassengerDTO, deleteResult)

    emit passengerRemoved(deleteResult.value());

    qDebug() << "Passenger removed:" << deleteResult.value();

    emit relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<PassengerDTO>(PassengerDTO());
}

bool CreatePassengerCommandHandler::s_mappingRegistered = false;

void CreatePassengerCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Simple::Domain::Passenger,
                                                           Contracts::DTO::Passenger::PassengerDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Passenger::CreatePassengerDTO,
                                                           Simple::Domain::Passenger>();
}