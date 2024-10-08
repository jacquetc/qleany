// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_passenger_command_handler.h"
#include "passenger/validators/create_passenger_command_validator.h"
#include "tools/automapper.h"

#include "car.h"

using namespace Simple;
using namespace Simple::Entities;
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
    Simple::Entities::Passenger passenger;
    CreatePassengerDTO createDTO = request.req;

    QList<Simple::Entities::Passenger> ownerEntityPassengers;

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
        passenger = Simple::Tools::AutoMapper::map<CreatePassengerDTO, Simple::Entities::Passenger>(createDTO);

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

        auto originalOwnerPassengersResult =
            m_repository->getEntitiesInRelationOf(Car::schema, ownerId, "passengers"_L1);
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
    Result<QList<Simple::Entities::Passenger>> updateResult =
        m_repository->updateEntitiesInRelationOf(Car::schema, ownerId, "passengers"_L1, ownerEntityPassengers);

    QLN_RETURN_IF_ERROR_WITH_ACTION(PassengerDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = passengerResult;

    auto passengerDTO =
        Simple::Tools::AutoMapper::map<Simple::Entities::Passenger, PassengerDTO>(passengerResult.value());
    Q_EMIT passengerCreated(passengerDTO);

    // send an insertion signal
    Q_EMIT relationWithOwnerInserted(passenger.id(), ownerId, position);

    qDebug() << "Passenger added:" << passengerDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Passenger as a Result object
    return Result<PassengerDTO>(passengerDTO);
}

Result<PassengerDTO> CreatePassengerCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(QList<int>() << entityId);

    QLN_RETURN_IF_ERROR(PassengerDTO, deleteResult)

    Q_EMIT passengerRemoved(deleteResult.value().value(Simple::Entities::Entities::EntityEnum::Passenger).first());

    qDebug() << "Passenger removed:" << deleteResult.value();

    Q_EMIT relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<PassengerDTO>(PassengerDTO());
}

bool CreatePassengerCommandHandler::s_mappingRegistered = false;

void CreatePassengerCommandHandler::registerMappings()
{
    Simple::Tools::AutoMapper::registerMapping<Simple::Entities::Passenger, Contracts::DTO::Passenger::PassengerDTO>(
        true, true);
    Simple::Tools::AutoMapper::registerMapping<Contracts::DTO::Passenger::CreatePassengerDTO,
                                               Simple::Entities::Passenger>();
}